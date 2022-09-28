use super::kirja::Kirja;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ResponseError {
    #[error("network error: {0}")]
    NetworkError(String),
    #[error("source returned an error: {0}")]
    SourceError(String),
    #[error("parser error")]
    ParserError(#[from] anyhow::Error),
    #[error("unknown error: {0}")]
    UnknownError(String),
}

pub type Response = Result<Vec<Kirja>, ResponseError>;
