use thiserror::Error;

#[derive(Debug, Error)]
pub enum ZertzError {
    #[error("That ring which you selected cannot be removed")]
    InvalidRingToRemove,
    #[error("That ring which you selected cannot take some new marble")]
    InvalidPuttingMarble,
}

pub type Result<T> = std::result::Result<T, ZertzError>;
