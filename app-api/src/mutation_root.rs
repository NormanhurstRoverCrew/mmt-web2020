use crate::custom_context::CustomContext;
use juniper::FieldResult;

pub struct MutationRoot;
#[juniper::graphql_object(
    Context = CustomContext
)]
impl MutationRoot {
    async fn update_tickets() -> FieldResult<bool> {
        Ok(true)
    }
}
