use crate::{
	db::FromDoc,
	models::{Ticket, User},
};
use mongodb::{oid::ObjectId, Document};

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

pub fn doc_get_user(item : &Document, key : &str) -> User {
	match item.get_document(key) {
		Ok(d) => User::from_doc(&d),
		_ => User::default(),
	}
}

pub fn doc_get_tickets(item : &Document, key : &str) -> Vec<Ticket> {
	match item.get_array(key) {
		Ok(d) => {
			dbg!(&d);
			d.iter()
				.map(|t| Ticket::from_doc(t.as_document().unwrap()))
				.collect()
		},
		_ => vec![],
	}
}
