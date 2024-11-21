#![feature(let_chains)]

pub mod lints;

use std::ops::Range;

use oq3_semantics::syntax_to_semantics::ParseResult;
use oq3_source_file::SourceTrait;

#[derive(Debug)]
pub struct Diagnostic {
    pub message: String,
    pub range_zero_indexed: Range<usize>,
    pub related_informations: Vec<RelatedInformation>,
}

#[derive(Debug)]
pub struct RelatedInformation {
    pub message: String,
    pub range_zero_indexed: Range<usize>,
}

pub fn run_lints<T: SourceTrait>(parsed: ParseResult<T>) -> Vec<Diagnostic> {
    vec![
        lints::measurement_twice::lint_measurement_twice(
            parsed.syntax_result().syntax_ast().tree().statements(),
        ),
        lints::op_after_measurement::lint_op_after_measurement(
            parsed.syntax_result().syntax_ast().tree().statements(),
        ),
    ]
    .into_iter()
    .flatten()
    .collect()
}
