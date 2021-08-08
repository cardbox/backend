use crate::entities::{Socials, User};
use crate::mappers::sqlx_error_to_user_create_error;
use crate::Database;
use cardbox_core::contracts::repo::{RepoResult, UserCreateError, UserRepo};
use cardbox_core::models;
use sqlx::types::Uuid;

#[async_trait]
impl UserRepo for Database {
    async fn user_find_by_id(&self, user_id: Uuid) -> RepoResult<Option<models::User>> {
        Ok(sqlx::query_as!(
            User,
            // language=PostgreSQL
            r#"
            SELECT 
                   u.id, u.accesso_id, u.first_name, u.last_name, u.username, u.bio, u.avatar, u.work,
                   (array_agg((s.id, s.user_id, s.name, s.link)) FILTER ( WHERE s.id IS NOT NULL )) AS "socials: Socials"
            FROM users AS u
                     LEFT OUTER JOIN socials s
                     ON u.id = s.user_id
            WHERE u.id = $1
            GROUP BY u.id
            "#,
            user_id
        )
        .fetch_optional(&self.pool)
        .await?
        .map(Into::into))
    }

    async fn user_find_by_accesso(&self, accesso_id: Uuid) -> RepoResult<Option<models::User>> {
        Ok(sqlx::query_as!(
            User,
            // language=PostgreSQL
            r#"
            SELECT 
                   u.id, u.accesso_id, u.first_name, u.last_name, u.username, u.bio, u.avatar, u.work,
                   (array_agg((s.id, s.user_id, s.name, s.link)) FILTER ( WHERE s.id IS NOT NULL )) AS "socials: Socials"
            FROM users AS u
                     LEFT OUTER JOIN socials s
                     ON u.id = s.user_id
            WHERE u.accesso_id = $1
            GROUP BY u.id
            "#,
            accesso_id
        )
        .fetch_optional(&self.pool)
        .await?
        .map(Into::into))
    }

    async fn user_find_by_username(&self, username: &str) -> RepoResult<Option<models::User>> {
        Ok(sqlx::query_as!(
            User,
            // language=PostgreSQL
            r#"
            SELECT 
                   u.id, u.accesso_id, u.first_name, u.last_name, u.username, u.bio, u.avatar, u.work,
                   (array_agg((s.id, s.user_id, s.name, s.link)) FILTER ( WHERE s.id IS NOT NULL )) AS "socials: Socials"
            FROM users AS u
                     LEFT OUTER JOIN socials s
                     ON u.id = s.user_id
            WHERE u.username = $1
            GROUP BY u.id
            "#,
            username
        )
            .fetch_optional(&self.pool)
            .await?
            .map(Into::into))
    }

    async fn user_update(&self, user: models::User) -> RepoResult<models::User> {
        let updated = sqlx::query_as!(
            User,
            // language=PostgreSQL
            r#"
            UPDATE users
            SET (accesso_id, first_name, last_name, username, bio, avatar, work) = ($1, $2, $3, $4, $5, $6, $7)
            WHERE id = $8
            RETURNING id, accesso_id, first_name, last_name, username, bio, avatar, work, NULL as "socials: Socials"
            "#,
            user.accesso_id,
            user.first_name,
            user.last_name,
            user.username,
            user.bio,
            user.avatar,
            user.work,
            user.id
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(updated.into())
    }

    async fn user_create(&self, user: models::UserCreate) -> Result<models::User, UserCreateError> {
        let user = sqlx::query_as!(
            User,
            // language=PostgreSQL
            r#"
            INSERT INTO users (accesso_id, first_name, last_name)
            VALUES ($1, $2, $3)
            RETURNING id, accesso_id, first_name, last_name, username, bio, avatar, work, NULL as "socials: Socials"
            "#,
            user.accesso_id,
            user.first_name,
            user.last_name
        )
        .fetch_one(&self.pool)
        .await
        .map_err(sqlx_error_to_user_create_error)?;

        Ok(user.into())
    }
}
