use crate::generated::{
    paths::session_delete::{Error, Response},
};
use actix_web::http::header::SET_COOKIE;
use actix_web::http::HeaderValue;
use actix_web::{web, Responder};
use eyre::WrapErr;
use cardbox_core::app::extractors::SessionToken;
use cardbox_core::app::session::{Session, SessionDeleteError};
use cookie::{CookieBuilder};

pub async fn route(
    session_config: web::Data<cardbox_app::SessionCookieConfig>,
    app: web::Data<cardbox_app::App>,
    token: SessionToken
) -> Result<impl Responder, Error> {

    app.session_delete(token.into_inner())
        .await
        .map_err(map_session_delete_error)?;

    let cookie = CookieBuilder::new(session_config.name.to_owned(), "")
        // TODO: extract to function or Trait
        .expires(time::OffsetDateTime::now_utc())
        .path(session_config.path.to_owned())
        .secure(session_config.secure)
        .http_only(session_config.http_only)
        .finish();

    let header_value = HeaderValue::from_str(&cookie.to_string())
        .wrap_err("Could not create header value for cookie!")?;

    Ok(Response::Ok.with_header((SET_COOKIE, header_value)))
}

fn map_session_delete_error(error: SessionDeleteError) -> Error {
    match error {
        SessionDeleteError::Unexpected(e) => e.into(),
    }
}
