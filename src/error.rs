use std::env::VarError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum OrderbookError {
    #[error("Authentication failed: {0}")]
    AuthError(Box<str>),

    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Environment variable error: {0}")]
    EnvVar(#[from] VarError),

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("WebSocket error: {0}")]
    // We Box this because tungstenite::Error is a very large enum
    WebSocket(#[from] Box<tokio_tungstenite::tungstenite::Error>),

    #[error("Internal worker error: {0}")]
    Internal(Box<str>),

    #[error("Task join error: {0}")]
    JoinError(#[from] tokio::task::JoinError),
}

// A convenient alias for our library
pub type Result<T> = std::result::Result<T, OrderbookError>;
