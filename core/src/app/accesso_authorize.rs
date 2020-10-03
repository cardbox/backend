use crate::{generator, models, repo, App};
use async_trait::async_trait;

#[async_trait]
pub trait AccessoAuthorize {
    async fn authorize(
        &mut self,
        user: UserInfo,
    ) -> Result<(models::User, models::AccessToken), UpdateUserFailure>;
}

#[derive(Debug, Clone, PartialEq)]
pub struct UserInfo {
    pub accesso_id: uuid::Uuid,
    pub first_name: String,
    pub last_name: String,
}

#[derive(Debug, PartialEq, Eq)]
pub enum UpdateUserFailure {
    Unexpected,
}

const ACCESS_TOKEN_LENGTH: u8 = 40;

#[async_trait]
impl<Database, Generator> AccessoAuthorize for App<Database, Generator>
where
    Database: repo::UserRepo + repo::AccessTokenRepo + Send + Sync,
    Generator: generator::Generator + Send + Sync,
{
    async fn authorize(
        &mut self,
        info: UserInfo,
    ) -> Result<(models::User, models::AccessToken), UpdateUserFailure> {
        let user = self.db.find_by_accesso(info.accesso_id).await?;

        let actual_user = if let Some(mut user) = user {
            user.set_first_name(info.first_name)
                .set_last_name(info.last_name);
            Ok(repo::UserRepo::save(&mut self.db, user).await?)
        } else {
            match repo::UserRepo::create(&mut self.db, info.clone().into()).await {
                Ok(user) => Ok(user),
                Err(repo::UserCreateError::UnexpectedFailure) => Err(UpdateUserFailure::Unexpected),

                // potentially impossible
                Err(repo::UserCreateError::UserAlreadyExists) => self
                    .db
                    .find_by_accesso(info.accesso_id)
                    .await?
                    .ok_or(UpdateUserFailure::Unexpected),
            }
        }?;

        let token = self.generator.secure_token(ACCESS_TOKEN_LENGTH);
        let access_token = models::AccessToken::new(actual_user.id, token);
        let token = repo::AccessTokenRepo::save(&mut self.db, access_token).await?;

        Ok((actual_user, token))
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
