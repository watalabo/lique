use rustpython_parser::ParseError;
use thiserror::Error;

pub type Result<T> = std::result::Result<T, LiqueError>;

#[derive(Error, Debug)]
pub enum LiqueError {
    #[error("{0}")]
    ParseError(#[from] ParseError),
}
