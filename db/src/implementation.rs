use crate::schema::*;
use async_trait::async_trait;
use cardbox_core::{
    models,
    repo::{RepoResult, UnexpectedError, UserCreate, UserCreateError, UserRepo},
};
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::r2d2::ConnectionManager;
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

        impl Into<models::User> for User {
            fn into(self) -> models::User {
                models::User {
                    id: self.id,
                    accesso_id: self.accesso_id,
                    first_name: self.first_name,
                    last_name: self.last_name,
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

fn diesel_to_unexpected(error: diesel::result::Error) -> UnexpectedError {
    log::error!(target: "services/database", "Unexpected error happened {:?}", error);
    UnexpectedError
}
