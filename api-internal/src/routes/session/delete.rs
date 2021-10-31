use crate::generated::{
    paths::session_delete::{Error},
};
use actix_web::http::header::SET_COOKIE;
use actix_web::http::{HeaderValue, StatusCode};
use actix_web::{web, Responder};
use actix_web::web::Json;
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

    Ok(Json(serde_json::value::Value::Null)
        .with_status(StatusCode::OK)
        .with_header((SET_COOKIE, header_value))
        .with_header(("content-type", "application/json")))
}

fn map_session_delete_error(error: SessionDeleteError) -> Error {
    match error {
        SessionDeleteError::Unexpected(e) => e.into(),
    }
}
