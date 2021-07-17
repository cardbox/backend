use cardbox_core::models;
use uuid::Uuid;

#[derive(Debug, sqlx::FromRow)]
pub(crate) struct User {
    pub(crate) id: Uuid,
    pub(crate) accesso_id: Uuid,
    pub(crate) first_name: String,
    pub(crate) last_name: String,
}

impl Into<models::User> for User {
    fn into(self) -> models::User {
        models::User {
            id: self.id,
            accesso_id: self.accesso_id,
            first_name: self.first_name,
            last_name: self.last_name,
        }
    }
}
