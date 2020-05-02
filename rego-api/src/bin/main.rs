#![feature(decl_macro, proc_macro_hygiene)]

use juniper::EmptySubscription;
use rocket::routes;
use std::io::Cursor;
use stripe::Client;

use libmmtapi::{
	db::PrimaryDb,
	graphql::{context::CustomContext, mutation_root::MutationRoot, query_root::QueryRoot},
	routes::{self, Schema},
};

pub struct CORS();

use rocket::{
	fairing::{Fairing, Info, Kind},
	http::{ContentType, Header, Method},
	Request, Response,
};
#[rocket::async_trait]
impl Fairing for CORS {
	fn info(&self) -> Info {
		Info {
			name : "Add CORS headers to requests",
			kind : Kind::Response,
		}
	}

    async fn on_response<'a>(&'a self, request: &'a Request<'_>, response: &'a mut Response<'_>) {
			response.set_header(Header::new(
				"Access-Control-Allow-Origin",
				"http://localhost:8080",
			));
			response.set_header(Header::new(
				"Access-Control-Allow-Methods",
				"POST, GET, OPTIONS",
			));
			response.set_header(Header::new("Access-Control-Allow-Headers", "Content-Type"));
			response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
	}
}

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

	let stripe = Client::new(std::env::var("STRIPE_API_KEY").expect("Stripe Api Key"));

	// libmmtapi::stripe::create_checkout_session(1);

	rocket::ignite()
		// .attach(cors)
		.attach(CORS())
		.attach(PrimaryDb::fairing())
		.manage(Schema::new(
			QueryRoot,
			MutationRoot,
			EmptySubscription::<CustomContext>::new(),
		))
		.manage(stripe)
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
