use crate::{
	graphql::context::CustomContext,
	models::Ticket,
};
use mmt::{DB, Create, Db, Update};
use bson::{doc, oid::ObjectId};
use juniper::ID;
use serde::{Deserialize, Serialize};

#[derive(GraphQLInputObject, Clone, Debug, Serialize)]
pub struct NewVehicle {
	pub rego :          String,
	pub driver_ticket : ObjectId,
}

impl Create for NewVehicle {
    const COLLECTION: &'static str = "vehicles";
}

#[DB(vehicles)]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Vehicle {
	#[serde(rename = "_id")]
	pub id :            ObjectId,
	pub rego :          String,
	pub driver_ticket : ObjectId,

    #[serde(default)]
    pub requested_tickets: Vec<ObjectId>,
}

impl Vehicle {
	pub fn new(rego : String, driver : &Ticket) -> Option<Self> {
		match rego {
			rego if rego.len() <= 6 => Some(Self {
				id : ObjectId::new(),
				rego,
				driver_ticket : driver.id.clone(),
                requested_tickets: vec![],
			}),
			_ => None,
		}
	}

	async fn get_driver(&self, context : &CustomContext) -> Ticket {
		Ticket::get(&context.db, &self.driver_ticket).await.unwrap()
	}

    async fn get_request(&self, context: &CustomContext) -> Vec<Ticket> {
        Ticket::find_ids(&context.db, &self.requested_tickets).await
    }

    async fn get_tickets(&self, context: &CustomContext) -> Vec<Ticket> {
        Ticket::find(&context.db, doc! {
            "vehicle_id": &self.id
        }).await
    }
}

#[juniper::graphql_object(Context = CustomContext)]
impl Vehicle {
	// object: "Contact Details of the person making the purchase"

	fn id(&self) -> ID { ID::from(self.id.to_hex()) }

	async fn driver(&self, context : &CustomContext) -> Ticket { self.get_driver(&context).await }

	fn rego(&self) -> &str { &self.rego }
    
    async fn requests(&self, context : &CustomContext) -> Vec<Ticket> { self.get_request(&context).await }
    
    async fn tickets(&self, context : &CustomContext) -> Vec<Ticket> { self.get_tickets(&context).await }
}
