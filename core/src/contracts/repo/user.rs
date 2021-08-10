use super::RepoResult;
use crate::models;
use async_trait::async_trait;
use uuid::Uuid;

#[cfg(feature = "testing")]
use mockall::*;

#[cfg_attr(feature = "testing", automock)]
#[async_trait]
pub trait UserRepo {
    async fn user_find_by_id(&self, user_id: Uuid) -> RepoResult<Option<models::User>>;
    async fn user_find_by_accesso(&self, accesso_id: Uuid) -> RepoResult<Option<models::User>>;
    async fn user_find_by_username(&self, username: &str) -> RepoResult<Option<models::User>>;
    async fn user_find_by_session_token(
        &self,
        session_token: String,
    ) -> RepoResult<Option<models::SessionUser>>;
    async fn user_update(&self, user: models::User) -> RepoResult<models::User>;
    async fn user_create(&self, user: models::UserCreate) -> Result<models::User, UserCreateError>;
}

#[derive(Debug, thiserror::Error)]
pub enum UserCreateError {
    #[error("User already exists")]
    UserAlreadyExists,
    #[error(transparent)]
    UnexpectedFailure(#[from] eyre::Report),
}

#[cfg(feature = "testing")]
#[async_trait]
impl UserRepo for crate::contracts::MockDb {
    async fn user_find_by_id(&self, user_id: Uuid) -> RepoResult<Option<models::User>> {
        self.users.user_find_by_id(user_id).await
    }

    async fn user_find_by_accesso(&self, accesso_id: Uuid) -> RepoResult<Option<models::User>> {
        self.users.user_find_by_accesso(accesso_id).await
    }

    async fn user_update(&self, user: models::User) -> RepoResult<models::User> {
        self.users.user_update(user).await
    }

    async fn user_create(&self, user: models::UserCreate) -> Result<models::User, UserCreateError> {
        self.users.user_create(user).await
    }

    async fn user_find_by_username(&self, username: &str) -> RepoResult<Option<models::User>> {
        self.users.user_find_by_username(username).await
    }

    async fn user_find_by_session_token(
        &self,
        session_token: String,
    ) -> RepoResult<Option<models::SessionUser>> {
        self.users.user_find_by_session_token(session_token).await
    }
}
