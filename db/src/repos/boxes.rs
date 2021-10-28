use crate::entities::{self, BoxType};
use crate::mappers::{sqlx_error_to_card_save_error, sqlx_error_to_card_unsave_error};
use crate::Database;
use cardbox_core::app::{CardSaveError, CardUnsaveError};
use cardbox_core::contracts::{BoxRepo, CardRepo, RepoResult};
use cardbox_core::models;
use cardbox_core::models::Card;
use uuid::Uuid;

#[async_trait]
impl BoxRepo for Database {
    async fn box_get_by_id(&self, id: Uuid) -> RepoResult<Option<models::Box>> {
        Ok(sqlx::query_as!(
            entities::Box,
            // language=PostgreSQL
            r#"
            SELECT id, user_id, type as "type!: BoxType", "default"
            FROM boxes
            WHERE id = $1
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await?
        .map(Into::into))
    }

    async fn box_get_user_default(&self, user_id: Uuid) -> RepoResult<models::Box> {
        Ok(sqlx::query_as!(
            entities::Box,
            // language=PostgreSQL
            r#"
            SELECT id, user_id, type as "type!: BoxType", "default"
            FROM boxes
            WHERE (user_id, "default") = ($1, true)
            "#,
            user_id
        )
        .fetch_one(&self.pool)
        .await?
        .into())
    }

    async fn box_add_card(
        &self,
        box_id: Uuid,
        card_id: Uuid,
    ) -> Result<models::Card, CardSaveError> {
        let card = self.card_find_by_id(card_id).await?;

        match card {
            Some((card, _)) => {
                let rows_affected = sqlx::query!(
                    // language=PostgreSQL
                    r#"
                    INSERT INTO boxes_cards
                    (box_id, card_id) VALUES ($1, $2)
                    "#,
                    box_id,
                    card_id
                )
                .execute(&self.pool)
                .await
                .map_err(sqlx_error_to_card_save_error)?
                .rows_affected();

                if rows_affected == 0 {
                    tracing::warn!(
                        "Something weird is going on! Zero rows affected when trying to add card"
                    );
                }

                Ok(card)
            }
            None => {
                tracing::error!(%card_id, "Could not find card");
                Err(CardSaveError::CardNotFound)
            }
        }
    }

    async fn box_remove_card(&self, box_id: Uuid, card_id: Uuid) -> Result<Card, CardUnsaveError> {
        let card = self.card_find_by_id(card_id).await?;

        match card {
            Some((card, _)) => {
                let rows_affected = sqlx::query!(
                    // language=PostgreSQL
                    r#"
                    DELETE FROM boxes_cards WHERE
                    (box_id, card_id) = ($1, $2)
                    "#,
                    box_id,
                    card_id
                )
                .execute(&self.pool)
                .await
                .map_err(sqlx_error_to_card_unsave_error)?
                .rows_affected();

                if rows_affected == 0 {
                    tracing::warn!(
                        "Something weird is going on! Zero rows affected when trying to remove card from box"
                    );
                }

                Ok(card)
            }
            None => {
                tracing::error!(%card_id, "Could not find card");
                Err(CardUnsaveError::CardNotFound)
            }
        }
    }
}
