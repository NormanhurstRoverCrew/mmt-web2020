use crate::models::Ticket;
use bson::{doc, oid::ObjectId};
use mmt::{Db, DB};
use mongodb::Database;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize)]
pub struct NewVehicle {
    pub rego: String,
    pub driver_ticket: ObjectId,
}

#[DB(vehicles)]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Vehicle {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub rego: String,
    pub name: String,
    pub driver_ticket: ObjectId,

    #[serde(default)]
    pub requested_tickets: Vec<ObjectId>,
}

#[allow(unused)]
impl Vehicle {
    pub fn new(rego: String, name: String, driver: &Ticket) -> Option<Self> {
        match rego {
            rego if rego.len() <= 6 => Some(Self {
                id: ObjectId::new(),
                rego,
                name,
                driver_ticket: driver.id.clone(),
                requested_tickets: vec![],
            }),
            _ => None,
        }
    }

    pub async fn get_driver(&self, db: &Database) -> Ticket {
        Ticket::get(&db, &self.driver_ticket).await.unwrap()
    }

    async fn get_tickets(&self, db: &Database) -> Vec<Ticket> {
        Ticket::find(
            &db,
            doc! {
                "vehicle_id": &self.id
            },
        )
        .await
    }
}
