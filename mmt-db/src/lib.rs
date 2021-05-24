use async_trait::async_trait;
use bson::{doc, oid::ObjectId, Document};
use futures::StreamExt;
use mongodb::results::DeleteResult;
use mongodb::results::UpdateResult;
use mongodb::Database;
use std::error::Error;

use serde::{Deserialize, Serialize};

#[async_trait]
pub trait Db<'a>: Send + Sized + for<'de> Deserialize<'de> + 'static {
    const COLLECTION: &'static str;

    async fn all(db: &'a Database) -> Vec<Self> {
        db.collection(Self::COLLECTION)
            .find(None, None)
            .await
            .unwrap()
            .filter_map(|item| async move { item.ok() })
            .map(|doc| bson::from_document(doc).expect("Decode error"))
            .collect()
            .await
    }

    async fn find_one(db: &'a Database, search: Document) -> Option<Self> {
        db.collection(Self::COLLECTION)
            .find_one(Some(search), None)
            .await
            .expect("DB Error")
            .map(|doc| bson::from_document(doc).expect("Decode error"))
    }

    async fn find(db: &'a Database, search: Document) -> Vec<Self> {
        db.collection(Self::COLLECTION)
            .find(Some(search), None)
            .await
            .expect("DB Error")
            .filter_map(|doc| async move { doc.ok() })
            .filter_map(|doc| async move { bson::from_document(doc).ok() })
            .collect()
            .await
    }

    async fn find_ids(db: &'a Database, ids: &Vec<ObjectId>) -> Vec<Self> {
        Self::find(
            db,
            doc! {
                "_id": {
                    "$in": ids,
                }
            },
        )
        .await
    }

    async fn get(db: &'a Database, id: &ObjectId) -> Option<Self> {
        Self::find_one(
            db,
            doc! {
                    "_id" : id,
            },
        )
        .await
    }

    async fn search(db: &'a Database, search: Document) -> Vec<Self> {
        db.collection(Self::COLLECTION)
            .find(Some(search), None)
            .await
            .unwrap()
            .filter_map(|item| async move { item.ok() })
            .map(|doc| bson::from_document(doc).expect("Decode error"))
            .collect()
            .await
    }
}

#[async_trait]
pub trait Create: Serialize {
    const COLLECTION: &'static str;

    async fn create(&self, db: &Database) -> Result<ObjectId, Box<dyn Error>> {
        let doc = bson::to_document(&self).unwrap().to_owned();

        db.collection(Self::COLLECTION)
            .insert_one(doc, None)
            .await
            .map_err(|e| e.into())
            .map(|i| i.inserted_id.as_object_id().unwrap().to_owned())
    }
}

#[async_trait]
pub trait Update: Serialize {
    const COLLECTION: &'static str;

    async fn update(&self, db: &Database) -> Result<UpdateResult, Box<dyn Error>> {
        let doc = bson::to_document(&self).unwrap().to_owned();
        let doc = doc! {
            "$set": doc
        };

        let id = doc.get("_id").map(|id| id.as_object_id()).flatten();

        let selector = match id {
            Some(id) => {
                doc! {
                    "_id": id
                }
            }
            None => return Err("Serialized Document does not contain \"_id\"".into()),
        };

        let coll = db.collection::<Document>(Self::COLLECTION);
        coll.update_one(selector, doc, None)
            .await
            .map_err(|e| e.into())
    }
}

#[async_trait]
pub trait Delete: Serialize {
    const COLLECTION: &'static str;

    async fn delete(&self, db: &Database) -> Result<DeleteResult, Box<dyn Error>> {
        let doc = bson::to_document(&self).unwrap().to_owned();

        let id = doc.get("_id").map(|id| id.as_object_id()).flatten();

        let selector = match id {
            Some(id) => {
                doc! {
                    "_id": id
                }
            }
            None => return Err("Serialized Document does not contain \"_id\"".into()),
        };

        db.collection::<Document>(Self::COLLECTION)
            .delete_one(selector, None)
            .await
            .map_err(|e| e.into())
    }
}
