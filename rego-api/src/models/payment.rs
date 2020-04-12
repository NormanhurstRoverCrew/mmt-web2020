use crate::{
	db::{helpers as DBHelper, FromDoc},
	graphql::context::Database,
	models::{utils::*, Booking},
};
use juniper::ID;
use mongodb::{oid::ObjectId, Document};

#[derive(Clone, Debug)]
pub struct Payment {
	pub id : String,
	pub no : i32,
}

impl FromDoc for Payment {
	fn from_doc(item : &Document) -> Self {
		Self {
			id : doc_get_id(&item),
			no : doc_get_i32(&item, "no", 9999999),
		}
	}
}

impl Payment {
	pub fn init(db : &Database, booking_id : &ObjectId) {
		match db.bookings_handel().update_one(
			doc! {"_id" => booking_id.to_owned()},
			doc! {"$set" => {
				"payment" => {
					"transactions" => []
				},
			}},
			None,
		) {
			Ok(_) => {},
			Err(e) => {
				eprintln!("{}", e);
				panic!("could not add payment info to booking");
			},
		};
	}
}

graphql_object!(Payment: Database |&self| {
	description: "The root order. This holds all details on an order
including contact, address and postage information"

	field id() -> ID { ID::from(self.id.to_owned()) }
	field no() -> i32 { self.no}

});
