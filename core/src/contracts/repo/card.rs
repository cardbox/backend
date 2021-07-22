#[cfg(feature = "testing")]
use mockall::*;

use super::RepoResult;
use crate::models::{Card, CardCreate};

#[cfg_attr(feature = "testing", automock)]
#[async_trait]
pub trait CardRepo {
    async fn card_create(&self, card: CardCreate) -> RepoResult<Card>;

    async fn cards_search(&self, query: &str, limit: Option<i64>) -> RepoResult<Vec<Card>>;
}
