use cardbox_core::models;
use chrono::{DateTime, Utc};

#[derive(Debug, sqlx::FromRow)]
pub(crate) struct SessionToken {
    pub(crate) user_id: uuid::Uuid,
    pub(crate) token: String,
    pub(crate) expires_at: DateTime<Utc>,
}

impl From<SessionToken> for models::SessionToken {
    fn from(token: SessionToken) -> Self {
        Self {
            user_id: token.user_id,
            token: token.token,
            expires_at: token.expires_at,
        }
    }
}

impl From<models::SessionToken> for SessionToken {
    fn from(token: models::SessionToken) -> Self {
        Self {
            user_id: token.user_id,
            token: token.token,
            expires_at: token.expires_at,
        }
    }
}
