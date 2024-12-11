use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct SourceMap {
    pub source_ranges: Vec<SourceRange>,
    pub generated_line_byte_offset: Vec<usize>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct SourceRange {
    pub start: Position,
    pub end: Position,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Position {
    pub line: usize,
    pub column: usize,
}
