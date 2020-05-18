use crate::{
	db::FromDoc,
	models::{Payment, Ticket, User},
};
use bson::{oid::ObjectId, Document};

pub fn doc_get_id(item : &Document) -> String { doc_get_id_key(item, "_id") }

pub fn doc_get_id_key(item : &Document, key : &str) -> String {
	match item.get_object_id(key) {
		Ok(oid) => oid.to_string(),
		_ => ObjectId::new().unwrap().to_string(),
	}
}

pub fn doc_get_i32(item : &Document, key : &str, default : i32) -> i32 {
	match item.get_i32(key) {
		Ok(q) => q as i32,
		_ => default,
	}
}

pub fn doc_get_bool(item : &Document, key : &str, default : bool) -> bool {
	match item.get_bool(key) {
		Ok(q) => q as bool,
		_ => default,
	}
}

pub fn doc_get_string(item : &Document, key : &str, default : &str) -> String {
	String::from(match item.get_str(key) {
		Ok(t) => t,
		_ => default,
	})
}
