use thiserror::Error;

#[derive(Debug, Error)]
pub enum OmenError {
    #[error("device unavailable: {0}")]
    Unavailable(String),
    #[error("not supported: {0}")]
    Unsupported(String),
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
    #[error("other: {0}")]
    Other(String),
}
