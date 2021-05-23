use crate::errors::ServiceError;
use alcoholic_jwt::{token_kid, validate, Validation, JWKS};
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::sync::RwLock;

lazy_static! {
    static ref JWKS_STORE: RwLock<Option<JWKS>> = RwLock::new(None);
}

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    company: String,
    exp: usize,
}

pub async fn validate_token(token: &str) -> Result<bool, ServiceError> {
    let authority = std::env::var("AUTHORITY").expect("AUTHORITY must be set");
    let url = format!("{}{}", authority.as_str(), ".well-known/jwks.json");

    let jwks_refresh = || async {
        let jwks = fetch_jwks(&url).await.ok();
        {
            *JWKS_STORE.write().unwrap() = jwks.clone();
        }
        jwks.unwrap()
    };

    let maybe_jwks = { JWKS_STORE.read().unwrap().as_ref().map(|v| v.clone()) };

    let mut jwks = match maybe_jwks {
        Some(v) => v,
        None => jwks_refresh().await,
    };

    loop {
        let validations = vec![
            Validation::Issuer(authority.clone()),
            Validation::SubjectPresent,
        ];
        let kid = match token_kid(&token) {
            Ok(res) => res.expect("failed to decode kid"),
            Err(_) => return Err(ServiceError::JWKSFetchError),
        };
        let jwk = match jwks.find(&kid) {
            Some(jwk) => jwk,
            None => {
                jwks = jwks_refresh().await;
                continue;
            }
        };
        let res = validate(token, jwk, validations);
        return Ok(res.is_ok());
    }
}

async fn fetch_jwks(uri: &str) -> Result<JWKS, Box<dyn Error>> {
    let mut res = reqwest::get(uri).await?;
    let val = res.json::<JWKS>().await?;
    return Ok(val);
}
