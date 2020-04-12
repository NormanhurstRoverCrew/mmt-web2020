use crate::{
	db::{helpers as DBHelper, FromDoc},
	graphql::context::Database,
	models::{utils::*, Booking, User, UserUpdate},
};
use juniper::ID;
use mongodb::{oid::ObjectId, Document};

#[derive(GraphQLInputObject, Clone, Debug)]
pub struct TicketUpdate {
	pub id :   String,
	pub user : UserUpdate,
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

	pub fn get_user(&self, db : &Database) -> Option<User> {
		DBHelper::find::<User>(
			&db.users_handel(),
			doc! {
				"_id" => ObjectId::with_string(&self.user_id).expect("User id could not be converted to ObjectID"),
			},
		)
	}

	pub fn delete(&self, db : &Database) -> bool {
		db.users_handel()
			.delete_one(
				doc! {
					"_id" => ObjectId::with_string(&self.user_id).expect("User id could not be converted to ObjectID"),
				},
				None,
			)
			.and_then(|_| {
				db.tickets_handel().delete_one(
					doc! {
						"_id" => ObjectId::with_string(&self.id).expect("User id could not be converted to ObjectID"),
					},
					None,
				)
			})
			.is_ok()
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

graphql_object!(Ticket: Database |&self| {
	// object: "Contact Details of the person making the purchase"

	field id() -> ID { ID::from(self.id.to_owned()) }

	field booking() -> Option<Booking> { self.get_booking() }

	field user(&exec) -> Option<User> { self.get_user(exec.context()) }
});
