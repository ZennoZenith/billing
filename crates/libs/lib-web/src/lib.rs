mod config;
mod error;

pub mod ctx;
pub mod extractors;
pub mod handlers;
pub mod log;
pub mod middleware;
pub mod model;
pub mod utils;

//  TODO: uncomment in prod
//  #[cfg(test)] // Commented during early development.
pub mod _dev_utils;

pub use config::web_config;
pub use error::Error;
