use super::Permissions;
use rocket::{Route};
use super::super::api::client;
use super::super::types::team::Team;
use super::super::types::point_log::PointLog;
use super::super::types::ticket::Ticket;
use rocket_contrib::json::Json;
use reqwest::{Client,Response, Url};
use serde::{Serialize};

#[get("/")]
fn index(_perm: Permissions) -> Json<Vec<Team>> {
	let client: Client = client().expect("Could not create client");
	let mut resp: Response = client.get("http://backend:3000/api/teams").send().expect("Request failed");
	let teams: Vec<Team> = resp.json().expect("Not valid JSON");
	let teams: Vec<Team> = teams.into_iter().map(|mut team| {
		team.get_users();
		team
	}).collect();
	Json(teams)
}

#[get("/<id>?<load_tickets>")]
fn show(_perm: Permissions, id: isize, load_tickets: Option<bool>) -> Json<Team> {
	let client: Client = client().expect("Could not create client");
	let url: Url = Url::parse("http://backend:3000/api/teams/").expect("Base url invalid");
	let url = url.join(&id.to_string()).expect("Could not append id to url");
	let mut resp: Response = client.get(url).send().expect("Request failed");
	let mut team: Team = resp.json().expect("Not valid JSON");
	match load_tickets {
		Some(true) => team.get_users(),
		_ => {}
	};
	Json(team)
}

#[post("/<id>/point_logs", format = "json", data = "<logs>")]
fn create_point_log(_perm: Permissions, id: isize, logs: Json<Vec<PointLog>>) -> Json<()> {
	let client: Client = client().expect("Could not create client");
	let url: Url = Url::parse("http://backend:3000/api/teams/").expect("Base url invalid");
	let url = url.join(&format!("{}/point_logs", &id.to_string())).expect("Could not append id to url");
	// println!("{}", serde_json::to_string(&logs.into_inner()).expect("Something"));
	let logs = logs.into_inner();
	for log in logs.into_iter() {
		let mut _resp: Response = client.post(url.as_ref())
			.json(&log)
			.send()
			.expect("Request failed");
	}
	// let team: Team = resp.json().expect("Not valid JSON");
	Json(())
}

#[get("/<id>/point_logs")]
fn get_point_logs(_perm: Permissions, id: isize) -> Json<Vec<PointLog>> {
	let client: Client = client().expect("Could not create client");
	let url: Url = Url::parse("http://backend:3000/api/teams/").expect("Base url invalid");
	let url = url.join(&format!("{}/point_logs", &id.to_string())).expect("Could not append id to url");
	// println!("{}", serde_json::to_string(&logs.into_inner()).expect("Something"));
	let mut resp: Response = client.get(url)
		.send()
		.expect("Request failed");
	// println!("bruh {:?}", &resp.text());
	let pls: Vec<PointLog> = resp.json().expect("Not valid JSON");
	Json(pls)
}


#[derive(Debug, Serialize)]
struct TicketUidOnly {
    uid: Option<String>
}

#[put("/<team_id>/ticket", format = "json", data = "<ticket>")]
fn add_ticket_to_team(_perm: Permissions, team_id: isize, ticket: Json<Ticket>) -> Json<()> {
	let client: Client = client().expect("Could not create client");
	let url: Url = Url::parse("http://backend:3000/api/teams/").expect("Base url invalid");
	let url = url.join(&format!("{}/ticket", &team_id.to_string())).expect("Could not append id to url");
	// println!("{}", serde_json::to_string(&logs.into_inner()).expect("Something"));
	let ticket = ticket.into_inner();
	let t_uid = TicketUidOnly {
		uid: ticket.uid()
	};
	let mut _resp: Response = client.patch(url.as_ref())
		.json(&t_uid)
		.send()
		.expect("Request failed");

	// let team: Team = resp.json().expect("Not valid JSON");
	Json(())
}

#[put("/<team_id>", format = "json", data = "<team>")]
fn update_team(_perm: Permissions, team_id: isize, team: Json<Team>) -> Json<()> {
	let client: Client = client().expect("Could not create client");
	let url: Url = Url::parse("http://backend:3000/api/teams/").expect("Base url invalid");
	let url = url.join(&format!("{}", &team_id.to_string())).expect("Could not append id to url");
	// println!("{}", serde_json::to_string(&logs.into_inner()).expect("Something"));
	let team = team.into_inner();
	let mut _resp: Response = client.patch(url.as_ref())
		.json(&team)
		.send()
		.expect("Request failed");

	// let team: Team = resp.json().expect("Not valid JSON");
	Json(())
}


pub fn routes() -> Vec<Route> {
	routes![index, show, create_point_log, get_point_logs, add_ticket_to_team, update_team]
}