use super::user::UserRole;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Jwt {
	/// sub is the user uuid
	pub sub :        String,
	pub user_roles : Vec<UserRole>,
	/// exp is the Expiration date, in unix timestamp form
	pub exp :        NaiveDateTime,
	/// iat is the Issue-At date, it is used for determining if the client
	/// should refresh or not.
	pub iat :        NaiveDateTime,
}

impl Default for Jwt {
	fn default() -> Self {
		Jwt {
			sub :        String::default(),
			user_roles : Vec::default(),
			exp :        NaiveDateTime::from_timestamp(0, 0),
			iat :        NaiveDateTime::from_timestamp(0, 0),
		}
	}
}

#[derive(Debug)]
pub enum Error {
	InternalServerError,
	BadRequest,
	ExpiredToken,
	IllegalToken,
	MalformedToken,
	NotAuthorized { reason : &'static str },
	MissingToken,
}
