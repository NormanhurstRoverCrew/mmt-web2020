use super::Ticket;
use bson::doc;
use bson::oid::ObjectId;
use mmt::{Db, DB};
use mongodb::Database;
use serde::{Deserialize, Serialize};

#[DB(bookings)]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Booking {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    user_id: ObjectId,
    pub no: i32,
    // pub payment: Payment,
}

impl Booking {
    pub async fn get_tickets(&self, db: &Database) -> Vec<Ticket> {
        let tickets: Vec<Ticket> = Ticket::search(
            &db,
            doc! {
             "booking_id" : &self.id,
            },
        )
        .await;

        tickets
    }
}
