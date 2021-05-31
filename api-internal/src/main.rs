#![deny(warnings)]
#![forbid(unsafe_code)]

use std::sync::Arc;
use tokio::sync::Mutex;

mod accesso;
mod routes;
mod server;

/// Useful to extract app data at handler
/// ```rust
/// async fn handler(app: web::Data<crate::App>) {}
/// ```
pub type App = Arc<Mutex<cardbox_core::App<cardbox_db::Database, cardbox_generator::Generator>>>;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    pretty_env_logger::init();

    let is_dev = read_var_bool("DEV");

    let openssl_validate = !read_var_bool("OPENSSL_NO_VALIDATE");

    let listen_port = read_var("LISTEN_PORT");
    let listen_host = read_var("LISTEN_HOST");
    let database_url = read_var("DATABASE_URL");

    let accesso_url = read_var("ACCESSO_URL");
    let accesso_client_id = read_var("ACCESSO_CLIENT_ID");
    let accesso_redirect_back_url = read_var("ACCESSO_REDIRECT_BACK_URL");
    let accesso_client_secret = read_var("ACCESSO_CLIENT_SECRET");

    let bind_address = format!("{host}:{port}", host = listen_host, port = listen_port);

    if is_dev {
        log::info!("api-internal run in DEVELOPMENT MODE");
    } else {
        log::info!("==> Be careful! api-internal in PRODUCTION MODE");
    }

    server::create_server(server::Config {
        accesso_client_id,
        accesso_client_secret,
        accesso_redirect_back_url,
        accesso_url,
        bind_address,
        database_url,
        openssl_validate
    })
    .await
}

#[inline]
fn read_var<T: AsRef<str>>(name: T) -> String {
    std::env::var(name.as_ref()).unwrap_or_else(|_| panic!("{}", name.as_ref()))
}

#[inline]
fn read_var_bool<T: AsRef<str>>(name: T) -> bool {
    std::env::var(name.as_ref())
        .map(|dev| dev != "false")
        .unwrap_or(false)
}
