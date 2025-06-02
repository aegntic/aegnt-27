//! Error handling for aegnt-27

use std::fmt;
use std::error::Error as StdError;

/// Main error type for aegnt-27
#[derive(Debug, Clone)]
pub enum AegntError {
    /// Configuration errors
    Config(String),
    /// I/O operation errors
    Io(String),
    /// Authentication/validation errors
    Auth(String),
    /// Internal processing errors
    Internal(String),
}

impl fmt::Display for AegntError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AegntError::Config(msg) => write!(f, "Configuration error: {}", msg),
            AegntError::Io(msg) => write!(f, "I/O error: {}", msg),
            AegntError::Auth(msg) => write!(f, "Authentication error: {}", msg),
            AegntError::Internal(msg) => write!(f, "Internal error: {}", msg),
        }
    }
}

impl StdError for AegntError {}

/// Result type alias for aegnt-27 operations
pub type Result<T> = std::result::Result<T, AegntError>;

impl AegntError {
    /// Create a configuration error
    pub fn config(msg: impl Into<String>) -> Self {
        Self::Config(msg.into())
    }
    
    /// Create an I/O error
    pub fn io(msg: impl Into<String>) -> Self {
        Self::Io(msg.into())
    }
    
    /// Create an authentication error
    pub fn auth(msg: impl Into<String>) -> Self {
        Self::Auth(msg.into())
    }
    
    /// Create an internal error
    pub fn internal(msg: impl Into<String>) -> Self {
        Self::Internal(msg.into())
    }
}