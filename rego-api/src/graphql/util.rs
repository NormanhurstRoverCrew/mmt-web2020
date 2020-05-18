use bson::{oid::ObjectId, Bson};
use juniper::graphql_value;

pub fn string_to_id(id : &str) -> Result<ObjectId, juniper::FieldError> {
	match ObjectId::with_string(&id) {
		Ok(oid) => Ok(oid),
		Err(_) => Err(juniper::FieldError::new(
			"UID is not valid",
			graphql_value!({
				"type": "INVALID_UID"
			}),
		)),
	}
}

pub fn bson_to_id(id : &Bson) -> Result<&ObjectId, juniper::FieldError> {
	match id.as_object_id() {
		Some(oid) => Ok(oid),
		None => Err(juniper::FieldError::new(
			"UID is not valid",
			graphql_value!({
				"type": "INVALID_UID"
			}),
		)),
	}
}
