//! Configuration system for aegnt-27

use serde::{Deserialize, Serialize};

/// Main configuration structure for aegnt-27
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AegntConfig {
    /// Enable mouse humanization
    pub mouse_enabled: bool,
    /// Enable typing humanization  
    pub typing_enabled: bool,
    /// Enable audio humanization
    pub audio_enabled: bool,
    /// Enable visual humanization
    pub visual_enabled: bool,
    /// Enable detection validation
    pub detection_enabled: bool,
    /// Target authenticity score (0.0 - 1.0)
    pub authenticity_target: f64,
}

impl Default for AegntConfig {
    fn default() -> Self {
        Self {
            mouse_enabled: true,
            typing_enabled: true,
            audio_enabled: true,
            visual_enabled: true,
            detection_enabled: true,
            authenticity_target: 0.95,
        }
    }
}