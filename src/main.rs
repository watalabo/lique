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
        if let Some(expr) = stmt.as_expr_stmt()
            && let Some(call) = expr.value.as_call_expr()
        {
            for other_stmt in stmts.iter().skip(i + 1) {
                if let Some(other_expr) = other_stmt.as_expr_stmt()
                    && let Some(other_call) = other_expr.value.as_call_expr()
                    && let Some(func) = call.func.as_attribute_expr()
                    && let Some(other_func) = other_call.func.as_attribute_expr()
                    && let Some(instance_name) = func.value.as_name_expr()
                    && let Some(other_instance_name) = other_func.value.as_name_expr()
                    && let Some(target_qubit) = call.args[0].as_constant_expr()
                    && let Some(other_target_qubit) = other_call.args[0].as_constant_expr()
                    && let Some(target_qubit) = target_qubit.value.as_int()
                    && let Some(other_target_qubit) = other_target_qubit.value.as_int()
                    && instance_name.id == other_instance_name.id
                    && func.attr == other_func.attr
                    && target_qubit == other_target_qubit
                {
                    println!("Found a duplicate call to {}", instance_name.id);
                }
            }
        }
    }
}
