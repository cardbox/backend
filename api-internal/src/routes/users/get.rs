use crate::generated::{
    components::{
        request_bodies::UsersGetRequestBody,
        responses::{UsersGetError as FailureVariant, UsersGetFailed as Failure, UsersGetSuccess},
        schemas,
    },
    paths::users_get::{Error, Response},
};
use actix_web::web::{Data, Json};
use cardbox_core::app::{UserGetError, Users};
use cardbox_core::models::{Social, User};

pub async fn route(
    app: Data<cardbox_app::App>,
    body: Json<UsersGetRequestBody>,
) -> Result<Response, Error> {
    let body = body.into_inner();

    let user = app
        .user_get(body.username)
        .await
        .map_err(map_user_get_error)?;

    Ok(Response::Ok(UsersGetSuccess { user: user.into() }))
}

fn map_user_get_error(error: UserGetError) -> Error {
    use UserGetError::*;

    match error {
        Unexpected(e) => Error::InternalServerError(e),
        UserNotFound => Failure {
            error: FailureVariant::UserNotFound,
        }
        .into(),
    }
}

impl From<cardbox_core::models::User> for schemas::User {
    #[inline]
    fn from(u: User) -> Self {
        let id_str = u.id.to_string();
        let username = u.username.unwrap_or(id_str);

        Self {
            id: u.id,
            first_name: u.first_name,
            last_name: u.last_name,
            socials: u
                .socials
                .unwrap_or_else(Vec::new)
                .into_iter()
                .map(Into::into)
                .collect(),
            avatar: u.avatar,
            work: u.work,
            bio: u.bio,
            username,
        }
    }
}

impl From<cardbox_core::models::Social> for schemas::Social {
    #[inline]
    fn from(s: Social) -> Self {
        Self {
            link: s.link,
            name: s.name,
        }
    }
}
