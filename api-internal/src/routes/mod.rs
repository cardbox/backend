use serde::{Deserialize, Serialize};

pub mod not_found;
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
