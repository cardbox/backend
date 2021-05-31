use actix_web::http::StatusCode;
use actix_web::ResponseError;
use thiserror::Error;

#[derive(Debug, Error, Eq, PartialEq)]
#[non_exhaustive]
pub enum SessionTokenExtractorError {
    #[error("No session token present in cookie")]
    NoSessionToken,
}

impl ResponseError for SessionTokenExtractorError {
    fn status_code(&self) -> StatusCode {
        StatusCode::BAD_REQUEST
    }
}
