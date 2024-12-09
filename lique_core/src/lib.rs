#![feature(let_chains)]

pub mod lints;
pub mod rule;

use std::ops::Range;

use oq3_semantics::syntax_to_semantics::ParseResult;
use oq3_source_file::SourceTrait;
use rule::Rule;

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

pub fn run_lints<T: SourceTrait>(parsed: ParseResult<T>, rules: &[Rule]) -> Vec<Diagnostic> {
    rules
        .iter()
        .flat_map(|rule| rule.lint(parsed.syntax_result().syntax_ast().tree().statements()))
        .collect()
}
