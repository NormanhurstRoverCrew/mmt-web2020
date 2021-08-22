use bson::doc;
use serde::{Serialize, Deserialize};
use mongodb::{Collection, Database};
use std::sync::Arc;
use stripe::Client;
use mmt::email::email_client::EmailClient;
use tonic::transport::Channel;

pub struct CustomContext {
	pub db :     Arc<Database>,
	pub stripe : Arc<Client>,
	pub email :  Arc<EmailClient<Channel>>,
}

impl CustomContext {
	pub fn bookings_handel(&self) -> Collection { self.db.collection("bookings") }
	pub fn users_handel(&self) -> Collection { self.db.collection("users") }
	pub fn tickets_handel(&self) -> Collection { self.db.collection("tickets") }
	pub fn vehicles_handel(&self) -> Collection { self.db.collection("vehicles") }
	pub async fn index(&self, i : &str) -> i32 {
		let indexes = self.db.collection::<Index>("indexes");
		loop {
			match indexes
				.find_one_and_update(
					doc! {
						"name" : i,
					},
					doc! {
						"$inc" : {
							"seq" : 1,
						}
					},
					None,
				)
				.await
			{
				Ok(Some(doc)) => {
					return doc.seq;
				},
				_ => {
					indexes
						.insert_one(
                            Index {name:i.to_owned(),
                            seq: 1},
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

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Index {
    name: String,
    seq:i32,
}
