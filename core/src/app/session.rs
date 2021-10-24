use crate::contracts::UnexpectedDatabaseError;

#[derive(Debug, thiserror::Error)]
pub enum SessionDeleteError {
    #[error(transparent)]
    Unexpected(#[from] eyre::Report),
}

#[async_trait]
pub trait Session {
    async fn session_delete(
        &self,
        token: String,
    ) -> Result<u64, SessionDeleteError>;
}

impl From<UnexpectedDatabaseError> for SessionDeleteError {
    fn from(e: UnexpectedDatabaseError) -> Self {
        Self::Unexpected(e.into())
    }
}
