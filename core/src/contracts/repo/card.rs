#[cfg(feature = "testing")]
use mockall::*;

use super::RepoResult;
use crate::models::{Card, CardCreate, User};
use futures::stream::BoxStream;

#[cfg_attr(feature = "testing", automock)]
#[async_trait]
pub trait CardRepo {
    async fn card_create(&self, card: CardCreate) -> RepoResult<Card>;

    fn cards_search<'a>(
        &'a self,
        query: &str,
        limit: Option<i64>,
    ) -> BoxStream<'a, RepoResult<(Card, User)>>;
}
