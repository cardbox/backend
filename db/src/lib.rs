#![deny(warnings)]
#![forbid(unsafe_code)]

#[macro_use]
extern crate async_trait;

mod entities;
mod mappers;
mod repos;
mod sql_state;

use cardbox_settings::Settings;
use sqlx::postgres::{PgConnectOptions, PgPoolOptions, PgSslMode};

type DbPool = sqlx::PgPool;

pub struct Database {
    pub(crate) pool: DbPool,
}

impl Database {
    pub fn new(settings: &Settings) -> Self {
        let mut connect_options = settings
            .database
            .connection_url()
            .parse::<PgConnectOptions>()
            .expect("Bad connection url!");

        if settings.database.openssl_validate {
            connect_options = connect_options.ssl_mode(PgSslMode::VerifyFull);
        };

        let pool = PgPoolOptions::new()
            .max_connections(settings.database.pool_size)
            .connect_lazy_with(connect_options);

        Self { pool }
    }
}

impl Clone for Database {
    fn clone(&self) -> Database {
        Database {
            pool: self.pool.clone(),
        }
    }
}
