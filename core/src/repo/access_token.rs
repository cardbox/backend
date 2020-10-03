use super::RepoResult;
use crate::models::AccessToken;
use async_trait::async_trait;
use uuid::Uuid;

#[async_trait]
pub trait AccessTokenRepo {
    async fn delete_by_user(&mut self, user_id: Uuid) -> RepoResult<u16>;
    async fn delete(&mut self, token: String) -> RepoResult<u16>;
    async fn find_by_token(&self, token: String) -> RepoResult<Option<AccessToken>>;
    // QUES: maybe use user: models::User instead of Uuid, because type of id is a detail of implementation
    async fn find_by_user(&self, user_id: Uuid) -> RepoResult<Option<AccessToken>>;
    async fn save(&mut self, token: AccessToken) -> RepoResult<AccessToken>;
}
