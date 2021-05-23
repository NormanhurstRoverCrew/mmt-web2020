use crate::CustomContext;
use actix_web::{dev::Payload, error::ErrorUnauthorized, Error, FromRequest, HttpRequest};
use futures::Future;
use serde::Deserialize;
use std::pin::Pin;

pub struct AdminUser {
    sub: String,
    scope: Vec<String>,
}

impl FromRequest for AdminUser {
    type Config = ();
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<AdminUser, Error>>>>;

    fn from_request(req: &HttpRequest, pl: &mut Payload) -> Self::Future {
        let bearer = match req.headers().get("authorization") {
            Some(auth) => auth,
            None => {
                return Box::pin(async { Err(ErrorUnauthorized("unauthorized")) });
            }
        };
        let bearer = match bearer.to_str() {
            Ok(b) => b,
            Err(_) => {
                return Box::pin(async { Err(ErrorUnauthorized("Deserialize error")) });
            }
        };

        let parts = bearer.split(" ").collect::<Vec<&str>>();
        let token = parts.get(1).unwrap();
        let parts = token.split(".").collect::<Vec<&str>>();

        let json = parts.get(1).unwrap();
        let json = base64::decode(json).unwrap();
        let ident = serde_json::from_slice::<Ident>(&json).unwrap();
        // let fut = Identity::from_request(req, pl);
        // let sessions: Option<&web::Data<RwLock<Sessions>>> = req.app_data();
        // if sessions.is_none() {
        //     dbg!("sessions is empty(none)!");
        //     return Box::pin(async { Err(ErrorUnauthorized("unauthorized")) });
        // }
        // let sessions = sessions.unwrap().clone();
        Box::pin(async move {
            Ok(AdminUser {
                sub: ident.sub,
                scope: ident
                    .scope
                    .split_ascii_whitespace()
                    .map(|s| s.to_owned())
                    .collect(),
            })
        })
    }
}

#[derive(Debug, Deserialize)]
struct Ident {
    sub: String,
    scope: String,
}

#[juniper::graphql_object(
    Context = CustomContext,
)]
impl AdminUser {
    fn sub(&self) -> &str {
        &self.sub
    }

    fn scopes(&self) -> Vec<&str> {
        self.scope.iter().map(|s| s.as_str()).collect()
    }
}
