use crate::entities::{Card, User};
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
            card.contents.unwrap_or_else(|| json!([]))
        )
        .fetch_one(&self.pool)
        .await?
        .into())
    }

    async fn cards_search(
        &self,
        query: &str,
        limit: Option<i64>,
    ) -> RepoResult<Vec<(models::Card, models::User)>> {
        #[derive(Debug, sqlx::FromRow)]
        struct UserCard {
            user: User,
            card: Card,
        }

        let found = sqlx::query_as!(
            UserCard,
            // language=PostgreSQL
            r#"
            SELECT 
               (u.id, u.accesso_id, u.first_name, u.last_name) as "user!: User",
               (c.id, c.author_id, c.title, c.created_at, c.updated_at, c.contents, c.tags) as "card!: Card"
            FROM cards as c
            JOIN users u on u.id = c.author_id
            WHERE c.title ILIKE $1
               OR c.tags @> (ARRAY [$2::varchar])
               OR jsonb_to_tsvector_multilang(
                    jsonb_path_query_array(c.contents, 'strict $.**.text'), '[
                      "string"
                    ]')
                @@ to_tsquery($2)
            LIMIT $3
            "#,
            format!("%{}%", query),
            query.replace(' ', " & "),
            limit.unwrap_or(100)
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(found
            .into_iter()
            .map(|user_card| (user_card.card.into(), user_card.user.into()))
            .collect())
    }
}
