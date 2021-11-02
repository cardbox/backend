use crate::health_service;
use actix_web::web::ServiceConfig;
use actix_web::{http::StatusCode, web, HttpRequest, Responder};
use cardbox_settings::Settings;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tracing::subscriber::set_global_default;
use tracing_appender::non_blocking::WorkerGuard;
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_log::LogTracer;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::{EnvFilter, Registry};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[allow(clippy::enum_variant_names)]
pub enum FailureCode {
    InvalidPayload,
    InvalidRoute,
    InvalidQueryParams,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AnswerFailure {
    pub error: FailureCode,
    pub message: Option<String>,
}

pub async fn not_found(_req: HttpRequest) -> impl Responder {
    web::Json(AnswerFailure {
        error: FailureCode::InvalidRoute,
        message: None,
    })
    .with_status(StatusCode::NOT_FOUND)
}

pub fn install_logger(app_name: String, settings: &Settings) -> Result<WorkerGuard, eyre::Report> {
    opentelemetry::global::set_text_map_propagator(
        opentelemetry_zipkin::Propagator::with_encoding(
            opentelemetry_zipkin::B3Encoding::SingleAndMultiHeader,
        ),
    );

    let env_filter = EnvFilter::try_from_default_env()?;
    LogTracer::init()?;

    let (writer, guard) = tracing_appender::non_blocking(std::io::stdout());

    let bunyan_layer = BunyanFormattingLayer::new(app_name.clone(), move || writer.clone());

    println!("{:?}", settings);
    if settings.use_opentelemetry {
        let tracer = opentelemetry_jaeger::new_pipeline()
            .with_collector_endpoint(std::env::var("OPENTELEMETRY_ENDPOINT_URL")?)
            .with_service_name(app_name)
            .install_batch(opentelemetry::runtime::TokioCurrentThread)?;

        let telemetry = tracing_opentelemetry::layer().with_tracer(tracer);

        set_global_default(
            Registry::default()
                .with(telemetry)
                .with(JsonStorageLayer)
                .with(bunyan_layer)
                .with(env_filter),
        )?;
    } else {
        set_global_default(
            Registry::default()
                .with(JsonStorageLayer)
                .with(bunyan_layer)
                .with(env_filter),
        )?;
    }

    Ok(guard)
}

pub fn configure(config: &mut ServiceConfig, settings: Arc<Settings>) {
    use crate::Service;
    use actix_web::web::Data;
    use actix_web::HttpResponse;
    use cardbox_core::contracts::{Generator, Repository};
    use cardbox_core::services;

    let db: Arc<dyn Repository> = Arc::new(cardbox_db::Database::new(&settings));

    let generator: Arc<dyn Generator> = Arc::new(services::Generator);

    let app = crate::App::builder()
        .with_service(Service::from(db))
        .with_service(Service::from(generator))
        .build();

    let session_cookie_config = crate::SessionCookieConfig {
        http_only: settings.cookies.http_only,
        secure: settings.cookies.secure,
        path: settings.cookies.path.clone(),
        name: settings.cookies.name.clone(),
    };

    config
        .app_data(Data::new(app))
        .app_data(Data::new(session_cookie_config))
        .app_data(Data::from(settings))
        .app_data(web::JsonConfig::default().error_handler(|err, _| {
            let error_message = format!("{}", err);
            actix_web::error::InternalError::from_response(
                err,
                HttpResponse::BadRequest().json(AnswerFailure {
                    error: FailureCode::InvalidPayload,
                    message: Some(error_message),
                }),
            )
            .into()
        }))
        .app_data(web::QueryConfig::default().error_handler(|err, _| {
            let error_message = format!("{}", err);
            actix_web::error::InternalError::from_response(
                err,
                HttpResponse::BadRequest().json(AnswerFailure {
                    error: FailureCode::InvalidQueryParams,
                    message: Some(error_message),
                }),
            )
            .into()
        }))
        .service(health_service);
}
