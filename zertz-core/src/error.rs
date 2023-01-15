use thiserror::Error;

#[derive(Debug, Error)]
pub enum ZertzCoreError {
    #[error("{0}")]
    IOErr(#[from] std::io::Error),
    #[error("That ring which you selected cannot be removed.")]
    InvalidRingToRemove,
    #[error("That ring which you selected cannot take some new marble.")]
    InvalidPuttingMarble,
    #[error("Failed to catch a marble. This is almost an internal bug.")]
    FailedToCatchMarble,
}

pub type Result<T> = std::result::Result<T, ZertzCoreError>;
