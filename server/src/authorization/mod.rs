use reqwest::{header, StatusCode};

use super::authorization;

use rocket::fairing::{Fairing, Info, Kind};
use rocket::request::{self, FromRequest};
use rocket::Outcome::{Forward, Success};
use rocket::{Data, Request};
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, Serialize, Deserialize)]
pub struct AuthorizationData {
	iss: String,
	sub: String,
	aud: Vec<String>,
	iat: usize,
	exp: usize,
	azp: String,
	scope: String,
	pub permissions: Vec<String>,
}

#[derive(Debug)]
pub enum AuthorizationDataError {
	Missing,
	Invalid,
	Failed,
	Unauthorized,
	BadCount,
}

// #[derive(Copy, Clone)]
struct Authorization(Option<AuthorizationData>);

impl Fairing for AuthorizationData {
	fn info(&self) -> Info {
		Info {
			name: "Auth0 Token Validator",
			kind: Kind::Request,
		}
	}

	/// Stores the start time of the request in request-local state.
	fn on_request(&self, request: &mut Request, _: &Data) {
		// println!("DEBUG Fairing");
		// If in development, skip auth so we dont depend on a valid key
		match std::env::var("ENVIRONMENT") {
			Ok(env) => match env.as_ref() {
				"development" => {
					request.local_cache(|| {
						Authorization(Some(AuthorizationData {
							iss: String::new(),
							sub: String::new(),
							aud: Vec::new(),
							iat: 0,
							exp: 0,
							azp: String::new(),
							scope: String::new(),
							permissions: vec![String::from("enabled")],
						}))
					});
					return;
				}
				_ => {}
			},
			Err(_) => {}
		};
		let auth_key: Vec<_> = request.headers().get("Authorization").collect();
		match auth_key.len() {
			0 => {
				request.local_cache(|| Authorization(None));
				return;
			}
			1 => match authorization::is_authenticated(auth_key[0]) {
				Ok(ad) => {
					for permission in ad.permissions.iter() {
						match permission.as_ref() {
							"enabled" => {
								request.local_cache(|| Authorization(Some(ad)));
								return;
							}
							_ => (),
						}
					}
					request.local_cache(|| Authorization(None));
					return;
				}
				Err(_) => {
					request.local_cache(|| Authorization(None));
					return;
				}
			},
			_ => {
				request.local_cache(|| Authorization(None));
				return;
			}
		}
	}
}

impl<'a, 'r> FromRequest<'a, 'r> for AuthorizationData {
	type Error = ();

	fn from_request(request: &'a Request<'r>) -> request::Outcome<AuthorizationData, ()> {
		match &*request.local_cache(|| Authorization(None)) {
			Authorization(Some(auth)) => Success(auth.clone()),
			Authorization(None) => Forward(()),
		}
	}
}

#[derive(Clone, Default, Debug, Serialize, Deserialize)]
pub struct Permissions(Vec<String>);

impl<'a, 'r> FromRequest<'a, 'r> for Permissions {
	type Error = ();

	fn from_request(request: &'a Request<'r>) -> request::Outcome<Permissions, ()> {
		match &*request.local_cache(|| Authorization(None)) {
			Authorization(Some(auth)) => Success(Permissions(auth.permissions.clone())),
			Authorization(None) => Forward(()),
		}
	}
}

pub fn is_authenticated(key: &str) -> Result<AuthorizationData, AuthorizationDataError> {
	let mut headers = header::HeaderMap::new();
	headers.insert(
		header::AUTHORIZATION,
		header::HeaderValue::from_str(key).unwrap(),
	);
	let client = match reqwest::Client::builder().default_headers(headers).build() {
		Ok(response) => response,
		Err(_) => return Err(AuthorizationDataError::Failed),
	};

	let mut res = match client.get("http://admin:3000/api/isadmin").send() {
		Ok(response) => response,
		Err(_) => return Err(AuthorizationDataError::Failed),
	};

	match res.status() {
		StatusCode::OK => match res.json::<AuthorizationData>() {
			Ok(ad) => Ok(ad),
			Err(_) => Err(AuthorizationDataError::Invalid),
		},
		StatusCode::UNAUTHORIZED => Err(AuthorizationDataError::Unauthorized),
		_ => Err(AuthorizationDataError::Unauthorized),
	}
}
