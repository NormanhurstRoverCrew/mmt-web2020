use crate::{
	graphql::context::CustomContext,
	models::{Payment, Ticket, Transaction, User, TICKET_PRICE, STRIPE_RATE},
};
use bson::{doc, oid::ObjectId};
use juniper::{FieldError, FieldResult, ID};
use mmt::{Create, Db, Update, DB};
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use stripe::{
	Client, CreatePaymentIntent, Currency, PaymentIntent, PaymentIntentStatus,
	 PaymentMethod, PaymentMethodId, UpdatePaymentIntent,
};

#[DB(bookings)]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Booking {
	#[serde(rename = "_id")]
	pub id :  ObjectId,
	user_id : ObjectId,
	pub no :  i32,

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
			payment : Payment::default(),
		}
	}
}

impl Booking {
	// Create a new booking with a booking user and 1 ticket containing the booking
	// user.
	pub async fn create(context : &CustomContext, user : &User) -> Option<ObjectId> {
		let seq = context.index("bookings");

		let booking = Booking {
			user_id : user.id.clone(),
			no : seq.await,
			..Self::default()
		};

		let booking_id = booking.create(&context.db).await.ok()?;

		Ticket::new(&booking_id, &user.id)
			.create(&context.db)
			.await
			.ok()
			.map(|_| booking_id)
	}

	pub async fn get_tickets(&self, context : &CustomContext) -> Vec<Ticket> {
		let tickets : Vec<Ticket> = Ticket::search(
			&context.db,
			doc! {
				"booking_id" : &self.id,
			},
		)
		.await;

		tickets
	}

	pub async fn get_user(&self, context : &CustomContext) -> User {
		User::get(&context.db, &self.user_id).await.unwrap()
	}

	async fn payment_description(&self, context : &CustomContext) -> String {
		let n_tickets = self.get_tickets(context).await.len();
		format!("{} x Magical Mystery Tour 2020 Ticket", n_tickets)
	}

	async fn price(&self, context : &CustomContext) -> f64 {
		let n_tickets = self.get_tickets(context).await.len();
		n_tickets as f64 * TICKET_PRICE
	}

	pub async fn create_stripe_payment_intent(
		&mut self,
		context : &CustomContext,
	) -> Option<PaymentIntent> {
		let user = self.get_user(&context).await;

        let balence = self.balence(&context).await;

        if balence < 0.01 {
            eprintln!("Balence 0");
            return None;
        }

        // Stripe Fee offset
        let price = (balence + 0.30) / (1.0 - STRIPE_RATE);
        let price = (price * 100.0) as i64;

		let mut cpi =
			CreatePaymentIntent::new(price, Currency::AUD);

		cpi.metadata = Some(
			[("booking_id".to_string(), self.id.to_string())]
				.iter()
				.cloned()
				.collect(),
		);

		let pd = self.payment_description(&context).await;
		cpi.description = Some(&pd);
		cpi.receipt_email = Some(&user.email);

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
			&pi_id.parse().unwrap(),
			UpdatePaymentIntent {
				amount : Some((self.price(&context).await / 100.0) as i64),
				application_fee_amount : None,
				currency : None,
				customer : None,
				description : Some(&self.payment_description(&context).await),
				metadata : None,
				payment_method : Some(pm_id),
				payment_method_data : None,
				payment_method_options : None,
				payment_method_types : None,
				receipt_email : None,
				shipping : None,
				transfer_group : None,
				setup_future_usage : None,
				statement_descriptor : None,
				statement_descriptor_suffix : None,
				transfer_data : None,
				expand : &[],
			},
		)
		.await;

		Ok(())
	}

	pub async fn add_transaction(&mut self, context : &CustomContext, t : Transaction) {
		self.payment.transactions.push(t);
		if let Ok(_) = dbg!(self.update(&context.db).await) {};
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
			let spi = PaymentIntent::retrieve(&context.stripe, &pi.parse().unwrap(), &[])
				.await
				.ok()?;

			match &spi.status {
				PaymentIntentStatus::Succeeded | PaymentIntentStatus::Canceled => continue,
				_ => return Some(spi),
			};
		}

		None
	}

	pub async fn amount_received(&self, client : &Client) -> f64 {
		let mut sum = 0.0;
		for t in self.payment.transactions.iter() {
			sum += t.value(&client).await;
		}

		sum
	}

	pub async fn balence(&self, context : &CustomContext) -> f64 {
		let (price, received) =
			futures::join!(self.price(&context), self.amount_received(&context.stripe));
		price - received
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


