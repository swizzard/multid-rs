//! # custom error type
use thiserror::Error;

/// custom error type
#[derive(Error, Debug)]
pub enum VError {
    /// incorrect dimensions
    #[error("Size mismatch error: expected {expected:?}, got {actual:?}")]
    SizingError { expected: usize, actual: usize },
}
