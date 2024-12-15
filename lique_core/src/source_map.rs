use serde::Deserialize;
use thiserror::Error;

#[derive(Clone, Debug, Deserialize)]
pub struct SourceMap {
    pub source_ranges: Vec<Option<usize>>,
    pub generated_line_byte_offset: Vec<usize>,
}

#[derive(Error, Debug)]
pub enum SourceMapError {
    #[error("Line number of source file is null")]
    LineNumberIsNull,
}
