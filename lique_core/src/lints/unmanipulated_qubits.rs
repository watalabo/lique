use core::ops::Range;
use std::collections::HashMap;

use crate::{rule::Rule, Diagnostic};

use oq3_syntax::{
    ast::{AstChildren, Expr, GateOperand, IndexKind, Stmt},
    AstNode,
};

use super::{collect_qubits, mark_manipulated_qubits, ManipulatedQubits};

pub fn lint_unmanipulated_qubits(stmts: AstChildren<Stmt>) -> Vec<Diagnostic> {
    let qubits = collect_qubits(stmts.clone());
    let mut manipulated_qubits = qubits
        .iter()
        .map(|(name, (num, _))| (name.clone(), vec![false; *num]))
        .collect::<HashMap<_, _>>();
    lint_unmanipulated_qubits_inner(stmts, &qubits, &mut manipulated_qubits)
}

fn lint_unmanipulated_qubits_inner(
    stmts: AstChildren<Stmt>,
    qubits: &HashMap<String, (usize, Range<usize>)>,
    manipulated_qubits: &mut ManipulatedQubits,
) -> Vec<Diagnostic> {
    let mut diags = Vec::new();
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
                    lint_unmanipulated_qubits_inner(then_stmts, qubits, manipulated_qubits);
                }
                if let Some(else_stmt) = if_stmt.else_branch() {
                    let else_stmts = else_stmt.statements();
                    lint_unmanipulated_qubits_inner(else_stmts, qubits, manipulated_qubits);
                }
            }
            Stmt::AssignmentStmt(assignment) => {
                if let Some(rhs) = assignment.rhs()
                    && let Expr::MeasureExpression(measurement) = rhs
                    && let Some(operand) = measurement.gate_operand()
                {
                    match operand {
                        GateOperand::IndexedIdentifier(indexed_identifier) => {
                            for operator in indexed_identifier.index_operators() {
                                if let Some(kind) = operator.index_kind()
                                    && let IndexKind::ExpressionList(list) = kind
                                {
                                    let qubit_name = indexed_identifier
                                        .identifier()
                                        .unwrap()
                                        .ident_token()
                                        .unwrap();
                                    for expr in list.exprs() {
                                        if let Expr::Literal(literal) = expr {
                                            let qubit_index = literal
                                                .syntax()
                                                .to_string()
                                                .parse::<usize>()
                                                .unwrap();
                                            if !manipulated_qubits.get(qubit_name.text()).unwrap()
                                                [qubit_index]
                                            {
                                                let diag = Diagnostic {
                                                    rule_id: Rule::UnmanipulatedQubit.into(),
                                                    message: format!("Qubit {}[{}] is measured without any prior manipulation", qubit_name, qubit_index),
                                                    range_zero_indexed: qubits.get(qubit_name.text()).unwrap().1.clone(),
                                                    related_informations: vec![],
                                                };
                                                diags.push(diag);
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        GateOperand::Identifier(identifier) => {
                            let qubit_name = identifier.ident_token().unwrap();
                            if manipulated_qubits
                                .get(qubit_name.text())
                                .unwrap()
                                .iter()
                                .all(|&b| !b)
                            {
                                let diag = Diagnostic {
                                    rule_id: Rule::UnmanipulatedQubit.into(),
                                    message: format!(
                                        "Qubit {} is measured without any prior manipulation",
                                        qubit_name
                                    ),
                                    range_zero_indexed: qubits
                                        .get(qubit_name.text())
                                        .unwrap()
                                        .1
                                        .clone(),
                                    related_informations: vec![],
                                };
                                diags.push(diag);
                            }
                        }
                        _ => {}
                    }
                }
            }
            _ => {}
        }
    }
    diags
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
bit[3] c;
qubit[3] q;
c[0] = measure q[0];
h q[1];
c[1] = measure q[1];"#;
        let result =
            syntax_to_semantics::parse_source_string(source, Some("test.qasm"), None::<&[String]>);
        let stmts = result.syntax_result().syntax_ast().tree().statements();
        let diags = lint_unmanipulated_qubits(stmts);

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
c[0] = measure q[0];
h q[1];
c[1] = measure q[1];"#;
        let result =
            syntax_to_semantics::parse_source_string(source, Some("test.qasm"), None::<&[String]>);
        let stmts = result.syntax_result().syntax_ast().tree().statements();
        let diags = lint_unmanipulated_qubits(stmts);

        assert_eq!(diags.len(), 0);
    }

    #[test]
    fn test_unmanipulated_qubit_if_stmt() {
        let source = r#"OPENQASM 3.0;
include "stdgates.inc";
bit[3] c;
qubit[3] q;
h q[0];
c[0] = measure q[0];
if (c[0] == 0) {
    h q[1];
}
c[1] = measure q[1];"#;
        let result =
            syntax_to_semantics::parse_source_string(source, Some("test.qasm"), None::<&[String]>);
        let stmts = result.syntax_result().syntax_ast().tree().statements();
        let diags = lint_unmanipulated_qubits(stmts);

        assert_eq!(diags.len(), 0);
    }

    #[test]
    fn test_unmanipulated_qubit_if_else_stmt() {
        let source = r#"OPENQASM 3.0;
include "stdgates.inc";
bit[3] c;
qubit[3] q;
h q[0];
c[0] = measure q[0];
if (c[0] == 0) {
    h q[2];
} else {
    x q[1];
}
c[1] = measure q[1];"#;
        let result =
            syntax_to_semantics::parse_source_string(source, Some("test.qasm"), None::<&[String]>);
        let stmts = result.syntax_result().syntax_ast().tree().statements();
        let diags = lint_unmanipulated_qubits(stmts);

        assert_eq!(diags.len(), 0);
    }
}
