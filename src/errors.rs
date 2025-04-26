use thiserror::Error;

// define Error types
#[derive(Debug, Error)]
pub enum FakitError {
    #[error("Stdin not detected")]
    StdinNotDetected,

    #[error("Io error")]
    IoError(#[from] std::io::Error),

    #[error("Parse int error")]
    ParseIntError(#[from] std::num::ParseIntError),

    #[error("Regex Error")]
    RegexError(#[from] regex::Error),

    #[error("Utf8 error")]
    Utf8Error(#[from] std::str::Utf8Error),
}
