use crate::CustomContext;
use bson::{doc, oid::ObjectId};
use juniper::ID;
use mmt::{Db, DB};
use serde::{Deserialize, Serialize};

use super::booking::Booking;
use super::user::User;
use super::vehicle::Vehicle;

pub const TICKET_PRICE: f64 = 30.0;

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

    async fn get_booking(&self, context: &CustomContext) -> Option<Booking> {
        Booking::get(&context.db, &self.booking_id).await
    }

    async fn get_vehicle(&self, context: &CustomContext) -> Option<Vehicle> {
        match &self.vehicle_id {
            Some(vid) => Vehicle::get(&context.db, vid).await,
            None => None,
        }
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

    async fn booking(&self, context: &CustomContext) -> Option<Booking> {
        self.get_booking(context).await
    }

    async fn vehicle(&self, context: &CustomContext) -> Option<Vehicle> {
        self.get_vehicle(context).await
    }
}
