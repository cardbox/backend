use actix_web::{dev, web, Error, HttpResponse};

#[derive(serde::Serialize)]
struct Health {
    status: String,
}

async fn handle(req: dev::ServiceRequest) -> Result<dev::ServiceResponse, Error> {
    Ok(req.into_response(HttpResponse::Ok().json(Health {
        status: "ok".to_owned(),
    })))
}

pub fn service() -> impl dev::HttpServiceFactory {
    web::service("/health").finish(handle)
}
