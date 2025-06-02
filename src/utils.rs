//! Utility functions for aegnt-27

use crate::error::{AegntError, Result};
use std::time::{SystemTime, UNIX_EPOCH};

/// Generate a unique session identifier
pub fn generate_session_id() -> String {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis();
    
    format!("aegnt_{}", timestamp)
}

/// Validate that a value is within an acceptable range
pub fn validate_range(value: f64, min: f64, max: f64, name: &str) -> Result<()> {
    if value < min || value > max {
        return Err(AegntError::config(format!(
            "{} must be between {} and {}, got {}",
            name, min, max, value
        )));
    }
    Ok(())
}

/// Calculate authenticity score based on multiple factors
pub fn calculate_authenticity_score(factors: &[f64]) -> f64 {
    if factors.is_empty() {
        return 0.0;
    }
    
    let sum: f64 = factors.iter().sum();
    sum / factors.len() as f64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_session_id_generation() {
        let id1 = generate_session_id();
        let id2 = generate_session_id();
        
        assert_ne!(id1, id2);
        assert!(id1.starts_with("aegnt_"));
        assert!(id2.starts_with("aegnt_"));
    }

    #[test]
    fn test_range_validation() {
        assert!(validate_range(0.5, 0.0, 1.0, "test").is_ok());
        assert!(validate_range(-0.1, 0.0, 1.0, "test").is_err());
        assert!(validate_range(1.1, 0.0, 1.0, "test").is_err());
    }

    #[test]
    fn test_authenticity_calculation() {
        assert_eq!(calculate_authenticity_score(&[]), 0.0);
        assert_eq!(calculate_authenticity_score(&[1.0]), 1.0);
        assert_eq!(calculate_authenticity_score(&[0.8, 0.9, 1.0]), 0.9);
    }
}