use crate::contracts::UnexpectedDatabaseError;
use crate::models;
use uuid::Uuid;

#[async_trait]
pub trait Cards {
    async fn card_create<'a>(
        &self,
        card: CardCreateForm<'a>,
        token: String,
    ) -> Result<models::Card, CardCreateError>;

    async fn cards_search(
        &self,
        query: &str,
        limit: Option<i64>,
    ) -> Result<Vec<(models::Card, models::User)>, CardSearchError>;

    async fn card_update<'a>(
        &self,
        card: CardUpdateForm<'a>,
        token: String,
    ) -> Result<models::Card, CardUpdateError>;
}

#[derive(Debug, Validate)]
pub struct CardUpdateForm<'a> {
    pub id: Uuid,
    #[validate(length(min = 1))]
    pub title: Option<String>,
    pub contents: Option<&'a serde_json::value::RawValue>,
    pub tags: Option<Vec<String>>,
}

#[derive(Debug, Validate)]
pub struct CardCreateForm<'a> {
    #[validate(length(min = 1))]
    pub title: String,
    pub contents: &'a serde_json::value::RawValue,
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
    #[error("Token expired")]
    TokenExpired,
    #[error(transparent)]
    ValidationError(#[from] validator::ValidationErrors),
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
