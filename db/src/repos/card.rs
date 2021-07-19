use crate::entities::Card;
use crate::Database;
use cardbox_core::contracts::{CardRepo, RepoResult};
use cardbox_core::models;
use serde_json::json;

#[async_trait]
impl CardRepo for Database {
    async fn create_card(&self, card: models::CardCreate) -> RepoResult<models::Card> {
        Ok(sqlx::query_as!(
            Card,
            // language=PostgreSQL
            r#"
            INSERT INTO cards (user_id, title, contents)
            VALUES ($1, $2, $3)
            RETURNING id, user_id, title, created_at, updated_at, contents, tags
            "#,
            card.user_id,
            card.title,
            card.contents.unwrap_or_else(|| json!({}))
        )
        .fetch_one(&self.pool)
        .await?
        .into())
    }
}
