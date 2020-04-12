#![feature(decl_macro, proc_macro_hygiene)]

use rocket::routes;

use libmmtapi::{
	db::PrimaryDb,
	graphql::schema::{MutationRoot, QueryRoot},
	routes::{self, Schema},
};

fn main() {
	rocket::ignite()
		.attach(PrimaryDb::fairing())
		.manage(Schema::new(QueryRoot, MutationRoot))
		.mount(
			"/",
			routes![
				routes::index,
				routes::get_graphql_handler,
				routes::post_graphql_handler
			],
		)
		.mount("/graphiql", routes![routes::graphiql])
		.launch();
}
