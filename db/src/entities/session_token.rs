use cardbox_core::models;
use chrono::{DateTime, Utc};
use sqlx::decode::Decode;
use sqlx::postgres::{PgTypeInfo, PgValueRef};
use sqlx::{Postgres, Type};

#[derive(Debug, sqlx::FromRow)]
pub(crate) struct SessionToken {
    pub(crate) user_id: uuid::Uuid,
    pub(crate) token: String,
    pub(crate) expires_at: DateTime<Utc>,
}

impl<'r> Decode<'r, Postgres> for SessionToken {
    fn decode(
        value: PgValueRef<'r>,
    ) -> Result<Self, Box<dyn ::std::error::Error + 'static + Send + Sync>> {
        let mut decoder = ::sqlx::postgres::types::PgRecordDecoder::new(value)?;
        let user_id = decoder.try_decode::<uuid::Uuid>()?;
        let token = decoder.try_decode::<String>()?;
        let expires_at = decoder.try_decode::<DateTime<Utc>>()?;
        Result::Ok(SessionToken {
            user_id,
            token,
            expires_at,
        })
    }
}

impl Type<Postgres> for SessionToken {
    fn type_info() -> PgTypeInfo {
        PgTypeInfo::with_name("SessionToken")
    }
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
