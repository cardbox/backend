use cardbox_core::app::session::{Session, SessionDeleteError};
use cardbox_core::contracts::{Repository};
use crate::{App, Service};

#[async_trait]
impl Session for App {
    async fn session_delete(
        &self,
        token: String
    ) -> Result<u64, SessionDeleteError> {
        let db = self.get::<Service<dyn Repository>>()?;
        db
            .token_delete(token)
            .await
            .map_err(Into::into)
    }
}
