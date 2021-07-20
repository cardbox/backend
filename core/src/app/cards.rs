use crate::contracts::UnexpectedDatabaseError;
use crate::models;

#[async_trait]
pub trait Cards {
    async fn new_card(
        &self,
        card: NewCardForm,
        token: String,
    ) -> Result<models::Card, NewCardError>;
}

#[derive(Debug, Validate)]
pub struct NewCardForm {
    #[validate(length(min = 1))]
    pub title: String,
    pub contents: serde_json::Value,
    pub tags: Vec<String>,
}

#[derive(Debug, thiserror::Error)]
pub enum NewCardError {
    #[error("Unauthorized")]
    Unauthorized,
    #[error(transparent)]
    ValidationError(#[from] validator::ValidationErrors),
    // BL domain where we can later add more errors
    // Maybe user exceeded his limit on cards
    #[error(transparent)]
    Unexpected(#[from] eyre::Report),
}

impl From<UnexpectedDatabaseError> for NewCardError {
    fn from(e: UnexpectedDatabaseError) -> Self {
        Self::Unexpected(e.into())
    }
}
