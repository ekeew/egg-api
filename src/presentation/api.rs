pub mod config;
pub mod endpoints;
pub mod errors;
pub mod main;
pub mod setup;

pub use main::api;
pub(crate) use errors::AppError;
