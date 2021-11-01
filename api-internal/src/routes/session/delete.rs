use crate::generated::paths::session_delete::Error;
use actix_web::http::header::SET_COOKIE;
use actix_web::http::{HeaderValue, StatusCode};
use actix_web::web::Json;
use actix_web::{web, Responder};
use cardbox_core::app::extractors::SessionToken;
use cardbox_core::app::session::{Session, SessionDeleteError, SessionDeleteStrategy};
use cookie::CookieBuilder;
use eyre::WrapErr;
use crate::generated::components::request_bodies;
use crate::generated::components::responses::{SessionDeleteFailure, SessionDeleteFailureError};

pub async fn route(
    body: web::Json<request_bodies::SessionDelete>,
    session_config: web::Data<cardbox_app::SessionCookieConfig>,
    app: web::Data<cardbox_app::App>,
    token: SessionToken
) -> Result<impl Responder, Error> {

    let token_str = token.into_inner();
    let strategy = match body.delete_all_sessions {
        true => SessionDeleteStrategy::All,
        false => SessionDeleteStrategy::Single(token_str.clone()),
    };

    app.session_delete(token_str, strategy)
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
        SessionDeleteError::TokenNotFound => SessionDeleteFailure {
            error: SessionDeleteFailureError::Unknown
        }.into()
    }
}
