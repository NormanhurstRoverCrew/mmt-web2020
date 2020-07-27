use crate::{db::helpers as DBHelper, graphql::context::CustomContext, models::Booking};
use juniper::FieldResult;

pub struct QueryRoot;
#[juniper::graphql_object(
    Context = CustomContext,
)]
impl QueryRoot {
	/// All bookings
	async fn bookings(context : &CustomContext) -> FieldResult<Vec<Booking>> {
		let bookings = context.bookings_handel();

		let booking = DBHelper::all(&bookings);

		Ok(booking.await)
	}
}
