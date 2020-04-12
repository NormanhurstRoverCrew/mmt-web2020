use super::Create;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Payment {
    pub paid: bool,
    pub due: f32,
}

impl Clone for Payment {
    fn clone(&self) -> Self {
        Payment {
            paid: self.paid.clone(),
            due: self.due.clone(),
        }
    }
}

impl Create for Payment {
    fn create(&self) -> Self {
        self.clone()
    }
}