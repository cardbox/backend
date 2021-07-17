use cardbox_core::models;
use chrono::{DateTime, Utc};

#[derive(Debug, sqlx::FromRow)]
pub struct SessionToken {
    pub user_id: uuid::Uuid,
    pub token: String,
    pub expires_at: DateTime<Utc>,
}

impl Into<models::SessionToken> for SessionToken {
    fn into(self) -> models::SessionToken {
        models::SessionToken {
            user_id: self.user_id,
            token: self.token,
            expires_at: self.expires_at,
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
