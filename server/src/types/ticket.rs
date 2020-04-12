use super::User;
use super::Create;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Ticket {
    #[serde(skip_serializing_if = "Option::is_none")]
    uid: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<isize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    team_id: Option<isize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    user: Option<User>,
}

impl Ticket {
    pub fn new(user: &User) -> Ticket {
        Ticket {
            uid: None,
            team_id: None,
            id: None,
            user: Some(user.clone()),
        }
    }

    pub fn uid(&self) -> Option<String> {
        match &self.uid {
            Some(uid) => Some(uid.clone()),
            None => None
        }
    }
}

impl Clone for Ticket {
    fn clone(&self) -> Self {
        Ticket {
            uid: self.uid.clone(),
            team_id: self.team_id.clone(),
            id: self.id,
            user: self.user.clone(),
        }
    }
}

impl Create for Ticket {
    fn create(&self) -> Self {
        self.clone()
    }
}