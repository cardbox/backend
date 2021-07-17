use crate::entities::SessionToken;
use crate::Database;
use cardbox_core::contracts::{RepoResult, SessionTokenRepo};
use cardbox_core::models;
use sqlx::types::Uuid;

#[async_trait]
impl SessionTokenRepo for Database {
    async fn delete_token_by_user(&self, user_id: Uuid) -> RepoResult<u64> {
        Ok(sqlx::query!(
            // language=PostgreSQL
            r#"
            DELETE
            FROM session_tokens
            WHERE user_id = $1
            "#,
            user_id
        )
        .execute(&self.pool)
        .await?
        .rows_affected())
    }

    async fn delete_token(&self, token: String) -> RepoResult<u64> {
        Ok(sqlx::query!(
            // language=PostgreSQL
            r#"
            DELETE 
            FROM session_tokens 
            WHERE token = $1
            "#,
            token
        )
        .execute(&self.pool)
        .await?
        .rows_affected())
    }

    async fn find_by_token(&self, token: String) -> RepoResult<Option<models::SessionToken>> {
        Ok(sqlx::query_as!(
            SessionToken,
            // language=PostgreSQL
            r#"
            SELECT user_id, token, expires_at
            FROM session_tokens
            WHERE token = $1
            "#,
            token
        )
        .fetch_optional(&self.pool)
        .await?
        .map(Into::into))
    }

    async fn find_token_by_user(&self, user_id: Uuid) -> RepoResult<Option<models::SessionToken>> {
        Ok(sqlx::query_as!(
            SessionToken,
            // language=PostgreSQL
            r#"
            SELECT user_id, token, expires_at
            FROM session_tokens
            WHERE user_id = $1
            "#,
            user_id
        )
        .fetch_optional(&self.pool)
        .await?
        .map(Into::into))
    }

    async fn create_token(&self, token: models::SessionToken) -> RepoResult<models::SessionToken> {
        Ok(sqlx::query_as!(
            SessionToken,
            // language=PostgreSQL
            r#"
            INSERT INTO session_tokens
                (user_id, token, expires_at)
            VALUES ($1, $2, $3)
            RETURNING user_id, token, expires_at
            "#,
            token.user_id,
            token.token,
            token.expires_at
        )
        .fetch_one(&self.pool)
        .await?
        .into())
    }
}
