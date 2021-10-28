use actix_web;
use actix_web::dev::AnyBody;
use actix_web::error::HttpError;
use actix_web::http::{header, HeaderValue};
use actix_web::HttpResponse;
use cardbox_core::models::SessionToken;
use cookie::{Cookie, CookieBuilder};

#[derive(Debug, Clone)]
pub struct SessionCookieConfig {
    pub name: String,
    pub path: String,
    pub secure: bool,
    pub http_only: bool,
}

impl SessionCookieConfig {
    pub fn to_cookie(
        &self,
        token: SessionToken,
    ) -> Result<Cookie<'static>, time::error::ComponentRange> {
        Ok(CookieBuilder::new(self.name.clone(), token.token)
            .expires(time::OffsetDateTime::from_unix_timestamp(
                token.expires_at.timestamp(),
            )?)
            .path(self.path.clone())
            .secure(self.secure)
            .http_only(self.http_only)
            .finish())
    }
}

/// Sealed trait until actix uses latest version of `cookie` crate
// TODO: remove after cookie is updated to 0.16 in `actix-web`
pub trait AddCookieExt: sealed::Sealed {
    fn add_cookie(&mut self, cookie: &Cookie<'_>) -> Result<(), HttpError>;
}

impl AddCookieExt for HttpResponse<AnyBody> {
    fn add_cookie(&mut self, cookie: &Cookie<'_>) -> Result<(), HttpError> {
        HeaderValue::from_str(&cookie.to_string())
            .map(|c| {
                self.headers_mut().append(header::SET_COOKIE, c);
            })
            .map_err(|e| e.into())
    }
}

#[doc(hidden)]
mod sealed {
    use actix_web::dev::AnyBody;
    use actix_web::HttpResponse;

    pub trait Sealed {}

    impl Sealed for HttpResponse<AnyBody> {}
}
