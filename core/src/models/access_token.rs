/// TODO: how to guarantee model validity
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AccessToken {
    /// cardbox user
    pub user_id: uuid::Uuid,
    pub token: String,
    pub expires_at: chrono::NaiveDateTime,
}

impl AccessToken {
    pub fn lifetime() -> chrono::Duration {
        chrono::Duration::days(14)
    }

    pub fn new(user_id: uuid::Uuid, token: String) -> Self {
        Self {
            user_id,
            token,
            expires_at: chrono::Utc::now().naive_utc() + Self::lifetime(),
        }
    }

    pub fn user_id(&self) -> uuid::Uuid {
        self.user_id
    }

    pub fn token(&self) -> String {
        self.token.clone()
    }

    pub fn expires_at(&self) -> chrono::NaiveDateTime {
        self.expires_at.clone()
    }

    /// Check, is access token expired from current time
    pub fn is_expired(&self) -> bool {
        chrono::Utc::now().naive_utc() > self.expires_at
    }
}
