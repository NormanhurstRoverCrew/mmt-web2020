use crate::{
	graphql::{context::CustomContext, mutation_root::MutationRoot, query_root::QueryRoot},
	models::{Booking, Transaction},
};
use actix_web::{web, Error, HttpResponse};
use bson::oid::ObjectId;
use juniper::{
	http::{graphiql::graphiql_source, GraphQLRequest},
	EmptySubscription, RootNode,
};
use mmt::{db::Db, email::email_client::EmailClient, email::Booking as EmailBooking};
use mongodb::Database;
use std::sync::Arc;
use stripe::{Client, EventObject, EventType, PaymentIntent,  WebhookEvent};
use tonic::transport::Channel;

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
	rpc_email : web::Data<EmailClient<Channel>>,
	db : web::Data<Database>,
	data : web::Json<GraphQLRequest>,
) -> Result<HttpResponse, Error> {
	let context = CustomContext {
		db :        db.get_ref().clone(),
		stripe :    stripe.into_inner(),
		rpc_email : rpc_email.into_inner(),
	};

	let res = data.execute(&schema, &context).await;
	// let res = serde_json::to_string(&res)?;
	Ok(HttpResponse::Ok()
		.content_type("application/json")
		.json(res))
}

pub async fn stripe_hook(
	stripe : web::Data<Client>,
	rpc_email : web::Data<EmailClient<Channel>>,
	db : web::Data<Database>,
	event : web::Json<WebhookEvent>,
) -> Result<HttpResponse, Error> {
	let context = CustomContext {
		db :        db.get_ref().clone(),
		stripe :    stripe.into_inner(),
		rpc_email : rpc_email.into_inner(),
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
	// Underpaid,
	Unknown,
	MetadataBookingNotFound,
	BookingNotFound,
	// CouldNotCommit,
    EmailError,
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
		let pi = PaymentIntent::retrieve(&stripe, &pi.id, &[])
			.await
			.map_err(|_| PaymentError::Unknown)?;
		let bid = pi
			.metadata
			.get("booking_id")
			.map(|bid| ObjectId::parse_str(bid).ok())
			.flatten()
			.ok_or(PaymentError::MetadataBookingNotFound)?;
		let mut booking = Booking::get(&context.db, &bid)
			.await
			.ok_or(PaymentError::BookingNotFound)?;

		if let None = booking.payment.transactions.iter().find(|t| match t {
			Transaction::Stripe {
				pi_id, ..
			} if pi_id == pi.id.as_str() => true,
			_ => false,
		}) {
			booking
				.add_transaction(&context, Transaction::stripe(pi.id.as_str().to_owned()))
				.await;
		}

		if booking.balence(&context).await < 2.0 {
            // Send email...

		    let mut rpc_email = (&*context.rpc_email).clone();

            let req = EmailBooking {
                id: booking.id.to_hex(),
            };
            dbg!();
		rpc_email
            .onboard_booking(req).await
			.map(|r| {
				dbg!(r.into_inner());
			})
			.map_err(|_| {
                PaymentError::EmailError
			})?;


        }
		Ok(())
	} else {
		Err(PaymentError::Unknown)
	};

	//TODO add payment and send emails?

	// if let Ok((booking_id, pi)) = &booking_id {
	// 	let mut booking : Booking = match Booking::get(&context.db,
	// &booking_id).await { 		Some(b) => b,
	// 		None => {
	// 			return Err(PaymentError::CouldNotCommit);
	// 		},
	// 	};

	// 	booking
	// 		.add_transaction(&context, Transaction::stripe(pi.id.as_str().to_string()))
	// 		.await;
	// };

	booking_id.map(|_| ())
}
