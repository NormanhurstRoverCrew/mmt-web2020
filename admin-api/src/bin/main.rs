use actix_cors::Cors;
use actix_web::{http::header, middleware, web, App, HttpServer};
use juniper::EmptySubscription;
use mongodb::Client as Mongo;
use stripe::Client;
use tonic::transport::Endpoint;
use mmt::email::email_client::EmailClient;
use libmmtapi::{
	graphql::{context::CustomContext, mutation_root::MutationRoot, query_root::QueryRoot},
	routes::{graphiql, graphql, Schema},
};

#[actix_rt::main]
async fn main() -> Result<(), std::io::Error> {
	std::env::set_var("RUST_LOG", "actix_web=info");
	env_logger::init();

	let client = Mongo::with_uri_str("mongodb://db:27017/").await.unwrap();
	let db = client.database("mmt_development");

	let stripe = std::env::var("STRIPE_API_KEY").expect("Stripe Api Key");
	let stripe = Client::new(stripe);

	let email_grpc = loop {
		if let Ok(email) = Endpoint::from_static("http://email:50051").connect().await {
			break email;
		}
        eprintln!("Could not connect to Email API");
		std::thread::sleep(std::time::Duration::from_secs(10));
	};

	let email_grpc = EmailClient::new(email_grpc.clone());

	// Create Juniper schema
	let schema = std::sync::Arc::new(Schema::new(
		QueryRoot,
		MutationRoot,
		EmptySubscription::<CustomContext>::new(),
	));

	// Start http server
	HttpServer::new(move || {
		let cors = Cors::default()
			.allowed_origin("http://localhost:8081")
			.allowed_origin("http://192.168.0.20:8081")
			.allowed_origin("http://localhost:8083")
			.allowed_origin("http://192.168.0.20:8083")
			.allowed_origin(dbg!(std::env::var("ADMIN_ORIGIN").unwrap_or("https://admin.mmt.normorovers.com".into())).as_str())
			.allowed_methods(vec!["GET", "POST", "OPTIONS"])
			.allowed_headers(vec![header::CONTENT_TYPE, header::AUTHORIZATION]);

		App::new()
			.data(schema.clone())
			.data(stripe.clone())
			.data(db.clone())
			.data(email_grpc.clone())
			.wrap(middleware::Logger::default())
			.wrap(cors)
			.service(web::resource("/graphql").route(web::post().to(graphql)))
			.service(web::resource("/graphiql").route(web::get().to(graphiql)))
	})
	.bind("0.0.0.0:8000")?
	.run()
	.await
}
