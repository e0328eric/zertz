use thiserror::Error;

#[allow(dead_code)]
#[derive(Debug, Error)]
pub enum ZertzTerminalError {
    #[error("{0}")]
    IOErr(#[from] std::io::Error),
    #[error("{0}")]
    ZertzCoreErr(#[from] zertz_core::error::ZertzCoreError),
    #[error("cannot get a proper key event")]
    CannotGetKeyEvent,
    #[error("cannot get a proper mouse event")]
    CannotGetMouseEvent,
    #[error("Input is not ready")]
    InputIsNotInitialized,
    #[error("Unexpected game state was given. Got GameState: {0:?}")]
    UNexpectedGameState(zertz_core::game::GameState),
}

pub type Result<T> = std::result::Result<T, ZertzTerminalError>;
