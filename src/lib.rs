//! # aegnt-27: The Human Peak Protocol
//! 
//! A sophisticated Rust library for achieving peak human authenticity through
//! 27 distinct behavioral patterns and advanced neural pattern simulation.

#![deny(missing_docs)]
#![warn(clippy::all)]

use std::error::Error;

// Core modules
pub mod config;
pub mod error;
pub mod utils;
pub mod mouse;
pub mod typing;
pub mod audio;
pub mod visual;
pub mod detection;

// Re-exports
pub use crate::config::AegntConfig;
pub use crate::error::{AegntError, Result};

/// Main aegnt-27 engine
pub struct AegntEngine {
    config: AegntConfig,
}

impl AegntEngine {
    /// Create a new aegnt-27 engine with default configuration
    pub async fn new() -> Result<Self> {
        Ok(Self {
            config: AegntConfig::default(),
        })
    }
    
    /// Create a new aegnt-27 engine with custom configuration
    pub async fn with_config(config: AegntConfig) -> Result<Self> {
        Ok(Self { config })
    }
}

/// Prelude module for convenient imports
pub mod prelude {
    pub use crate::{AegntEngine, AegntConfig, AegntError, Result};
    pub use crate::init;
}

/// Initialize aegnt-27 with default configuration
pub async fn init() -> Result<()> {
    env_logger::try_init().ok();
    log::info!("aegnt-27 v{} initialized", env!("CARGO_PKG_VERSION"));
    Ok(())
}

/// Initialize aegnt-27 with custom configuration
pub async fn init_with_config(config: AegntConfig) -> Result<()> {
    env_logger::try_init().ok();
    log::info!("aegnt-27 v{} initialized with custom config", env!("CARGO_PKG_VERSION"));
    let _ = config; // Use config in actual implementation
    Ok(())
}