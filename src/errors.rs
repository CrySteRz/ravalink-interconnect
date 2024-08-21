use serde_derive::{Deserialize, Serialize};
use std::fmt;

#[derive(Deserialize, Debug, Serialize, Clone)]
pub enum BotError {
    InvalidCommand(String),
    ConnectionFailed(String),
    JobFailed(String),
    JobExpired(String),
    ResourceNotFound(String),
    InternalServerError(String),
}

impl fmt::Display for BotError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BotError::InvalidCommand(err) => write!(f, "Invalid command: {}", err),
            BotError::ConnectionFailed(err) => write!(f, "Connection failed: {}", err),
            BotError::JobFailed(err) => write!(f, "Job failed: {}", err),
            BotError::JobExpired(err) => write!(f, "Job expired: {}", err),
            BotError::ResourceNotFound(err) => write!(f, "Resource not found: {}", err),
            BotError::InternalServerError(err) => write!(f, "Internal server error: {}", err),
        }
    }
}

impl std::error::Error for BotError {}