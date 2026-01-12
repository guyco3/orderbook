use thiserror::Error;
use std::env::VarError;

#[derive(Error, Debug)]
pub enum OrderbookError {
    #[error("Authentication failed: {0}")]
    AuthError(String),

    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Environment variable error: {0}")]
    EnvVar(#[from] VarError),
    
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("WebSocket error: {0}")]
    WebSocket(#[from] tokio_tungstenite::tungstenite::Error),

    #[error("Internal worker error: {0}")]
    Internal(String),

    #[error("Task join error: {0}")]
    JoinError(#[from] tokio::task::JoinError),

    #[error("Database error: {0}")]
    Database(#[from] duckdb::Error),

    #[error("Analysis error: {0}")]
    Analysis(String),
}

// A convenient alias for our library
pub type Result<T> = std::result::Result<T, OrderbookError>;