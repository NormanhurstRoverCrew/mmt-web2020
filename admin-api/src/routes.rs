use rocket::{get, post, response::content, State};

use juniper::{EmptySubscription, RootNode};

use crate::{
	auth::AdminUser,
	db::PrimaryDb,
	graphql::{context::SharedContext, mutation_root::MutationRoot, query_root::QueryRoot},
};

pub type Schema = RootNode<'static, QueryRoot, MutationRoot, EmptySubscription<SharedContext>>;

pub fn schema() -> Schema {
	Schema::new(
		QueryRoot,
		MutationRoot,
		EmptySubscription::<SharedContext>::new(),
	)
}

#[get("/")]
pub fn index() -> &'static str { "Hello, world!" }

#[get("/")]
pub fn graphiql() -> content::Html<String> { juniper_rocket::graphiql_source("/graphql", None) }

#[get("/graphql?<request>")]
pub fn get_graphql_handler(
	context : PrimaryDb,
	request : juniper_rocket::GraphQLRequest,
	schema : State<Schema>,
	admin : AdminUser,
) -> juniper_rocket::GraphQLResponse {
	request.execute_sync(
		schema.inner(),
		&SharedContext {
			connection : context,
			auth :       admin,
		},
	)
}

#[post("/graphql", data = "<request>")]
pub fn post_graphql_handler(
	context : PrimaryDb,
	request : juniper_rocket::GraphQLRequest,
	schema : State<Schema>,
	admin : AdminUser,
) -> juniper_rocket::GraphQLResponse {
	request.execute_sync(
		schema.inner(),
		&SharedContext {
			connection : context,
			auth :       admin,
		},
	)
}
