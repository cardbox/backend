use crate::entities::SessionToken;
use crate::entities::Socials;
use cardbox_core::models;
use sqlx::decode::Decode;
use sqlx::postgres::types::PgRecordDecoder;
use sqlx::postgres::{PgTypeInfo, PgValueRef};
use sqlx::{Postgres, Type};
use uuid::Uuid;

#[derive(Debug, sqlx::FromRow)]
pub(crate) struct User {
    pub(crate) id: Uuid,
    pub(crate) accesso_id: Uuid,
    pub(crate) first_name: String,
    pub(crate) last_name: String,
    pub(crate) username: Option<String>,
    pub(crate) bio: Option<String>,
    pub(crate) avatar: Option<String>,
    pub(crate) work: Option<String>,
    pub(crate) socials: Option<Socials>,
}

impl<'r> Decode<'r, Postgres> for User {
    fn decode(
        value: PgValueRef<'r>,
    ) -> Result<Self, Box<dyn ::std::error::Error + 'static + Send + Sync>> {
        let mut decoder = PgRecordDecoder::new(value)?;
        let id = decoder.try_decode::<Uuid>()?;
        let accesso_id = decoder.try_decode::<Uuid>()?;
        let first_name = decoder.try_decode::<String>()?;
        let last_name = decoder.try_decode::<String>()?;
        let username = decoder.try_decode::<Option<String>>()?;
        let bio = decoder.try_decode::<Option<String>>()?;
        let avatar = decoder.try_decode::<Option<String>>()?;
        let work = decoder.try_decode::<Option<String>>()?;
        let socials = decoder.try_decode::<Option<Socials>>()?;

        Result::Ok(User {
            id,
            accesso_id,
            first_name,
            last_name,
            username,
            bio,
            avatar,
            work,
            socials,
        })
    }
}

impl Type<Postgres> for User {
    fn type_info() -> PgTypeInfo {
        PgTypeInfo::with_name("User")
    }
}

#[derive(Debug, sqlx::FromRow, sqlx::Type)]
pub(crate) struct SessionUser {
    pub(crate) id: Uuid,
    pub(crate) accesso_id: Uuid,
    pub(crate) first_name: String,
    pub(crate) last_name: String,
    pub(crate) session_token: SessionToken,
}

impl From<User> for models::User {
    #[inline]
    fn from(u: User) -> Self {
        Self {
            id: u.id,
            accesso_id: u.accesso_id,
            first_name: u.first_name,
            last_name: u.last_name,
            username: u.username,
            bio: u.bio,
            avatar: u.avatar,
            work: u.work,
            socials: u.socials.map(|s| s.0.into_iter().map(Into::into).collect()),
        }
    }
}

impl From<SessionUser> for models::SessionUser {
    #[inline]
    fn from(u: SessionUser) -> Self {
        Self {
            id: u.id,
            accesso_id: u.accesso_id,
            first_name: u.first_name,
            last_name: u.last_name,
            expired: models::SessionToken::from(u.session_token).is_expired(),
        }
    }
}
