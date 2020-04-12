use super::Permissions;
use rocket::{Route};
use super::super::api::client;
use super::super::types::payment::Payment;
use rocket_contrib::json::Json;
use reqwest::{Client,Response, Url};

#[get("/<uid>")]
fn show(_perm: Permissions, uid: String) -> Json<Payment> {
	let client: Client = client().expect("Could not create client");
	let url: Url = Url::parse("http://backend:3000/api/tickets/").expect("Base url invalid");
	let url = url.join(&format!("{}/paid", &uid)).expect("Could not append id to url");
	let mut resp: Response = client.get(url).send().expect("Request failed");
	let payment: Payment = resp.json().expect("Not valid JSON");
	Json(payment)
}

pub fn routes() -> Vec<Route> {
	routes![show]
}