use stripe::Client;
use crate::{
	graphql::context::CustomContext,
	models::{Payment, Ticket, Transaction, User, TICKET_PRICE},
};
use bson::{doc, oid::ObjectId};
use juniper::{FieldError, FieldResult, ID};
use mmt::{Create, Db, Update, DB, db::Delete};
use mongodb::results::UpdateResult;
use serde::{Deserialize, Serialize};
use std::{error::Error, str::FromStr};
use stripe::{
	CreatePaymentIntent, Currency, PaymentIntent, PaymentIntentId, PaymentIntentStatus, PaymentIntentUpdateParams,
	PaymentMethod, PaymentMethodId,
};

#[DB(bookings)]
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

		let booking_id = booking.create(&context.db).await.ok();
		if let Some(b) = &booking_id {
			let ticket = Ticket::new(b, &user.id);
			ticket.create(&context.db).await.ok();
		}

		booking_id
	}

	pub async fn delete_booking(&self, context : &CustomContext) -> bool {
        for ticket in self.get_tickets(&context).await.iter() {
            if let Err(e) = ticket.delete(&context.db).await {
                return false;
            }
        }
        self.delete(&context.db).await.is_ok()
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

	async fn stripe_price(&self, context : &CustomContext) -> i64 {
		let n_tickets = self.get_tickets(context).await.len();
		n_tickets as i64 * (TICKET_PRICE * 100.0) as i64
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

	pub async fn add_transaction(&mut self, context : &CustomContext, t : Transaction) {
		self.payment.transactions.push(t);
		self.update(&context.db).await.expect("DB Error");
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
            .filter_map(|t| {
                PaymentIntentId::from_str(t).ok()
            })
			.collect::<Vec<PaymentIntentId>>();

		for pi in pis {
			let spi = PaymentIntent::retrieve(&context.stripe, &pi, &[]).await.ok()?;

			match &spi.status {
				PaymentIntentStatus::Succeeded | PaymentIntentStatus::Canceled => continue,
				_ => return Some(spi),
			};
		}

		None
	}

	async fn price(&self, context : &CustomContext) -> f64 {
		let n_tickets = self.get_tickets(context).await.len();
		n_tickets as f64 * TICKET_PRICE
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

	async fn payment(&self) -> Payment { self.payment.clone() }
}
