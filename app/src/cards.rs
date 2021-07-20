use crate::{App, Service};
use cardbox_core::app::{CardCreateError, CardCreateForm, Cards};
use cardbox_core::contracts::Repository;
use cardbox_core::models::{Card, CardCreate};
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
}
