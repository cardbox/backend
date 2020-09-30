use crate::{models, repo, App};
use async_trait::async_trait;

#[async_trait]
pub trait AccessoAuthorize {
    async fn authorize(&mut self, user: UserInfo) -> Result<models::User, UpdateUserFailure>;
}

#[derive(Debug, Clone, PartialEq)]
pub struct UserInfo {
    accesso_id: uuid::Uuid,
    first_name: String,
    last_name: String,
}

#[derive(Debug, PartialEq, Eq)]
pub enum UpdateUserFailure {
    Unexpected,
}

#[async_trait]
impl<Database> AccessoAuthorize for App<Database>
where
    Database: repo::UserRepo + Send + Sync,
{
    async fn authorize(&mut self, info: UserInfo) -> Result<models::User, UpdateUserFailure> {
        let user = self.db.find_by_accesso(info.accesso_id).await?;

        if let Some(mut user) = user {
            user.set_first_name(info.first_name)
                .set_last_name(info.last_name);

            Ok(self.db.save(user).await?)
        } else {
            match self.db.create(info.clone().into()).await {
                Ok(user) => Ok(user),
                Err(repo::UserCreateError::UnexpectedFailure) => Err(UpdateUserFailure::Unexpected),
                Err(repo::UserCreateError::UserAlreadyExists) => self.authorize(info).await,
            }
        }
    }
}

impl From<repo::UnexpectedError> for UpdateUserFailure {
    fn from(_: repo::UnexpectedError) -> Self {
        UpdateUserFailure::Unexpected
    }
}

impl Into<repo::UserCreate> for UserInfo {
    fn into(self) -> repo::UserCreate {
        repo::UserCreate {
            accesso_id: self.accesso_id,
            first_name: self.first_name,
            last_name: self.last_name,
        }
    }
}
