use thiserror::Error;

#[derive(Debug, Error)]
pub enum ZertzError {
    #[error("That ring which you selected cannot be removed")]
    InvalidRingToRemove,
    #[error("Failed to calculating components")]
    FailedToCalculateComponents,
}

pub type Result<T> = std::result::Result<T, ZertzError>;
