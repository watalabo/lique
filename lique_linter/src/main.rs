use std::borrow::Borrow;

use lique_core::{lints, SourceCode};
use rustpython_parser::source_code::RandomLocator;

fn main() {
    let path = "./test.py";
    let code = SourceCode::read_from_path(path);
    let module = code.parse().unwrap();

    let mut locator = RandomLocator::new(code.borrow());
    let stmts = &module.body;
    let diags = lints::measurement_twice::lint_measurement_twice(stmts);
    for diag in diags {
        let start = locator.locate(diag.range.start());
        let end = locator.locate(diag.range.end());
        println!("{} in {:?}:{:?}", diag.message, start, end);
    }
}
