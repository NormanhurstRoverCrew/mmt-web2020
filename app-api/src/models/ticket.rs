use crate::CustomContext;
use bson::{doc, oid::ObjectId};
use juniper::ID;
use mmt::{Db, DB};
use serde::{Deserialize, Serialize};

use super::user::User;

#[DB(tickets)]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Ticket {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    user_id: ObjectId,
    booking_id: ObjectId,
    pub vehicle_id: Option<ObjectId>,
}

impl Ticket {
    async fn get_user(&self, context: &CustomContext) -> Option<User> {
        User::get(&context.db, &self.user_id).await
    }
}

#[juniper::graphql_object(Context = CustomContext)]
impl Ticket {
    // object: "Contact Details of the person making the purchase"

    fn id(&self) -> ID {
        ID::from(self.id.to_hex())
    }

    async fn user(&self, context: &CustomContext) -> Option<User> {
        self.get_user(context).await
    }
}
