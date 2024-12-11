#![feature(let_chains)]

pub mod byte_offset;
pub mod lints;
pub mod rule;
pub mod source_map;

use std::ops::Range;

use byte_offset::{ByteOffsetError, ByteOffsetLocator};
use oq3_semantics::syntax_to_semantics::ParseResult;
use oq3_source_file::SourceTrait;
use rule::Rule;
use source_map::SourceMap;

#[derive(Debug)]
pub struct Diagnostic {
    pub message: String,
    pub range_zero_indexed: Range<usize>,
    pub related_informations: Vec<RelatedInformation>,
}

pub fn locate_in_source_file(
    qasm_range: &Range<usize>,
    source_map: &SourceMap,
    source_file_locator: &ByteOffsetLocator,
) -> Result<Range<usize>, ByteOffsetError> {
    let instruction_index = source_map
        .generated_line_byte_offset
        .binary_search(&qasm_range.start)
        // We need to find the diagnostic line's byte offset of the start of the line
        // Each element of `generated_line_byte_offset` is the byte offset of the start of the line.
        // If there is the exact match, it returns the index of the element.
        // Otherwise, it means the diagnostic starts in the middle of the line.
        // In this case, we get `index - 1` since binary_search returns the index of the next element.
        .unwrap_or_else(|index| index - 1);
    let start_byte_offset =
        source_file_locator.locate(&source_map.source_ranges[instruction_index].start)?;
    let end_byte_offset =
        source_file_locator.locate(&source_map.source_ranges[instruction_index].end)?;
    let converted_range = start_byte_offset..end_byte_offset;
    Ok(converted_range)
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
