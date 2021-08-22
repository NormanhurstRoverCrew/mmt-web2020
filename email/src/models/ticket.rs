use crate::models::User;
use bson::{doc, oid::ObjectId};
use mmt::{Db, DB};
use mongodb::Database;
use serde::{Deserialize, Serialize};

#[DB(tickets)]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Ticket {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub user_id: ObjectId,
    booking_id: ObjectId,
    pub vehicle_id: Option<ObjectId>,
}

impl Ticket {
    pub async fn user(&self, db: &Database) -> Option<User> {
        User::find_one(
            &db,
            doc! {
                "_id" : &self.user_id
            },
        )
        .await
    }
}
