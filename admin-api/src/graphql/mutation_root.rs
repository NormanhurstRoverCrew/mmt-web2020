use crate::{
	db::helpers as DBHelper,
	email::MyEmail,
	graphql::{
		context::Database,
		util::{bson_to_id, string_to_id},
	},
	models::{BasicUser, Booking, Ticket, TicketUpdate, Transaction, TransactionInput, User},
};
use juniper::{graphql_value, FieldError, FieldResult};
use mongodb::{oid::ObjectId, Bson, Document};
use rand::{distributions::Alphanumeric, Rng};
use std::iter::Iterator;

pub struct MutationRoot;
#[juniper::object(
    Context = Database
)]
impl MutationRoot {
	fn newUser(context : &Database, user : BasicUser) -> FieldResult<Option<User>> {
		let users = context.users_handel();

		let random_code = rand::thread_rng()
			.sample_iter(&Alphanumeric)
			.take(16)
			.collect::<String>();

		let result = users
			.insert_one(
				doc! {
					"name" => &user.name,
					"email" => &user.email,
					"mobile" => &user.mobile,
					"crew" => &user.crew,
					"code" => random_code,
				},
				None,
			)
			.unwrap();

		let user = match result.acknowledged {
			true => match result.inserted_id {
				Some(id) => match bson_to_id(&id) {
					Ok(user_id) => match DBHelper::get::<User>(&users, user_id) {
						Some(user) => user,
						_ => {
							return Err(juniper::FieldError::new(
								"User does not exist",
								graphql_value!({
									"type": "NO_WHATEVER"
								}),
							))
						},
					},
					Err(e) => return Err(e),
				},
				None => {
					return Err(juniper::FieldError::new(
						"User ID not created",
						graphql_value!({"type": "ID_MISSING"}),
					))
				},
			},
			false => {
				return Err(juniper::FieldError::new(
					"User not created",
					graphql_value!({"type": "INSERT_FAILED"}),
				))
			},
		};

		MyEmail::from_user(&user)
			.verify_email()
			.expect("Verification Email Not Sent...");

		Ok(Some(user))
	}

