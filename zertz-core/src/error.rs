use thiserror::Error;

#[derive(Debug, Error)]
pub enum ZertzCoreError {
    #[error("{0}")]
    IOErr(#[from] std::io::Error),
    #[error("Invalid board size was given. Only [37, 40, 43, 44, 48, 61] are possible. got = {0}")]
    InvalidBoardSize(u8),
    #[error("That ring which you selected cannot be removed.")]
    InvalidRingToRemove,
    #[error("That ring which you selected cannot take some new marble.")]
    InvalidPuttingMarble,
    #[error("Failed to catch a marble. This is almost an internal bug.")]
    FailedToCatchMarble,
    #[error("invalid input data was given")]
    InvalidInputData,
    #[error("cannot load the game data from a json file. Detail: {0}")]
    LoadFailed(serde_json::error::Error),
    #[error("cannot save the game data into a json file. Detail: {0}")]
    SaveFailed(serde_json::error::Error),
}

pub type Result<T> = std::result::Result<T, ZertzCoreError>;
