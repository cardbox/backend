#[macro_use]
pub extern crate diesel;

mod implementation;
pub mod schema;

pub use implementation::Database;
