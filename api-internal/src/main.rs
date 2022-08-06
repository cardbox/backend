#![deny(warnings)]
#![forbid(unsafe_code)]

use actix_web::middleware;
use actix_web::web;
use cardbox_app::install_logger;
use cardbox_settings::Settings;
use eyre::WrapErr;
use std::sync::Arc;
use tracing_actix_web::TracingLogger;
use url::Url;

mod accesso;
mod generated;
mod routes;

use actix_http::HttpServiceBuilder;
use actix_server::Server;
use actix_service::map_config;
use actix_web::dev::AppConfig;
use actix_web_prometheus::PrometheusMetricsBuilder;
use shrinkwraprs::Shrinkwrap;

#[derive(Debug, Shrinkwrap, Clone)]
#[shrinkwrap(mutable)]
pub struct AccessoAuthorizeUrl(pub Url);

#[derive(Debug, Shrinkwrap, Clone)]
#[shrinkwrap(mutable)]
pub struct AccessoPublicApiUrl(pub Url);

impl AccessoPublicApiUrl {
    pub fn append_path(&self, path: &str) -> Self {
        let mut clone = self.clone();
        let new_path = format!("{}{}", self.0.path(), path);
        clone.0.set_path(&new_path);
        clone
    }

}

pub static APP_NAME: &str = "cardbox-api-internal";

pub fn create_request_client(config: &Settings) -> Result<reqwest::Client, eyre::Report> {
    let mut builder = reqwest::ClientBuilder::new();

    if !config.accesso.ssl_validate {
        tracing::warn!(
            "!!! SSL validation is disabled in config, check if this is what you REALLY want !!!"
        );
        builder = builder.danger_accept_invalid_certs(true);
    }

    builder.build().wrap_err("Could not create http client!")
}

