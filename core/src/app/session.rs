use crate::{models, repo, App};
use async_trait::async_trait;

#[async_trait]
pub trait Session {
    async fn user_resolve_by_session_token(
        &mut self,
        token_string: String,
    ) -> Result<Option<models::User>, ResolveBySessionTokenError>;
}

#[derive(Debug, PartialEq, Clone)]
pub enum ResolveBySessionTokenError {
    Unexpected,
}

#[async_trait]
impl<Database> Session for App<Database>
where
    Database: repo::UserRepo + repo::SessionTokenRepo + Send + Sync,
{
    async fn user_resolve_by_session_token(
        &mut self,
        token_string: String,
    ) -> Result<Option<models::User>, ResolveBySessionTokenError> {
        let session_token = self.db.find_by_token((token_string)).await?;

        match session_token {
            Some(session_token) => Ok(self.db.find_by_id(session_token.user_id).await?),
            None => Ok(None),
        }
    }
}

impl From<repo::UnexpectedError> for ResolveBySessionTokenError {
    fn from(_: repo::UnexpectedError) -> Self {
        ResolveBySessionTokenError::Unexpected
    }
}
