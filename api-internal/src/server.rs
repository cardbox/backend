use actix_web::{
    error, http::StatusCode, middleware, web, App, HttpRequest, HttpResponse, HttpServer, Responder,
};
use serde::{Deserialize, Serialize};

pub struct Config {
    pub bind_address: String,
    pub database_url: String,
    pub accesso_url: String,
}

pub async fn create_server(config: Config) -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Compress::default())
            .wrap(middleware::Logger::default())
            .app_data(web::JsonConfig::default().error_handler(|err, _| {
                let error_message = format!("{}", err);
                error::InternalError::from_response(
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
                error::InternalError::from_response(
                    err,
                    HttpResponse::BadRequest().json(AnswerFailure {
                        error: FailureCode::InvalidQueryParams,
                        message: Some(error_message),
                    }),
                )
                .into()
            }))
            .wrap(
                middleware::DefaultHeaders::new()
                    // .header("X-Frame-Options", "deny")
                    .header("X-Content-Type-Options", "nosniff")
                    .header("X-XSS-Protection", "1; mode=block"),
            )
            .default_service(web::route().to(not_found))
    })
    .bind(config.bind_address)?
    .run()
    .await
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
enum FailureCode {
    InvalidPayload,
    InvalidRoute,
    InvalidQueryParams,
}

#[derive(Debug, Serialize, Deserialize)]
struct AnswerFailure {
    pub error: FailureCode,
    pub message: Option<String>,
}

async fn not_found(_req: HttpRequest) -> impl Responder {
    web::Json(AnswerFailure {
        error: FailureCode::InvalidRoute,
        message: None,
    })
    .with_status(StatusCode::NOT_FOUND)
}
