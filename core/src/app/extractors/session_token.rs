use crate::app::error::SessionTokenExtractorError;
use actix_web::dev::Payload;
use actix_web::error::Error;
use actix_web::{FromRequest, HttpRequest};
use futures::future::{err, ok, Ready};

/// Extractor for the session token. Right now works only for cookies.
///
/// # Examples
/// ```rust
/// use cardbox_core::app::extractors::SessionToken;
///
/// #[get("/")]
/// async fn index(session_token: SessionToken) -> String {
///     format!("Request from user with session token: {}!", session_token.into_inner())
/// }
/// ```
pub struct SessionToken(pub(crate) String);

impl FromRequest for SessionToken {
    type Config = ();
    type Error = actix_web::Error;
    type Future = Ready<Result<Self, Error>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        req
            // TODO: extract session token name into config
            .cookie("session_token")
            .map(|cookie| ok(SessionToken(cookie.to_string())))
            .unwrap_or_else(move || {
                let e = SessionTokenExtractorError::NoSessionToken;

                log::debug!(
                    "Failed during SessionToken extractor from cookies. Cookies: {:?}",
                    req.cookies()
                );

                err(e.into())
            })
    }
}

impl SessionToken {
    pub fn into_inner(self) -> String {
        self.0
    }
}
