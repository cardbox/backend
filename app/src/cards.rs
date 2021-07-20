use crate::{App, Service};
use cardbox_core::app::{Cards, NewCardError, NewCardForm};
use cardbox_core::contracts::Repository;
use cardbox_core::models::{Card, CardCreate};
use validator::Validate;

#[async_trait]
impl Cards for App {
    async fn new_card(&self, card: NewCardForm, token: String) -> Result<Card, NewCardError> {
        card.validate()?;

        let db = self.get::<Service<dyn Repository>>()?;
        let token = db.find_token(token).await?;

        let card_create = token
            .and_then(|token| token.is_expired().then(|| token))
            .map(|token| token.user_id)
            .map(|user_id| CardCreate {
                user_id,
                title: card.title,
                tags: card.tags,
                contents: Some(card.contents),
            });

        match card_create {
            Some(card) => Ok(db.create_card(card).await?),
            None => Err(NewCardError::Unauthorized),
        }
    }
}
