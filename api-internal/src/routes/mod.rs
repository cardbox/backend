use actix_web::{web, Scope};
use serde::{Deserialize, Serialize};

mod accesso;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FailureCode {
    InvalidPayload,
    InvalidRoute,
    InvalidQueryParams,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AnswerFailure {
    pub error: FailureCode,
    pub message: Option<String>,
}

pub fn scope() -> Scope {
    web::scope("/").service(accesso::scope())
}
