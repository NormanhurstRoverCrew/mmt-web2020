use juniper::ID;
use mongodb::{oid::ObjectId, Bson, Document};
pub struct Todo {
	pub id :        ID,
	pub title :     String,
	pub completed : bool,
}

impl Todo {
	pub fn from_doc(item : Document) -> Self {
		Self {
			id :        Self::doc_get_id(&item),
			title :     Self::doc_get_title(&item),
			completed : Self::doc_get_completed(&item),
		}
	}

	pub fn doc_get_id(item : &Document) -> ID {
		ID::from(match item.get_object_id("_id") {
			Ok(oid) => oid.to_string(),
			_ => ObjectId::new().unwrap().to_string(),
		})
	}

	pub fn doc_get_title(item : &Document) -> String {
		String::from(match item.get_str("title") {
			Ok(t) => t,
			_ => "",
		})
	}

	pub fn doc_get_completed(item : &Document) -> bool {
		match item.get_bool("completed") {
			Ok(c) => c,
			_ => false,
		}
	}
}

pub struct Booking {
	pub id :    ID,
	pub users : Vec<User>,
}

impl Booking {
	pub fn from_doc(item : Document) -> Self {
		Self {
			id :    Self::doc_get_id(&item),
			users : Self::doc_get_users(&item),
		}
	}

	pub fn doc_get_id(item : &Document) -> ID {
		ID::from(match item.get_object_id("_id") {
			Ok(oid) => oid.to_string(),
			_ => ObjectId::new().unwrap().to_string(),
		})
	}

	pub fn doc_get_users(item : &Document) -> Vec<User> {
		match item.get_array("users") {
			Ok(t) => t
				.iter()
				.filter_map(|i| match i {
					Bson::Document(d) => Some(User::from_doc(d.to_owned())),
					_ => None,
				})
				.collect(), //t.iter().map(|i| User::from_doc(i.to_owned())).collect(),
			_ => Vec::new(),
		}
	}
}

#[derive(Clone, Debug)]
pub struct User {
	pub id :     ID,
	pub name :   String,
	pub email :  Option<String>,
	pub mobile : Option<String>,
	pub crew :   Option<String>,
}

impl User {
	pub fn from_doc(item : Document) -> Self {
		Self {
			id :     Self::doc_get_id(&item),
			name :   Self::doc_get_name(&item),
			email :  Self::doc_get_email(&item),
			mobile : Self::doc_get_mobile(&item),
			crew :   Self::doc_get_crew(&item),
		}
	}

	pub fn doc_get_id(item : &Document) -> ID {
		ID::from(match item.get_object_id("_id") {
			Ok(oid) => oid.to_string(),
			_ => ObjectId::new().unwrap().to_string(),
		})
	}

	pub fn doc_get_name(item : &Document) -> String {
		String::from(match item.get_str("name") {
			Ok(t) => t,
			_ => "",
		})
	}

	pub fn doc_get_mobile(item : &Document) -> Option<String> {
		match item.get_str("mobile") {
			Ok(c) => Some(String::from(c)),
			_ => None,
		}
	}

	pub fn doc_get_email(item : &Document) -> Option<String> {
		match item.get_str("email") {
			Ok(c) => Some(String::from(c)),
			_ => None,
		}
	}

	pub fn doc_get_crew(item : &Document) -> Option<String> {
		match item.get_str("crew") {
			Ok(c) => Some(String::from(c)),
			_ => None,
		}
	}
}
