use super::{RepoResult, UnexpectedError};
use crate::models;
use async_trait::async_trait;
use uuid::Uuid;

#[async_trait]
pub trait UserRepo {
    async fn find_by_id(&self, user_id: Uuid) -> RepoResult<Option<models::User>>;
    async fn find_by_accesso(&self, accesso_id: Uuid) -> RepoResult<Option<models::User>>;
    async fn save(&mut self, user: models::User) -> RepoResult<models::User>;
    async fn create(&mut self, user: UserCreate) -> Result<models::User, UserCreateError>;
}

pub struct UserCreate {
    pub accesso_id: Uuid,
    pub first_name: String,
    pub last_name: String,
}

#[derive(PartialEq, Debug, Clone, Eq)]
pub enum UserCreateError {
    UserAlreadyExists,
    UnexpectedFailure,
}
impl From<UnexpectedError> for UserCreateError {
    fn from(_: UnexpectedError) -> Self {
        UserCreateError::UnexpectedFailure
    }
}
