use actix_web::{Error, HttpRequest, HttpResponse, Responder};

#[derive(serde::Serialize)]
struct Health {
    status: String,
}

#[actix_web::get("/health")]
pub async fn health_service(_req: HttpRequest) -> Result<impl Responder, Error> {
    Ok(HttpResponse::Ok().json(Health {
        status: "ok".into(),
    }))
}
