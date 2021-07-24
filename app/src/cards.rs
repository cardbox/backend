use crate::{App, Service};
use cardbox_core::app::{
    CardCreateError, CardCreateForm, CardSearchError, CardUpdateError, CardUpdateForm, Cards,
};
use cardbox_core::contracts::Repository;
use cardbox_core::models::{Card, CardCreate, CardUpdate, User};
use itertools::Itertools;
use sqlx_core::types::Json;
use validator::Validate;

#[async_trait]
impl Cards for App {
    async fn card_create<'a>(
        &self,
        card: CardCreateForm<'a>,
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
                contents: Json(card.contents),
            });

        match card_create {
            Some(card) => Ok(db.card_create(card).await?),
            None => Err(CardCreateError::Unauthorized),
        }
    }

    async fn cards_search(
        &self,
        query: &str,
        limit: Option<i64>,
    ) -> Result<Vec<(Card, User)>, CardSearchError> {
        let db = self.get::<Service<dyn Repository>>()?;

        let search_results = db.cards_search(query, limit).await?;

        Ok(search_results
            .into_iter()
            .unique_by(|(c, _)| c.id)
            .collect())
    }

    async fn card_update<'a>(
        &self,
        card: CardUpdateForm<'a>,
        token: String,
    ) -> Result<Card, CardUpdateError> {
        let db = self.get::<Service<dyn Repository>>()?;

        let token = db.token_find(token).await?;

        if let Some(token) = token {
            let updated = db
                .card_update(
                    CardUpdate {
                        id: card.id,
                        contents: card.contents.map(Json),
                        title: card.title,
                        tags: card.tags,
                    },
                    token.user_id,
                )
                .await?;

            match updated {
                Some(card) => Ok(card),
                None => Err(CardUpdateError::CardNotFound),
            }
        } else {
            Err(CardUpdateError::TokenNotFound)
        }
    }
}
