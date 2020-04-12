use super::super::api::client;
use super::Create;
use super::Ticket;
use reqwest::{Client, Response, Url};

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct Teams {
    #[serde(flatten)]
    pub teams: Vec<Team>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Team {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<isize>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub uid: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub registration: Option<String>,

    #[serde(skip_deserializing, skip_serializing_if = "Option::is_none")]
    pub tickets: Option<Vec<Ticket>>,
}

impl Team {
    pub fn new() -> Team {
        Team {
            id: None,
            uid: None,
            name: None,
            registration: None,
            tickets: None,
        }
    }

    pub fn get_users(&mut self) {
        let client: Client = client().expect("Could not create client");
        let url: Url = Url::parse("http://backend:3000/api/teams/").expect("Base url invalid");
	    let url = url.join(&format!("{}/tickets", self.id.unwrap())).expect("Could not append id to url");
        let mut resp: Response = client.get(url).send().expect("Request failed");
        let tickets: Vec<Ticket> = resp.json().expect("Could not convert JSON to Ticket");
        self.tickets = Some(tickets);
    }
}

impl Clone for Team {
    fn clone(&self) -> Self {
        Team {
            id: self.id.clone(),
            uid: self.uid.clone(),
            name: self.name.clone(),
            registration: self.registration.clone(),
            tickets: self.tickets.clone(),
        }
    }
}

impl Create for Team {
    fn create(&self) -> Self {
        self.clone()
    }
}
