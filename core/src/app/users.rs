use crate::contracts::UnexpectedDatabaseError;
use crate::models;

#[async_trait]
pub trait Users {
    async fn user_get(&self, username: String) -> Result<models::User, UserGetError>;
}

#[derive(Debug, thiserror::Error)]
pub enum UserGetError {
    #[error(transparent)]
    Unexpected(#[from] eyre::Report),
    #[error("User not found")]
    UserNotFound,
}

impl From<UnexpectedDatabaseError> for UserGetError {
    #[inline]
    fn from(e: UnexpectedDatabaseError) -> Self {
        Self::Unexpected(e.into())
    }
}
