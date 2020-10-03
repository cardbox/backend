pub mod app;
pub mod models;
pub mod repo;

#[derive(Clone)]
pub struct App<Database = ()> {
    pub db: Database,
}
