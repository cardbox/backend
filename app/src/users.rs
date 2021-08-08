use crate::{App, Service};
use cardbox_core::app::{UserGetError, Users};
use cardbox_core::contracts::Repository;
use cardbox_core::models::User;
use sqlx_core::types::Uuid;

#[async_trait]
impl Users for App {
    async fn user_get(&self, user_id: Uuid) -> Result<User, UserGetError> {
        let db = self.get::<Service<dyn Repository>>()?;

        let user = db.user_find_by_id(user_id).await?;

        match user {
            Some(user) => Ok(user),
            None => Err(UserGetError::UserNotFound),
        }
    }
}
