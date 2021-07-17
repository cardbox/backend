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

// const ACCESS_TOKEN_LENGTH: u8 = 60;
//
// #[async_trait]
// impl<Database, Generator> AccessoAuthorize for App<Database, Generator>
// where
//     Database: repo::UserRepo + repo::SessionTokenRepo + Send + Sync,
//     Generator: generator::Generator + Send + Sync,
// {
//     async fn authorize(
//         &mut self,
//         info: UserInfo,
//     ) -> Result<(models::User, models::SessionToken), UpdateUserFailure> {
//         let user = self.db.find_by_accesso(info.accesso_id).await?;
//
//         let actual_user = if let Some(mut user) = user {
//             user.set_first_name(info.first_name)
//                 .set_last_name(info.last_name);
//             Ok(repo::UserRepo::user_update(&mut self.db, user).await?)
//         } else {
//             match repo::UserRepo::user_create(&mut self.db, info.clone().into()).await {
//                 Ok(user) => Ok(user),
//                 Err(repo::UserCreateError::UnexpectedFailure) => Err(UpdateUserFailure::Unexpected),
//
//                 // potentially impossible
//                 Err(repo::UserCreateError::UserAlreadyExists) => self
//                     .db
//                     .find_by_accesso(info.accesso_id)
//                     .await?
//                     .ok_or(UpdateUserFailure::Unexpected),
//             }
//         }?;
//
//         let token = self.generator.secure_token(ACCESS_TOKEN_LENGTH);
//         let access_token = models::SessionToken::new(actual_user.id, token);
//         let token = repo::SessionTokenRepo::create(&mut self.db, access_token).await?;
//
//         Ok((actual_user, token))
//     }
// }
