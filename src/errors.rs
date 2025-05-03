//! # custom error type
use thiserror::Error;

/// custom error type
#[derive(Error, Debug)]
pub enum VError {
    /// incorrect dimensions
    #[error("Size mismatch error: expected {expected:?}, got {actual:?}")]
    SizingError { expected: usize, actual: usize },
}

impl VError {
    pub fn size_error(expected: usize, actual: usize) -> Self {
        VError::SizingError { expected, actual }
    }
}
