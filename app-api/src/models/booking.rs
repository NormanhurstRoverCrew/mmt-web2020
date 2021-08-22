use crate::models::payment::{Payment, Transaction};
use crate::models::ticket::Ticket;
use crate::models::ticket::TICKET_PRICE;
use crate::models::user::User;
use crate::CustomContext;
use bson::{doc, oid::ObjectId};
use juniper::{FieldError, FieldResult, ID};
use mmt::{db::Delete, Create, Db, Update, DB};
use mongodb::results::UpdateResult;
use serde::{Deserialize, Serialize};
use std::{error::Error, str::FromStr};
use stripe::Client;
use stripe::{
    CreatePaymentIntent, Currency, PaymentIntent, PaymentIntentId, PaymentIntentStatus,
    PaymentIntentUpdateParams, PaymentMethod, PaymentMethodId,
};

#[DB(bookings)]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Booking {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    user_id: ObjectId,
    pub no: i32,

    #[serde(skip)]
    pub tickets: Vec<Ticket>,

    pub payment: Payment,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct BookingUserOnly {}

impl Default for Booking {
    fn default() -> Self {
        Self {
            id: ObjectId::new(),
            user_id: ObjectId::new(),
            no: 999999,
            tickets: vec![],
            payment: Payment::default(),
        }
    }
}

impl Booking {
    pub async fn get_tickets(&self, context: &CustomContext) -> Vec<Ticket> {
        let tickets: Vec<Ticket> = Ticket::search(
            &context.db,
            doc! {
                    "booking_id" : &self.id,
            },
        )
        .await;

        tickets
    }

    pub async fn get_user(&self, context: &CustomContext) -> User {
        User::get(&context.db, &self.user_id).await.unwrap()
    }

    async fn stripe_price(&self, context: &CustomContext) -> i64 {
        let n_tickets = self.get_tickets(context).await.len();
        n_tickets as i64 * (TICKET_PRICE * 100.0) as i64
    }

    pub async fn add_transaction(&mut self, context: &CustomContext, t: Transaction) {
        self.payment.transactions.push(t);
        self.update(&context.db).await.expect("DB Error");
    }

    pub async fn get_stripe_pi(&self, context: &CustomContext) -> Option<PaymentIntent> {
        let pis = self
            .payment
            .transactions
            .iter()
            .filter_map(|t| {
                if let Transaction::Stripe { pi_id, .. } = t {
                    Some(pi_id)
                } else {
                    None
                }
            })
            .filter_map(|t| PaymentIntentId::from_str(t).ok())
            .collect::<Vec<PaymentIntentId>>();

        for pi in pis {
            let spi = PaymentIntent::retrieve(&context.stripe, &pi, &[])
                .await
                .ok()?;

            match &spi.status {
                PaymentIntentStatus::Succeeded | PaymentIntentStatus::Canceled => continue,
                _ => return Some(spi),
            };
        }

        None
    }

    async fn price(&self, context: &CustomContext) -> f64 {
        let n_tickets = self.get_tickets(context).await.len();
        n_tickets as f64 * TICKET_PRICE
    }

    pub async fn amount_received(&self, context: &CustomContext) -> f64 {
        let mut sum = 0.0;
        for t in self.payment.transactions.iter() {
            sum += t.value(&context).await;
        }

        sum
    }

    pub async fn balence(&mut self, context: &CustomContext) -> f64 {
        let price = self.price(&context).await;
        let received = self.amount_received(&context).await;
        price - received
    }
}

#[juniper::graphql_object(Context = CustomContext)]
impl Booking {
    /// The root order. This holds all details on an order
    /// including contact, address and postage information

    fn id(&self) -> ID {
        ID::from(self.id.to_hex())
    }

    fn no(&self) -> i32 {
        self.no
    }

    /// Contact details
    async fn user(&self, context: &CustomContext) -> User {
        self.get_user(context).await
    }

    async fn tickets(&self, context: &CustomContext) -> Vec<Ticket> {
        self.get_tickets(context).await
    }

    async fn payment(&self, context: &CustomContext) -> Payment {
        let mut payment = self.payment.clone();
        payment.set_num_tickets(self.get_tickets(&context).await.len());
        payment
    }
}
