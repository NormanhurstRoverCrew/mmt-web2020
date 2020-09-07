#![feature(decl_macro, proc_macro_hygiene)]

use actix_cors::Cors;
use actix_web::{http::header, middleware, web, App, HttpServer};
use juniper::EmptySubscription;
use mongodb::Client as Mongo;
use stripe::Client;
use tonic::transport::Endpoint;
use mmt::email::email_client::EmailClient;

use libmmtapi::{
	graphql::{context::CustomContext, mutation_root::MutationRoot, query_root::QueryRoot},
	routes::{graphiql, graphql, stripe_hook, Schema},
};

#[actix_rt::main]
async fn main() -> Result<(), std::io::Error> {
	std::env::set_var("RUST_LOG", "actix_web=info");
	env_logger::init();

	let client = Mongo::with_uri_str("mongodb://db:27017/").await.unwrap();
	let db = client.database("mmt_development");

	let stripe = std::env::var("STRIPE_API_KEY").expect("Stripe Api Key");
	let stripe = Client::new(stripe);

    let grpc_email = Endpoint::from_static("http://email:50051").connect().await.unwrap();
    let rpc_email = EmailClient::new(grpc_email.clone());

	// Create Juniper schema
	let schema = std::sync::Arc::new(Schema::new(
		QueryRoot,
		MutationRoot,
		EmptySubscription::<CustomContext>::new(),
	));

	// Start http server
	HttpServer::new(move || {
		let cors = Cors::new()
			.allowed_origin("http://localhost:8082")
			.allowed_origin("http://localhost:8080")
			.allowed_methods(vec!["GET", "POST", "OPTIONS"])
			.allowed_headers(vec![header::CONTENT_TYPE])
			.finish();

		App::new()
			.data(schema.clone())
			.data(stripe.clone())
			.data(db.clone())
            .data(rpc_email.clone())
			.wrap(middleware::Logger::default())
			.wrap(cors)
			.service(web::resource("/graphql").route(web::post().to(graphql)))
			.service(web::resource("/graphiql").route(web::get().to(graphiql)))
			.service(web::resource("/stripe/hooks").route(web::post().to(stripe_hook)))
	})
	.bind("0.0.0.0:8000")?
	.run()
	.await
}
