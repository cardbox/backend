use sqlx::postgres::PgTypeInfo;
use sqlx::{Postgres, Type};
use uuid::Uuid;

#[derive(Debug, sqlx::FromRow, sqlx::Type)]
pub(crate) struct Social {
    pub(crate) id: Uuid,
    pub(crate) user_id: Uuid,
    pub(crate) name: String,
    pub(crate) link: String,
}

#[derive(Debug, sqlx::Decode)]
pub(crate) struct Socials(pub(crate) Vec<Social>);

impl Type<Postgres> for Socials {
    fn type_info() -> PgTypeInfo {
        PgTypeInfo::with_name("_socials")
    }
}

impl From<Social> for cardbox_core::models::Social {
    #[inline]
    fn from(s: Social) -> Self {
        Self {
            id: s.id,
            user_id: s.user_id,
            name: s.name,
            link: s.link,
        }
    }
}
