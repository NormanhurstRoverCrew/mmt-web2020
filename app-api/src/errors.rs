use actix_web::{
    dev::Body, error::ResponseError, http::StatusCode, web::Json, BaseHttpResponse, HttpResponse,
};
use derive_more::Display;

#[derive(Debug, Display)]
pub enum ServiceError {
    #[display(fmt = "Internal Server Error")]
    InternalServerError,

    #[display(fmt = "BadRequest: {}", _0)]
    BadRequest(String),

    #[display(fmt = "JWKSFetchError")]
    JWKSFetchError,
}

// impl ResponseError trait allows to convert our errors into http responses with appropriate data
impl ResponseError for ServiceError {
    fn error_response(&self) -> BaseHttpResponse<Body> {
        match self {
            ServiceError::InternalServerError => {
                BaseHttpResponse::internal_server_error() //.json("Internal Server Error, Please try later")
            }
            ServiceError::BadRequest(ref message) => BaseHttpResponse::bad_request(), //.json(message),
            ServiceError::JWKSFetchError => {
                BaseHttpResponse::internal_server_error() // .json("Could not fetch JWKS")
            }
        }
    }
}
