// src/ingestor/mod.rs

pub mod auth;
pub mod ingestor;
pub mod logger;

// Internal re-exports so main.rs has a cleaner API
pub use ingestor::run as run_ingestor;
pub use logger::run as run_logger;