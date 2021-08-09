use crate::{App, Service};
use cardbox_core::app::{
    CardCreateError, CardCreateForm, CardDeleteError, CardGetError, CardSaveError, CardSearchError,
    CardUpdateError, CardUpdateForm, Cards, CardsListError,
};
use cardbox_core::contracts::Repository;
use cardbox_core::models::{Card, CardCreate, CardUpdate, User};
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

        Ok(search_results)
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

    #[tracing::instrument]
    async fn card_add_to_box(
        &self,
        card_id: Uuid,
        box_id: Option<Uuid>,
        token: String,
    ) -> Result<(Card, Uuid), CardSaveError> {
        let db = self.get::<Service<dyn Repository>>()?;
        let token = db.token_find(token).await?;

        if let Some(token) = token {
            if token.is_expired() {
                return Err(CardSaveError::TokenExpired);
            }

            let card_to_save = db.card_find_by_id(card_id).await?;

            match card_to_save {
                Some(card) => {
                    if card.author_id != token.user_id {
                        return Err(CardSaveError::NoAccess);
                    }

                    let box_id = match box_id {
                        Some(id) => {
                            let r#box = db.box_get_by_id(id).await?;

                            match r#box {
                                Some(_) => id,
                                None => return Err(CardSaveError::BoxNotFound),
                            }
                        }
                        None => db.box_get_user_default(token.user_id).await?.id,
                    };

                    let card = db.box_add_card(box_id, card_id).await?;

                    Ok((card, box_id))
                }
                None => return Err(CardSaveError::CardNotFound),
            }
        } else {
            Err(CardSaveError::TokenNotFound)
        }
    }

    async fn cards_list(
        &self,
        author_id: Option<Uuid>,
        token: Option<String>,
        favorites: bool,
    ) -> Result<Vec<Card>, CardsListError> {
        let db = self.get::<Service<dyn Repository>>()?;

        let get_cards = |id: Uuid| {
            Box::pin(async move {
                match favorites {
                    true => db.cards_favorites_of_user(id).await,
                    false => db.cards_list(id).await,
                }
            })
        };

        match token {
            None => match author_id {
                Some(author_id) => Ok(get_cards(author_id).await?),
                None => Err(CardsListError::Unauthorized),
            },
            Some(token) => match author_id {
                Some(author_id) => Ok(get_cards(author_id).await?),
                None => {
                    let token = db.token_find(token).await?;

                    match token {
                        None => Err(CardsListError::Unauthorized),
                        Some(token) => Ok(get_cards(token.user_id).await?),
                    }
                }
            },
        }
    }

    async fn card_get(&self, card_id: Uuid) -> Result<Card, CardGetError> {
        let db = self.get::<Service<dyn Repository>>()?;

        let card = db.card_find_by_id(card_id).await?;

        card.map(Ok).unwrap_or(Err(CardGetError::CardNotFound))
    }
}

#[cfg(test)]
mod tests {
    use crate::mock_app;
    use cardbox_core::app::{
        CardCreateError, CardCreateForm, CardDeleteError, CardSaveError, Cards,
    };
    use cardbox_core::contracts::MockDb;
    use cardbox_core::models::{self, Card, SessionToken};
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

    #[actix_rt::test]
    async fn card_save_fails_if_no_token_in_database() -> eyre::Result<()> {
        let mut mock_db = MockDb::new();

        mock_db
            .session_tokens
            .expect_token_find()
            .returning(|_| Ok(None));

        let mock_app = mock_app(mock_db);

        let random_card = Card::create_random();
        let token = "non-existent session token".to_string();

        let result = mock_app.card_add_to_box(random_card.id, None, token).await;

        assert!(matches!(result, Err(CardSaveError::TokenNotFound)));

        Ok(())
    }

    #[actix_rt::test]
    async fn card_save_fails_if_token_expired() -> eyre::Result<()> {
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

        let result = mock_app.card_add_to_box(random_card.id, None, token).await;

        assert!(matches!(result, Err(CardSaveError::TokenExpired)));

        Ok(())
    }

    #[actix_rt::test]
    async fn card_save_happy_path_success() -> eyre::Result<()> {
        let user_id = Uuid::new_v4();
        let card_id = Uuid::new_v4();
        let box_id = Uuid::new_v4();

        let mut random_box = models::Box::create_random();
        random_box.user_id = user_id;
        random_box.id = box_id;

        let mut random_card = Card::create_random();
        random_card.author_id = user_id;
        random_card.id = card_id;

        let mut mock_db = MockDb::new();

        mock_db
            .boxes
            .expect_box_add_card()
            .returning(move |_, _| Ok(random_card.clone()));

        mock_db
            .boxes
            .expect_box_get_user_default()
            .returning(move |user_id| {
                let random_box = random_box.clone();
                assert_eq!(user_id, random_box.user_id);
                Ok(random_box)
            });

        mock_db.cards.expect_card_find_by_id().returning(move |_| {
            let mut card = Card::create_random();

            card.id = card_id;
            card.author_id = user_id;
            Ok(Some(card))
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

        let (card, ret_box_id) = mock_app.card_add_to_box(card_id, None, token).await?;

        assert_eq!(card.id, card_id);
        assert_eq!(box_id, ret_box_id);

        Ok(())
    }
}
