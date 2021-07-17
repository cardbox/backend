use actix_web::cookie::{Cookie, CookieBuilder};
use cardbox_core::models::SessionToken;

#[derive(Debug, Clone)]
pub struct SessionCookieConfig {
    pub name: String,
    pub path: String,
    pub secure: bool,
    pub http_only: bool,
}

impl SessionCookieConfig {
    pub fn to_cookie(&self, token: SessionToken) -> Cookie<'static> {
        CookieBuilder::new(self.name.clone(), token.token)
            .expires(time::OffsetDateTime::from_unix_timestamp(
                token.expires_at.timestamp(),
            ))
            .path(self.path.clone())
            .secure(self.secure)
            .http_only(self.http_only)
            .finish()
    }
}
