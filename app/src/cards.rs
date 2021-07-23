use crate::{App, Service};
use cardbox_core::app::{CardCreateError, CardCreateForm, CardSearchError, Cards};
use cardbox_core::contracts::Repository;
use cardbox_core::models::{Card, CardCreate, User};
use futures::stream::BoxStream;
use futures::{StreamExt, TryStreamExt};
use validator::Validate;

#[async_trait]
impl Cards for App {
    async fn card_create(
        &self,
        card: CardCreateForm,
        token: String,
    ) -> Result<Card, CardCreateError> {
        card.validate()?;

        let db = self.get::<Service<dyn Repository>>()?;
        let token = db.token_find(token).await?;

        let card_create = token
            .and_then(|token| token.is_expired().then(|| token))
            .map(|token| token.user_id)
            .map(|user_id| CardCreate {
                author_id: user_id,
                title: card.title,
                tags: card.tags,
                contents: Some(card.contents),
            });

        match card_create {
            Some(card) => Ok(db.card_create(card).await?),
            None => Err(CardCreateError::Unauthorized),
        }
    }

    fn cards_search<'a>(
        &'a self,
        query: &str,
        limit: Option<i64>,
    ) -> Result<BoxStream<'a, Result<(Card, User), CardSearchError>>, eyre::Report> {
        let db = self.get::<Service<dyn Repository>>()?;

        Ok(db.cards_search(query, limit).map_err(Into::into).boxed())
    }
}
