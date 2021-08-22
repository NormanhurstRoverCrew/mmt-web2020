use bson::oid::ObjectId;
use mmt::{Update, DB};
use mongodb::Database;
use rand::Rng;
use serde::{Deserialize, Serialize};

#[DB(users)]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct User {
    #[serde(rename = "_id")]
    id: ObjectId,
    name: String,
    email: String,
    code: Option<String>,
    email_verified: bool,
}

#[allow(unused)]
impl User {
    pub fn id(&self) -> &ObjectId {
        &self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn email(&self) -> &str {
        &self.email
    }

    pub fn code(&self) -> Option<&str> {
        self.code.as_ref().map(|code| code.as_str())
    }

    pub async fn generate_code(&self, db: &Database) -> String {
        if self.code == None {
            let mut user = self.clone();
            let code = rand::thread_rng()
                .sample_iter(&rand::distributions::Alphanumeric)
                .take(16)
                .map(|c| c as char)
                .collect::<String>();
            let ret = code.clone();

            user.code = Some(code);
            if user.update(db).await.is_ok() {
                ret
            } else {
                String::from("")
            }
        } else {
            self.code.as_ref().unwrap().clone()
        }
    }

    pub fn verified(&self) -> bool {
        self.email_verified
    }
}
