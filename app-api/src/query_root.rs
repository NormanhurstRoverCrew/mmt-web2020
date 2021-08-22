use crate::{
    custom_context::CustomContext,
    models::{admin_user::AdminUser, event::Event, ticket::Ticket, vehicle::Vehicle},
};
use bson::{doc, oid::ObjectId};
use mmt::Db;

pub struct QueryRoot;

#[juniper::graphql_object(
    Context = CustomContext,
)]
impl QueryRoot {
    fn ping() -> &str {
        "pong"
    }

    fn root() -> Root {
        Root
    }

    async fn ticket(context: &CustomContext, id: ObjectId) -> Option<Ticket> {
        Ticket::get(&context.db, &id).await
    }

    async fn vehicle(context: &CustomContext, id: ObjectId) -> Option<Vehicle> {
        Vehicle::get(&context.db, &id).await
    }

    async fn events(context: &CustomContext, base_idx: Option<i32>) -> Vec<Event> {
        match base_idx {
            Some(base_idx) => {
                Vehicle::find(
                    &context.db,
                    doc! {
                        "events.base_idx":base_idx
                    },
                )
                .await
            }
            None => Vehicle::all(&context.db).await,
        }
        .into_iter()
        .map(|v| v.events)
        .flatten()
        .collect()
    }
}

struct Root;

#[juniper::graphql_object(
    Context = CustomContext,
)]
impl Root {
    fn user(context: &CustomContext) -> &AdminUser {
        &context.admin_user
    }

    async fn tickets(context: &CustomContext) -> Vec<Ticket> {
        Ticket::all(&context.db).await
    }

    async fn vehicles(context: &CustomContext) -> Vec<Vehicle> {
        Vehicle::all(&context.db).await
    }
}
