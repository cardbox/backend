#[macro_use]
extern crate diesel;

mod routes;
mod server;
mod services;

/// Useful to extract app data at handler
/// ```rust
/// async fn handler(app: web::Data<crate::App>) {}
/// ```
pub type App = std::sync::RwLock<cardbox_logic::App<services::Database>>;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    // env_logger::init();
    pretty_env_logger::init();

    let is_dev = std::env::var("DEV")
        .map(|dev| dev != "false")
        .unwrap_or(false);

    let listen_port = read_var("LISTEN_PORT");
    let listen_host = read_var("LISTEN_HOST");
    let database_url = read_var("DATABASE_URL");

    let accesso_url = read_var("ACCESSO_URL");
    let accesso_client_id = read_var("ACCESSO_CLIENT_ID");
    let accesso_redirect_back_url = read_var("ACCESSO_REDIRECT_BACK_URL");

    let bind_address = format!("{host}:{port}", host = listen_host, port = listen_port);

    if is_dev {
        log::info!("api-internal run in DEVELOPMENT MODE");
    } else {
        println!("==> Be careful! api-internal in PRODUCTION MODE");
    }

    server::create_server(server::Config {
        bind_address,
        database_url,
        accesso_url,
        accesso_client_id,
        accesso_redirect_back_url,
    })
    .await
}

#[inline]
fn read_var<T: AsRef<str>>(name: T) -> String {
    std::env::var(name.as_ref()).expect(name.as_ref())
}
