use crate::entities::{Card, User};
use crate::Database;
use cardbox_core::contracts::{CardRepo, RepoResult};
use cardbox_core::models;
use cardbox_core::models::CardUpdate;
use uuid::Uuid;

/// Helper struct
#[derive(Debug, sqlx::FromRow)]
struct UserCard {
    user: User,
    card: Card,
}

#[async_trait]
impl CardRepo for Database {
    async fn card_create<'a>(&self, card: models::CardCreate<'a>) -> RepoResult<models::Card> {
        Ok(sqlx::query_as!(
            Card,
            // language=PostgreSQL
            r#"
            INSERT INTO cards (author_id, title, contents, tags)
            VALUES ($1, $2, $3, $4)
            RETURNING id, author_id, title, created_at, updated_at, contents, tags
            "#,
            card.author_id,
            card.title,
            &card.contents as _,
            &card.tags
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
        let found = sqlx::query_as!(
            UserCard,
            // language=PostgreSQL
            r#"
            SELECT DISTINCT ON (c.id)
               (u.id, u.accesso_id, u.first_name, u.last_name, u.username, u.bio, u.avatar, u.work, (array_agg((s.id, s.user_id, s.name, s.link)) FILTER ( WHERE (s.id IS NOT NULL) ))) as "user!: User",
               (c.id, c.author_id, c.title, c.created_at, c.updated_at, c.contents, c.tags) as "card!: Card"
            FROM cards as c
            JOIN users u on u.id = c.author_id
            LEFT OUTER JOIN socials s ON u.id = s.user_id
            WHERE c.title ILIKE $1
               OR c.tags @> (ARRAY [$2::varchar])
               OR jsonb_to_tsvector_multilang(
                    jsonb_path_query_array(c.contents, 'strict $.**.text'), '[
                      "string"
                    ]')
                @@ to_tsquery($2)
            GROUP BY u.id, c.id
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

    async fn card_update<'a>(
        &self,
        card: CardUpdate<'a>,
        user_id: Uuid,
    ) -> RepoResult<Option<models::Card>> {
        Ok(sqlx::query_as!(
            Card,
            // language=PostgreSQL
            r#"
            UPDATE cards SET
                title = coalesce($1, title),
                contents = coalesce($2, contents),
                tags = coalesce($3, tags)
            WHERE id = $4 AND author_id = $5
            RETURNING id, author_id, title, created_at, updated_at, contents, tags
            "#,
            card.title,
            &card.contents as _,
            card.tags.as_deref(),
            card.id,
            user_id
        )
        .fetch_optional(&self.pool)
        .await?
        .map(Into::into))
    }

    async fn card_delete(&self, card_id: Uuid, user_id: Uuid) -> RepoResult<Option<Uuid>> {
        Ok(sqlx::query_scalar!(
            // language=PostgreSQL
            r#"
            DELETE
            FROM cards
            WHERE (id, author_id) = ($1, $2)
            RETURNING id
            "#,
            card_id,
            user_id
        )
        .fetch_optional(&self.pool)
        .await?)
    }

    async fn card_find_by_id(
        &self,
        card_id: Uuid,
    ) -> RepoResult<Option<(models::Card, models::User)>> {
        Ok(sqlx::query_as!(
            UserCard,
            // language=PostgreSQL
            r#"
            SELECT 
                   (u.id, u.accesso_id, u.first_name, u.last_name, u.username, u.bio, u.avatar, u.work, (array_agg((s.id, s.user_id, s.name, s.link)) FILTER ( WHERE (s.id IS NOT NULL) ))) as "user!: User",
                   (c.id, c.author_id, c.title, c.created_at, c.updated_at, c.contents, c.tags) as "card!: Card" 
            FROM cards as c
                LEFT JOIN users u ON u.id = c.author_id
                LEFT OUTER JOIN socials s ON s.user_id = u.id
            WHERE c.id = $1
            GROUP BY u.id, c.id
            "#,
            card_id
        )
        .fetch_optional(&self.pool)
        .await?
        .map(|uc| (uc.card.into(), uc.user.into())))
    }

    async fn cards_list(&self, user_id: Uuid) -> RepoResult<Vec<(models::Card, models::User)>> {
        Ok(sqlx::query_as!(
            UserCard,
            // language=PostgreSQL
            r#"
            SELECT
                (u.id, u.accesso_id, u.first_name, u.last_name, u.username, u.bio, u.avatar, u.work, (array_agg((s.id, s.user_id, s.name, s.link)) FILTER ( WHERE (s.id IS NOT NULL) ))) as "user!: User",
                (c.id, c.author_id, c.title, c.created_at, c.updated_at, c.contents, c.tags) as "card!: Card"
            FROM cards as c
                LEFT JOIN users u ON u.id = $1
                LEFT OUTER JOIN socials s ON u.id = $1
            WHERE author_id = $1
            GROUP BY u.id, c.id
            "#,
            user_id
        )
        .fetch_all(&self.pool)
        .await?
        .into_iter()
        .map(|user_card| (user_card.card.into(), user_card.user.into()))
        .collect())
    }

    async fn cards_favorites_of_user(
        &self,
        user_id: Uuid,
    ) -> RepoResult<Vec<(models::Card, models::User)>> {
        Ok(sqlx::query_as!(
            UserCard,
            // language=PostgreSQL
            r#"
            SELECT 
                (u.id, u.accesso_id, u.first_name, u.last_name, u.username, u.bio, u.avatar, u.work, (array_agg((s.id, s.user_id, s.name, s.link)) FILTER ( WHERE (s.id IS NOT NULL) ))) as "user!: User",
                (c.id, c.author_id, c.title, c.created_at, c.updated_at, c.contents, c.tags) as "card!: Card"
            FROM boxes AS b
                     LEFT JOIN boxes_cards bc
                     ON b.id = bc.box_id
                     LEFT JOIN cards c
                     ON c.id = bc.card_id
                     LEFT JOIN users u ON u.id = $1
                     LEFT OUTER JOIN socials s ON u.id = $1
            WHERE b.user_id = $1
              AND b."default" = TRUE
              AND EXISTS(SELECT 1 FROM cards WHERE id = bc.card_id)
            GROUP BY u.id, c.id
            "#,
            user_id
        )
        .fetch_all(&self.pool)
        .await?
        .into_iter()
        .map(|user_card| (user_card.card.into(), user_card.user.into()))
        .collect())
    }

    async fn cards_latest(&self) -> RepoResult<Vec<(models::Card, models::User)>> {
        // TODO: replace with actual logic later
        Ok(sqlx::query_as!(
            UserCard,
            // language=PostgreSQL
            r#"
            SELECT
                (u.id, u.accesso_id, u.first_name, u.last_name, u.username, u.bio, u.avatar, u.work, (array_agg((s.id, s.user_id, s.name, s.link)) FILTER ( WHERE (s.id IS NOT NULL) ))) as "user!: User",
                (c.id, c.author_id, c.title, c.created_at, c.updated_at, c.contents, c.tags) as "card!: Card"
            FROM cards as c
                 LEFT JOIN users u ON u.id = c.author_id
                 LEFT OUTER JOIN socials s ON s.user_id = u.id
            GROUP BY u.id, c.id
            ORDER BY c.created_at DESC
            LIMIT 6
            "#
        )
        .fetch_all(&self.pool)
        .await?
        .into_iter()
        .map(|user_card| (user_card.card.into(), user_card.user.into()))
        .collect())
    }

    async fn cards_top(&self) -> RepoResult<Vec<(models::Card, models::User)>> {
        // TODO: implement actual logic
        self.cards_latest().await
    }
}
