#[macro_use]
extern crate validator_derive;

pub mod app;
pub mod models;
pub mod repo;

#[derive(Clone)]
pub struct App<Database = ()> {
    pub db: Database,
}
