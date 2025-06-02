//! Typing humanization module for aegnt-27

use crate::error::{AegntError, Result};
use serde::{Deserialize, Serialize};

/// Represents a keystroke for typing humanization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Keystroke {
    /// The character or key
    pub key: String,
    /// Timing in milliseconds from start
    pub timing_ms: u64,
}

/// Typing sequence for humanization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TypingSequence {
    /// Text to type
    pub text: String,
    /// Individual keystrokes with timing
    pub keystrokes: Vec<Keystroke>,
    /// Total duration in milliseconds
    pub total_duration_ms: u64,
}

impl TypingSequence {
    /// Create a new typing sequence from text
    pub fn from_text(text: &str) -> Self {
        let mut keystrokes = Vec::new();
        let mut timing = 0;
        
        for ch in text.chars() {
            timing += 100 + (timing % 50); // Basic timing variation
            keystrokes.push(Keystroke {
                key: ch.to_string(),
                timing_ms: timing,
            });
        }
        
        Self {
            text: text.to_string(),
            keystrokes: keystrokes.clone(),
            total_duration_ms: timing,
        }
    }
}

/// Typing humanization functionality
pub struct TypingHumanizer {
    authenticity_target: f64,
}

impl TypingHumanizer {
    /// Create a new typing humanizer
    pub fn new() -> Self {
        Self {
            authenticity_target: 0.95,
        }
    }
    
    /// Humanize a typing sequence
    pub async fn humanize_sequence(&self, sequence: TypingSequence) -> Result<TypingSequence> {
        log::info!("Humanizing typing sequence of {} characters", sequence.text.len());
        
        // Basic implementation - actual humanization would add timing variations,
        // occasional mistakes and corrections, fatigue simulation, etc.
        Ok(sequence)
    }
    
    /// Humanize text with target WPM
    pub async fn humanize_text(&self, text: &str, wpm: u32) -> Result<TypingSequence> {
        log::info!("Humanizing text with {} WPM target", wpm);
        
        let mut keystrokes = Vec::new();
        let mut timing = 0;
        
        // Calculate base timing from WPM (words per minute -> characters per second)
        let chars_per_second = (wpm * 5) as f64 / 60.0; // Assuming 5 chars per word
        let base_delay_ms = (1000.0 / chars_per_second) as u64;
        
        for ch in text.chars() {
            // Add natural variation (±30%)
            let variation = (timing % 30) as i64 - 15;
            let delay = (base_delay_ms as i64 + variation).max(10) as u64;
            timing += delay;
            
            keystrokes.push(Keystroke {
                key: ch.to_string(),
                timing_ms: timing,
            });
        }
        
        Ok(TypingSequence {
            text: text.to_string(),
            keystrokes,
            total_duration_ms: timing,
        })
    }
}

impl Default for TypingHumanizer {
    fn default() -> Self {
        Self::new()
    }
}