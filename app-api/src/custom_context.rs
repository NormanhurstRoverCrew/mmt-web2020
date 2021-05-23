use bson::doc;
use mongodb::Database;
use std::sync::Arc;

use crate::models::admin_user::AdminUser;

pub struct CustomContext {
    pub db: Arc<Database>,
    pub admin_user: AdminUser,
}

impl CustomContext {
    pub async fn index(&self, i: &str) -> i32 {
        let indexes = self.db.collection("indexes");
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
                    return doc.get_i32("seq").expect("Sequence Number");
                }
                _ => {
                    indexes
                        .insert_one(
                            doc! {
                                    "name" :i,
                                    "seq" :1,
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
