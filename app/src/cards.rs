use crate::{App, Service};
use cardbox_core::app::{
    CardCreateError, CardCreateForm, CardDeleteError, CardSearchError, CardUpdateError,
    CardUpdateForm, Cards,
};
use cardbox_core::contracts::Repository;
use cardbox_core::models::{Card, CardCreate, CardUpdate, User};
use itertools::Itertools;
use sqlx_core::types::{Json, Uuid};
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
            .and_then(|token| (!token.is_expired()).then(|| token))
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
            if token.is_expired() {
                return Err(CardUpdateError::TokenExpired);
            }

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

    async fn card_delete(&self, card_id: Uuid, token: String) -> Result<Uuid, CardDeleteError> {
        let db = self.get::<Service<dyn Repository>>()?;
        let token = db.token_find(token).await?;

        if let Some(token) = token {
            if token.is_expired() {
                return Err(CardDeleteError::TokenExpired);
            }

            let card_to_delete = db.card_find_by_id(card_id).await?;

            match card_to_delete {
                Some(card) => {
                    if card.author_id != token.user_id {
                        return Err(CardDeleteError::NoAccess);
                    }

                    let updated = db.card_delete(card_id, token.user_id).await?;

                    match updated {
                        Some(card_id) => Ok(card_id),
                        None => Err(CardDeleteError::CardNotFound),
                    }
                }
                None => return Err(CardDeleteError::CardNotFound),
            }
        } else {
            Err(CardDeleteError::TokenNotFound)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::mock_app;
    use cardbox_core::app::{CardCreateError, CardCreateForm, CardDeleteError, Cards};
    use cardbox_core::contracts::MockDb;
    use cardbox_core::models::{Card, SessionToken};
    use lazy_static::lazy_static;
    use uuid::Uuid;

    lazy_static! {
        static ref JSON_CONTENT: Box<serde_json::value::RawValue> =
            serde_json::value::RawValue::from_string("[]".into()).unwrap();
    }

    #[actix_rt::test]
    async fn card_create_fails_if_no_token_in_database() -> eyre::Result<()> {
        let mut mock_db = MockDb::new();

        mock_db
            .session_tokens
            .expect_token_find()
            .returning(|_| Ok(None));

        let mock_app = mock_app(mock_db);

        let create_card = CardCreateForm {
            tags: vec![],
            contents: &JSON_CONTENT,
            title: "My card :3".into(),
        };
        let token = "non-existent session token".to_string();

        let result = mock_app.card_create(create_card, token).await;

        assert!(matches!(result, Err(CardCreateError::Unauthorized)));

        Ok(())
    }

    #[actix_rt::test]
    async fn card_create_fails_if_title_is_empty() -> eyre::Result<()> {
        let mock_db = MockDb::new();

        let mock_app = mock_app(mock_db);

        let create_card = CardCreateForm {
            tags: vec![],
            contents: &JSON_CONTENT,
            title: "".into(),
        };

        // We do not care about token here
        // because we don't want to even try going to the db if request is ill-formed
        let result = mock_app.card_create(create_card, "".into()).await;

        assert!(matches!(result, Err(CardCreateError::ValidationError(_))));

        Ok(())
    }

    #[actix_rt::test]
    async fn card_create_fails_if_token_is_expired() -> eyre::Result<()> {
        let mut mock_db = MockDb::new();
        mock_db.session_tokens.expect_token_find().returning(|_| {
            Ok(Some(SessionToken {
                // Token is expired by 2 weeks
                expires_at: chrono::Utc::now() - SessionToken::lifetime(),
                user_id: Uuid::new_v4(),
                token: "token".into(),
            }))
        });

        let mock_app = mock_app(mock_db);

        let create_card = CardCreateForm {
            tags: vec![],
            contents: &JSON_CONTENT,
            title: "My card :3".into(),
        };

        let result = mock_app.card_create(create_card, "token".into()).await;

        assert!(matches!(result, Err(CardCreateError::Unauthorized)));

        Ok(())
    }

    #[actix_rt::test]
    async fn card_create_happy_path_success() -> eyre::Result<()> {
        let mock_card = Card::create_random();

        let mut mock_db = MockDb::new();

        mock_db.session_tokens.expect_token_find().returning(|_| {
            Ok(Some(SessionToken {
                expires_at: chrono::Utc::now() + SessionToken::lifetime(),
                user_id: Uuid::new_v4(),
                token: "token".into(),
            }))
        });

        let card_clone = mock_card.clone();
        mock_db.cards.expect_card_create().returning(move |_| {
            let card_clone = card_clone.clone();
            Ok(card_clone)
        });

        let mock_app = mock_app(mock_db);

        let create_card = CardCreateForm {
            tags: vec![],
            contents: &JSON_CONTENT,
            title: mock_card.title.clone(),
        };

        let result = mock_app.card_create(create_card, "token".into()).await;

        assert!(matches!(result, Ok(card) if card.id == mock_card.id));

        Ok(())
    }

    #[actix_rt::test]
    async fn card_delete_fails_if_no_token_in_database() -> eyre::Result<()> {
        let mut mock_db = MockDb::new();

        mock_db
            .session_tokens
            .expect_token_find()
            .returning(|_| Ok(None));

        let mock_app = mock_app(mock_db);

        let random_card = Card::create_random();
        let token = "non-existent session token".to_string();

        let result = mock_app.card_delete(random_card.id, token).await;

        assert!(matches!(result, Err(CardDeleteError::TokenNotFound)));

        Ok(())
    }

    #[actix_rt::test]
    async fn card_delete_fails_if_token_expired() -> eyre::Result<()> {
        let mut mock_db = MockDb::new();

        mock_db.session_tokens.expect_token_find().returning(|_| {
            Ok(Some(SessionToken {
                // Token is expired by 2 weeks
                expires_at: chrono::Utc::now() - SessionToken::lifetime(),
                user_id: Uuid::new_v4(),
                token: "token".into(),
            }))
        });

        let mock_app = mock_app(mock_db);

        let random_card = Card::create_random();
        let token = "token".to_string();

        let result = mock_app.card_delete(random_card.id, token).await;

        assert!(matches!(result, Err(CardDeleteError::TokenExpired)));

        Ok(())
    }

    #[actix_rt::test]
    async fn card_delete_fails_if_trying_to_delete_another_user_card() -> eyre::Result<()> {
        let user_id = Uuid::new_v4();
        let card_id = Uuid::new_v4();

        let mut mock_db = MockDb::new();

        mock_db.cards.expect_card_find_by_id().returning(move |_| {
            let mut random_card = Card::create_random();
            random_card.author_id = card_id;

            Ok(Some(random_card))
        });

        mock_db
            .session_tokens
            .expect_token_find()
            .returning(move |_| {
                Ok(Some(SessionToken {
                    expires_at: chrono::Utc::now() + SessionToken::lifetime(),
                    user_id,
                    token: "token".into(),
                }))
            });

        let mock_app = mock_app(mock_db);

        let token = "token".to_string();

        let result = mock_app.card_delete(card_id, token).await;

        assert!(matches!(result, Err(CardDeleteError::NoAccess)));

        Ok(())
    }

    #[actix_rt::test]
    async fn card_delete_happy_path_success() -> eyre::Result<()> {
        let user_id = Uuid::new_v4();
        let card_id = Uuid::new_v4();
        let mut random_card = Card::create_random();
        random_card.author_id = user_id;
        random_card.id = card_id;

        let mut mock_db = MockDb::new();

        mock_db.cards.expect_card_find_by_id().returning(move |_| {
            let mut card = Card::create_random();

            card.id = card_id;
            card.author_id = user_id;
            Ok(Some(card))
        });

        mock_db
            .cards
            .expect_card_delete()
            .returning(move |_, _| Ok(Some(card_id)));

        mock_db
            .session_tokens
            .expect_token_find()
            .returning(move |_| {
                Ok(Some(SessionToken {
                    expires_at: chrono::Utc::now() + SessionToken::lifetime(),
                    user_id,
                    token: "token".into(),
                }))
            });

        let mock_app = mock_app(mock_db);

        let token = "token".to_string();

        let id = mock_app.card_delete(card_id, token).await?;

        assert_eq!(id, card_id);

        Ok(())
    }
}
