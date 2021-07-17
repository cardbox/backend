use crate::{App, Service};
use cardbox_core::app::{AccessoAuthorize, UpdateUserFailure, UserInfo};
use cardbox_core::contracts::{Generator, Repository, UserCreateError};
use cardbox_core::models::{SessionToken, User};

const ACCESS_TOKEN_LENGTH: u8 = 60;

#[async_trait]
impl AccessoAuthorize for App {
    async fn authorize(&self, info: UserInfo) -> Result<(User, SessionToken), UpdateUserFailure> {
        let db = self.get::<Service<dyn Repository>>()?;
        let generator = self.get::<Service<dyn Generator>>()?;

        let user = db.find_by_accesso(info.accesso_id).await?;

        let actual_user = if let Some(mut user) = user {
            user.first_name = info.first_name;
            user.last_name = info.last_name;
            Ok(db.user_update(user).await?)
        } else {
            match db.user_create(info.clone().into()).await {
                Ok(user) => Ok(user),
                Err(UserCreateError::UnexpectedFailure(e)) => Err(UpdateUserFailure::Unexpected(e)),

                // potentially impossible
                Err(err @ UserCreateError::UserAlreadyExists) => db
                    .find_by_accesso(info.accesso_id)
                    .await?
                    .ok_or(UpdateUserFailure::Unexpected(err.into())),
            }
        }?;

        let token = generator.secure_token(ACCESS_TOKEN_LENGTH);
        let access_token = SessionToken::new(actual_user.id, token);
        let token = db.create_token(access_token).await?;

        Ok((actual_user, token))
    }
}
