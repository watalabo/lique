use lique::lints;
use rustpython_parser::{ast::Mod, parse, source_code::RandomLocator, Mode};

fn main() {
    let path = "./test.py";
    let python_source = std::fs::read_to_string(path).unwrap();
    let module = parse(&python_source, Mode::Module, &path).unwrap();
    dbg!(&module);
    let Mod::Module(module) = module else {
        panic!("Expected a module");
    };

    let mut locator = RandomLocator::new(&python_source);
    let stmts = &module.body;
    if let Some((_, other_call)) = lints::measurement_twice::lint_measurement_twice(stmts) {
        let range = other_call.range;
        let start = range.start();
        let end = range.end();
        println!(
            "Found a duplicate call at {:?}:{:?}",
            locator.locate(start),
            locator.locate(end)
        );
    }
}
