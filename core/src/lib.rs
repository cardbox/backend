#![deny(warnings)]
#![forbid(unsafe_code)]

pub mod app;
pub mod generator;
pub mod models;
pub mod repo;

#[derive(Clone)]
pub struct App<Database = (), Generator = ()> {
    pub db: Database,
    pub generator: Generator,
}
