use super::{User, Ticket};
use super::Create;

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Booking {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uid: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<User>,
    pub tickets: Vec<Ticket>,
}

impl Booking {
    pub fn new() -> Booking {
        Booking {
            uid: None,
            tickets: Vec::new(),
            user: None,
        }
    }

    pub fn add_ticket(&mut self, ticket: Ticket) {
        self.tickets.push(ticket);
    }
    
    pub fn user(&mut self, user: &User) {
        self.user = Some(user.clone());
    }
}

impl Clone for Booking {
    fn clone(&self) -> Self {
        Booking {
            uid: self.uid.clone(),
            user: self.user.clone(),
            tickets: self.tickets.clone(),
        }
    }
}

impl Create for Booking {
    fn create(&self) -> Self {
        self.clone()
    }
}