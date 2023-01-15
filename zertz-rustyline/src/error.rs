use thiserror::Error;

#[derive(Debug, Error)]
pub enum ZertzTerminalError {
    #[error("{0}")]
    ZertzCoreError(#[from] zertz_core::error::ZertzCoreError),
    #[error("{0}")]
    RustylineErr(#[from] rustyline::error::ReadlineError),
}

pub type Result<T> = anyhow::Result<T>;
