#![feature(decl_macro, proc_macro_hygiene)]

use rocket::routes;

use libmmtapi::{
	db::PrimaryDb,
	routes::{self, schema},
};

use libmmtapi::auth::Jwks;

fn main() {
	// let allowed_origins = AllowedOrigins::some_exact(&["http://localhost:8080"]);

	// You can also deserialize this
	let cors = match (rocket_cors::CorsOptions {
		// allowed_origins,
		// allowed_methods: vec![Method::Post].into_iter().map(From::from).collect(),
		// allowed_headers: AllowedHeaders::some(&["Authorization", "Accept"]),
		// allow_credentials: true,
		send_wildcard : true,
		..Default::default()
	}
	.to_cors())
	{
		Ok(c) => c,
		_ => panic!("Cors header not set up"),
	};

	let jwks = Jwks::get().expect("Could not get JWKS");
	dbg!(&jwks);

	rocket::ignite()
		.attach(cors)
		.attach(PrimaryDb::fairing())
		.manage(schema())
		.manage(jwks)
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
