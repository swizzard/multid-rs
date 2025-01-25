use thiserror::Error;

#[derive(Error, Debug)]
pub enum VError {
    #[error("Size mismatch error: expected {expected:?}, got {actual:?}")]
    SizingError { expected: usize, actual: usize },
    #[error("Error: {field_name:?} out of range")]
    OutOfRangeError { field_name: &'static str },
}
