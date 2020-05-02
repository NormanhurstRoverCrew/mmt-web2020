use crate::{
	db::{helpers as DBHelper, FromDoc},
	graphql::context::CustomContext,
	models::{utils::*, BasicUser, Booking, User},
};
use juniper::ID;
use mongodb::{oid::ObjectId, Document};

#[derive(GraphQLInputObject, Clone, Debug)]
pub struct TicketUpdate {
	pub id :   String,
	pub user : BasicUser,
}

#[derive(Clone, Debug)]
pub struct Ticket {
	pub id :  String,
	user_id : String,
}

impl Ticket {
	pub fn default() -> Self {
		Self {
			id :      String::default(),
			user_id : String::default(),
		}
	}

	pub fn get_booking(&self) -> Option<Booking> {
		let _id = dbg!(&self.id);

		None
	}

	pub fn get_user_id(&self) -> ObjectId {
		ObjectId::with_string(&self.user_id).expect("User id could not be converted to ObjectID")
	}

	pub fn get_user_id_opt(&self) -> Option<ObjectId> { ObjectId::with_string(&self.user_id).ok() }

	pub fn get_user(&self, db : &CustomContext) -> Option<User> {
		DBHelper::find::<User>(
			&db.users_handel(),
			doc! {
				"_id" => ObjectId::with_string(&self.user_id).expect("User id could not be converted to ObjectID"),
			},
		)
	}
}

impl FromDoc for Ticket {
	fn from_doc(item : &Document) -> Self {
		Self {
			id :      doc_get_id(item),
			user_id : doc_get_id_key(item, "user_id"),
		}
	}
}

#[juniper::graphql_object(Context = CustomContext)]
impl Ticket {
	// object: "Contact Details of the person making the purchase"

	fn id(&self) -> ID { ID::from(self.id.to_owned()) }

	fn booking(&self) -> Option<Booking> { self.get_booking() }

	fn user(&self, context : &CustomContext) -> Option<User> { self.get_user(context) }
}
