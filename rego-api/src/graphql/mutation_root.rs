use crate::{
	db::helpers as DBHelper,
	email::MyEmail,
	graphql::{context::CustomContext, util::string_to_id},
	models::{BasicUser, Booking, Ticket, TicketUpdate, Transaction, User},
};
use bson::{doc, oid::ObjectId, Document};
use futures::{stream::FuturesUnordered, StreamExt};
use juniper::{graphql_value, FieldError, FieldResult};
use mongodb::results::InsertOneResult;
use std::iter::Iterator;
use stripe::{PaymentIntent, PaymentIntentUpdateParams};

pub struct MutationRoot;
#[juniper::graphql_object(
    Context = CustomContext
)]
impl MutationRoot {
	async fn newUser(context : &CustomContext, user : BasicUser) -> FieldResult<Option<User>> {
		let users = context.users_handel();

		let user : User = user.into();

		let doc = bson::to_bson(&user)?;

		let result = match doc {
			bson::Bson::Document(document) => users.insert_one(document, None),
			_ => {
				return Err(juniper::FieldError::new(
					"Failed to create bew User",
					graphql_value!({"type": "DB_ERROR"}),
				))
			},
		};

		let user_id = match result.await {
			Ok(InsertOneResult {
				inserted_id,
			}) => inserted_id.as_object_id().unwrap().to_owned(),
			Err(_) => {
				return Err(juniper::FieldError::new(
					"User ID not created",
					graphql_value!({"type": "ID_MISSING"}),
				))
			},
		};

		let user = match DBHelper::get::<User>(&users, &user_id).await {
			Some(user) => user,
			_ => {
				return Err(juniper::FieldError::new(
					"User does not exist",
					graphql_value!({
						"type": "USER_NOT_FOUND"
					}),
				))
			},
		};

		MyEmail::from_user(&user)
			.verify_email()
			.map(|_| Some(user))
			.map_err(|_| {
				juniper::FieldError::new(
					"Failed to send email",
					graphql_value!({
						"type": "EMAIL_FAIL"
					}),
				)
			})
	}

	async fn verifyUser(context : &CustomContext, id : String, code : String) -> FieldResult<User> {
		let users = context.users_handel();
		let id = match string_to_id(&id) {
			Ok(id) => id,
			Err(e) => return Err(e),
		};

		let mut user : User = match DBHelper::get(&users, &id).await {
			Some(user) => user,
			None => {
				return Err(juniper::FieldError::new(
					"User not found",
					graphql_value!({
						"type": "USER_NOT_FOUND"
					}),
				))
			},
		};

		match user.is_code_valid(&code) {
			Ok(()) => {
				user.set_email_verified(&context, true).await;
			},
			Err(e) => {
				return Err(juniper::FieldError::new(
					e,
					graphql_value!({"type": "INVALID_CODE"}),
				))
			},
		};

		Ok(user)
	}

	async fn add_tickets_to_booking(
		context : &CustomContext,
		booking_id : String,
		users : Vec<BasicUser>,
	) -> FieldResult<Booking> {
		let bid = string_to_id(&booking_id).or_else(|_| {
			Err(FieldError::new(
				"Booking does not exist",
				graphql_value!({"type":"BOOKING_NOT_FOUND"}),
			))
		})?;

		let booking = DBHelper::get(&context.bookings_handel(), &bid).await;

		if booking.is_none() || users.is_empty() {
			return Ok(booking.unwrap());
		}

		let users : Vec<Document> = users
			.iter()
			.map(|user| -> User { user.clone().into() })
			.filter_map(|user| match bson::to_bson(&user).ok() {
				Some(bson::Bson::Document(doc)) => Some(doc),
				_ => None,
			})
			.collect();

		let tickets = context
			.users_handel()
			.insert_many(users, None)
			.await
			.map(|users| users.inserted_ids)
			.map(|ids| {
				ids.iter()
					.filter_map(|id| id.1.as_object_id().map(|a| a.to_owned()))
					.collect::<Vec<ObjectId>>()
			})
			.map(|users| {
				users
					.iter()
					.map(|user_id| -> Ticket { Ticket::new(&bid, user_id) })
					.filter_map(|ticket| match bson::to_bson(&ticket) {
						Ok(bson::Bson::Document(doc)) => Some(doc),
						_ => None,
					})
					.collect::<Vec<Document>>()
			})?;

		let tickets_result = context.tickets_handel().insert_many(tickets, None).await;

		if tickets_result.is_ok() {
			Ok(booking.unwrap())
		} else {
			Err(FieldError::new(
				"Could not add Tickets to DB",
				graphql_value!({"type":"DB_ERROR"}),
			))
		}
	}

