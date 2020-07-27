use bson::{doc, oid::ObjectId, Document};
use futures::StreamExt;
use mongodb::Collection;
use serde::Deserialize;

pub async fn all<'a, T : Deserialize<'a>>(coll : &'a Collection) -> Vec<T> {
        coll.find(None, None)
                .await
                .unwrap()
                .filter_map(|item| async move { item.ok() })
                .map(|doc| bson::from_bson(bson::Bson::Document(doc)).expect("Decode error"))
                .collect()
                .await
}

pub async fn get<'a, T : Deserialize<'a>>(coll : &'a Collection, id : &ObjectId) -> Option<T> {
        find(
                coll,
                doc! {
                        "_id" => id,
                },
        )
        .await
}

pub async fn find<'a, T : Deserialize<'a>>(coll : &'a Collection, search : Document) -> Option<T> {
        coll.find_one(Some(search), None)
                .await
                .expect("DB Error")
                .map(|doc| bson::from_bson(bson::Bson::Document(doc)).expect("Decode error"))
}

pub async fn search<'a, T : Deserialize<'a>>(coll : &'a Collection, search : Document) -> Vec<T> {
        coll.find(Some(search), None)
                .await
                .unwrap()
                .filter_map(|item| async move { item.ok() })
                .map(|doc| bson::from_bson(bson::Bson::Document(doc)).expect("Decode error"))
                .collect()
                .await
}
