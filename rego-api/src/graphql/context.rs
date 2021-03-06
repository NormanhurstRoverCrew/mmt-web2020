use bson::doc;
use mmt::email::email_client::EmailClient;
use mongodb::{Collection, Database};
use std::sync::Arc;
use stripe::Client;
use tonic::transport::Channel;

pub struct CustomContext {
	pub db :        Arc<Database>,
	pub stripe :    Arc<Client>,
	pub rpc_email : Arc<EmailClient<Channel>>,
}

impl CustomContext {
	pub fn bookings_handel(&self) -> Collection { self.db.collection("bookings") }
	pub fn users_handel(&self) -> Collection { self.db.collection("users") }
	pub fn tickets_handel(&self) -> Collection { self.db.collection("tickets") }
	pub async fn index(&self, i : &str) -> i32 {
		let indexes = self.db.collection("indexes");
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
					return doc.get_i32("seq").expect("Sequence Number");
				},
				_ => {
					indexes
						.insert_one(
							doc! {
								"name" : i,
								"seq" : 1,
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
