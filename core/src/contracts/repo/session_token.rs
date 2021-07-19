#[cfg(feature = "testing")]
use mockall::*;

use super::RepoResult;
use crate::models::SessionToken;
use async_trait::async_trait;
use uuid::Uuid;

#[cfg_attr(feature = "testing", automock)]
#[async_trait]
pub trait SessionTokenRepo {
    async fn delete_token_by_user(&self, user_id: Uuid) -> RepoResult<u64>;
    async fn delete_token(&self, token: String) -> RepoResult<u64>;
    async fn find_token(&self, token: String) -> RepoResult<Option<SessionToken>>;
    // QUES: maybe use user: models::User instead of Uuid, because type of id is a detail of implementation
    async fn find_token_by_user(&self, user_id: Uuid) -> RepoResult<Option<SessionToken>>;
    async fn create_token(&self, token: SessionToken) -> RepoResult<SessionToken>;
}
