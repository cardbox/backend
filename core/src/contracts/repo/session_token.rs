#[cfg(feature = "testing")]
use mockall::*;

use super::RepoResult;
use crate::models::SessionToken;
use uuid::Uuid;

#[cfg_attr(feature = "testing", automock)]
#[async_trait]
pub trait SessionTokenRepo {
    async fn token_delete_by_user(&self, user_id: Uuid) -> RepoResult<u64>;
    async fn token_delete(&self, token: String) -> RepoResult<u64>;
    async fn token_find(&self, token: String) -> RepoResult<Option<SessionToken>>;
    // QUES: maybe use user: models::User instead of Uuid, because type of id is a detail of implementation
    async fn token_find_by_user(&self, user_id: Uuid) -> RepoResult<Option<SessionToken>>;
    async fn token_create(&self, token: SessionToken) -> RepoResult<SessionToken>;
}

#[cfg(feature = "testing")]
#[async_trait]
impl SessionTokenRepo for crate::contracts::MockDb {
    async fn token_delete_by_user(&self, user_id: Uuid) -> RepoResult<u64> {
        self.session_tokens.token_delete_by_user(user_id).await
    }

    async fn token_delete(&self, token: String) -> RepoResult<u64> {
        self.session_tokens.token_delete(token).await
    }

    async fn token_find(&self, token: String) -> RepoResult<Option<SessionToken>> {
        self.session_tokens.token_find(token).await
    }

    async fn token_find_by_user(&self, user_id: Uuid) -> RepoResult<Option<SessionToken>> {
        self.session_tokens.token_find_by_user(user_id).await
    }

    async fn token_create(&self, token: SessionToken) -> RepoResult<SessionToken> {
        self.session_tokens.token_create(token).await
    }
}
