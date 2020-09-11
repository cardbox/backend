use actix_web::{web, Scope};

mod auth_url;

pub fn scope() -> Scope {
    web::scope("/accesso").route("/auth-url", web::post().to(auth_url::route))
}
