use super::Permissions;
use rocket::{Route};
use super::super::api::client;
use super::super::types::ticket::Ticket;
use rocket_contrib::json::Json;
use reqwest::{Client,Response, Url};


#[get("/")]
fn index(_perm: Permissions) -> Json<Vec<Ticket>> {
	let client: Client = client().expect("Could not create client");
	let mut resp: Response = client.get("http://backend:3000/api/tickets").send().expect("Request failed");
	let tickets: Vec<Ticket> = resp.json().expect("Not valid JSON");
	Json(tickets)
}

#[get("/<id>")]
fn show(_perm: Permissions, id: isize) -> Json<Ticket> {
	let client: Client = client().expect("Could not create client");
	let url: Url = Url::parse("http://backend:3000/api/tickets/").expect("Base url invalid");
	let mut url = url.join(&id.to_string()).expect("Could not append id to url");
	url.set_query(Some("byid=true"));
	let mut resp: Response = client.get(url).send().expect("Request failed");
	let ticket: Ticket = resp.json().expect("Not valid JSON");
	Json(ticket)
}

pub fn routes() -> Vec<Route> {
	routes![index, show]
}