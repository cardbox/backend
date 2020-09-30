use crate::{models, repo, App};
use async_trait::async_trait;
use validator::Validate;

#[async_trait]
pub trait AccessoUpdate {
    async fn upsert_user(&mut self, user: UpsertUser) -> Result<models::User, UpdateUserFailure>;
}

#[derive(Debug, Clone, Validate, PartialEq)]
pub struct UpsertUser {
    acesso_id: uuid::Uuid,
    first_name: String,
    last_name: String,
}

#[derive(Debug, PartialEq, Eq)]
pub enum UpdateUserFailure {
    Unexpected,
}

#[async_trait]
impl<Database> AccessoUpdate for App<Database>
where
    Database: repo::UserRepo + Send,
{
    async fn upsert_user(&mut self, user: UpsertUser) -> Result<models::User, UpdateUserFailure> {
        unimplemented!()
    }
}
