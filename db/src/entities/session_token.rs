use cardbox_core::models;
use chrono::{DateTime, Utc};

#[derive(Debug, sqlx::FromRow)]
pub struct SessionToken {
    pub user_id: uuid::Uuid,
    pub token: String,
    pub expires_at: DateTime<Utc>,
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
