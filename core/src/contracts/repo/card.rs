#[cfg(feature = "testing")]
use mockall::*;

use super::RepoResult;
use crate::models::{Card, CardCreate, CardUpdate, User};
use uuid::Uuid;

#[cfg_attr(feature = "testing", automock)]
#[async_trait]
pub trait CardRepo {
    async fn card_create(&self, card: CardCreate) -> RepoResult<Card>;

    async fn cards_search(&self, query: &str, limit: Option<i64>) -> RepoResult<Vec<(Card, User)>>;

    async fn card_update(&self, card: CardUpdate, user_id: Uuid) -> RepoResult<Option<Card>>;
}
