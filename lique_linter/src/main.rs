use lique_core::{lints, SourceCode};
use rustpython_parser::{ast::Fold, source_code::RandomLocator};

fn main() {
    let path = "./test.py";
    let code = SourceCode::read_from_path(path);
    let module = code.parse().unwrap();

    let mut locator = RandomLocator::new(&code);
    let module = locator.fold(module).unwrap();
    let stmts = &module.body;
    let diags = lints::measurement_twice::lint_measurement_twice(stmts);
    for diag in diags {
        let start = diag.range.start;
        let end = diag.range.end.unwrap();
        println!("{} in {:?}:{:?}", diag.message, start, end);
    }
}
