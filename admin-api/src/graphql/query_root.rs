use crate::{
	graphql::context::CustomContext,
	models::{Booking, Vehicle},
};
use mmt::{Db};
use juniper::FieldResult;

pub struct QueryRoot;
#[juniper::graphql_object(
    Context = CustomContext,
)]
impl QueryRoot {
	/// All bookings
	async fn bookings(context : &CustomContext) -> FieldResult<Vec<Booking>> {
		Ok(Booking::all(&context.db).await)
	}

	/// All vehicles
	async fn vehicles(context : &CustomContext) -> FieldResult<Vec<Vehicle>> {
		Ok(Vehicle::all(&context.db).await)
	}
}
