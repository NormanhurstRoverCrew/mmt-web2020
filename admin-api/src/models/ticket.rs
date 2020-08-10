use crate::{
	db::{Create, Db, Update},
	graphql::context::CustomContext,
	models::{Booking, User, UserUpdate},
};
use bson::{doc, oid::ObjectId, Document};
use juniper::ID;
use serde::{Deserialize, Serialize};

#[derive(GraphQLInputObject, Clone, Debug)]
pub struct TicketUpdate {
	pub id :   ObjectId,
	pub user : UserUpdate,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Ticket {
	#[serde(rename = "_id")]
	pub id :         ObjectId,
	user_id :        ObjectId,
	booking_id :     ObjectId,
	pub vehicle_id : Option<ObjectId>,
}

impl Db<'_> for Ticket {
	const COLLECTION : &'static str = "tickets";
}

impl Create for Ticket {
	const COLLECTION : &'static str = "tickets";
}

impl Update for Ticket {
	const COLLECTION : &'static str = "tickets";
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

	pub async fn get_user(&self, db : &CustomContext) -> Option<User> {
		User::get(&db, &self.user_id).await
	}
}

#[juniper::graphql_object(Context = CustomContext)]
impl Ticket {
	// object: "Contact Details of the person making the purchase"

	fn id(&self) -> ID { ID::from(self.id.to_hex()) }

	fn booking(&self) -> Option<Booking> { self.get_booking() }

	async fn user(&self, context : &CustomContext) -> Option<User> { self.get_user(context).await }
}
