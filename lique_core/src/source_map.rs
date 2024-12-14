use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct SourceMap {
    pub source_ranges: Vec<SourceRange>,
    pub generated_line_byte_offset: Vec<usize>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct SourceRange {
    pub line: usize,
}
