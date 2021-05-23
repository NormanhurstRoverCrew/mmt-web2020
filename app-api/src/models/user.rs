use crate::CustomContext;
use bson::oid::ObjectId;
use mmt::DB;
use serde::{Deserialize, Serialize};

#[DB(users)]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct User {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub name: String,
    pub email: String,
    pub mobile: String,
    pub crew: String,
    pub diet: Option<String>,
    pub email_verified: bool,

    // Used to verify if the supplied email is valid
    code: Option<String>,
}

#[juniper::graphql_object(
    Context = CustomContext,
)]
impl User {
    fn id(&self) -> &ObjectId {
        &self.id
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn email(&self) -> &str {
        &self.email
    }

    fn mobile(&self) -> &str {
        &self.mobile
    }

    fn crew(&self) -> &str {
        &self.crew
    }

    fn diet(&self) -> Option<&String> {
        self.diet.as_ref()
    }
}
