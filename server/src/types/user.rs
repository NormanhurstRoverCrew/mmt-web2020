use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct User {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uid: Option<String>,

    pub name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub crew: Option<String>,
    pub mobile: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub diet: Option<String>,
}

impl User {
    pub fn new(name: String, mobile: String, crew: String) -> User {
        User {
            uid: None,
            name: name,
            crew: Some(crew),
            mobile: mobile,
            diet: None,
        }
    }
}

impl Clone for User {
    fn clone(&self) -> Self {
        User {
            uid: self.uid.clone(),
            name: self.name.clone(),
            crew: self.crew.clone(),
            mobile: self.mobile.clone(),
            diet: self.diet.clone(),
        }
    }
}