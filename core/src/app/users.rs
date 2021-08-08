use crate::contracts::UnexpectedDatabaseError;
use crate::models;

#[async_trait]
pub trait Users {
    async fn user_get_by_username(&self, username: String) -> Result<models::User, UserGetError>;
    async fn user_get_by_token(&self, token: String) -> Result<models::SessionUser, UserGetError>;
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

impl From<UnexpectedDatabaseError> for UserGetError {
    #[inline]
    fn from(e: UnexpectedDatabaseError) -> Self {
        Self::Unexpected(e.into())
    }
}
