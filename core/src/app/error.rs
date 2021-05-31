use thiserror::Error;
use actix_web::ResponseError;
use actix_web::http::StatusCode;

#[derive(Debug, Error)]
#[non_exhaustive]
pub enum SessionTokenExtractorError {
    #[error("No session token present in cookie")]
    NoSessionToken
}

impl ResponseError for SessionTokenExtractorError {
    fn status_code(&self) -> StatusCode {
        StatusCode::BAD_REQUEST
    }
}