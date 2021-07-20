use crate::contracts::UnexpectedDatabaseError;
use crate::models;

#[async_trait]
pub trait Cards {
    async fn card_create(
        &self,
        card: CardCreateForm,
        token: String,
    ) -> Result<models::Card, CardCreateError>;
}

#[derive(Debug, Validate)]
pub struct CardCreateForm {
    #[validate(length(min = 1))]
    pub title: String,
    pub contents: serde_json::Value,
    pub tags: Vec<String>,
}

#[derive(Debug, thiserror::Error)]
pub enum CardCreateError {
    #[error("Unauthorized")]
    Unauthorized,
    #[error(transparent)]
    ValidationError(#[from] validator::ValidationErrors),
    // BL domain where we can later add more errors
    // Maybe user exceeded his limit on cards
    #[error(transparent)]
    Unexpected(#[from] eyre::Report),
}

impl From<UnexpectedDatabaseError> for CardCreateError {
    fn from(e: UnexpectedDatabaseError) -> Self {
        Self::Unexpected(e.into())
    }
}
