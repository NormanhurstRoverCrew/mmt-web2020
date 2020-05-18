use crate::{
	db::{helpers as DBHelper, FromDoc},
	graphql::{context::CustomContext, util::string_to_id},
	models::{utils::*, Payment, Ticket, User, TICKET_PRICE},
};
use bson::{doc, oid::ObjectId, Document};
use juniper::{FieldError, FieldResult, ID};
use std::str::FromStr;
use stripe::{
	CreatePaymentIntent, Currency, PaymentIntent, PaymentIntentUpdateParams, PaymentMethod,
	PaymentMethodId,
};
use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Booking {
    #[serde(rename = "_id")]
	pub id :                ObjectId,
	user_id : ObjectId,
	pub no :                i32,

    #[serde(skip)]
	pub tickets :           Vec<Ticket>,

	stripe_payment_intent : Option<String>,
    pub payment: Option<Payment>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct BookingUserOnly {
}

impl Booking {
	// Create a new booking with a booking user and 1 ticket containing the booking
	// user.
	pub async fn create(db : &CustomContext, user : &User) -> Option<ObjectId> {
		let bookings = db.bookings_handel();
		let seq = db.index("bookings");

		let booking_id = bookings
			.insert_one(
				doc! {
					"user_id" => &user.id,
					"no" => seq.await,
				},
				None,
			)
            .await
			.map(|b| b.inserted_id.as_object_id().unwrap().to_owned())
			.ok();

		if let Some(b) = &booking_id {
			let tickets = db.tickets_handel();
			tickets
				.insert_one(
					doc! {
						"booking_id" => b.to_owned(),
						"user_id" => &user.id,
					},
					None,
				)
                .await
				.ok();

			Payment::init(&db, &b).await;
		}

		match booking_id {
			Some(b) => Some(b),
			None => None,
		}
	}

	pub async fn get_tickets(&self, db : &CustomContext) -> Vec<Ticket> {
		let tickets = db.tickets_handel();

		let tickets : Vec<Ticket> = DBHelper::search::<Ticket>(
			&tickets,
			doc! {
				"booking_id" => &self.id,
			},
		).await;

		tickets
	}

	pub async fn get_user(&self, db : &CustomContext) -> User {
		DBHelper::get::<User>(
			&db.users_handel(),
			&self.user_id,
		)
            .await
		.unwrap()
	}

	pub async fn stripe_payment_intent(&self, db : &CustomContext) -> Option<PaymentIntent> {
		match &self.stripe_payment_intent {
			Some(spi) => PaymentIntent::retrieve(&db.stripe, &spi).await.ok(),
			None => None,
		}
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

		let id = pi.clone()
			.map(|cpi| cpi.id.as_str().to_string())?;
			self.update_stripe_payment_intent(&context, &id).await;

		pi
	}

	async fn update_stripe_payment_intent(&mut self, context : &CustomContext, id : &str) -> Option<()> {
		context
			.bookings_handel()
			.update_one(
				doc! {
					"_id" => &self.id,
				},
				doc! {
					"$set" => {
						"stripe_payment_intent" => id,
					}
				},
				None,
			)
            .await
			.ok()
			.map(|_| {
                self.stripe_payment_intent = Some(id.to_string());
                ()
            })
	}

	pub async fn add_stripe_payment_method(
		&self,
		context : &CustomContext,
		pm_id : &str,
	) -> FieldResult<()> {
		match &self.stripe_payment_intent {
			Some(pi_id) => {
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
			},
			None => Ok(()), // Dont error. we will create the pi later
		}
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

	async fn tickets(&self, context : &CustomContext) -> Vec<Ticket> { self.get_tickets(context).await }
}
