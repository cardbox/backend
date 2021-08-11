use crate::contracts::UnexpectedDatabaseError;
use crate::models;

#[async_trait]
pub trait Users {
    async fn user_get_by_username(&self, username: String) -> Result<models::User, UserGetError>;
    async fn user_get_by_token(&self, token: String) -> Result<models::SessionUser, UserGetError>;
    async fn users_search(&self, query: &str) -> Result<Vec<models::User>, UserSearchError>;
}

#[derive(Debug, thiserror::Error)]
pub enum UserGetError {
    #[error(transparent)]
    Unexpected(#[from] eyre::Report),
    #[error("User not found")]
    UserNotFound,
    #[error("Token not found")]
    TokenNotFound,
    #[error("Token expired")]
    TokenExpired,
}

#[derive(Debug, thiserror::Error)]
pub enum UserSearchError {
    #[error(transparent)]
    Unexpected(#[from] eyre::Report),
}

impl From<UnexpectedDatabaseError> for UserGetError {
    #[inline]
    fn from(e: UnexpectedDatabaseError) -> Self {
        Self::Unexpected(e.into())
    }
}

impl From<UnexpectedDatabaseError> for UserSearchError {
    #[inline]
    fn from(e: UnexpectedDatabaseError) -> Self {
        Self::Unexpected(e.into())
    }
}
