use crate::models;
use async_trait::async_trait;
use uuid::Uuid;

#[async_trait]
pub trait UserRepo {
    async fn upsert_user_by_accesso(
        &mut self,
        accesso_id: Uuid,
        update: AccessoUserUpdate,
    ) -> models::User;
}

pub struct AccessoUserUpdate {
    pub first_name: String,
    pub last_name: String,
}
