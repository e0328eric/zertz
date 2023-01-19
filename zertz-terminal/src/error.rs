use thiserror::Error;

#[allow(dead_code)]
#[derive(Debug, Error)]
pub enum ZertzTerminalError {
    #[error("{0}")]
    IOErr(#[from] std::io::Error),
    #[error("cannot get a proper key event")]
    CannotGetKeyEvent,
    #[error("cannot get a proper mouse event")]
    CannotGetMouseEvent,
}

pub type Result<T> = std::result::Result<T, ZertzTerminalError>;
