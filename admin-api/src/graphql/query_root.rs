use crate::{
	db::Db,
	graphql::context::CustomContext,
	models::{Booking, Vehicle},
};
use juniper::FieldResult;

pub struct QueryRoot;
#[juniper::graphql_object(
    Context = CustomContext,
)]
impl QueryRoot {
	/// All bookings
	async fn bookings(context : &CustomContext) -> FieldResult<Vec<Booking>> {
		Ok(Booking::all(&context).await)
	}

	/// All vehicles
	async fn vehicles(context : &CustomContext) -> FieldResult<Vec<Vehicle>> {
		Ok(Vehicle::all(&context).await)
	}
}
