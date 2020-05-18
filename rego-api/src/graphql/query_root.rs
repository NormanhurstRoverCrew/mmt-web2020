use crate::{
	db::helpers as DBHelper,
	graphql::{context::CustomContext, util::string_to_id},
	models::{Booking, User, TICKET_PRICE},
};
use juniper::{graphql_value, FieldResult};

pub struct QueryRoot;
#[juniper::graphql_object(
    Context = CustomContext,
)]
impl QueryRoot {
	/// All bookings
	async fn booking(context : &CustomContext, id : String) -> FieldResult<Booking> {
		let bookings = context.bookings_handel();
		let id = match string_to_id(&id) {
			Ok(id) => id,
			Err(e) => return Err(e),
		};

		let booking = match DBHelper::get(&bookings, &id).await {
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
	async fn booking_from_user(context : &CustomContext, id : String) -> FieldResult<Booking> {
		let id = match string_to_id(&id) {
			Ok(id) => id,
			Err(e) => return Err(e),
		};

		let users = context.users_handel();

		let user : User = match DBHelper::get(&users, &id).await {
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
}
