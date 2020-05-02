use crate::db::PrimaryDb;
use mongodb::{coll::Collection, db::ThreadedDatabase};
use stripe::Client;

pub struct CustomContext {
	pub connection : PrimaryDb,
	pub stripe :     Client,
}

impl CustomContext {
	pub fn bookings_handel(&self) -> Collection { self.connection.collection("bookings") }
	pub fn users_handel(&self) -> Collection { self.connection.collection("users") }
	pub fn tickets_handel(&self) -> Collection { self.connection.collection("tickets") }
	pub fn index(&self, i : &str) -> i32 {
		let indexes = self.connection.collection("indexes");
		loop {
			match indexes.find_one_and_update(
				doc! {
					"name" => i,
				},
				doc! {
					"$inc" => {
						"seq" => 1,
					}
				},
				None,
			) {
				Ok(Some(doc)) => {
					return doc.get_i32("seq").expect("Sequence Number");
				},
				_ => {
					indexes
						.insert_one(
							doc! {
								"name" => i,
								"seq" => 1,
							},
							None,
						)
						.expect("Insert new index");
				},
			};
		}
	}
}

impl juniper::Context for CustomContext {}
