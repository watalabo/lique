use rustpython_parser::{ast::Stmt, source_code::SourceRange};

use crate::Diagnostic;

pub fn lint_measurement_twice(stmts: &[Stmt<SourceRange>]) -> Vec<Diagnostic> {
    let mut diags = Vec::new();
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
                    && func.attr.as_str() == "measure"
                    && func.attr == other_func.attr
                    && target_qubit == other_target_qubit
                {
                    let diag = Diagnostic {
                        message: format!("Measurement of the same qubit twice: {}", target_qubit),
                        range: other_call.range,
                    };
                    diags.push(diag);
                }
            }
        }
    }
    diags
}

#[cfg(test)]
mod tests {
    use rustpython_parser::{
        ast::{Fold, Mod},
        source_code::RandomLocator,
    };

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
        let mut locator = RandomLocator::new(source);
        let module = locator.fold(module).unwrap();
        let stmts = &module.body;
        let diags = lint_measurement_twice(stmts);

        let range = diags[0].range;
        let start = range.start;
        let end = range.end.unwrap();
        assert_eq!(start.row.to_usize(), 8);
        assert_eq!(start.column.to_usize(), 1);
        assert_eq!(end.row.to_usize(), 8);
        assert_eq!(end.column.to_usize(), 22);
    }
}
