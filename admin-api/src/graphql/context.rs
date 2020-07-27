use bson::doc;
use mongodb::{Collection, Database};
use std::sync::Arc;
use stripe::Client;

pub struct CustomContext {
	pub db :     Arc<Database>,
	pub stripe : Arc<Client>,
}

impl CustomContext {
	pub fn bookings_handel(&self) -> Collection { self.db.collection("bookings") }
	pub fn users_handel(&self) -> Collection { self.db.collection("users") }
	pub fn tickets_handel(&self) -> Collection { self.db.collection("tickets") }
	pub fn vehicles_handel(&self) -> Collection { self.db.collection("vehicles") }
	pub async fn index(&self, i : &str) -> i32 {
		let indexes = self.db.collection("indexes");
		loop {
			match indexes
				.find_one_and_update(
					doc! {
							"name" => i,
					},
					doc! {
							"$inc" => {
									"seq" => 1,
							}
					},
					None,
				)
				.await
			{
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
						.await
						.expect("Insert new index");
				},
			};
		}
	}
}

impl juniper::Context for CustomContext {}
