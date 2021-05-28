use crate::schema::*;
use async_trait::async_trait;
use cardbox_core::{models, repo};
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::r2d2::ConnectionManager;
use repo::{RepoResult, UnexpectedError};
use uuid::Uuid;

type Connection = r2d2::PooledConnection<ConnectionManager<PgConnection>>;
type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[derive(Clone)]
pub struct Database {
    pub(crate) pool: DbPool,
}

impl Database {
    pub fn new(connection_url: String) -> Result<Self, r2d2::Error> {
        let manager = ConnectionManager::<PgConnection>::new(connection_url);
        let pool = r2d2::Pool::builder().build(manager)?;

        Ok(Self { pool })
    }

    /// Waits for at most the configured connection timeout before returning an
    /// error.
    pub fn conn(&self) -> Connection {
        self.pool.get().expect("Database connection timeout")
    }
}

mod impl_user {
    use super::*;
    use repo::{UserCreate, UserCreateError, UserRepo};

    #[async_trait]
    impl UserRepo for Database {
        async fn find_by_id(&self, user_id: Uuid) -> RepoResult<Option<models::User>> {
            let conn = self.conn();

            users::table
                .filter(users::id.eq(user_id))
                .get_result::<map::User>(&conn)
                .map(Into::into)
                .optional()
                .map_err(diesel_to_unexpected)
        }

        async fn find_by_accesso(&self, accesso_id: Uuid) -> RepoResult<Option<models::User>> {
            let conn = self.conn();

            users::table
                .filter(users::accesso_id.eq(accesso_id))
                .get_result::<map::User>(&conn)
                .map(Into::into)
                .optional()
                .map_err(diesel_to_unexpected)
        }

        async fn save(&mut self, user: models::User) -> RepoResult<models::User> {
            let conn = self.conn();

            diesel::update(users::table.find(user.id))
                .set(map::User::from(user))
                .get_result::<map::User>(&conn)
                .map(Into::into)
                .map_err(diesel_to_unexpected)
        }

        async fn create(&mut self, user: UserCreate) -> Result<models::User, UserCreateError> {
            let conn = self.conn();

            diesel::insert_into(users::table)
                .values(map::UserNew::from(user))
                .get_result::<map::User>(&conn)
                .map(Into::into)
                .map_err(diesel_to_user_create_error)
        }
    }

    fn diesel_to_user_create_error(error: diesel::result::Error) -> UserCreateError {
        use diesel::result::{DatabaseErrorKind, Error as DieselError};
        match error {
            DieselError::DatabaseError(DatabaseErrorKind::UniqueViolation, _) => {
                UserCreateError::UserAlreadyExists
            }
            failure => {
                log::error!(target: "services/database", "Unable to create user {:?}", failure);
                UserCreateError::UnexpectedFailure
            }
        }
    }

    mod map {
        use crate::schema::users;
        use cardbox_core::{models, repo};

        #[derive(Identifiable, Insertable, Queryable, AsChangeset)]
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

        #[derive(Insertable)]
        #[table_name = "users"]
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
            let conn = self.conn();

            diesel::delete(session_tokens::table)
                .filter(session_tokens::user_id.eq(user_id))
                .execute(&conn)
                .map(|count| count as u16)
                .map_err(diesel_to_unexpected)
        }

        async fn delete(&mut self, token: String) -> RepoResult<u16> {
            let conn = self.conn();

            diesel::delete(session_tokens::table)
                .filter(session_tokens::token.eq(token))
                .execute(&conn)
                .map(|count| count as u16)
                .map_err(diesel_to_unexpected)
        }

        async fn find_by_token(&self, token: String) -> RepoResult<Option<models::SessionToken>> {
            let conn = self.conn();

            session_tokens::table
                .filter(session_tokens::token.eq(token))
                .get_result::<map::SessionToken>(&conn)
                .map(Into::into)
                .optional()
                .map_err(diesel_to_unexpected)
        }

        async fn find_by_user(&self, user_id: Uuid) -> RepoResult<Option<models::SessionToken>> {
            let conn = self.conn();

            session_tokens::table
                .filter(session_tokens::user_id.eq(user_id))
                .get_result::<map::SessionToken>(&conn)
                .map(Into::into)
                .optional()
                .map_err(diesel_to_unexpected)
        }

        async fn create(
            &mut self,
            token: models::SessionToken,
        ) -> RepoResult<models::SessionToken> {
            let conn = self.conn();

            diesel::insert_into(session_tokens::table)
                .values(map::SessionToken::from(token))
                .get_result::<map::SessionToken>(&conn)
                .map(Into::into)
                .map_err(diesel_to_unexpected)
        }
    }

    mod map {
        use crate::schema::session_tokens;
        use cardbox_core::models;

        #[derive(Identifiable, Insertable, Queryable, AsChangeset)]
        #[primary_key(token)]
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

fn diesel_to_unexpected(error: diesel::result::Error) -> UnexpectedError {
    log::error!(target: "services/database", "Unexpected error happened {:?}", error);
    UnexpectedError
}
