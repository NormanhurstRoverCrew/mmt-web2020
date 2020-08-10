use crate::{
	db::{Db, Create, Update},
	graphql::context::CustomContext,
	models::{Payment, Ticket, Transaction, User, TICKET_PRICE},
};
use bson::{doc, oid::ObjectId};
use juniper::{FieldError, FieldResult, ID};
use mongodb::results::UpdateResult;
use serde::{Deserialize, Serialize};
use std::{error::Error, str::FromStr};
use stripe::{
	CreatePaymentIntent, Currency, PaymentIntent, PaymentIntentStatus, PaymentIntentUpdateParams,
	PaymentMethod, PaymentMethodId,
};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Booking {
	#[serde(rename = "_id")]
	pub id :  ObjectId,
	user_id : ObjectId,
	pub no :  i32,

	#[serde(skip)]
	pub tickets : Vec<Ticket>,

	pub payment : Payment,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct BookingUserOnly {}

impl Default for Booking {
	fn default() -> Self {
		Self {
			id :      ObjectId::new(),
			user_id : ObjectId::new(),
			no :      999999,
			tickets : vec![],
			payment : Payment::default(),
		}
	}
}

impl Db<'_> for Booking {
    const COLLECTION : &'static str = "bookings";
}

impl Booking {
	// Create a new booking with a booking user and 1 ticket containing the booking
	// user.
	pub async fn create(db : &CustomContext, user : &User) -> Option<ObjectId> {
		let bookings = db.bookings_handel();
		let seq = db.index("bookings");

		let booking = Booking {
			user_id : user.id.clone(),
			no : seq.await,
			..Self::default()
		};

		match bson::to_bson(&booking) {
			Ok(bson::Bson::Document(doc)) => {
				let booking_id = bookings
					.insert_one(doc, None)
					.await
					.map(|b| b.inserted_id.as_object_id().unwrap().to_owned())
					.ok();

				if let Some(b) = &booking_id {
					let ticket = Ticket::new(b, &user.id);
					match bson::to_bson(&ticket) {
						Ok(bson::Bson::Document(doc)) => {
							db.tickets_handel().insert_one(doc, None).await.ok();
						},
						_ => {},
					}
				}

				match booking_id {
					Some(b) => Some(b),
					None => None,
				}
			},
			_ => None,
		}
	}

	pub async fn get_tickets(&self, context : &CustomContext) -> Vec<Ticket> {
		let tickets : Vec<Ticket> = Ticket::search(
			&context,
			doc! {
				"booking_id" : &self.id,
			},
		)
		.await;

		tickets
	}

	pub async fn get_user(&self, context : &CustomContext) -> User {
		User::get(&context, &self.user_id)
			.await
			.unwrap()
	}

	async fn payment_description(&self, context : &CustomContext) -> String {
		let n_tickets = self.get_tickets(context).await.len();
		format!("{} x Magical Mystery Tour 2020 Ticket", n_tickets)
	}

	async fn stripe_price(&self, context : &CustomContext) -> u64 {
		let n_tickets = self.get_tickets(context).await.len();
		n_tickets as u64 * (TICKET_PRICE * 100.0) as u64
	}

	pub async fn create_stripe_payment_intent(
		&mut self,
		context : &CustomContext,
	) -> Option<PaymentIntent> {
		let user = self.get_user(&context).await;

		let mut cpi = CreatePaymentIntent::new(self.stripe_price(&context).await, Currency::AUD);

		cpi.metadata = Some(
			[("booking_id".to_string(), self.id.to_string())]
				.iter()
				.cloned()
				.collect(),
		);

		cpi.receipt_email = Some(&user.email);

		let pd = self.payment_description(&context).await;
		cpi.description = Some(&pd);

		let pi = PaymentIntent::create(&context.stripe, cpi).await.ok();

		let id = pi.clone().map(|cpi| cpi.id.as_str().to_string())?;
		// self.update_stripe_payment_intent(&context, &id).await;

		self.add_transaction(&context, Transaction::stripe(id))
			.await;

		pi
	}

	pub async fn add_stripe_payment_method(
		&self,
		context : &CustomContext,
		pm_id : &str,
		pi_id : &str,
	) -> FieldResult<()> {
		let pm_id = PaymentMethodId::from_str(pm_id).unwrap();

		match PaymentMethod::retrieve(&context.stripe, &pm_id, &[]).await {
			Ok(_pm) => {},
			Err(_) => {
				return Err(FieldError::new(
					"Payment Method does not exist",
					graphql_value!({"type":"STRIPE_ERROR"}),
				));
			},
		};

		let _ = PaymentIntent::update(
			&context.stripe,
			&pi_id,
			PaymentIntentUpdateParams {
				amount :                  Some(self.stripe_price(&context).await),
				application_fee_amount :  None,
				currency :                None,
				customer :                None,
				description :             Some(&self.payment_description(&context).await),
				metadata :                None,
				payment_method :          Some(&pm_id),
				receipt_email :           None,
				save_source_to_customer : None,
				shipping :                None,
				source :                  None,
				transfer_group :          None,
			},
		)
		.await;

		Ok(())
	}

	pub async fn add_transaction(&mut self, context : &CustomContext, t : Transaction) {
		self.payment.transactions.push(t);
		self.commit(context).await.expect("DB Error");
	}

	pub async fn get_stripe_pi(&self, context : &CustomContext) -> Option<PaymentIntent> {
		let pis = self
			.payment
			.transactions
			.iter()
			.filter_map(|t| {
				if let Transaction::Stripe {
					pi_id, ..
				} = t
				{
					Some(pi_id)
				} else {
					None
				}
			})
			.collect::<Vec<&String>>();

		for pi in pis {
			let spi = PaymentIntent::retrieve(&context.stripe, &pi).await.ok()?;

			match &spi.status {
				PaymentIntentStatus::Succeeded | PaymentIntentStatus::Canceled => continue,
				_ => return Some(spi),
			};
		}

		None
	}

	pub async fn commit(&self, context : &CustomContext) -> Result<UpdateResult, Box<dyn Error>> {
		let doc = bson::to_bson(&self)
			.unwrap()
			.as_document()
			.unwrap()
			.to_owned();
		context
			.bookings_handel()
			.update_one(
				doc! {
					"_id" : &self.id,
				},
				doc,
				None,
			)
			.await
			.map_err(|e| e.into())
	}
}

#[juniper::graphql_object(Context = CustomContext)]
impl Booking {
	/// The root order. This holds all details on an order
	/// including contact, address and postage information

	fn id(&self) -> ID { ID::from(self.id.to_hex()) }

	fn no(&self) -> i32 { self.no }

	/// Contact details
	async fn user(&self, context : &CustomContext) -> User { self.get_user(context).await }

	async fn tickets(&self, context : &CustomContext) -> Vec<Ticket> {
		self.get_tickets(context).await
	}
}
