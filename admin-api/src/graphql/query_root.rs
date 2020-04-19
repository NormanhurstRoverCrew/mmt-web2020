use crate::{db::helpers as DBHelper, graphql::context::SharedContext, models::Booking};
use juniper::FieldResult;

pub struct QueryRoot;
#[juniper::graphql_object(
    Context = SharedContext,
)]
impl QueryRoot {
	/// All bookings
	fn bookings(context : &SharedContext) -> FieldResult<Vec<Booking>> {
		let bookings = context.bookings_handel();

		let booking = DBHelper::all(&bookings);

		Ok(booking)
	}
}
