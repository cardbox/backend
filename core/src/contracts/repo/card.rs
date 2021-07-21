#[cfg(feature = "testing")]
use mockall::*;

use super::RepoResult;
use crate::models::{Card, CardCreate};

#[cfg_attr(feature = "testing", automock)]
#[async_trait]
pub trait CardRepo {
    async fn card_create(&self, card: CardCreate) -> RepoResult<Card>;

    async fn cards_find_by_title(&self, title: &str) -> RepoResult<Vec<Card>>;

    async fn cards_find_by_content(&self, content: &str) -> RepoResult<Vec<Card>>;

    async fn cards_find_by_tag(&self, tag: &str) -> RepoResult<Vec<Card>>;
}