#[actix_rt::main]
async fn main() -> eyre::Result<()> {
    dotenv::dotenv().wrap_err("Failed to initialize dotenv")?;

    let settings = Arc::new(Settings::new("internal").wrap_err("failed to parse settings")?);

    if settings.debug {
        tracing::info!("==> Starting {} in DEBUG mode!", APP_NAME);
        color_eyre::install()?;
    } else {
        tracing::info!("==> Starting {} in PRODUCTION mode!", APP_NAME);
    };

    let _guard = install_logger(APP_NAME.into(), &settings)?;

    let bind_address = settings.server.bind_address();

    let client = create_request_client(&settings)?;
    let use_opentelemetry = settings.use_opentelemetry;

    let settings_clone = settings.clone();
    let client_clone = client.clone();

    let accesso_authorize_url = Arc::new(AccessoAuthorizeUrl(Url::parse(&settings.accesso.authorize_url)?));
    let accesso_public_api_url = Arc::new(AccessoPublicApiUrl(Url::parse(&settings.accesso.public_api_url)?));

    let prometheus = PrometheusMetricsBuilder::new("api_internal")
        .endpoint("/metrics")
        .build()?;

    let app = move || {
        let settings = settings_clone.clone();
        let client = client_clone.clone();
        let accesso_authorize_url = accesso_authorize_url.clone();
        let accesso_public_api_url = accesso_public_api_url.clone();
        actix_web::App::new()
            .configure(move |config| {
                let settings = settings.clone();
                cardbox_app::configure(config, settings);
            })
            //.wrap(middleware::Compress::default())
            .wrap(middleware::NormalizePath::trim())
            .wrap(
                middleware::DefaultHeaders::new()
                    // .header("X-Frame-Options", "deny")
                    .header("X-Content-Type-Options", "nosniff")
                    .header("X-XSS-Protection", "1; mode=block"),
            )
            .wrap(TracingLogger::default())
            .wrap(prometheus.clone())
            .app_data(web::Data::new(client))
            .app_data(web::Data::from(accesso_authorize_url))
            .app_data(web::Data::from(accesso_public_api_url))
            .service(
                generated::api::create()
                    .bind_auth_params(routes::accesso::auth_params::route)
                    .bind_auth_done(routes::accesso::auth_done::route)
                    .bind_cards_create(routes::cards::create::route)
                    .bind_cards_search(routes::cards::search::route)
                    .bind_cards_edit(routes::cards::edit::route)
                    .bind_cards_delete(routes::cards::delete::route)
                    .bind_cards_save(routes::cards::save::route)
                    .bind_cards_unsave(routes::cards::unsave::route)
                    .bind_cards_list(routes::cards::list::route)
                    .bind_cards_get(routes::cards::get::route)
                    .bind_cards_feed(routes::cards::feed::route)
                    .bind_session_get(routes::session::get::route)
                    .bind_session_delete(routes::session::delete::route)
                    .bind_users_get(routes::users::get::route)
                    .bind_users_search(routes::users::search::route),
            )
            .default_service(web::route().to(cardbox_app::not_found))
    };

    let mut server = Server::build();

    if let Some(workers) = settings.server.workers {
        server = server.workers(workers as usize);
    }
    if let Some(backlog) = settings.server.backlog {
        server = server.backlog(backlog);
    }

    if settings.server.use_ssl {
        let tls_cfg = {
            let cert = load_certs(&std::env::var("TLS_CERT_FILE")?)?;
            let key = load_private_key(&std::env::var("TLS_KEY_FILE")?)?;

            rustls::ServerConfig::builder()
                .with_safe_defaults()
                .with_no_client_auth()
                .with_single_cert(cert, key)?
        };
        server
            .bind("http/2-ssl", bind_address, move || {
                let tls_cfg = tls_cfg.clone();

                let mut builder = HttpServiceBuilder::new();

                if let Some(keep_alive) = settings.server.keep_alive {
                    builder =
                        builder.keep_alive(actix_http::KeepAlive::Timeout(keep_alive as usize));
                }

                builder
                    .h2(map_config(app(), |_| AppConfig::default()))
                    .rustls(tls_cfg)
            })?
            .run()
            .await?;
    } else {
        if settings.server.use_h1 {
            server.bind("http/1.1", bind_address, move || {
                let mut builder = HttpServiceBuilder::new();

                if let Some(keep_alive) = settings.server.keep_alive {
                    builder =
                        builder.keep_alive(actix_http::KeepAlive::Timeout(keep_alive as usize));
                }

                builder
                    .h1(map_config(app(), |_| AppConfig::default()))
                    .tcp()
            })?
        } else {
            server.bind("http/2", bind_address, move || {
                let mut builder = HttpServiceBuilder::new();

                if let Some(keep_alive) = settings.server.keep_alive {
                    builder =
                        builder.keep_alive(actix_http::KeepAlive::Timeout(keep_alive as usize));
                }

                builder
                    .h2(map_config(app(), |_| AppConfig::default()))
                    .tcp()
            })?
        }
        .run()
        .await?
    }

    if use_opentelemetry {
        opentelemetry::global::shutdown_tracer_provider();
    }

    Ok(())
}

fn load_certs(filename: &str) -> eyre::Result<Vec<rustls::Certificate>> {
    use rustls_pemfile as pemfile;
    use std::{fs, io};
    let certfile =
        fs::File::open(filename).map_err(|e| eyre::eyre!("failed to open {}: {}", filename, e))?;
    let mut reader = io::BufReader::new(certfile);

    pemfile::certs(&mut reader)
        .map(|certs| certs.into_iter().map(rustls::Certificate).collect())
        .map_err(|_| eyre::eyre!("failed to load certificate"))
}

// Load private key from file.
fn load_private_key(filename: &str) -> eyre::Result<rustls::PrivateKey> {
    use rustls_pemfile as pemfile;
    use std::{fs, io};
    let keyfile =
        fs::File::open(filename).map_err(|e| eyre::eyre!("failed to open {}: {}", filename, e))?;
    let mut reader = io::BufReader::new(keyfile);

    let keys = pemfile::rsa_private_keys(&mut reader)
        .map_err(|_| eyre::eyre!("failed to load private key"))?;
    if keys.len() != 1 {
        return Err(eyre::eyre!("expected a single private key"));
    }
    Ok(rustls::PrivateKey(keys[0].clone()))
}
