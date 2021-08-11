use crate::generated::{
    components::{request_bodies::UsersSearchRequestBody, responses::UsersSearchSuccess},
    paths::users_search::{Error, Response},
};
use actix_web::web::{self, Data};
use cardbox_core::app::{UserSearchError, Users};

pub async fn route(
    app: Data<cardbox_app::App>,
    search: web::Json<UsersSearchRequestBody>,
) -> Result<Response, Error> {
    let body = search.into_inner();

    let users = app
        .users_search(&body.query)
        .await
        .map_err(map_users_search_error)?;

    Ok(Response::Ok(UsersSearchSuccess {
        users: users.into_iter().map(Into::into).collect(),
    }))
}

fn map_users_search_error(error: UserSearchError) -> Error {
    use UserSearchError::*;

    match error {
        Unexpected(e) => Error::InternalServerError(e),
    }
}
