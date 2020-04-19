//! The auth module deals with authenticating users on the site.
//! Passwords are hashed with scrypt.
//! JSON Web Tokens are returned to the user.
//! JWTs should be included in http requests to the site under the
//! `Authorization` header. Because of signature checking, the server can trust
//! the contents of the JWT payload and can use them to guard access to
//! protected APIs. FromRequest is implemented for some dummy user types.
//! They will only succeed in creating themselves if the JWT contains the role
//! the user type corresponds to. By specifying one of these user types on a
//! routable method, rocket will not route the request to it unless it can
//! resolve the role in the JWT in the request header.

extern crate rocket;
extern crate rocket_contrib;

extern crate chrono;
extern crate rand;

mod banned_set;
mod jwks;
mod jwt;
mod secret;

pub use banned_set::BannedSet;
pub use jwks::Jwks;
pub use jwt::{AdminUser, ServerJwt};
pub use secret::Secret;
