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

    async fn card_delete(&self, card_id: Uuid, user_id: Uuid) -> RepoResult<Option<Uuid>>;

    async fn card_find_by_id(&self, card_id: Uuid) -> RepoResult<Option<(Card, User)>>;

    async fn cards_list(&self, user_id: Uuid) -> RepoResult<Vec<(Card, User)>>;

    async fn cards_favorites_of_user(&self, user_id: Uuid) -> RepoResult<Vec<(Card, User)>>;

    async fn cards_top(&self) -> RepoResult<Vec<(Card, User)>>;

    async fn cards_latest(&self) -> RepoResult<Vec<(Card, User)>>;
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

    async fn card_delete(&self, card_id: Uuid, user_id: Uuid) -> RepoResult<Option<Uuid>> {
        self.cards.card_delete(card_id, user_id).await
    }

    async fn card_find_by_id(&self, card_id: Uuid) -> RepoResult<Option<(Card, User)>> {
        self.cards.card_find_by_id(card_id).await
    }

    async fn cards_list(&self, user_id: Uuid) -> RepoResult<Vec<(Card, User)>> {
        self.cards.cards_list(user_id).await
    }

    async fn cards_favorites_of_user(&self, user_id: Uuid) -> RepoResult<Vec<(Card, User)>> {
        self.cards.cards_favorites_of_user(user_id).await
    }

    async fn cards_latest(&self) -> RepoResult<Vec<(Card, User)>> {
        self.cards.cards_latest().await
    }

    async fn cards_top(&self) -> RepoResult<Vec<(Card, User)>> {
        self.cards.cards_top().await
    }
}
