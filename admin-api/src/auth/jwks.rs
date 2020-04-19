use reqwest::Result as ReqwestResult;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct JwksKeys {
	keys : Vec<Jwks>,
}

#[derive(Deserialize, Debug)]
pub struct Jwks {
	kty :   String,
	pub e : String,
	kid :   String,
	pub n : String,
}

impl Jwks {
	pub fn get() -> ReqwestResult<Self> {
		let mut body : JwksKeys =
			reqwest::blocking::get("https://normorovers.au.auth0.com/.well-known/jwks.json")?
				.json()?;
		Ok(body.keys.remove(0))
	}
}
