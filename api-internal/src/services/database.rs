use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::r2d2::ConnectionManager;

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
