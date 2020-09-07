use crate::{
    db::Db,
	graphql::{
		context::CustomContext, mutation_root::MutationRoot, query_root::QueryRoot,
		util::string_to_id,
	},
	models::{Booking, Transaction},
};
use tonic::transport::Channel;
use mmt::email::email_client::EmailClient;
use actix_web::{web, Error, HttpResponse};
use juniper::{
	http::{graphiql::graphiql_source, GraphQLRequest},
	EmptySubscription, RootNode,
};
use mongodb::Database;
use std::sync::Arc;
use stripe::{Client, Event, EventObject, EventType, PaymentIntent, PaymentIntentStatus};

pub type Schema = RootNode<'static, QueryRoot, MutationRoot, EmptySubscription<CustomContext>>;

pub async fn graphiql() -> HttpResponse {
	let html = graphiql_source("http://localhost:8082/graphql", None);
	HttpResponse::Ok()
		.content_type("text/html; charset=utf-8")
		.body(html)
}

pub async fn graphql(
	schema : web::Data<Arc<Schema>>,
	stripe : web::Data<Client>,
    rpc_email: web::Data<EmailClient<Channel>>,
	db : web::Data<Database>,
	data : web::Json<GraphQLRequest>,
) -> Result<HttpResponse, Error> {
	let context = CustomContext {
		db :     db.into_inner(),
		stripe : stripe.into_inner(),
        rpc_email: rpc_email.into_inner(),
	};

	let res = data.execute(&schema, &context).await;
	let res = serde_json::to_string(&res)?;
	Ok(HttpResponse::Ok()
		.content_type("application/json")
		.body(res))
}

pub async fn stripe_hook(
	stripe : web::Data<Client>,
    rpc_email: web::Data<EmailClient<Channel>>,
	db : web::Data<Database>,
	event : web::Json<Event>,
) -> Result<HttpResponse, Error> {
	let context = CustomContext {
		db :     db.into_inner(),
		stripe : stripe.into_inner(),
        rpc_email: rpc_email.into_inner(),
	};
	match event.event_type {
		EventType::PaymentIntentSucceeded => {
			let _ = handle_pi_update(&context, &context.stripe, &event.data.object).await;
		},
		_ => {
			dbg!(event);
		},
	};
	Ok(HttpResponse::Ok().finish())
}

#[derive(Debug)]
enum PaymentError {
	Underpaid,
	Unknown,
	MetadataBookingNotFound,
	CouldNotCommit,
}

impl std::fmt::Display for PaymentError {
	fn fmt(&self, f : &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "{:?}", self) }
}

impl std::error::Error for PaymentError {}

async fn handle_pi_update(
	context : &CustomContext,
	stripe : &Client,
	obj : &EventObject,
) -> Result<(), PaymentError> {
	let booking_id = if let EventObject::PaymentIntent(pi) = obj {
		match PaymentIntent::retrieve(&stripe, pi.id.as_str()).await {
			Ok(PaymentIntent {
				amount_received: Some(ar),
				status: PaymentIntentStatus::Succeeded,
				metadata,
				..
			}) if ar >= 2000 => metadata
				.get("booking_id")
				.ok_or(PaymentError::MetadataBookingNotFound)
				.map(|v| (string_to_id(v).unwrap(), pi)),
			Ok(PaymentIntent {
				amount_received: Some(ar),
				..
			}) if ar < 2000 => Err(PaymentError::Underpaid),
			_ => Err(PaymentError::Unknown),
		}
	} else {
		Err(PaymentError::Unknown)
	};

	//TODO add payment and send emails?

	if let Ok((booking_id, pi)) = &booking_id {
		let mut booking : Booking =
			match Booking::get(&context, &booking_id).await {
				Some(b) => b,
				None => {
					return Err(PaymentError::CouldNotCommit);
				},
			};

		booking
			.add_transaction(&context, Transaction::stripe(pi.id.as_str().to_string()))
			.await;
	};

	booking_id.map(|_| ())
}
