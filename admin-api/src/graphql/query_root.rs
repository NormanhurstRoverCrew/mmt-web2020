use crate::{db::helpers as DBHelper, graphql::context::Database, models::Booking};
use juniper::FieldResult;

pub struct QueryRoot;
#[juniper::object(
    Context = Database,
)]
impl QueryRoot {
	/// All bookings
	fn bookings(context : &Database) -> FieldResult<Vec<Booking>> {
		let bookings = context.bookings_handel();

		let booking = DBHelper::all(&bookings);

		Ok(booking)
	}
}
