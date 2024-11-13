use lique_core::{lints, SourceCode};
use rustpython_parser::{ast::Fold, source_code::RandomLocator};

fn main() {
    let path = "./test.py";
    let code = SourceCode::read_from_path(path);
    let module = code.parse().unwrap();

    let mut locator = RandomLocator::new(&code);
    let module = locator.fold(module).unwrap();
    let stmts = &module.body;
    let diags = vec![
        lints::measurement_twice::lint_measurement_twice(stmts),
        lints::op_after_measurement::lint_op_after_measurement(stmts),
    ]
    .into_iter()
    .flatten()
    .collect::<Vec<_>>();
    for diag in diags {
        let start = diag.range.start;
        let end = diag.range.end.unwrap();
        println!("{} in {:?}:{:?}", diag.message, start, end);
    }
}
