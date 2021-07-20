#![deny(warnings)]
#![forbid(unsafe_code)]

#[macro_use]
extern crate async_trait;

#[macro_use]
extern crate validator_derive;

pub mod app;
pub mod contracts;
pub mod models;
pub mod services;
