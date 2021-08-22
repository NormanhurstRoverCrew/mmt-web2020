use crate::{
	graphql::context::CustomContext,
	models::{BasicUser, Booking, BookingTicketUpdate, Ticket, TicketUpdate, User, Vehicle},
};
use bson::{doc, oid::ObjectId, Document};
use futures::{
	future::{join, join4, join3, try_join},
	stream::FuturesUnordered,
	FutureExt, StreamExt,
};
use juniper::{graphql_value, FieldError, FieldResult};
use mmt::{
	db::{Create, Db, Update},
	email::User as EmailUser,
	email::Vehicle as EmailVehicle,
    email::UpdateType
        ,email::TicketTeamUpdate,
};
use std::{collections::HashSet, iter::Iterator};
use stripe::{PaymentIntent, UpdatePaymentIntent};

pub struct MutationRoot;
#[juniper::graphql_object(
    Context = CustomContext
)]
impl MutationRoot {
	async fn newUser(context : &CustomContext, user : BasicUser) -> FieldResult<Option<User>> {
        dbg!(&user);
		let mut user : User = user.into();

        user.email.retain(|c| !c.is_whitespace());
        user.mobile.retain(|c| !c.is_whitespace());

		let mut errors = vec![];

			if user.name.len() == 0 {
				errors.push(("name", "Too short"));
			}

			if user.email.len() == 0 {
				errors.push(("email", "Too short"));
			}

            if !validator::validate_email(&user.email) {
				errors.push(("email", "Invalid: Please use a correct email"));
            }

			if user.mobile.len() == 0 {
				errors.push(("mobile", "Too short"));
			}

            if !user.mobile.chars().all(char::is_numeric) {
                errors.push(("mobile", "Can only contain digits 0-9. No hyphens, text or other markings"));
            }
           
			if user.crew.len() == 0 {
				errors.push(("crew", "Select a crew"));
			}
            

		if !errors.is_empty() {
			let mut o = juniper::Object::with_capacity(2);
			o.add_field("type", "FIELD_VALIDATION".into());
			let errors = errors
				.into_iter()
				.map(|error| {
					let mut o = juniper::Object::with_capacity(2);
					o.add_field("field", error.0.into());
					o.add_field("advice", error.1.into());
					o.into()
				})
				.collect::<Vec<juniper::Value>>();
			o.add_field("advice", juniper::Value::list(errors));

			return Err(juniper::FieldError::new(
				"Field Validation failed",
				o.into(),
			));
		}

		let user_id = match user.create(&context.db).await {
			Ok(inserted_id) => inserted_id,
			Err(_) => {
				return Err(juniper::FieldError::new(
					"User ID not created",
					graphql_value!({"type": "ID_MISSING"}),
				))
			},
		};

		let user = match User::get(&context.db, &user_id).await {
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

		let euser = EmailUser {
			id : user.id.to_hex(),
		};

		let mut rpc_email = (&*context.rpc_email).clone();

		match rpc_email.verify(euser).await{
            Ok(resp) if resp.get_ref().success => {
                Ok(Some(user))
            }
            Ok(_) => {
				Err(juniper::FieldError::new(
					"Failed to send email, please try again later or contact Admin",
					graphql_value!({
						"type": "EMAIL_ERROR"
					}),
				))
            }
            Err(_) => {
				Err(juniper::FieldError::new(
					"User is not owner of booking",
					graphql_value!({
						"type": "USER_BOOKING_NOT_FOUND"
					}),
				))
            }
        }
	}

	async fn verifyUser(
		context : &CustomContext,
		id : ObjectId,
		code : String,
	) -> FieldResult<User> {
		let mut user = match User::get(&context.db, &id).await {
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

	async fn update_booking_tickets(
		context : &CustomContext,
		booking_id : ObjectId,
		tickets : Vec<BookingTicketUpdate>,
	) -> FieldResult<Booking> {
		let booking =
			Booking::get(&context.db, &booking_id)
				.await
				.ok_or(juniper::FieldError::new(
					"Booking not found",
					graphql_value!({"type": "BOOKING_NOT_FOUND"}),
				))?;

		let mut errors = vec![];
		for (i, ticket) in tickets.iter().enumerate() {
			let BookingTicketUpdate {
				user: BasicUser {
					name,
					email,
					mobile,
					crew,
				},
				id,
			} = ticket;
			let id = id
				.as_ref()
				.map(|id| id.to_hex())
				.unwrap_or(format!("{}", i));

			if name.len() == 0 {
				errors.push((i, id.clone(), "name", "Too short"));
			}

			if email.len() == 0 {
				errors.push((i, id.clone(), "email", "Too short"));
			}

			if mobile.len() == 0 {
				errors.push((i, id.clone(), "mobile", "Too short"));
			}

			if crew.len() == 0 {
				errors.push((i, id.clone(), "crew", "Too short"));
			}
		}

		if !errors.is_empty() {
			let mut o = juniper::Object::with_capacity(2);
			o.add_field("type", "FIELD_VALIDATION".into());
			let errors = errors
				.into_iter()
				.map(|error| {
					let mut o = juniper::Object::with_capacity(4);
					o.add_field("idx", (error.0 as i32).into());
					o.add_field("id", error.1.into());
					o.add_field("field", error.2.into());
					o.add_field("advice", error.3.into());
					o.into()
				})
				.collect::<Vec<juniper::Value>>();
			o.add_field("advice", juniper::Value::list(errors));

			return Err(juniper::FieldError::new(
				"Field Validation failed",
				o.into(),
			));
		}

		let updated_tickets : HashSet<ObjectId> =
			futures::stream::iter(tickets.iter().filter(|t| t.id.is_some()))
				.map(|ticket| (ticket.id.clone().unwrap(), ticket.user.clone()))
				.filter_map(|(tid, update)| async move {
					Ticket::get(&context.db, &tid)
						.await
						.map(|t| (tid.clone(), t, update))
				})
				.filter_map(|(tid, real_ticket, update)| async move {
					real_ticket.user(&context).await.map(|u| (tid, u, update))
				})
				.map(|(tid, mut user, update)| {
					let BasicUser {
						name,
						email,
						mobile,
						crew,
					} = update;
					user.name = name;
					user.email = email;
					user.mobile = mobile;
					user.crew = crew;

					(tid, user)
				})
				.filter_map(|(tid, user)| async move {
					user.update(&context.db).await.ok().map(|_| tid)
				})
				.collect()
				.await;

		let new_tickets : HashSet<ObjectId> =
			futures::stream::iter(tickets.iter().filter(|t| t.id.is_none()))
				.map(|t| t.user.clone().into())
				// Insert New User
				.then(|user : User| async move { user.create(&context.db).await })
				.map(|oid_result| oid_result.ok())
				.filter_map(|oid_option| async move { oid_option })
				.map(|user_id| Ticket::new(&booking_id, &user_id))
				//Insert New Ticket for each new user
				.then(|ticket| async move { ticket.create(&context.db).await })
				.map(|oid_result| oid_result.ok())
				.filter_map(|oid_option| async move { oid_option })
				.collect()
				.await;

		let mut original : HashSet<ObjectId> = booking
			.get_tickets(&context)
			.await
			.iter()
			.map(|ticket| ticket.id.clone())
			.collect();

		original = &original - &updated_tickets;
		original = &original - &new_tickets;

		futures::stream::iter(original.iter())
			.filter_map(|tid| async move { Ticket::get(&context.db, &tid).await })
			.then(|ticket| async move {
				ticket.destroy(&context).await;
			})
			.collect::<()>()
			.await;

		Ok(booking)
	}

	async fn add_tickets_to_booking(
		context : &CustomContext,
		booking_id : ObjectId,
		users : Vec<BasicUser>,
	) -> FieldResult<Booking> {
		let booking = Booking::get(&context.db, &booking_id).await;

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
					.map(|user_id| -> Ticket { Ticket::new(&booking_id, user_id) })
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
		let get_ticket = |id : ObjectId| async move { Ticket::get(&context.db, &id).await };

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
						"$set" : user,
					},
					doc! {
						"_id" : user_id
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

	async fn create_stripe_payment_intent_for_booking(
		context : &CustomContext,
		booking_id : ObjectId,
	) -> FieldResult<String> {
		let mut booking = match Booking::get(&context.db, &booking_id).await {
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
				.ok_or(
					FieldError::new(
						"Could not create Payment Intent",
						graphql_value!({"type":"STRIPE_PAYMENT_INTENT"}),
					)
				)?,
		};

		// if let Ok(_) = PaymentIntent::update(
		// 	&context.stripe,
		// 	&spi.id.parse().unwrap(),
		// 	UpdatePaymentIntent {
		// 		payment_method : Some(payment_method_id.parse().unwrap()),
		// 		..UpdatePaymentIntent::default()
		// 	},
		// )
		// .await
		// {
		spi.client_secret.ok_or_else(|| {
			FieldError::new(
				"Booking not found",
				graphql_value!({"type":"BOOKING_NOT_FOUND"}),
			)
		})
		// } else {
		// 	Err(FieldError::new(
		// 		"Could not update PaymentIntent",
		// 		graphql_value!({"type":"STRIPE_ERROR"}),
		// 	))
		// }
	}

	async fn attachStripePaymentMethodToBooking(
		context : &CustomContext,
		booking_id : ObjectId,
		payment_method_id : String,
	) -> FieldResult<String> {
		let mut booking = match Booking::get(&context.db, &booking_id).await {
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

		if let Ok(_) = PaymentIntent::update(
			&context.stripe,
			&spi.id.parse().unwrap(),
			UpdatePaymentIntent {
				payment_method : Some(payment_method_id.parse().unwrap()),
				..UpdatePaymentIntent::default()
			},
		)
		.await
		{
			spi.client_secret.ok_or_else(|| {
				FieldError::new(
					"Booking not found",
					graphql_value!({"type":"BOOKING_NOT_FOUND"}),
				)
			})
		} else {
			Err(FieldError::new(
				"Could not update PaymentIntent",
				graphql_value!({"type":"STRIPE_ERROR"}),
			))
		}
	}

	async fn driver_add_vehicle(
		context : &CustomContext,
		driver_ticket : ObjectId,
		rego : String,
		name : String,
	) -> FieldResult<Vehicle> {
        let mut rego = rego.to_uppercase();
        rego.retain(|c|!c.is_whitespace());

		let mut ticket = match join3(
			Vehicle::find_one(
				&context.db,
				doc! {
					"rego": &rego,
				},
			),
			Ticket::get(&context.db, &driver_ticket),
			Vehicle::find(
				&context.db,
				doc! {
					"driver_ticket": &driver_ticket
				},
			)
			.map(|vdt| vdt.len()),
		)
		.await
		{
			(None, Some(t), 0) => t,
			(
				Some(v),
				Some(Ticket {
					id, ..
				}),
				1,
			) if v.driver_ticket.eq(&id) => return Ok(v),
			(_, Some(_), vdt) if vdt > 0 => {
				return Err(FieldError::new(
					"Ticket already a driver",
					graphql_value!({"type":"DB_ERROR"}),
				));
			},
			(Some(_), _, _) => {
				return Err(FieldError::new(
					"Vehicle already claimed by another Ticket",
					graphql_value!({"type":"DB_ERROR"}),
				));
			},
			(_, None, _) => {
				return Err(FieldError::new(
					"Ticket does not exist",
					graphql_value!({"type":"DB_ERROR"}),
				))
			},
			r => {
				dbg!(r);
				return Err(FieldError::new(
					"I don't know how we go too this state. Look at the logs",
					graphql_value!({"type":"WTF_ERROR"}),
				));
			},
		};

		let vehicle = Vehicle::new(rego, name, &ticket).ok_or(FieldError::new(
			"Rego too short",
			graphql_value!({"type":"DB_ERROR"}),
		))?;

		vehicle.create(&context.db).await.map_err(|_| {
			FieldError::new(
				"Could not create Vehicle",
				graphql_value!({"type":"DB_ERROR"}),
			)
		})?;

		ticket.vehicle_id = Some(vehicle.id.clone());
		ticket.update(&context.db).await.map_err(|_| {
			FieldError::new(
				"Could not add Vehicle ID to Ticket",
				graphql_value!({"type":"DB_ERROR"}),
			)
		})?;

		Ok(vehicle)
	}

	async fn passenger_add_vehicle(
		context : &CustomContext,
		passenger_ticket : ObjectId,
		rego : String,
	) -> FieldResult<Vehicle> {
        let mut rego = rego.to_uppercase();
        rego.retain(|c|!c.is_whitespace());

		let (mut vehicle, ticket) = match join4(
			Vehicle::find_one(
				&context.db,
				doc! {
					"rego": &rego,
				},
			),
			Ticket::get(&context.db, &passenger_ticket),
			Vehicle::find(
				&context.db,
				doc! {
					"requested_tickets": {
						"$all": [&passenger_ticket]
					}
				},
			)
			.map(|vdt| vdt.len()),
            Vehicle::find_one(
                &context.db,
                doc! {
                    "driver_ticket": &passenger_ticket,
                }
            )
		)
		.await
		{
			(_, _, _, Some(_)) => {
				return Err(FieldError::new(
					"You are the driver of a team, you cannot join another team. If you want this changed, please contact Admin",
					graphql_value!({"type":"ALREADY_DRIVER_OF_TEAM"}),
				));
			},
            // Ticket already in vehicle
			(Some(vehicle), Some(ticket), _, _) if vehicle.requested_tickets.contains(&ticket.id) => {
                eprintln!("Ticket already contained in Vehicle");
				return Ok(vehicle);
			},
			(_, _, number_of_vehicles, _) if number_of_vehicles > 0 => {
				return Err(FieldError::new(
					"You are already part of another Team/Vehicle",
					graphql_value!({"type":"ALREADY_IN_TEAM"}),
				));
			},
			(None, _, _, _) => {
				return Err(FieldError::new(
					"Vehicle does not exist",
					graphql_value!({"type":"VEHICLE_NOT_FOUND"}),
				));
			},
			(_, None, _, _) => {
				return Err(FieldError::new(
					"Ticket does not exist",
					graphql_value!({"type":"TICKET_NOT_FOUND"}),
				));
			},
			(Some(vehicle), Some(ticket), _, _) => (vehicle, ticket),
		};

        dbg!();
		vehicle.request_ticket(&ticket);
		let ret = vehicle
			.update(&context.db)
			.await
			.map(|_| vehicle)
			.map_err(|_| {
				FieldError::new(
					"Could not update database with requested ticket",
					graphql_value!({"type":"DB_ERROR"}),
				)
			})?;

		let user = ticket.user(&context).await.unwrap();

		let evehicle = EmailVehicle {
			id : ret.id.to_hex(),
		};

		let mut rpc_email = (&*context.rpc_email).clone();

		rpc_email
			.notify_driver_new_passenger(evehicle)
			.await
			.map(|r| {
				dbg!(r.into_inner());
				Some(user)
			})
			.map_err(|_| {
				juniper::FieldError::new(
					"User is not owner of booking",
					graphql_value!({
						"type": "USER_BOOKING_NOT_FOUND"
					}),
				)
			})?;

		Ok(ret)
	}

	async fn vehicle_accept_ticket(
		context : &CustomContext,
		vehicle : ObjectId,
		ticket : ObjectId,
	) -> FieldResult<Vehicle> {
		// Get Vehicle and Ticket at the same time from mongo...
		// If there is an error then exit
		let (mut vehicle, mut ticket) = match join(
			Vehicle::get(&context.db, &vehicle),
			Ticket::get(&context.db, &ticket),
		)
		.await
		{
			(Some(v), Some(t)) => (v, t),
			_ => {
				return Err(FieldError::new(
					"Vehicle and/or Ticket does not exist",
					graphql_value!({"type":"DB_ERROR"}),
				))
			},
		};

		if let None = vehicle.requested_tickets.iter().find(|t| t == &&ticket.id) {
			return Err(FieldError::new(
				"Ticket has not requested to be added to this vehicle",
				graphql_value!({"type":"UNWARRENTED_TICKET_REQUEST"}),
			));
		};

		// Set the ticket vehicle officially
		ticket.vehicle_id = Some(vehicle.id.clone());
		// remove the request from the vehicle
		vehicle.requested_tickets.retain(|t| t != &ticket.id);

		// write out both operations simultaneously
		try_join(ticket.update(&context.db), vehicle.update(&context.db))
			.await
			.map_err(|_| {
				FieldError::new(
					"Booking not found",
					graphql_value!({"type":"BOOKING_NOT_FOUND"}),
				)
			})?;

let mut rpc_email = (&*context.rpc_email).clone();

let update = TicketTeamUpdate {
    vehicle_id:vehicle.id.to_string(),
    ticket_id:ticket.id.to_string(),
        update_type: UpdateType::Accept as i32,
};

		rpc_email.ticket_team_update(update).await
			.map(|r| {
				dbg!(r.into_inner());
                vehicle
			})
			.map_err(|_| {
				juniper::FieldError::new(
					"Failed to send email",
					graphql_value!({
						"type": "EMAIL_ERROR"
					}),
				)
			})
	}

	async fn vehicle_decline_ticket(
		context : &CustomContext,
		vehicle : ObjectId,
		ticket : ObjectId,
	) -> FieldResult<Vehicle> {
		// Get Vehicle and Ticket at the same time from mongo...
		// If there is an error then exit
		let (mut vehicle, ticket) = match join(
			Vehicle::get(&context.db, &vehicle),
			Ticket::get(&context.db, &ticket),
		)
		.await
		{
			(Some(v), Some(t)) => (v, t),
			_ => {
				return Err(FieldError::new(
					"Vehicle and/or Ticket does not exist",
					graphql_value!({"type":"DB_ERROR"}),
				))
			},
		};

        // Remove only the ticket id specified
        vehicle.requested_tickets.retain(|t| t != &ticket.id);

		// write out both operations simultaneously
		vehicle.update(&context.db)
			.await
			.map_err(|_| {
				FieldError::new(
					"Booking not found",
					graphql_value!({"type":"BOOKING_NOT_FOUND"}),
				)
			})?;

let mut rpc_email = (&*context.rpc_email).clone();

let update = TicketTeamUpdate {
    vehicle_id:vehicle.id.to_string(),
    ticket_id:ticket.id.to_string(),
        update_type: UpdateType::Decline as i32,
};

		rpc_email.ticket_team_update(update).await
			.map(|r| {
				dbg!(r.into_inner());
                vehicle
			})
			.map_err(|_| {
				juniper::FieldError::new(
					"Failed to send email",
					graphql_value!({
						"type": "EMAIL_ERROR"
					}),
				)
			})
	}

	async fn vehicle_remove_ticket(
		context : &CustomContext,
		vehicle : ObjectId,
		ticket : ObjectId,
	) -> FieldResult<Vehicle> {
		// Get Vehicle and Ticket at the same time from mongo...
		// If there is an error then exit
		let (vehicle, mut ticket) = match join(
			Vehicle::get(&context.db, &vehicle),
			Ticket::get(&context.db, &ticket),
		)
		.await
		{
			(Some(v), Some(t)) => (v, t),
			_ => {
				return Err(FieldError::new(
					"Vehicle and/or Ticket does not exist",
					graphql_value!({"type":"DB_ERROR"}),
				))
			},
		};

        if vehicle.driver_ticket == ticket.id {
				return Err(FieldError::new(
					"Cannot remove driver from Team",
					graphql_value!({"type":"DRIVER_OWNS_TEAM"}),
				))
        }

        ticket.vehicle_id = None;

		// write out both operations simultaneously
		ticket.update(&context.db)
			.await
			.map_err(|_| {
				FieldError::new(
					"Booking not found",
					graphql_value!({"type":"BOOKING_NOT_FOUND"}),
				)
			})?;

let mut rpc_email = (&*context.rpc_email).clone();

let update = TicketTeamUpdate {
    vehicle_id:vehicle.id.to_string(),
    ticket_id:ticket.id.to_string(),
        update_type: UpdateType::Remove as i32,
};

		rpc_email.ticket_team_update(update).await
			.map(|r| {
				dbg!(r.into_inner());
                vehicle
			})
			.map_err(|_| {
				juniper::FieldError::new(
					"Failed to send email",
					graphql_value!({
						"type": "EMAIL_ERROR"
					}),
				)
			})

	}
}
