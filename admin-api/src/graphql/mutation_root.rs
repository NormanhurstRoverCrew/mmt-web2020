use crate::{
	db::Db,
	graphql::{context::CustomContext, util::string_to_id},
	models::{Booking, Ticket, TicketUpdate, Transaction, User, UserUpdate},
	wire::TransactionInput,
};
use bson::{doc, oid::ObjectId, Bson, Document};
use futures::{stream::FuturesUnordered, StreamExt};
use juniper::{graphql_value, FieldError, FieldResult};
use std::iter::Iterator;

pub struct MutationRoot;
#[juniper::graphql_object(
    Context = CustomContext
)]
impl MutationRoot {
	async fn update_tickets(
		context : &CustomContext,
		tickets : Vec<TicketUpdate>,
	) -> FieldResult<Vec<Ticket>> {
		let get_ticket = |id : ObjectId| async move { Ticket::get(&context, &id).await };

		let update_ticket = |user_id : Document, data : Document| async move {
			let _ = context
				.users_handel()
				.update_one(user_id.to_owned(), data.to_owned(), None)
				.await;
		};

		tickets
                        .iter()
                        .map(|ticket| get_ticket(ticket.id.clone()))
                        .collect::<FuturesUnordered<_>>()
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
                            let user : Bson = bson::to_bson(&ticket.user).unwrap();
                            let user = user.as_document().unwrap();
                                            (
                                                    doc! {
                                                            "$set" : user,
                                                    },
                                                    doc! {
                                                            "_id" :user_id
                                                    },
                                            )
                                    })
                                    .map(|(doc, user_id)| {
                            update_ticket(user_id, doc)
                        })
                        .collect::<FuturesUnordered<_>>()
                            .collect::<()>()
                            .await;

		// let futures = tickets
		//      .iter()
		//      .map(|ticket| {
		//         Box::pin(get_tickets(&ticket.id))
		//      })
		//      .collect::<Vec<Pin<Box<dyn Future<Output = Option<Ticket>>>>>>();

		Ok(tickets
			.iter()
			.map(|ticket| get_ticket(ticket.id.clone()))
			.collect::<FuturesUnordered<_>>()
			.filter_map(|a| async move { a })
			.collect::<Vec<Ticket>>()
			.await)
	}

	async fn update_ticket(context : &CustomContext, ticket : TicketUpdate) -> FieldResult<Ticket> {
		match Ticket::get(&context, &ticket.id).await {
			Some(t) => {
				let user : Bson = bson::to_bson(&ticket.user).unwrap();

				&context.users_handel().update_one(
					doc! {
						"_id" : t.get_user_id(),
					},
					doc! {
						"$set" : user,
					},
					None,
				);

				Ok(t)
			},
			None => Err(FieldError::new(
				"Could not find ticket to update",
				graphql_value!({"type":"TICKET_NOT_FOUND"}),
			)),
		}
	}

	async fn delete_tickets(
		context : &CustomContext,
		ticket_ids : Vec<String>,
	) -> FieldResult<f64> {
		let ticket_ids = ticket_ids
			.iter()
			.filter_map(|id| string_to_id(id).ok())
			.collect::<Vec<ObjectId>>();

		context
			.tickets_handel()
			.delete_many(
				doc! {
					"_id" : {
						"$in" : ticket_ids,
					}
				},
				None,
			)
			.await
			.map(|dr| dr.deleted_count as f64)
			.map_err(|_| {
				FieldError::new(
					"Failed to delete Tickets from DB",
					graphql_value!({"type":"DB_ERROR"}),
				)
			})
	}

	async fn addTransaction(
		context : &CustomContext,
		booking_id : String,
		transaction : TransactionInput,
	) -> FieldResult<bool> {
		let transaction : Transaction = transaction.into();
		// if context.auth.permissions.payments_add {
		let transaction : Bson = bson::to_bson(&transaction).unwrap();
		context
			.bookings_handel()
			.update_one(
				doc! {"_id": string_to_id(&booking_id).unwrap()},
				doc! {
				"$push" : {
					"payment.transactions" : transaction,
				}
				},
				None,
			)
			.await
			.map_or_else(
				|_| {
					Err(FieldError::new(
						"Could not add Tickets to DB",
						graphql_value!({"type":"DB_ERROR"}),
					))
				},
				|_| Ok(true),
			)
		// } else {
		// 	Err(FieldError::new(
		// 		"Not Authorized to create transactions",
		// 		graphql_value!({"type":"UNAUTHORIZED_PAYMENTS_ADD"}),
		// 	))
		// }
	}

	async fn delete_booking(context : &CustomContext, booking_id : String) -> FieldResult<bool> {
		match Booking::get(&context, &string_to_id(&booking_id).expect("ObjectID")).await {
			Some(b) => Ok(b.delete(&context).await),
			None => Err(FieldError::new(
				"Booking not found",
				graphql_value!({"type":"BOOKING_NOT_FOUND"}),
			)),
		}
	}
}
