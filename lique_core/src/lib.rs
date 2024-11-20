#![feature(let_chains)]

pub mod error;
pub mod lints;

use std::ops::Range;

pub struct Diagnostic {
    pub message: String,
    pub range_zero_indexed: Range<usize>,
    pub related_informations: Vec<RelatedInformation>,
}

pub struct RelatedInformation {
    pub message: String,
    pub range_zero_indexed: Range<usize>,
}
