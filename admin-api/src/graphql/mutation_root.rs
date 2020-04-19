use crate::{
	db::helpers as DBHelper,
	graphql::{context::SharedContext, util::string_to_id},
	models::{Booking, Ticket, TicketUpdate, TransactionInput},
};
use juniper::{graphql_value, FieldError, FieldResult};
use mongodb::{oid::ObjectId, Document};
use std::iter::Iterator;

pub struct MutationRoot;
#[juniper::graphql_object(
    Context = SharedContext
)]
impl MutationRoot {
	fn update_tickets(
		context : &SharedContext,
		tickets : Vec<TicketUpdate>,
	) -> FieldResult<Vec<Ticket>> {
		let _ = tickets
			.iter()
			.map(|ticket| {
				let tdb = DBHelper::get(
					&context.tickets_handel(),
					&string_to_id(&ticket.id).unwrap(),
				);
				(ticket, tdb)
			})
			.collect::<Vec<(&TicketUpdate, Option<Ticket>)>>()
			.iter()
			.map(|(ticket, tdb)| (ticket, tdb.as_ref().map(|t| t.get_user_id_opt())))
			.collect::<Vec<(&&TicketUpdate, Option<Option<ObjectId>>)>>()
			.iter()
			.map(|(ticket, user_id)| (ticket, user_id.clone().flatten()))
			.collect::<Vec<(&&&TicketUpdate, Option<ObjectId>)>>()
			.iter()
			.filter_map(|(ticket, user_id)| {
				user_id.as_ref().map(|user_id| (ticket.to_owned(), user_id))
			})
			.collect::<Vec<(&&&TicketUpdate, &ObjectId)>>()
			.iter()
			.map(|(ticket, user_id)| {
				(
					doc! {
						"$set" => {
							"name" => &ticket.user.name,
							"mobile" => &ticket.user.mobile,
							"email" => &ticket.user.email,
							"crew" => &ticket.user.crew,
						}
					},
					doc! {
						"_id" => user_id.to_owned().to_owned()
					},
				)
			})
			.collect::<Vec<(Document, Document)>>()
			.iter()
			.map(|(doc, user_id)| {
				match context
					.users_handel()
					.update_one(user_id.to_owned(), doc.to_owned(), None)
				{
					Ok(_) => Some(()),
					Err(_) => None,
				}
			})
			.collect::<Vec<Option<()>>>();

		let tickets = tickets
			.iter()
			.filter_map(|ticket| string_to_id(&ticket.id).ok())
			.collect::<Vec<ObjectId>>()
			.iter()
			.filter_map(|tid| DBHelper::get(&context.tickets_handel(), &tid))
			.collect::<Vec<Ticket>>();

		Ok(tickets)
	}

	fn update_ticket(context : &SharedContext, ticket : TicketUpdate) -> FieldResult<Ticket> {
		let t : Option<Ticket> = DBHelper::get(
			&context.tickets_handel(),
			&string_to_id(&ticket.id).expect("Ticket ID"),
		);
		dbg!(&ticket);
		if let Some(t) = &t {
			&context.users_handel().update_one(
				doc! {
					"_id" => t.get_user_id(),
				},
				doc! {
					"$set" => {
						"name" => ticket.user.name,
						"email" => ticket.user.email,
						"crew" => ticket.user.crew,
						"mobile" => ticket.user.mobile,
						"diet" => ticket.user.diet,
						"email_verified" => ticket.user.email_verified,
					}
				},
				None,
			);
		}

		Ok(t.unwrap())
	}

	fn delete_tickets(context : &SharedContext, ticket_ids : Vec<String>) -> FieldResult<f64> {
		let tickets = ticket_ids
			.iter()
			.filter_map(|id| string_to_id(id).ok())
			.collect::<Vec<ObjectId>>()
			.iter()
			.filter_map(|id| DBHelper::get(&context.tickets_handel(), id))
			.collect::<Vec<Ticket>>();

		let mut out : u64 = 0;
		for ticket in tickets {
			out += ticket.delete(&context) as u64;
		}

		Ok(out as f64)
	}

	fn addTransaction(
		context : &SharedContext,
		booking_id : String,
		transaction : TransactionInput,
	) -> FieldResult<bool> {
		if context.auth.permissions.payments_add {
			context
				.bookings_handel()
				.update_one(
					doc! {"_id"=>string_to_id(&booking_id).unwrap()},
					doc! {
					"$push" => {
						"payment.transactions" => {
							"_id" => ObjectId::new().unwrap(),
							"value" => transaction.value,
							"method" => transaction.method.to_string()
						}
					}
					},
					None,
				)
				.map_or_else(
					|_| {
						Err(FieldError::new(
							"Could not add Tickets to DB",
							graphql_value!({"type":"DB_ERROR"}),
						))
					},
					|_| Ok(true),
				)
		} else {
			Err(FieldError::new(
				"Not Authorized to create transactions",
				graphql_value!({"type":"UNAUTHORIZED_PAYMENTS_ADD"}),
			))
		}
	}

	fn delete_Booking(context : &SharedContext, booking_id : String) -> FieldResult<bool> {
		let booking = DBHelper::get::<Booking>(
			&context.bookings_handel(),
			&string_to_id(&booking_id).expect("ObjectID"),
		);

		match booking {
			Some(b) => Ok(b.delete(&context)),
			None => Err(FieldError::new(
				"Booking not found",
				graphql_value!({"type":"BOOKING_NOT_FOUND"}),
			)),
		}
	}
}
