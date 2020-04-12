use crate::db::FromDoc;
use mongodb::{coll::Collection, oid::ObjectId, ordered::OrderedDocument};

pub fn all<T : FromDoc>(coll : &Collection) -> Vec<T> {
	coll.find(None, None)
		.unwrap()
		.into_iter()
		.filter_map(|item| match item {
			Ok(item) => Some(T::from_doc(&item)),
			Err(_) => None,
		})
		.collect()
}

pub fn get<T : FromDoc>(coll : &Collection, id : &ObjectId) -> Option<T> {
	match coll.find_one(
		Some(doc! {
			"_id" => id.to_owned(),
		}),
		None,
	) {
		Ok(Some(o)) => Some(T::from_doc(&o)),
		_ => None,
	}
}

pub fn find<T : FromDoc>(coll : &Collection, search : OrderedDocument) -> Option<T> {
	match coll.find_one(Some(search), None) {
		Ok(Some(o)) => Some(T::from_doc(&o)),
		_ => None,
	}
}

pub fn search<T : FromDoc>(coll : &Collection, search : OrderedDocument) -> Vec<T> {
	match coll.find(Some(search), None) {
		Ok(o) => o.map(|d| T::from_doc(&d.unwrap())).collect(),
		_ => vec![],
	}
}
