use crate::{
	db::helpers as DBHelper,
	graphql::{context::Database, util::string_to_id},
	models::{Booking, User},
};
use juniper::{graphql_value, FieldResult};

pub struct QueryRoot;
#[juniper::object(
    Context = Database,
)]
impl QueryRoot {
	/// All bookings
	fn booking(context : &Database, id : String) -> FieldResult<Booking> {
		let bookings = context.bookings_handel();
		let id = match string_to_id(&id) {
			Ok(id) => id,
			Err(e) => return Err(e),
		};

		let booking = match DBHelper::get(&bookings, &id) {
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
	fn booking_from_user(context : &Database, id : String) -> FieldResult<Booking> {
		let id = match string_to_id(&id) {
			Ok(id) => id,
			Err(e) => return Err(e),
		};

		let users = context.users_handel();

		let user : User = match DBHelper::get(&users, &id) {
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

		match user.get_booking(&context) {
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
}
