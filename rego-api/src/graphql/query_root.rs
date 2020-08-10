use crate::{
	db::{Db},
	graphql::{context::CustomContext, util::string_to_id},
	models::{Booking, User, TICKET_PRICE, Vehicle},
};
use bson::oid::ObjectId;
use bson::doc;
use juniper::{graphql_value, FieldResult};

pub struct QueryRoot;
#[juniper::graphql_object(
    Context = CustomContext,
)]
impl QueryRoot {
	/// All bookings
	async fn booking(context : &CustomContext, id : ObjectId) -> FieldResult<Booking> {
		let booking = match Booking::get(&context, &id).await {
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
		let user = match User::get(&context, &id).await {
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
    async fn vehicle_from_ticket(context : &CustomContext, ticket_id: ObjectId) -> FieldResult<Option<Vehicle>> {
        let vehicle = Vehicle::find_one(&context, doc! {
            "driver_ticket": &ticket_id
        }).await;

        Ok(vehicle)
    }
}
