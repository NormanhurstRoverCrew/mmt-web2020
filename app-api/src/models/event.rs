use crate::custom_context::CustomContext;
use mmt::Db;
use std::convert::TryInto;

use bson::{doc, oid::ObjectId};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::vehicle::Vehicle;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Event {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub time: DateTime<Utc>,

    pub base_idx: i32,

    // which normo person did this
    pub who: String,

    #[serde(flatten)]
    pub event: EventType,
}

impl Event {
    pub fn new(time: DateTime<Utc>, base_idx: i32, who: String, event: EventType) -> Self {
        Self {
            id: ObjectId::new(),
            time,
            base_idx,
            who,
            event,
        }
    }
}

#[juniper::graphql_object(Context = CustomContext)]
impl Event {
    fn id(&self) -> &ObjectId {
        &self.id
    }

    fn time(&self) -> &DateTime<Utc> {
        &self.time
    }

    fn base_idx(&self) -> i32 {
        self.base_idx
    }

    fn event_type(&self) -> &str {
        match &self.event {
            EventType::CheckIn => "CheckIn",
            EventType::CheckOut => "CheckOut",
            EventType::Points(_) => "Points",
            EventType::Comment(_) => "Comment",
        }
    }

    async fn vehicle(&self, context: &CustomContext) -> Option<Vehicle> {
        Vehicle::find_one(
            &context.db,
            doc! {
                "events._id": &self.id,
            },
        )
        .await
    }

    fn points(&self) -> Option<&i32> {
        self.event.points()
    }

    fn comment(&self) -> Option<&str> {
        self.event.comment()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(tag = "event_tag", content = "event_data")]
pub enum EventType {
    CheckIn,
    CheckOut,
    Points(i32),
    Comment(String),
}

impl EventType {
    pub fn points(&self) -> Option<&i32> {
        match &self {
            Self::Points(p) => Some(p),
            _ => None,
        }
    }

    pub fn comment(&self) -> Option<&str> {
        match &self {
            Self::Comment(c) => Some(c.as_str()),
            _ => None,
        }
    }
}

#[derive(juniper::GraphQLInputObject, Debug)]
pub struct EventInput {
    time: DateTime<Utc>,
    base_idx: i32,
    event_type: String,
    points: Option<i32>,
    comment: Option<String>,
}

impl TryInto<Event> for EventInput {
    type Error = String;

    fn try_into(self) -> Result<Event, Self::Error> {
        let Self {
            time,
            base_idx,
            event_type,
            points,
            comment,
        } = self;

        let event = {
            use EventType::*;
            match event_type.as_str() {
                "CheckIn" => CheckIn,
                "CheckOut" => CheckOut,
                "Points" => Points(points.ok_or(String::from("Must provide points value"))?),
                "Comment" => Comment(comment.ok_or(String::from("Must provide comment value"))?),
                _ => return Err(String::from("Unknown event type")),
            }
        };

        Ok(Event {
            id: ObjectId::new(),
            time,
            base_idx,
            who: "".into(),
            event,
        })
    }
}