	async fn update_tickets(
		context : &CustomContext,
		tickets : Vec<TicketUpdate>,
	) -> FieldResult<Vec<Ticket>> {
		let get_ticket =
			|id : ObjectId| async move { DBHelper::get(&context.tickets_handel(), &id).await };

		let futures : FuturesUnordered<_> = tickets
			.iter()
			.map(|ticket| get_ticket(ticket.id.clone()))
			.collect();

		// let futures = tickets
		// 	.iter()
		// 	.map(|ticket| {
		//         Box::pin(get_tickets(&ticket.id))
		// 	})
		// 	.collect::<Vec<Pin<Box<dyn Future<Output = Option<Ticket>>>>>>();

		let upadate_ticket = |user_id : Document, data : Document| async move {
			let _ = context
				.users_handel()
				.update_one(user_id.to_owned(), data.to_owned(), None)
				.await;
		};

		let futures : FuturesUnordered<_> = futures
            .collect::<Vec<Option<Ticket>>>()
            .await
            .iter()
            .zip(tickets.iter())
			.map(|(tdb, ticket): (&Option<Ticket>, &TicketUpdate)| -> (&TicketUpdate, Option<Option<ObjectId>>){
                (ticket, tdb.as_ref().map(|t|
                    t.get_user_id_opt())
                )
            })
            .filter_map(|(ticket, user_id)| { user_id.flatten().map(|user_id| (ticket, user_id)) })
			.map(|(ticket, user_id)| {
                let user : User = ticket.user.clone().into();
                let user = bson::to_bson(&user).unwrap();
                let user = user.as_document().unwrap();
				(
					doc! {
						"$set" => user,
					},
					doc! {
						"_id" => user_id
					},
				)
			})
			.map(|(doc, user_id)| {
                upadate_ticket(user_id, doc)
            })
			.collect();

		let _ = futures.collect::<Vec<()>>().await;

		let tickets : FuturesUnordered<_> = tickets
			.iter()
			.map(|ticket| get_ticket(ticket.id.clone()))
			.collect();

		Ok(tickets
			.filter_map(|a| async move { a })
			.collect::<Vec<Ticket>>()
			.await)
	}

	async fn attachStripePaymentMethodToBooking(
		context : &CustomContext,
		booking_id : String,
		payment_method_id : String,
	) -> FieldResult<String> {
		let mut booking : Booking =
			match DBHelper::get(&context.bookings_handel(), &string_to_id(&booking_id)?).await {
				Some(b) => b,
				None => {
					return Err(FieldError::new(
						"Booking not found",
						graphql_value!({"type":"BOOKING_NOT_FOUND"}),
					));
				},
			};

		let spi = match booking.get_stripe_pi(context).await {
			Some(spi) => spi,
			None => booking
				.create_stripe_payment_intent(context)
				.await
				.expect("PaymentIntent not created"),
		};

        PaymentIntent::update(&context.stripe, spi.id.as_str(), PaymentIntentUpdateParams {
            payment_method: Some(&payment_method_id),
            ..PaymentIntentUpdateParams::default()
        }).await;

		spi.client_secret.ok_or_else(|| {
			FieldError::new(
				"Booking not found",
				graphql_value!({"type":"BOOKING_NOT_FOUND"}),
			)
		})
	}
}
