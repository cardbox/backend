use crate::contracts::UnexpectedDatabaseError;
use crate::models;

#[async_trait]
pub trait AccessoAuthorize {
    async fn authorize(
        &self,
        user: UserInfo,
    ) -> Result<(models::User, models::SessionToken), UpdateUserFailure>;
}

#[derive(Debug, Clone)]
pub struct UserInfo {
    pub accesso_id: uuid::Uuid,
    pub first_name: String,
    pub last_name: String,
}

#[derive(Debug, thiserror::Error)]
pub enum UpdateUserFailure {
    #[error("Unexpected update user failure: {0}")]
    Unexpected(#[from] eyre::Report),
}

impl From<UnexpectedDatabaseError> for UpdateUserFailure {
    fn from(e: UnexpectedDatabaseError) -> Self {
        Self::Unexpected(e.into())
    }
}
