use juniper::graphql_value;

pub fn string_to_id(id : &str) -> Result<mongodb::oid::ObjectId, juniper::FieldError> {
	match mongodb::oid::ObjectId::with_string(&id) {
		Ok(oid) => Ok(oid),
		Err(_) => Err(juniper::FieldError::new(
			"UID is not valid",
			graphql_value!({
				"type": "INVALID_UID"
			}),
		)),
	}
}

pub fn bson_to_id(id : &mongodb::Bson) -> Result<&mongodb::oid::ObjectId, juniper::FieldError> {
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
