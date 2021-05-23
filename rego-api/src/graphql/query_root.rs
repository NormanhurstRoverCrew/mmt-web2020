use crate::{
	graphql::context::CustomContext,
	models::{Booking, Ticket, User, Vehicle, TICKET_PRICE},
};
use bson::{doc, oid::ObjectId};
use juniper::{graphql_value, FieldResult};
use mmt::db::Db;

pub struct QueryRoot;
#[juniper::graphql_object(
    Context = CustomContext,
)]
impl QueryRoot {
	/// All bookings
	async fn booking(context : &CustomContext, id : ObjectId) -> FieldResult<Booking> {
		let booking = match Booking::get(&context.db, &id).await {
			Some(booking) => booking,
			None => {
				return Err(juniper::FieldError::new(
					"Booking not found",
					graphql_value!({
						"type": "BOOKING_NOT_FOUND"
					}),
				))
			},
		};

		Ok(booking)
	}

	/// Get a user
	async fn booking_from_user(context : &CustomContext, id : ObjectId) -> FieldResult<Booking> {
		let user = match User::get(&context.db, &id).await {
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

		match user.get_booking(&context).await {
			Some(booking) => Ok(booking),
			None => {
				return Err(juniper::FieldError::new(
					"User is not owner of booking",
					graphql_value!({
						"type": "USER_BOOKING_NOT_FOUND"
					}),
				))
			},
		}
	}

	fn ticket_price() -> f64 { TICKET_PRICE }

	/// If ticket is a driver, return the vehicle they own
	async fn vehicle_from_ticket(
		context : &CustomContext,
		ticket_id : ObjectId,
	) -> FieldResult<Option<Vehicle>> {
		let vehicle = Vehicle::find_one(
			&context.db,
			doc! {
				"driver_ticket": &ticket_id
			},
		)
		.await;

		Ok(vehicle)
	}

	/// If ticket is a driver, return the vehicle they own
	async fn driver_name_from_rego(context : &CustomContext, rego : String) -> FieldResult<String> {
		let vehicle = match Vehicle::find_one(
			&context.db,
			doc! {
				"rego": &rego
			},
		)
		.await
		{
			Some(v) => v,
			None => {
				return Err(juniper::FieldError::new(
					"Vehicle does not exist",
					graphql_value!({
						"type": "VEHICLE_NOT_FOUND"
					}),
				));
			},
		};

		let ticket = match Ticket::find_one(
			&context.db,
			doc! {
				"_id": &vehicle.driver_ticket,
			},
		)
		.await
		{
			Some(t) => t,
			None => {
				return Err(juniper::FieldError::new(
					"Driver has gone missing!",
					graphql_value!({
						"type": "TICKET_NOT_FOUND"
					}),
				));
			},
		};

		match ticket.user(&context).await {
			Some(user) => Ok(user.name.clone()),
			None => {
				return Err(juniper::FieldError::new(
					"Drivers name has gone missing",
					graphql_value!({
						"type": "USER_NOT_FOUND"
					}),
				));
			},
		}
	}
}
