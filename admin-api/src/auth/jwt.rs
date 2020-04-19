use crate::auth::Jwks;
use chrono::DateTime;
use jsonwebtoken::{decode, errors::Result as JwtResult, Algorithm, DecodingKey, Validation};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Claims {
	#[serde(with = "jwt_numeric_date")]
	exp :         DateTime<Utc>, /* Required (validate_exp defaults to true in validation).
	                              * Expiration */
	#[serde(with = "jwt_numeric_date")]
	iat :         DateTime<Utc>, // Optional. Issued
	iss :         String, // Optional. Issuer
	sub :         String, // Optional. Subject (whom token refers to)
	aud :         Vec<String>,
	permissions : Vec<String>,
}

mod jwt_numeric_date {
	//! Custom serialization of DateTime<Utc> to conform with the JWT spec (RFC
	//! 7519 section 2, "Numeric Date")
	use chrono::{DateTime, TimeZone, Utc};
	use serde::{self, Deserialize, Deserializer, Serializer};

	/// Serializes a DateTime<Utc> to a Unix timestamp (milliseconds since
	/// 1970/1/1T00:00:00T)
	pub fn serialize<S>(date : &DateTime<Utc>, serializer : S) -> Result<S::Ok, S::Error>
	where
		S : Serializer,
	{
		let timestamp = date.timestamp();
		serializer.serialize_i64(timestamp)
	}

	/// Attempts to deserialize an i64 and use as a Unix timestamp
	pub fn deserialize<'de, D>(deserializer : D) -> Result<DateTime<Utc>, D::Error>
	where
		D : Deserializer<'de>,
	{
		Utc.timestamp_opt(i64::deserialize(deserializer)?, 0)
			.single() // If there are multiple or no valid DateTimes from timestamp, return None
			.ok_or_else(|| serde::de::Error::custom("invalid Unix timestamp value"))
	}
}

/// Because the JWT struct lives in the wire crate,
/// this NewType is used to define other functions on it.
#[derive(Clone, PartialEq, Debug)]
pub struct ServerJwt(pub Claims);

impl ServerJwt {
	pub fn decode_jwt_string(jwt_str : &str, jwk : &Jwks) -> JwtResult<ServerJwt> {
		let token = decode::<Claims>(
			jwt_str,
			&DecodingKey::from_rsa_components(&jwk.n, &jwk.e),
			&Validation::new(Algorithm::RS256),
		);

		token.map(|t| ServerJwt(t.claims))
	}
}

use rocket::{
	http::Status,
	request::{self, FromRequest, Request},
	Outcome, State,
};

use chrono::Utc;

use crate::wire::jwt::Error;

/// Raw JWTs can be gotten via the request
/// This should only be used for reauth.
impl<'a, 'r> FromRequest<'a, 'r> for ServerJwt {
	type Error = Error;

	fn from_request(request : &'a Request<'r>) -> request::Outcome<ServerJwt, Error> {
		let jwt : ServerJwt = extract_jwt_from_request(request)?;
		// let jwt : ServerJwt = validate_jwt_expiry_time(jwt)?;

		Outcome::Success(jwt)
	}
}

/// Given a request, extract the JWT struct from the headers in the request.
fn extract_jwt_from_request(request : &Request) -> request::Outcome<ServerJwt, Error> {
	let keys : Vec<_> = request.headers().get("Authorization").collect();
	if keys.len() != 1 {
		return Outcome::Failure((Status::Unauthorized, Error::MissingToken));
	};

	let key = keys[0];

	// You can get the state secret from another request guard
	let jwks : &Jwks = match request.guard::<State<Jwks>>() {
		Outcome::Success(s) => s.inner(),
		_ => {
			println!("Couldn't get jwks from state.");
			return Outcome::Failure((Status::InternalServerError, Error::InternalServerError));
		},
	};

	let authorization_words : Vec<String> = key
		.to_string()
		.split_whitespace()
		.map(String::from)
		.collect();

	if authorization_words.len() != 2 {
		return Outcome::Failure((Status::Unauthorized, Error::MalformedToken));
	}
	if authorization_words[0] != "Bearer" {
		return Outcome::Failure((Status::Unauthorized, Error::MalformedToken));
	}
	let jwt_str : &str = &authorization_words[1];

	match ServerJwt::decode_jwt_string(jwt_str, jwks) {
		Ok(jwt) => Outcome::Success(jwt),
		Err(_) => {
			println!("Token couldn't be deserialized.");
			Outcome::Failure((Status::Unauthorized, Error::IllegalToken))
		},
	}
}

trait FromJwt {
	fn from_jwt(jwt : &Claims) -> Result<Self, RoleError>
	where
		Self : Sized;
	fn get_uuid(&self) -> String;
}

pub enum RoleError {
	InsufficientRights,
}

#[derive(Debug)]
pub struct AdminUser {
	pub user_uuid :   String,
	pub permissions : Permissions,
}

#[derive(Debug)]
pub struct Permissions {
	pub delete_all :    bool,
	pub email_send :    bool,
	pub payments_add :  bool,
	pub payments_edit : bool,
	pub payments_view : bool,
	pub users_edit :    bool,
}

impl From<&Vec<String>> for Permissions {
	fn from(p : &Vec<String>) -> Self {
		Self {
			delete_all :    p.contains(&String::from("delete:all")),
			email_send :    p.contains(&String::from("email:send")),
			payments_add :  p.contains(&String::from("payments:add")),
			payments_edit : p.contains(&String::from("payments:edit")),
			payments_view : p.contains(&String::from("payments:view")),
			users_edit :    p.contains(&String::from("users:edit")),
		}
	}
}

impl FromJwt for AdminUser {
	fn from_jwt(jwt : &Claims) -> Result<AdminUser, RoleError> {
		if jwt.permissions.contains(&String::from("enabled")) {
			Ok(AdminUser {
				user_uuid :   jwt.sub.to_string(),
				permissions : (&jwt.permissions).into(),
			})
		} else {
			Err(RoleError::InsufficientRights)
		}
	}
	fn get_uuid(&self) -> String { self.user_uuid.to_string() }
}

impl<'a, 'r> FromRequest<'a, 'r> for AdminUser {
	type Error = Error;

	fn from_request(request : &'a Request<'r>) -> request::Outcome<AdminUser, Error> {
		extract_role_from_request::<AdminUser>(request)
	}
}

fn extract_role_from_request<T>(request : &Request) -> request::Outcome<T, Error>
where
	T : FromJwt,
{
	// Get the jwt from the request's header
	let jwt : ServerJwt = extract_jwt_from_request(request)?;

	let user = match T::from_jwt(&jwt.0) {
		Ok(user) => user,
		Err(_) => {
			return Outcome::Failure((
				Status::Forbidden,
				Error::NotAuthorized {
					reason : "User does not have that role.",
				},
			))
		},
	};

	// // Check for stateful banned status
	// match request.guard::<State<BannedSet>>() {
	// 	Outcome::Success(set) => {
	// 		if set.is_user_banned(&user.get_uuid()) {
	// 			return Outcome::Failure((Status::Unauthorized, Error::BadRequest));
	// 		}
	// 	},
	// 	_ => {
	// 		println!("Couldn't get banned set from state.");
	// 		return Outcome::Failure((Status::InternalServerError,
	// Error::InternalServerError)); 	},
	// }

	Outcome::Success(user)
}
