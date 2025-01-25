use thiserror::Error;

#[derive(Error, Debug)]
pub enum VError {
    #[error("Size mismatch error: expected {expected:?}, got {actual:?}")]
    SizingError { expected: usize, actual: usize },
}
