use crate::{App, Service};
use cardbox_core::app::session::{Session, SessionDeleteError, SessionDeleteStrategy};
use cardbox_core::contracts::Repository;

#[async_trait]
impl Session for App {
    async fn session_delete(
        &self,
        token: String,
        strategy: SessionDeleteStrategy,
    ) -> Result<u64, SessionDeleteError> {
        let db = self.get::<Service<dyn Repository>>()?;
        match strategy {
            SessionDeleteStrategy::All => {
                let session_token = db.token_find(token).await?;
                if let Some(session_token) = session_token {
                    db.token_delete_by_user(session_token.user_id)
                        .await
                        .map_err(Into::into)
                } else {
                    Err(SessionDeleteError::TokenNotFound)
                }
            }

            SessionDeleteStrategy::Single(token) => {
                db.token_delete(token).await.map_err(Into::into)
            }
        }
    }
}
