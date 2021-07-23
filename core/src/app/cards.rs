use crate::contracts::UnexpectedDatabaseError;
use crate::models;
use uuid::Uuid;

#[async_trait]
pub trait Cards {
    async fn card_create(
        &self,
        card: CardCreateForm,
        token: String,
    ) -> Result<models::Card, CardCreateError>;

    async fn cards_search(
        &self,
        query: &str,
        limit: Option<i64>,
    ) -> Result<Vec<(models::Card, models::User)>, CardSearchError>;

    async fn card_update(
        &self,
        card: CardUpdateForm,
        token: String,
    ) -> Result<models::Card, CardUpdateError>;
}

#[derive(Debug, Validate)]
pub struct CardUpdateForm {
    pub id: Uuid,
    #[validate(length(min = 1))]
    pub title: Option<String>,
    pub contents: Option<serde_json::Value>,
    pub tags: Option<Vec<String>>,
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

#[derive(Debug, thiserror::Error)]
pub enum CardSearchError {
    #[error(transparent)]
    Unexpected(#[from] eyre::Report),
}

#[derive(Debug, thiserror::Error)]
pub enum CardUpdateError {
    #[error(transparent)]
    Unexpected(#[from] eyre::Report),
    #[error("Card not found")]
    CardNotFound,
    #[error("Token not found")]
    TokenNotFound,
}

impl From<UnexpectedDatabaseError> for CardCreateError {
    #[inline]
    fn from(e: UnexpectedDatabaseError) -> Self {
        Self::Unexpected(e.into())
    }
}

impl From<UnexpectedDatabaseError> for CardSearchError {
    #[inline]
    fn from(e: UnexpectedDatabaseError) -> Self {
        Self::Unexpected(e.into())
    }
}

impl From<UnexpectedDatabaseError> for CardUpdateError {
    #[inline]
    fn from(e: UnexpectedDatabaseError) -> Self {
        Self::Unexpected(e.into())
    }
}
