use crate::app::error::SessionTokenExtractorError;
use actix_web::dev::Payload;
use actix_web::error::Error;
use actix_web::{FromRequest, HttpRequest};
use futures::future::{err, ok, Ready};

/// Extractor for the session token. Right now works only for cookies.
///
/// # Examples
/// ```
/// use cardbox_core::app::extractors::SessionToken;
/// use actix_web::get;
///
/// #[get("/")]
/// async fn index(session_token: SessionToken) -> String {
///     format!("Request from user with session token: {}!", session_token.into_inner())
/// }
/// ```
#[derive(Debug)]
pub struct SessionToken(String);

impl FromRequest for SessionToken {
    type Error = actix_web::Error;
    type Future = Ready<Result<Self, Error>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        req
            // TODO: extract session token name into config
            .cookie("session-token")
            .map(|cookie| ok(SessionToken(cookie.value().to_string())))
            .unwrap_or_else(move || {
                let e = SessionTokenExtractorError::NoSessionToken;

                tracing::debug!(
                    cookies = ?req.cookies(),
                    "Failed during SessionToken extractor from cookies"
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

#[cfg(test)]
mod test {
    use crate::app::error::SessionTokenExtractorError;
    use crate::app::extractors::SessionToken;
    use actix_web::cookie::Cookie;
    use actix_web::dev::Payload;
    use actix_web::FromRequest;

    #[actix_rt::test]
    async fn session_token_extracts_correctly() -> Result<(), actix_web::Error> {
        let req = actix_web::test::TestRequest::get()
            .cookie(Cookie::new("session-token", "mytoken"))
            .to_http_request();

        let session_token = SessionToken::from_request(&req, &mut Payload::None).await?;
        assert_eq!(session_token.into_inner(), "mytoken".to_string());
        Ok(())
    }

    #[actix_rt::test]
    async fn returns_error_if_no_session_token_cookie() -> Result<(), actix_web::Error> {
        let req = actix_web::test::TestRequest::get()
            .cookie(Cookie::new("not-a-session-token", "mytoken"))
            .to_http_request();

        let session_token = SessionToken::from_request(&req, &mut Payload::None).await;

        session_token
            .err()
            .map(|e| {
                assert!(matches!(
                    e.as_error::<SessionTokenExtractorError>(),
                    Some(&SessionTokenExtractorError::NoSessionToken)
                ))
            })
            .unwrap();

        Ok(())
    }
}
