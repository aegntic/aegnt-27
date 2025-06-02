//! Mouse humanization module for aegnt-27

use crate::error::{AegntError, Result};
use serde::{Deserialize, Serialize};

/// Represents a 2D point for mouse operations
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Point {
    /// X coordinate
    pub x: f64,
    /// Y coordinate  
    pub y: f64,
}

impl Point {
    /// Create a new point
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }
}

/// Mouse path for humanization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MousePath {
    /// Starting point
    pub start: Point,
    /// Ending point
    pub end: Point,
    /// Duration in milliseconds
    pub duration_ms: u64,
}

impl MousePath {
    /// Create a linear mouse path
    pub fn linear(start: Point, end: Point) -> Self {
        let distance = ((end.x - start.x).powi(2) + (end.y - start.y).powi(2)).sqrt();
        let duration_ms = (distance * 2.0) as u64; // Simple duration calculation
        
        Self {
            start,
            end,
            duration_ms,
        }
    }
}

/// Mouse humanization functionality
pub struct MouseHumanizer {
    authenticity_target: f64,
}

impl MouseHumanizer {
    /// Create a new mouse humanizer
    pub fn new() -> Self {
        Self {
            authenticity_target: 0.95,
        }
    }
    
    /// Humanize a mouse path
    pub async fn humanize_path(&self, path: MousePath) -> Result<MousePath> {
        // Basic implementation - actual humanization would add micro-movements,
        // natural curves, timing variations, etc.
        log::info!("Humanizing mouse path with {:.1}% authenticity target", 
                   self.authenticity_target * 100.0);
        
        // For now, just return the path with slight timing adjustment
        let mut humanized = path;
        humanized.duration_ms = (humanized.duration_ms as f64 * 1.1) as u64; // Add slight delay
        
        Ok(humanized)
    }
}

impl Default for MouseHumanizer {
    fn default() -> Self {
        Self::new()
    }
}