	fn verifyUser(context : &Database, id : String, code : String) -> FieldResult<User> {
		let users = context.users_handel();
		let id = match string_to_id(&id) {
			Ok(id) => id,
			Err(e) => return Err(e),
		};

		let mut user : User = match DBHelper::get(&users, &id) {
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
				let users = context.users_handel();
				let res = users.update_one(
					doc! {
						"_id" => id.to_owned()
					},
					doc! {
						"$set" => {
							"email_verified" => true,
						},
					},
					None,
				);

				match res {
					Ok(_) => {
						user.email_verified = true;
					},
					_ => {},
				};
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

	/// Take in the details of a user, how they would like to receive their
	/// order and possibly their address.
	fn newBooking(
		context : &Database,
		name : String,
		email : String,
		mobile : String,
	) -> FieldResult<Option<Booking>> {
		let bookings = context.bookings_handel();

		let result = bookings
			.insert_one(
				doc! {
					"user" => {
						"name" => &name,
						"email" => &email,
						"mobile" => &mobile,
					},
				},
				None,
			)
			.unwrap();

		let id = result.inserted_id.unwrap_or(Bson::ObjectId(
			ObjectId::new().expect("Converting mongoID to juniperID failed"),
		));

		let id = id.as_object_id().expect("Unwrap string").to_string();

		// let mut params = stripe::PaymentIntentCreateParams::new(
		// 	SCARVE_PRICE * quantity as u64 + post_price,
		// 	stripe::Currency::AUD,
		// );

		// let desc = format!(
		// 	"{}: Scarves x{} for {}",
		// 	name,
		// 	&quantity,
		// 	match delivery_method {
		// 		CollectionMethod::Pickup => "Pickup",
		// 		CollectionMethod::Post => "Postage",
		// 	}
		// );

		// params.description = Some(&desc);

		// let cus = String::from(&email);
		// let mut meta = stripe::Metadata::new();
		// meta.insert("email".to_string(), cus);
		// meta.insert("quantity".to_string(), quantity.to_string());
		// params.metadata = Some(meta);

		// let pi = match stripe::PaymentIntent::create(&stripe_client, params) {
		// 	Ok(pi) => {
		// 		orders
		// 			.update_one(
		// 				doc! {"_id" => ObjectId::with_string(&id).unwrap()},
		// 				doc! {
		// 					"$set" => {
		// 						"payment" => {
		// 							"stripe" => {
		// 								"pi" => pi.id.as_str(),
		// 							}
		// 						}
		// 					}
		// 				},
		// 				None,
		// 			)
		// 			.expect("Updating Order failed");
		// 		pi
		// 	},
		// 	_ => {
		// 		return Err(juniper::FieldError::new(
		// 			"Failed to create payment intent",
		// 			graphql_value!({
		// 				"type": "NO_WHATEVER"
		// 			}),
		// 		))
		// 	},
		// };

		let id = match mongodb::oid::ObjectId::with_string(&id) {
			Ok(oid) => oid,
			Err(_) => {
				return Err(juniper::FieldError::new(
					"UID is not valid",
					graphql_value!({
						"type": "INVALID_UID"
					}),
				))
			},
		};

		let mut booking = match DBHelper::get::<Booking>(&bookings, &id) {
			Some(order) => order,
			_ => {
				return Err(juniper::FieldError::new(
					"Booking does not exist",
					graphql_value!({
						"type": "NO_WHATEVER"
					}),
				))
			},
		};

		// let mut payment = &mut order
		// 	.payment
		// 	.as_mut()
		// 	.expect("Unwrapping mutable payment failed");

		// let mut stripe = &mut payment
		// 	.stripe
		// 	.as_mut()
		// 	.expect("Unwrapping mutable stripe payment failed");

		// stripe.client_secret = Some(pi.client_secret.expect("Unwrapping client secret
		// failed"));

		Ok(Some(booking))
	}

	fn add_tickets_to_booking(
		context : &Database,
		booking_id : String,
		users : Vec<BasicUser>,
	) -> FieldResult<Booking> {
		let booking = string_to_id(&booking_id)
			.and_then(|booking_id| {
				let bookings = context.bookings_handel();
				Ok(DBHelper::get(&bookings, &booking_id))
			})
			.or_else(|_| {
				return Err(FieldError::new(
					"Booking does not exist",
					graphql_value!({"type":"BOOKING_NOT_FOUND"}),
				));
			})
			.map(|b : Option<Booking>| b.expect("BOOKING"));

		if booking.is_err() || users.len() == 0 {
			return booking;
		}

		let users : Vec<Document> = users
			.iter()
			.map(|user| {
				doc! {
					"name" => &user.name,
					"email"=> &user.email,
					"mobile"=> &user.mobile,
					"crew"=> &user.crew,
				}
			})
			.collect();

		let tickets_result = context
			.users_handel()
			.insert_many(users, None)
			.map(|users| users.inserted_ids.unwrap())
			.map(|ids| {
				ids.iter()
					.filter_map(|id| id.1.as_object_id().map(|a| a.to_owned()))
					.collect::<Vec<ObjectId>>()
			})
			.map(|users| {
				users
					.iter()
					.map(|user_id| {
						doc! {
							"booking_id" => string_to_id(&booking_id).unwrap().to_owned(),
							"user_id" => user_id.to_owned(),
						}
					})
					.collect::<Vec<Document>>()
			})
			.map(|tickets| context.tickets_handel().insert_many(tickets, None));

		if tickets_result.is_ok() {
			booking
		} else {
			return Err(FieldError::new(
				"Could not add Tickets to DB",
				graphql_value!({"type":"DB_ERROR"}),
			));
		}
	}

	fn update_tickets(
		context : &Database,
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
			.filter_map(|(ticket, tdb)| Some((ticket, tdb.as_ref().map(|t| t.get_user_id_opt()))))
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

	fn update_ticket(context : &Database, ticket : TicketUpdate) -> FieldResult<Ticket> {
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

	fn delete_tickets(context : &Database, ticket_ids : Vec<String>) -> FieldResult<f64> {
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
		context : &Database,
		booking_id : String,
		transaction : TransactionInput,
	) -> FieldResult<bool> {
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
	}
}
