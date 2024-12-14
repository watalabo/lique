#![feature(let_chains)]

pub mod byte_offset;
pub mod lints;
pub mod rule;
pub mod source_map;

use std::ops::Range;

use oq3_semantics::syntax_to_semantics::ParseResult;
use oq3_source_file::SourceTrait;
use rule::Rule;
use source_map::{SourceMap, SourceRange};

#[derive(Debug)]
pub struct Diagnostic {
    pub rule_id: String,
    pub message: String,
    pub range_zero_indexed: Range<usize>,
    pub related_informations: Vec<RelatedInformation>,
}

#[derive(Debug)]
pub struct RelatedInformation {
    pub message: String,
    pub range_zero_indexed: Range<usize>,
}

pub fn run_lints<T: SourceTrait>(parsed: ParseResult<T>, rules: &[Rule]) -> Vec<Diagnostic> {
    rules
        .iter()
        .flat_map(|rule| rule.lint(parsed.syntax_result().syntax_ast().tree().statements()))
        .collect()
}

pub fn resolve_qasm_range<'a>(
    qasm_range: &Range<usize>,
    source_map: &'a SourceMap,
) -> &'a SourceRange {
    let instruction_index = source_map
        .generated_line_byte_offset
        // We need to find the diagnostic line's byte offset of the start of the line
        // Each element of `generated_line_byte_offset` is the byte offset of the start of the line.
        // If there is the exact match, it returns the index of the element.
        // Otherwise, it means the diagnostic starts in the middle of the line.
        // In this case, we get `index - 1` since binary_search returns the index of the next element.
        .binary_search(&qasm_range.start)
        .unwrap_or_else(|index| index - 1);
    &source_map.source_ranges[instruction_index]
}
