use crate::{
    custom_context::CustomContext,
    models::{admin_user::AdminUser, ticket::Ticket},
};
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
}
