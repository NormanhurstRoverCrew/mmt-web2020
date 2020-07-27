use crate::{
	db::Db,
	graphql::context::CustomContext,
	models::{Booking, Ticket, User, UserUpdate},
};
use bson::{doc, oid::ObjectId, Document};
use juniper::ID;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Vehicle {
	#[serde(rename = "_id")]
	pub id :            ObjectId,
	pub rego :          String,
	pub driver_ticket : ObjectId,
}

impl Db<'_> for Vehicle {
	const COLLECTION : &'static str = "vehicles";
}

impl Vehicle {
	pub fn new(rego : String, driver : &Ticket) -> Option<Self> {
		match rego {
			rego if rego.len() <= 6 => Some(Self {
				id : ObjectId::new().unwrap(),
				rego,
				driver_ticket : driver.id.clone(),
			}),
			_ => None,
		}
	}

	async fn get_driver(&self, db : &CustomContext) -> Ticket {
		Ticket::get(&db, &self.driver_ticket).await.unwrap()
	}
}

#[juniper::graphql_object(Context = CustomContext)]
impl Vehicle {
	// object: "Contact Details of the person making the purchase"

	fn id(&self) -> ID { ID::from(self.id.to_hex()) }

	async fn driver(&self, context : &CustomContext) -> Ticket { self.get_driver(&context).await }
}
