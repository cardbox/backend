use crate::generated::{
    components::responses::SessionGetSuccess,
    components::schemas,
    paths::session_get::{Error, Response},
};
use actix_web::web::Data;
use cardbox_core::app::extractors::SessionToken;
use cardbox_core::app::{UserGetError, Users};

pub async fn route(app: Data<cardbox_app::App>, token: SessionToken) -> Result<Response, Error> {
    let token = token.into_inner();

    let user = app
        .user_get_by_token(token)
        .await
        .map_err(map_user_get_error)?;

    Ok(Response::Ok(SessionGetSuccess {
        user: schemas::SessionUser {
            id: user.id,
            first_name: user.first_name,
            last_name: user.last_name,
        },
    }))
}

fn map_user_get_error(error: UserGetError) -> Error {
    use UserGetError::*;

    match error {
        Unexpected(e) => Error::InternalServerError(e),
        TokenNotFound => Error::Unauthorized,
        TokenExpired => Error::Unauthorized,
        UserNotFound => Error::Unauthorized,
    }
}
