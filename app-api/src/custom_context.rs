use bson::doc;
use mongodb::Database;
use serde::Deserialize;
use serde::Serialize;
use std::sync::Arc;
use stripe::Client;

use crate::models::admin_user::AdminUser;

pub struct CustomContext {
    pub db: Arc<Database>,
    pub admin_user: AdminUser,
    pub stripe: Arc<Client>,
}

impl CustomContext {
    pub async fn index(&self, i: &str) -> i32 {
        let indexes = self.db.collection::<Index>("indexes");
        loop {
            match indexes
                .find_one_and_update(
                    doc! {
                            "name" :i,
                    },
                    doc! {
                            "$inc" :{
                                    "seq" :1,
                            }
                    },
                    None,
                )
                .await
            {
                Ok(Some(doc)) => {
                    return doc.seq;
                }
                _ => {
                    indexes
                        .insert_one(
                            Index {
                                name: i.to_owned(),
                                seq: 1,
                            },
                            None,
                        )
                        .await
                        .expect("Insert new index");
                }
            };
        }
    }
}

impl juniper::Context for CustomContext {}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Index {
    name: String,
    seq: i32,
}
