use chrono::{DateTime, Utc};

/// TODO: how to guarantee model validity
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SessionToken {
    /// cardbox user
    pub user_id: uuid::Uuid,
    pub token: String,
    pub expires_at: DateTime<Utc>,
}

impl SessionToken {
    pub fn lifetime() -> chrono::Duration {
        chrono::Duration::days(14)
    }

    pub fn new(user_id: uuid::Uuid, token: String) -> Self {
        Self {
            user_id,
            token,
            expires_at: chrono::Utc::now() + Self::lifetime(),
        }
    }

    /// Check, is session token expired from current time
    pub fn is_expired(&self) -> bool {
        chrono::Utc::now() > self.expires_at
    }
}
