use thiserror::Error;

// define Error types
#[derive(Debug, Error)]
pub enum FakitError {
    #[error("Stdin not detected")]
    StdinNotDetected,

    #[error("File not found")]
    FileNotFound,

    #[error("Two pass error")]
    TwoPassNotAllowedStdin,

    #[error("Io error")]
    IoError(#[from] std::io::Error),

    #[error("Parse int error")]
    ParseIntError(#[from] std::num::ParseIntError),

    #[error("Regex Error")]
    RegexError(#[from] regex::Error),

    #[error("Utf8 error")]
    Utf8Error(#[from] std::str::Utf8Error),

    #[error("ParaseqFastaError")]
    ParaseqFastaError(#[from] paraseq::fasta::Error),

    #[error("ProcessError")]
    ProcessError(#[from] paraseq::parallel::ProcessError),
}
