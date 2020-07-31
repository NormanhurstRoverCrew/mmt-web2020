use async_trait::async_trait;
use bson::{doc, oid::ObjectId, Document};
use futures::StreamExt;

use crate::graphql::context::CustomContext;
use serde::Deserialize;

#[async_trait]
pub trait Db<'a>: Send + Sized + Deserialize<'static> + 'static {
	const COLLECTION : &'static str;

	async fn all(context : &'a CustomContext) -> Vec<Self> {
		context
			.db
			.collection(Self::COLLECTION)
			.find(None, None)
			.await
			.unwrap()
			.filter_map(|item| async move { item.ok() })
			.map(|doc| bson::from_bson(bson::Bson::Document(doc)).expect("Decode error"))
			.collect()
			.await
	}

	async fn find(context : &'a CustomContext, search : Document) -> Option<Self> {
		context
			.db
			.collection(Self::COLLECTION)
			.find_one(Some(search), None)
			.await
			.expect("DB Error")
			.map(|doc| bson::from_bson(bson::Bson::Document(doc)).expect("Decode error"))
	}

	async fn get(context : &'a CustomContext, id : &ObjectId) -> Option<Self> {
		Self::find(
			context,
			doc! {
					"_id" : id,
			},
		)
		.await
	}

	async fn search(context : &'a CustomContext, search : Document) -> Vec<Self> {
		context
			.db
			.collection(Self::COLLECTION)
			.find(Some(search), None)
			.await
			.unwrap()
			.filter_map(|item| async move { item.ok() })
			.map(|doc| bson::from_bson(bson::Bson::Document(doc)).expect("Decode error"))
			.collect()
			.await
	}
}
