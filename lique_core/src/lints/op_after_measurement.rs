use rustpython_parser::{ast::Stmt, source_code::SourceRange};

use crate::Diagnostic;

pub fn lint_op_after_measurement(stmts: &[Stmt<SourceRange>]) -> Vec<Diagnostic> {
    let mut diags = Vec::new();
    dbg!(stmts);
    for (i, measure_stmt) in stmts.iter().enumerate() {
        if let Some(measure_expr) = measure_stmt.as_expr_stmt()
            && let Some(measure_call) = measure_expr.value.as_call_expr()
        {
            for op_stmt in stmts.iter().skip(i + 1) {
                if let Some(op_expr) = op_stmt.as_expr_stmt()
                    && let Some(op_call) = op_expr.value.as_call_expr()
                    && let Some(measure_func) = measure_call.func.as_attribute_expr()
                    && let Some(op_func) = op_call.func.as_attribute_expr()
                    && let Some(measure_instance_name) = measure_func.value.as_name_expr()
                    && let Some(op_instance_name) = op_func.value.as_name_expr()
                    && let Some(measure_target_qubit) = measure_call.args[0].as_constant_expr()
                    && let Some(op_target_qubit) = op_call.args[0].as_constant_expr()
                    && let Some(measure_target_qubit) = measure_target_qubit.value.as_int()
                    && let Some(op_target_qubit) = op_target_qubit.value.as_int()
                    && measure_instance_name.id == op_instance_name.id
                    && measure_func.attr.as_str() == "measure"
                    && op_func.attr.as_str() != "measure"
                    && measure_target_qubit == op_target_qubit
                {
                    let diag = Diagnostic {
                        message: format!(
                            "Operation after measurement of the same qubit: {}",
                            measure_target_qubit
                        ),
                        range: op_call.range,
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
circuit.measure(0, 0)
circuit.h(1)
circuit.h(0)"#;
        let Mod::Module(module) = parse_python_source(source) else {
            panic!("Expected a module");
        };
        let mut locator = RandomLocator::new(source);
        let module = locator.fold(module).unwrap();
        let stmts = &module.body;
        let diags = lint_op_after_measurement(stmts);

        let range = diags[0].range;
        let start = range.start;
        let end = range.end.unwrap();
        assert_eq!(start.row.to_usize(), 6);
        assert_eq!(start.column.to_usize(), 1);
        assert_eq!(end.row.to_usize(), 6);
        assert_eq!(end.column.to_usize(), 13);
    }
}
