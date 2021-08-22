use crate::{
	graphql::context::CustomContext,
	models::{BasicUser, Booking, User},
};
use bson::{doc, oid::ObjectId};
use juniper::ID;
use mmt::{db::Delete, Db, DB};
use serde::{Deserialize, Serialize};

// pub const TICKET_PRICE : f64 = 30.0;
pub const TICKET_PRICE : f64 = 1.0;
pub const STRIPE_RATE : f64 = 0.0175; // %

#[derive(GraphQLInputObject, Clone, Debug)]
pub struct TicketUpdate {
	pub id :   ObjectId,
	pub user : BasicUser,
}

#[derive(GraphQLInputObject, Clone, Debug)]
pub struct BookingTicketUpdate {
	pub id :   Option<ObjectId>,
	pub user : BasicUser,
}

#[DB(tickets)]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Ticket {
	#[serde(rename = "_id")]
	pub id :         ObjectId,
	user_id :        ObjectId,
	booking_id :     ObjectId,
	pub vehicle_id : Option<ObjectId>,
}

impl Ticket {
	pub fn default() -> Self {
		Self {
			id :         ObjectId::new(),
			user_id :    ObjectId::new(),
			booking_id : ObjectId::new(),
			vehicle_id : None,
		}
	}

	pub fn new(booking_id : &ObjectId, user_id : &ObjectId) -> Self {
		Self {
			booking_id : booking_id.clone(),
			user_id : user_id.clone(),
			..Self::default()
		}
	}

	pub fn get_booking(&self) -> Option<Booking> {
		let _id = dbg!(&self.id);

		None
	}

	pub fn get_user_id(&self) -> &ObjectId { &self.user_id }

	pub fn get_user_id_opt(&self) -> Option<ObjectId> { Some(self.user_id.to_owned()) }

	pub async fn user(&self, context : &CustomContext) -> Option<User> {
		User::find_one(
			&context.db,
			doc! {
				"_id" : &self.user_id
			},
		)
		.await
	}

	pub async fn destroy(&self, context : &CustomContext) {
		if let Some(user) = self.user(&context).await {
			if let Err(e) = user.delete(&context.db).await {
            eprintln!("Could not delete user from db: {}",e);
            return;
            }
            
		}
		if let Err(e) = self.delete(&context.db).await {
            eprintln!("Could not delete ticket from db: {}",e);
        }
	}
}

#[juniper::graphql_object(Context = CustomContext)]
impl Ticket {
	// object: "Contact Details of the person making the purchase"

	fn id(&self) -> ID { ID::from(self.id.to_hex()) }

	fn booking(&self) -> Option<Booking> { self.get_booking() }

	async fn user(&self, context : &CustomContext) -> User { self.user(context).await.unwrap() }
}


