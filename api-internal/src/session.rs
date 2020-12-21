use actix_http::HttpMessage;
use actix_web::{
    error::{ErrorInternalServerError, ErrorUnauthorized},
    web,
};
use async_trait::async_trait;
use futures::future::{err, ok};

#[derive(Debug)]
pub struct Session {
    pub user: cardbox_core::models::User,
    pub token: String,
}

#[async_trait]
impl actix_web::FromRequest for Session {
    type Error = actix_web::Error;
    type Future = futures::future::Ready<Result<Self, Self::Error>>;
    type Config = ();

    async fn from_request(
        req: &actix_web::HttpRequest,
        _payload: &mut actix_web::dev::Payload,
    ) -> Self::Future {
        use cardbox_core::app::{ResolveBySessionTokenError::Unexpected, Session};

        let session_config = req.app_data::<web::Data<crate::server::ConfigSession>>();
        let app = req.app_data::<web::Data<crate::App>>();

        if let (Some(session_config), Some(app)) = (session_config, app) {
            if let Some(ref cookie) = req.cookie(&session_config.name) {
                let app = app.lock().await;
                let token = cookie.value().to_owned();

                match app.db.user_resolve_by_session_token(token.clone()).await {
                    Err(Unexpected) => err(ErrorInternalServerError(Null)),
                    Ok(None) => err(ErrorUnauthorized(Null)),
                    Ok(Some(user)) => ok(Self { user, token }),
                }
            } else {
                log::trace!("no cookie found");
                err(ErrorUnauthorized(Null))
            }
        } else {
            log::error!("failed to resolve crate::cookie::SessionCookieConfig or/and crate::App");
            err(ErrorInternalServerError(Null))
        }
    }
}

#[derive(Debug, serde::Serialize)]
struct Null;

impl std::fmt::Display for Null {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "")
    }
}
