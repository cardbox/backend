#![deny(warnings)]
#![forbid(unsafe_code)]

use config::{Config, ConfigError, Environment, File};
use serde::Deserialize;
use std::env;

#[derive(Debug, Deserialize, Clone)]
pub struct Settings {
    pub debug: bool,
    pub database: Database,
    pub cookies: Cookies,
    pub server: Server,
    pub accesso: Accesso,
    pub use_opentelemetry: bool,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Accesso {
    pub client_id: String,
    pub client_secret: String,
    pub redirect_back_url: String,
    pub url: String,
    pub ssl_validate: bool,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Cookies {
    pub http_only: bool,
    pub secure: bool,
    pub path: String,
    pub name: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Database {
    pub user: String,
    pub password: String,
    pub host: String,
    pub port: i32,
    pub database: String,
    pub pool_size: u32,
    pub ssl_validate: bool,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Server {
    pub port: u16,
    pub use_ssl: bool,
    pub host: String,
    pub workers: Option<u16>,
    pub backlog: Option<u32>,
    pub keep_alive: Option<u16>,
    pub client_shutdown: Option<u64>,
}

impl Settings {
    pub fn new<N: AsRef<str>>(api_name: N) -> Result<Self, ConfigError> {
        let mut config = Config::new();
        let mode = env::var("CARDBOX_MODE").unwrap_or_else(|_| "development".to_owned());
        let api = api_name.as_ref();

        // Load environment config
        let env = match mode.as_ref() {
            "development" => "development",
            "production" => "production",
            mode => panic!("invalid CARDBOX_MODE {}", mode),
        };

        config.merge(File::with_name("config/default"))?;

        let files = vec![
            format!("config/default-{}", env), // config/default-production.toml
            format!("config/{}", api),         // config/internal.toml
            format!("config/{}-{}", api, env), // config/internal-production.toml
            // locals
            ".config".to_owned(),               // .config.toml
            format!(".config-{}", env),         // .config-production.toml
            format!(".config-{}", api),         // .config-internal.toml
            format!(".config-{}-{}", api, env), // .config-internal-production.toml
        ];

        for path in files.iter() {
            config.merge(File::with_name(path).required(false))?;
        }

        // Add in settings from the environment (with a prefix of CARDBOX)
        // Eg.. `CARDBOX__DEBUG=true ./target/app` would set the `debug` key
        // Note: we need to use double underscore here, because otherwise variables containing
        //       underscore cant be set from environmnet.
        // https://github.com/mehcode/config-rs/issues/73
        config.merge(Environment::with_prefix("CARDBOX").separator("__"))?;

        config.try_into()
    }
}

impl Server {
    pub fn bind_address(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}

impl Database {
    pub fn connection_url(&self) -> String {
        format!(
            "postgres://{user}:{password}@{host}:{port}/{db}",
            user = self.user,
            password = self.password,
            host = self.host,
            port = self.port,
            db = self.database,
        )
    }
}
