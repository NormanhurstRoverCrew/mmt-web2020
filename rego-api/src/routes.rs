use rocket::{get, post, response::content, State};

use juniper::{EmptySubscription, RootNode};
use stripe::Client;

use crate::{
	db::PrimaryDb,
	graphql::{context::CustomContext, mutation_root::MutationRoot, query_root::QueryRoot},
};

pub type Schema = RootNode<'static, QueryRoot, MutationRoot, EmptySubscription<CustomContext>>;

#[get("/")]
pub fn index() -> &'static str { "Hello, world!" }

#[get("/")]
pub fn graphiql() -> content::Html<String> { juniper_rocket_async::graphiql_source("/graphql") }

#[get("/graphql?<request>")]
pub async fn get_graphql_handler(
	context : PrimaryDb,
	request : juniper_rocket_async::GraphQLRequest,
	schema :  State<'_, Schema>,
	stripe :  State<'_, Client>,
) -> juniper_rocket_async::GraphQLResponse {
	request
		.execute(
			&schema,
			&CustomContext {
				connection : context,
				stripe :     stripe.inner().clone(),
			},
		)
		.await
}

#[post("/graphql", data = "<request>")]
pub async fn post_graphql_handler(
	context : PrimaryDb,
	request : juniper_rocket_async::GraphQLRequest,
	schema : State<'_, Schema>,
	stripe : State<'_, Client>,
) -> juniper_rocket_async::GraphQLResponse {
	request
		.execute(
			&schema,
			&CustomContext {
				connection : context,
				stripe :     stripe.inner().clone(),
			},
		)
		.await
}
