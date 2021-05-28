#![deny(warnings)]
#![forbid(unsafe_code)]

#[macro_use]
pub extern crate diesel;

mod implementation;
pub mod schema;

pub use implementation::Database;
