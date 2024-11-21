#![feature(let_chains)]

pub mod lints;

use std::ops::Range;

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
