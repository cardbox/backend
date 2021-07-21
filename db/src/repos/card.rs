use crate::entities::Card;
use crate::Database;
use cardbox_core::contracts::{CardRepo, RepoResult};
use cardbox_core::models;
use serde_json::json;

#[async_trait]
impl CardRepo for Database {
    async fn card_create(&self, card: models::CardCreate) -> RepoResult<models::Card> {
        Ok(sqlx::query_as!(
            Card,
            // language=PostgreSQL
            r#"
            INSERT INTO cards (author_id, title, contents)
            VALUES ($1, $2, $3)
            RETURNING id, author_id, title, created_at, updated_at, contents, tags
            "#,
            card.author_id,
            card.title,
            card.contents.unwrap_or_else(|| json!({}))
        )
        .fetch_one(&self.pool)
        .await?
        .into())
    }

    async fn cards_find_by_title(&self, title: &str) -> RepoResult<Vec<models::Card>> {
        let found = sqlx::query_as!(
            Card,
            // language=PostgreSQL
            r#"
            SELECT id, author_id, title, created_at, updated_at, contents, tags
            FROM cards
            WHERE title ILIKE $1
            "#,
            format!("%{}%", title),
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(found.into_iter().map(Into::into).collect())
    }

    async fn cards_find_by_content(&self, content: &str) -> RepoResult<Vec<models::Card>> {
        let found = sqlx::query_as!(
            Card,
            // language=PostgreSQL
            r#"
            SELECT id, author_id, title, created_at, updated_at, contents, tags
            FROM cards
            WHERE jsonb_to_tsvector('english', jsonb_path_query_array(contents, 'strict $.**.text'), '[
              "string"
            ]')
            @@ to_tsquery($1)
            "#,
            content,
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(found.into_iter().map(Into::into).collect())
    }

    async fn cards_find_by_tag(&self, tag: &str) -> RepoResult<Vec<models::Card>> {
        let found = sqlx::query_as!(
            Card,
            // language=PostgreSQL
            r#"
            SELECT id, author_id, title, created_at, updated_at, contents, tags
            FROM cards
            WHERE tags @> (ARRAY[$1::varchar])
            "#,
            tag
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(found.into_iter().map(Into::into).collect())
    }
}
