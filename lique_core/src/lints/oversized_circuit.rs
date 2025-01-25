use core::convert::Into;
use std::collections::HashMap;

use crate::{rule::Rule, Diagnostic};

use oq3_syntax::ast::{AstChildren, Expr, Stmt};

use super::{collect_qubits, mark_manipulated_qubits, ManipulatedQubits};

pub fn lint_oversized_circuit(stmts: AstChildren<Stmt>) -> Vec<Diagnostic> {
    let qubits = collect_qubits(stmts.clone());
    let mut manipulated_qubits = qubits
        .iter()
        .map(|(name, (num, _))| (name.clone(), vec![false; *num]))
        .collect::<HashMap<_, _>>();

    lint_oversized_circuit_inner(stmts, &mut manipulated_qubits);

    let mut diags = Vec::new();
    for (qubit_name, (_, qubit_range)) in qubits {
        for (qubit_index, is_manipulated) in manipulated_qubits
            .get(&qubit_name)
            .unwrap()
            .iter()
            .enumerate()
        {
            if !is_manipulated {
                diags.push(Diagnostic {
                    rule_id: Rule::OversizedCircuit.into(),
                    range_zero_indexed: qubit_range.clone(),
                    message: format!("Qubit {}[{}] is not unused", qubit_name, qubit_index),
                    related_informations: vec![],
                });
            }
        }
    }
    diags
}

fn lint_oversized_circuit_inner(
    stmts: AstChildren<Stmt>,
    manipulated_qubits: &mut ManipulatedQubits,
) {
    for stmt in stmts {
        match stmt {
            Stmt::ExprStmt(expr_stmt) => {
                if let Some(expr) = expr_stmt.expr()
                    && let Expr::GateCallExpr(gate_call) = expr
                    && let Some(qubit_list) = gate_call.qubit_list()
                {
                    for operand in qubit_list.gate_operands() {
                        mark_manipulated_qubits(manipulated_qubits, &operand);
                    }
                }
            }
            Stmt::IfStmt(if_stmt) => {
                if let Some(then_stmt) = if_stmt.then_branch() {
                    let then_stmts = then_stmt.statements();
                    lint_oversized_circuit_inner(then_stmts, manipulated_qubits);
                }
                if let Some(else_stmt) = if_stmt.else_branch() {
                    let else_stmts = else_stmt.statements();
                    lint_oversized_circuit_inner(else_stmts, manipulated_qubits);
                }
            }
            Stmt::AssignmentStmt(assignment) => {
                if let Some(rhs) = assignment.rhs()
                    && let Expr::MeasureExpression(measurement) = rhs
                    && let Some(operand) = measurement.gate_operand()
                {
                    mark_manipulated_qubits(manipulated_qubits, &operand);
                }
            }
            _ => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use oq3_semantics::syntax_to_semantics;
    use oq3_source_file::SourceTrait;

    #[test]
    fn test_unmanipulated_qubit() {
        let source = r#"OPENQASM 3.0;
include "stdgates.inc";
bit[2] c;
qubit[2] q;
h q[1];
c[1] = measure q[1];"#;
        let result =
            syntax_to_semantics::parse_source_string(source, Some("test.qasm"), None::<&[String]>);
        let stmts = result.syntax_result().syntax_ast().tree().statements();
        let diags = lint_oversized_circuit(stmts);

        assert_eq!(diags.len(), 1);
        let range = &diags[0].range_zero_indexed;
        let start = range.start;
        let end = range.end;
        assert_eq!(start, 48);
        assert_eq!(end, 59);
    }

    #[test]
    fn test_manipulated_qubit() {
        let source = r#"OPENQASM 3.0;
include "stdgates.inc";
bit[3] c;
qubit[3] q;
h q[0];
h q[1];
h q[2];"#;
        let result =
            syntax_to_semantics::parse_source_string(source, Some("test.qasm"), None::<&[String]>);
        let stmts = result.syntax_result().syntax_ast().tree().statements();
        let diags = lint_oversized_circuit(stmts);

        assert_eq!(diags.len(), 0);
    }

    #[test]
    fn test_manipulated_qubit_if_stmt() {
        let source = r#"OPENQASM 3.0;
include "stdgates.inc";
bit[2] c;
qubit[2] q;
h q[0];
c[0] = measure q[0];
if (c[0] == 1) {
    h q[1];
}
c[1] = measure q[1];"#;
        let result =
            syntax_to_semantics::parse_source_string(source, Some("test.qasm"), None::<&[String]>);
        let stmts = result.syntax_result().syntax_ast().tree().statements();
        let diags = lint_oversized_circuit(stmts);

        assert_eq!(diags.len(), 0);
    }

    #[test]
    fn test_manipulated_qubit_if_else_stmt() {
        let source = r#"OPENQASM 3.0;
include "stdgates.inc";
bit[3] c;
qubit[3] q;
h q[0];
c[0] = measure q[0];
if (c[0] == 1) {
    h q[2];
} else {
    h q[1];
}
c[1] = measure q[1];"#;
        let result =
            syntax_to_semantics::parse_source_string(source, Some("test.qasm"), None::<&[String]>);
        let stmts = result.syntax_result().syntax_ast().tree().statements();
        let diags = lint_oversized_circuit(stmts);

        assert_eq!(diags.len(), 0);
    }
}
