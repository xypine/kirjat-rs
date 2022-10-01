use super::kirja::Kirja;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ResponseError {
    #[error("network error: {0}")]
    NetworkError(String),
    #[error("{0} palautti virheen: {1}")]
    SourceError(String, String),
    #[error("parser error")]
    ParserError(#[from] anyhow::Error),
    #[error("tuntematon virhe: {0}")]
    UnknownError(String),
}

pub type Response = Result<Vec<Kirja>, ResponseError>;
