#![feature(let_chains)]
use rustpython_parser::{
    ast::{self, Mod},
    parse, Mode,
};

fn extract_call_name(stmt: &ast::Stmt) -> Option<&ast::ExprName> {
    let expr = stmt.as_expr_stmt()?;
    let call = expr.value.as_call_expr()?;
    call.func.as_name_expr()
}

fn main() {
    let path = "./test.py";
    let python_source = std::fs::read_to_string(path).unwrap();
    let module = parse(&python_source, Mode::Module, &path).unwrap();
    dbg!(&module);
    let Mod::Module(module) = module else {
        panic!("Expected a module");
    };
    let stmts = &module.body;
    for (i, stmt) in stmts.iter().enumerate() {
        if let Some(name) = extract_call_name(stmt) {
            for other_stmt in stmts.iter().skip(i + 1) {
                if let Some(other_name) = extract_call_name(other_stmt)
                    && name.id == other_name.id
                {
                    dbg!(name);
                }
            }
        }
    }
}
