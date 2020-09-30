use async_trait::async_trait;
use cardbox_core::{
    models,
    repo::{RepoResult, UnexpectedError, UserCreate, UserCreateError, UserRepo},
};
use cardbox_db::schema::*;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::r2d2::ConnectionManager;
use std::sync::RwLock;
use uuid::Uuid;

type Connection = r2d2::PooledConnection<ConnectionManager<PgConnection>>;
type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[derive(Clone)]
pub struct Database(DbPool);

impl Database {
    pub fn new(connection_url: String) -> Result<Self, r2d2::Error> {
        let manager = ConnectionManager::<PgConnection>::new(connection_url);
        let pool = r2d2::Pool::builder().build(manager)?;

        Ok(Self(pool))
    }

    /// Waits for at most the configured connection timeout before returning an
    /// error.
    pub fn conn(&self) -> Connection {
        self.0.get().expect("Database connection timeout")
    }
}

#[async_trait]
impl UserRepo for Database {
    async fn find_by_id(&self, user_id: Uuid) -> RepoResult<Option<models::User>> {
        unimplemented!()
    }
    async fn find_by_accesso(&self, accesso_id: Uuid) -> RepoResult<Option<models::User>> {
        unimplemented!()
    }
    async fn save(&mut self, user: models::User) -> RepoResult<models::User> {
        unimplemented!()
    }
    async fn create(&mut self, user: UserCreate) -> Result<models::User, UserCreateError> {
        unimplemented!()
    }
}
