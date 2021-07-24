#[cfg(feature = "testing")]
use mockall::*;

use super::RepoResult;
use crate::models::{Card, CardCreate, CardUpdate, User};
use uuid::Uuid;

#[cfg_attr(feature = "testing", automock)]
#[async_trait]
pub trait CardRepo {
    async fn card_create<'a>(&self, card: CardCreate<'a>) -> RepoResult<Card>;

    async fn cards_search(&self, query: &str, limit: Option<i64>) -> RepoResult<Vec<(Card, User)>>;

    async fn card_update<'a>(
        &self,
        card: CardUpdate<'a>,
        user_id: Uuid,
    ) -> RepoResult<Option<Card>>;
}

#[cfg(feature = "testing")]
#[async_trait]
impl CardRepo for crate::contracts::MockDb {
    async fn card_create<'a>(&self, card: CardCreate<'a>) -> RepoResult<Card> {
        self.cards.card_create(card).await
    }

    async fn cards_search(&self, query: &str, limit: Option<i64>) -> RepoResult<Vec<(Card, User)>> {
        self.cards.cards_search(query, limit).await
    }

    async fn card_update<'a>(
        &self,
        card: CardUpdate<'a>,
        user_id: Uuid,
    ) -> RepoResult<Option<Card>> {
        self.cards.card_update(card, user_id).await
    }
}