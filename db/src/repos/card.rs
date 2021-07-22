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

    async fn cards_search(&self, query: &str, limit: Option<i64>) -> RepoResult<Vec<models::Card>> {
        let found = sqlx::query_as!(
            Card,
            // language=PostgreSQL
            r#"
            SELECT id, author_id, title, created_at, updated_at, contents, tags
            FROM cards
            WHERE title ILIKE $1
               OR tags @> (ARRAY [$2::varchar])
               OR jsonb_to_tsvector('english',
                    jsonb_path_query_array(contents, 'strict $.**.text'), '[
                      "string"
                    ]')
                @@ to_tsquery($2)
            LIMIT $3
            "#,
            format!("%{}%", query),
            query,
            limit.unwrap_or(100)
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(found.into_iter().map(Into::into).collect())
    }
}
