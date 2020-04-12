use crate::authorization::Permissions;
use rocket::{Route};
use crate::api::client;
use crate::types::point_log::PointLog;
use rocket_contrib::json::Json;
use reqwest::{Client,Response, Url, StatusCode};

#[post("/", format = "json", data = "<logs>")]
fn add(_perm: Permissions, logs: Json<Vec<PointLog>>) -> rocket::http::Status {

    let mut sum_of_errors: isize = 0;
    for log in logs.iter() {
        sum_of_errors += match create_point_log_for_team(log) {
            Ok(_) => 0,
            Err(_) => 1,
        }
    }
	
    match sum_of_errors {
        0 => rocket::http::Status::Ok,
        _ => rocket::http::Status::InternalServerError
    }
	
}

pub enum PointLogError {
    RequestFailed
}

fn create_point_log_for_team(log: &PointLog) -> Result<(), PointLogError> {
    let url: Url = get_create_point_log_url_for_team(log.team_id);

    let client: Client = client().expect("Could not create client");
    let resp: Response = client.post(url)
        .json(log)
        .send()
        .expect("Request failed");
    match resp.status() {
        StatusCode::OK | StatusCode::CREATED => Ok(()),
        _ => Err(PointLogError::RequestFailed)
    }
}

fn get_create_point_log_url_for_team(uid: isize) -> Url {
    let url: Url = Url::parse("http://backend:3000/api/teams/").expect("Base url invalid");
	url.join(&format!("{}/point_logs", &uid)).expect("Could not append id to url")
}

pub fn routes() -> Vec<Route> {
	routes![add]
}