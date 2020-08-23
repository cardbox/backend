use crate::routes::{AnswerFailure, FailureCode};
use actix_web::{http::StatusCode, web, HttpRequest, Responder};

pub async fn route(_req: HttpRequest) -> impl Responder {
    web::Json(AnswerFailure {
        error: FailureCode::InvalidRoute,
        message: None,
    })
    .with_status(StatusCode::NOT_FOUND)
}
