use juniper::graphql_value;

pub fn string_to_id(id : &str) -> Result<bson::oid::ObjectId, juniper::FieldError> {
	match bson::oid::ObjectId::with_string(&id) {
		Ok(oid) => Ok(oid),
		Err(_) => Err(juniper::FieldError::new(
			"UID is not valid",
			graphql_value!({
				"type": "INVALID_UID"
			}),
		)),
	}
}

pub fn bson_to_id(id : &bson::Bson) -> Result<&bson::oid::ObjectId, juniper::FieldError> {
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
