use std::collections::HashSet;

use crate::{models::ticket::Ticket, CustomContext};
use bson::{doc, oid::ObjectId};
use chrono::{DateTime, Utc};
use juniper::ID;
use mmt::{Create, Db, DB};
use serde::{Deserialize, Serialize};

use super::event::{Event, EventType};

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

    #[serde(default)]
    pub events: Vec<Event>,
}

impl Vehicle {
    async fn get_driver(&self, context: &CustomContext) -> Ticket {
        Ticket::get(&context.db, &self.driver_ticket).await.unwrap()
    }

    async fn get_request(&self, context: &CustomContext) -> Vec<Ticket> {
        Ticket::find_ids(&context.db, &self.requested_tickets).await
    }

    async fn get_tickets(&self, context: &CustomContext) -> Vec<Ticket> {
        Ticket::find(
            &context.db,
            doc! {
                "vehicle_id": &self.id
            },
        )
        .await
    }

    pub async fn get_by_event(context: &CustomContext, event_id: ObjectId) -> Option<Vehicle> {
        Vehicle::find_one(
            &context.db,
            doc! {
                "events._id": &event_id
            },
        )
        .await
    }
}

#[juniper::graphql_object(Context = CustomContext)]
impl Vehicle {
    // object: "Contact Details of the person making the purchase"

    fn id(&self) -> ID {
        ID::from(self.id.to_hex())
    }

    async fn driver(&self, context: &CustomContext) -> Ticket {
        self.get_driver(&context).await
    }

    fn rego(&self) -> &str {
        &self.rego
    }

    fn name(&self) -> &str {
        &self.name
    }

    async fn requests(&self, context: &CustomContext) -> Vec<Ticket> {
        self.get_request(&context).await
    }

    async fn tickets(&self, context: &CustomContext) -> Vec<Ticket> {
        self.get_tickets(&context).await
    }

    fn base(&self, base_idx: Option<i32>) -> Vec<Base> {
        match base_idx {
            Some(base_idx) => Base::get(&self, base_idx).into_iter().collect(),
            None => {
                let mut base_idxs: HashSet<i32> = self.events.iter().map(|e| e.base_idx).collect();
                base_idxs
                    .into_iter()
                    .filter_map(|idx| Base::get(&self, idx))
                    .collect()
            }
        }
    }
}

struct Base<'v>(Vec<&'v Event>, i32);
impl<'v> Base<'v> {
    fn get(vehicle: &'v Vehicle, base_idx: i32) -> Option<Self> {
        let mut events: Vec<&Event> = vehicle
            .events
            .iter()
            .filter(|e| e.base_idx == base_idx)
            .collect();
        events.sort_by(|a, b| a.time.cmp(&b.time));

        if !events.is_empty() {
            Some(Self(events, base_idx))
        } else {
            None
        }
    }

    fn check_in(&self) -> Option<&Event> {
        self.0
            .iter()
            .cloned()
            .filter(|e| e.event == EventType::CheckIn)
            .nth(0)
    }

    fn check_out(&self) -> Option<&Event> {
        self.0
            .iter()
            .cloned()
            .filter(|e| e.event == EventType::CheckOut)
            .nth(0)
    }

    fn all_points(&'v self) -> impl Iterator<Item = &'v Event> + '_ {
        self.0.iter().cloned().filter_map(|e| match &e.event {
            EventType::Points(_) => Some(e),
            _ => None,
        })
    }

    fn points(&self) -> i32 {
        self.all_points().fold(0, |i, e| match e.event {
            EventType::Points(p) => i + p,
            _ => unreachable!("Should not be receiving any events other than Points"),
        })
    }
}

#[juniper::graphql_object(Context = CustomContext)]
impl<'a> Base<'a> {
    fn idx(&self) -> i32 {
        self.1
    }

    fn events(&self) -> &Vec<&Event> {
        &self.0
    }

    fn check_in(&self) -> Option<&Event> {
        self.check_in()
    }

    fn check_out(&self) -> Option<&Event> {
        self.check_out()
    }

    fn all_points(&self) -> Vec<&Event> {
        self.all_points().collect()
    }

    fn points(&self) -> i32 {
        self.points()
    }
}
