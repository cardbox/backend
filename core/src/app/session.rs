use crate::contracts::UnexpectedDatabaseError;

#[derive(Debug, thiserror::Error)]
pub enum SessionDeleteError {
    #[error(transparent)]
    Unexpected(#[from] eyre::Report),
    #[error("Token not found")]
    TokenNotFound,
}

pub enum SessionDeleteStrategy {
    All,
    Single(String),
}

#[async_trait]
pub trait Session {
    async fn session_delete(
        &self,
        token: String,
        strategy: SessionDeleteStrategy,
    ) -> Result<u64, SessionDeleteError>;
}

impl From<UnexpectedDatabaseError> for SessionDeleteError {
    fn from(e: UnexpectedDatabaseError) -> Self {
        Self::Unexpected(e.into())
    }
}
