use crate::{
	db::{helpers as DBHelper, FromDoc},
	graphql::context::CustomContext,
	models::{utils::*, BasicUser, Booking, User},
};
use bson::{doc, oid::ObjectId, Document};
use juniper::ID;
use serde::{Deserialize, Serialize};

pub const TICKET_PRICE : f64 = 30.0;

#[derive(GraphQLInputObject, Clone, Debug)]
pub struct TicketUpdate {
	pub id :   ObjectId,
	pub user : BasicUser,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Ticket {
	#[serde(rename = "_id")]
	pub id :     ObjectId,
	user_id :    ObjectId,
	booking_id : ObjectId,
}

impl Ticket {
	pub fn default() -> Self {
		Self {
			id :         ObjectId::new(),
			user_id :    ObjectId::new(),
			booking_id : ObjectId::new(),
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

	pub async fn get_user(&self, db : &CustomContext) -> Option<User> {
		DBHelper::find::<User>(
			&db.users_handel(),
			doc! {
				"_id" : &self.user_id
			},
		)
		.await
	}
}

#[juniper::graphql_object(Context = CustomContext)]
impl Ticket {
	// object: "Contact Details of the person making the purchase"

	fn id(&self) -> ID { ID::from(self.id.to_hex()) }

	fn booking(&self) -> Option<Booking> { self.get_booking() }

	async fn user(&self, context : &CustomContext) -> Option<User> { self.get_user(context).await }
}
