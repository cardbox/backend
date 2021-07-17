use crate::entities::User;
use crate::mappers::sqlx_error_to_user_create_error;
use crate::Database;
use cardbox_core::contracts::repo::{RepoResult, UserCreateError, UserRepo};
use cardbox_core::models;
use sqlx::types::Uuid;

#[async_trait]
impl UserRepo for Database {
    async fn find_by_id(&self, user_id: Uuid) -> RepoResult<Option<models::User>> {
        Ok(sqlx::query_as!(
            User,
            // language=PostgreSQL
            r#"
            SELECT users.*
            FROM users
            WHERE users.id = $1
            "#,
            user_id
        )
        .fetch_optional(&self.pool)
        .await?
        .map(Into::into))
    }

    async fn find_by_accesso(&self, accesso_id: Uuid) -> RepoResult<Option<models::User>> {
        Ok(sqlx::query_as!(
            User,
            // language=PostgreSQL
            r#"
            SELECT users.*
            FROM users
            WHERE users.accesso_id = $1
            "#,
            accesso_id
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
            SET (accesso_id, first_name, last_name) = ($1, $2, $3)
            WHERE id = $4
            RETURNING id, accesso_id, first_name, last_name
            "#,
            user.accesso_id,
            user.first_name,
            user.last_name,
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
            RETURNING id, accesso_id, first_name, last_name
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
