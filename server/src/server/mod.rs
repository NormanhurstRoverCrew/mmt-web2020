use super::authorization::Permissions;
use rocket::{Route};
pub mod teams;
pub mod tickets;
pub mod payments;
pub mod base;

pub fn get_routes() -> Vec<Route> {
	routes![hello_auth]
}

#[get("/", rank = 0)]
fn hello_auth(perm: Permissions) -> String {
	format!("You are authorized\n{:?}\n", perm)
}