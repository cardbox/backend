use crate::Error;
use async_trait::async_trait;
use cardbox_core::{models, repo};
use repo::{RepoResult, UnexpectedError};
use sqlx::pool::PoolConnection;
use sqlx::Postgres;
use uuid::Uuid;

type DbPool = sqlx::PgPool;

#[derive(Clone)]
pub struct Database {
    pub(crate) pool: DbPool,
}

impl Database {
    pub async fn new(connection_url: &str) -> Result<Self, Error> {
        let pool = DbPool::connect(connection_url).await?;

        Ok(Self { pool })
    }

    /// Waits for at most the configured connection timeout before returning an
    /// error.
    pub async fn conn(&self) -> Result<PoolConnection<Postgres>, Error> {
        self.pool.acquire().await.map_err(Into::into)
    }
}

mod impl_user {
    use super::*;
    use repo::{UserCreate, UserCreateError, UserRepo};
    use sqlx::postgres::PgDatabaseError;

    #[async_trait]
    impl UserRepo for Database {
        async fn find_by_id(&self, user_id: Uuid) -> RepoResult<Option<models::User>> {
            sqlx::query_as!(
                map::User,
                "SELECT id, accesso_id, first_name, last_name FROM users WHERE id = $1",
                user_id
            )
            .fetch_optional(&self.pool)
            .await
            .map_err(sqlx_to_unexpected)
            .map(|opt| opt.map(Into::into))
        }

        async fn find_by_accesso(&self, accesso_id: Uuid) -> RepoResult<Option<models::User>> {
            sqlx::query_as!(
                map::User,
                "SELECT id, accesso_id, first_name, last_name FROM users WHERE accesso_id = $1",
                accesso_id
            )
            .fetch_optional(&self.pool)
            .await
            .map_err(sqlx_to_unexpected)
            .map(|opt| opt.map(Into::into))
        }

        async fn save(&mut self, user: models::User) -> RepoResult<models::User> {
            sqlx::query!(
                r#"
                    UPDATE users SET
                    id = $1,
                    accesso_id = $2,
                    first_name = $3,
                    last_name = $4
                    WHERE id = $1
                "#,
                &user.id,
                &user.accesso_id,
                &user.first_name,
                &user.last_name
            )
            .execute(&self.pool)
            .await
            .map_err(sqlx_to_unexpected)?;
            Ok(user)
        }

        async fn create(&mut self, user: UserCreate) -> Result<models::User, UserCreateError> {
            sqlx::query_as!(
                map::User,
                r#"
                    INSERT INTO users (accesso_id, first_name, last_name)
                    VALUES ($1, $2, $3)
                    RETURNING id, accesso_id, first_name, last_name
                "#,
                &user.accesso_id,
                &user.first_name,
                &user.last_name
            )
            .fetch_one(&self.pool)
            .await
            .map(Into::into)
            .map_err(sqlx_to_user_create_error)
        }
    }

    fn sqlx_to_user_create_error(error: sqlx::Error) -> UserCreateError {
        use sqlx::error::Error;
        match error {
            Error::Database(e) => {
                let pg_error = e.downcast_ref::<PgDatabaseError>();
                match pg_error.message() {
                    "unique_violation" => UserCreateError::UserAlreadyExists,
                    _ => UserCreateError::UnexpectedFailure,
                }
            }
            failure => {
                log::error!(target: "services/database", "Unable to create user {:?}", failure);
                UserCreateError::UnexpectedFailure
            }
        }
    }

    mod map {
        use cardbox_core::{models, repo};

        #[derive(sqlx::FromRow, Debug)]
        pub struct User {
            pub id: uuid::Uuid,
            pub accesso_id: uuid::Uuid,
            pub first_name: String,
            pub last_name: String,
        }

        impl From<User> for models::User {
            fn from(user: User) -> Self {
                Self {
                    id: user.id,
                    accesso_id: user.accesso_id,
                    first_name: user.first_name,
                    last_name: user.last_name,
                }
            }
        }

        impl From<models::User> for User {
            fn from(user: models::User) -> Self {
                Self {
                    id: user.id(),
                    accesso_id: user.accesso_id(),
                    first_name: user.first_name(),
                    last_name: user.last_name(),
                }
            }
        }

        #[derive(sqlx::FromRow)]
        pub struct UserNew {
            pub accesso_id: uuid::Uuid,
            pub first_name: String,
            pub last_name: String,
        }

        impl From<repo::UserCreate> for UserNew {
            fn from(create: repo::UserCreate) -> Self {
                Self {
                    accesso_id: create.accesso_id,
                    first_name: create.first_name,
                    last_name: create.last_name,
                }
            }
        }
    }
}

mod impl_access_token {
    use super::*;
    use repo::SessionTokenRepo;

    #[async_trait]
    impl SessionTokenRepo for Database {
        async fn delete_by_user(&mut self, user_id: Uuid) -> RepoResult<u16> {
            sqlx::query!("DELETE FROM session_tokens WHERE user_id = $1", user_id)
                .execute(&self.pool)
                .await
                .map(|query_result| query_result.rows_affected() as u16)
                .map_err(sqlx_to_unexpected)
        }

        async fn delete(&mut self, token: String) -> RepoResult<u16> {
            sqlx::query!("DELETE FROM session_tokens WHERE token = $1", token)
                .execute(&self.pool)
                .await
                .map(|query_result| query_result.rows_affected() as u16)
                .map_err(sqlx_to_unexpected)
        }

        async fn find_by_token(&self, token: String) -> RepoResult<Option<models::SessionToken>> {
            sqlx::query_as!(
                map::SessionToken,
                "SELECT user_id, token, expires_at FROM session_tokens WHERE token = $1",
                token
            )
            .fetch_optional(&self.pool)
            .await
            .map_err(sqlx_to_unexpected)
            .map(|opt| opt.map(Into::into))
        }

        async fn find_by_user(&self, user_id: Uuid) -> RepoResult<Option<models::SessionToken>> {
            sqlx::query_as!(
                map::SessionToken,
                "SELECT user_id, token, expires_at FROM session_tokens WHERE user_id = $1",
                user_id
            )
            .fetch_optional(&self.pool)
            .await
            .map_err(sqlx_to_unexpected)
            .map(|opt| opt.map(Into::into))
        }

        async fn create(
            &mut self,
            token: models::SessionToken,
        ) -> RepoResult<models::SessionToken> {
            sqlx::query!(
                "INSERT INTO session_tokens VALUES ($1, $2, $3)",
                &token.user_id,
                &token.token,
                &token.expires_at
            )
            .execute(&self.pool)
            .await
            .map_err(sqlx_to_unexpected)?;
            Ok(token)
        }
    }

    mod map {
        use cardbox_core::models;

        #[derive(sqlx::FromRow)]
        pub struct SessionToken {
            pub user_id: uuid::Uuid,
            pub token: String,
            pub expires_at: chrono::NaiveDateTime,
        }

        impl From<SessionToken> for models::SessionToken {
            fn from(token: SessionToken) -> Self {
                Self {
                    user_id: token.user_id,
                    token: token.token,
                    expires_at: token.expires_at,
                }
            }
        }

        impl From<models::SessionToken> for SessionToken {
            fn from(token: models::SessionToken) -> Self {
                Self {
                    user_id: token.user_id(),
                    token: token.token(),
                    expires_at: token.expires_at(),
                }
            }
        }
    }
}

fn sqlx_to_unexpected(error: sqlx::Error) -> UnexpectedError {
    log::error!(target: "services/database", "Unexpected error happened {:?}", error);
    UnexpectedError
}
