use rustpython_parser::ast::{ExprCall, Stmt};

pub fn lint_measurement_twice(stmts: &[Stmt]) -> Option<(&ExprCall, &ExprCall)> {
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
                    return Some((call, other_call));
                }
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use rustpython_parser::{ast::Mod, source_code::RandomLocator};

    use super::*;
    use crate::tests::parse_python_source;

    #[test]
    fn test_statements_module_root() {
        let source = r#"from qiskit import QuantumCircuit
circuit = QuantumCircuit(3, 3)
circuit.h(0)
circuit.cx(0, 1)
circuit.cx(1, 2)
circuit.measure(0, 0)
circuit.measure(2, 2)
circuit.measure(0, 1)"#;
        let Mod::Module(module) = parse_python_source(source) else {
            panic!("Expected a module");
        };
        let stmts = &module.body;
        let (call, other_call) = lint_measurement_twice(stmts).unwrap();
        let mut locator = RandomLocator::new(source);

        let range = call.range;
        let start = locator.locate(range.start());
        let end = locator.locate(range.end());
        assert_eq!(start.row.to_usize(), 6);
        assert_eq!(start.column.to_usize(), 1);
        assert_eq!(end.row.to_usize(), 6);
        assert_eq!(end.column.to_usize(), 22);

        let range = other_call.range;
        let start = locator.locate(range.start());
        let end = locator.locate(range.end());
        assert_eq!(start.row.to_usize(), 8);
        assert_eq!(start.column.to_usize(), 1);
        assert_eq!(end.row.to_usize(), 8);
        assert_eq!(end.column.to_usize(), 22);
    }
}
