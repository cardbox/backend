use super::routes::{self, AnswerFailure, FailureCode};
use actix_http::Response;
use actix_web::{error, middleware, web, App, HttpResponse, HttpServer};
use cardbox_db::Database;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Clone, Debug)]
pub struct Config {
    pub accesso_client_id: String,
    pub accesso_client_secret: String,
    pub accesso_redirect_back_url: String,
    pub accesso_url: String,
    pub bind_address: String,
    pub database_url: String,
    pub openssl_validate: bool,
}

#[derive(Clone, Debug)]
pub struct ConfigSession {
    pub name: String,
    pub path: String,
    pub secure: bool,
    pub http_only: bool,
}

pub fn create_request_client(config: &Config) -> awc::Client {
    use awc::{Client, Connector};
    use openssl::ssl::{SslConnector, SslMethod, SslVerifyMode};

    let mut builder = SslConnector::builder(SslMethod::tls()).unwrap();
    builder.set_verify(if config.openssl_validate {
        SslVerifyMode::PEER
    } else {
        SslVerifyMode::NONE
    });

    let connector = Connector::new().ssl(builder.build());

    Client::builder().connector(connector).finish()
}

pub async fn create_server(config: Config) -> std::io::Result<()> {
    let database_url = config.database_url.clone();
    let bind_address = config.bind_address.clone();
    let app = cardbox_core::App {
        db: Database::new(database_url).expect("Failed to create database"),
        generator: cardbox_generator::Generator::new(),
    };

    let app_lock: crate::App = Arc::new(Mutex::new(app));
    let app_data = web::Data::new(app_lock);

    let config_session = ConfigSession {
        http_only: config.openssl_validate,
        secure: config.openssl_validate,
        path: "/".to_owned(),
        name: "session-token".to_owned(),
    };

    HttpServer::new(move || {
        App::new()
            .app_data(app_data.clone())
            .data(config.clone())
            .data(config_session.clone())
            .wrap(middleware::Compress::default())
            .wrap(middleware::Logger::default())
            .app_data(web::JsonConfig::default().error_handler(|err, _| {
                let error_message = format!("{}", err);
                error::InternalError::from_response(
                    err,
                    Response::from(HttpResponse::BadRequest().json(AnswerFailure {
                        error: FailureCode::InvalidPayload,
                        message: Some(error_message),
                    })),
                )
                .into()
            }))
            .app_data(web::QueryConfig::default().error_handler(|err, _| {
                let error_message = format!("{}", err);
                error::InternalError::from_response(
                    err,
                    Response::from(HttpResponse::BadRequest().json(AnswerFailure {
                        error: FailureCode::InvalidQueryParams,
                        message: Some(error_message),
                    })),
                )
                .into()
            }))
            .wrap(
                middleware::DefaultHeaders::new()
                    // .header("X-Frame-Options", "deny")
                    .header("X-Content-Type-Options", "nosniff")
                    .header("X-XSS-Protection", "1; mode=block"),
            )
            .service(routes::scope())
            .default_service(web::route().to(routes::not_found::route))
    })
    .bind(bind_address)?
    .run()
    .await
}
