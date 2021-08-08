use crate::{App, Service};
use cardbox_core::app::{UserGetError, Users};
use cardbox_core::contracts::Repository;
use cardbox_core::models::{SessionUser, User};
use eyre::WrapErr;

#[async_trait]
impl Users for App {
    async fn user_get_by_username(&self, username: String) -> Result<User, UserGetError> {
        let db = self.get::<Service<dyn Repository>>()?;

        let user = db.user_find_by_username(&username).await?;

        match user {
            Some(user) => Ok(user),
            None => {
                tracing::warn!(%username, "User not found by username, maybe exists by id");
                match db
                    .user_find_by_id(
                        username
                            .parse()
                            .wrap_err("Could not parse username as id")?,
                    )
                    .await?
                {
                    Some(user) => Ok(user),
                    None => Err(UserGetError::UserNotFound),
                }
            }
        }
    }

    #[tracing::instrument]
    async fn user_get_by_token(&self, token: String) -> Result<SessionUser, UserGetError> {
        let db = self.get::<Service<dyn Repository>>()?;

        let user = db.user_find_by_session_token(token.clone()).await?;

        match user {
            Some(user) => {
                if user.expired {
                    tracing::warn!(%token, "Token is expired!");
                    Err(UserGetError::TokenExpired)
                } else {
                    Ok(user)
                }
            }
            None => Err(UserGetError::TokenNotFound),
        }
    }
}
