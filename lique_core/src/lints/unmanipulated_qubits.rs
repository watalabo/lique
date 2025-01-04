use core::ops::Range;

use crate::{rule::Rule, Diagnostic};

use oq3_syntax::{
    ast::{AstChildren, Expr, GateOperand, IndexKind, Stmt},
    AstNode,
};

pub fn lint_unmanipulated_qubits(stmts: AstChildren<Stmt>) -> Vec<Diagnostic> {
    let mut num_qubits = 0;
    let mut qubit_range: Range<usize> = 0..0;
    for stmt in stmts.clone() {
        if let Stmt::QuantumDeclarationStatement(declaration) = stmt.clone()
            && let Some(qubit) = declaration.qubit_type()
            && let Some(designator) = qubit.designator()
            && let Some(expr) = designator.expr()
            && let Expr::Literal(bits) = expr
        {
            num_qubits += bits.to_string().parse::<usize>().unwrap();
            qubit_range = stmt.syntax().text_range().into();
        }
    }

    let mut diags = Vec::new();
    let mut manipulated_mask = 0;
    for stmt in stmts {
        dbg!(manipulated_mask);
        match stmt {
            Stmt::ExprStmt(expr_stmt) => {
                if let Some(expr) = expr_stmt.expr()
                    && let Expr::GateCallExpr(gate_call) = expr
                    && let Some(qubit_list) = gate_call.qubit_list()
                {
                    for operand in qubit_list.gate_operands() {
                        match operand.clone() {
                            GateOperand::IndexedIdentifier(indexed_identifier) => {
                                for operator in indexed_identifier.index_operators() {
                                    if let Some(kind) = operator.index_kind()
                                        && let IndexKind::ExpressionList(list) = kind
                                    {
                                        for expr in list.exprs() {
                                            if let Expr::Literal(literal) = expr {
                                                let qubit_index = literal
                                                    .syntax()
                                                    .to_string()
                                                    .parse::<usize>()
                                                    .unwrap();
                                                manipulated_mask |= 1 << qubit_index;
                                            }
                                        }
                                    }
                                }
                            }
                            GateOperand::Identifier(_) => {
                                manipulated_mask = (1 << num_qubits) - 1;
                            }
                            _ => {}
                        }
                    }
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
                                    for expr in list.exprs() {
                                        if let Expr::Literal(literal) = expr {
                                            let qubit_index = literal
                                                .syntax()
                                                .to_string()
                                                .parse::<usize>()
                                                .unwrap();
                                            if manipulated_mask & (1 << qubit_index) == 0 {
                                                let diag = Diagnostic {
                                                    rule_id: Rule::UnmanipulatedQubit.into(),
                                                    message: format!("Qubit {} is measured without any prior manipulation", qubit_index),
                                                    range_zero_indexed: qubit_range.clone(),
                                                    related_informations: vec![],
                                                };
                                                diags.push(diag);
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        GateOperand::Identifier(_) => {
                            for i in 0..num_qubits {
                                if manipulated_mask & (1 << i) == 0 {
                                    let diag = Diagnostic {
                                        rule_id: Rule::UnmanipulatedQubit.into(),
                                        message: format!(
                                            "Qubit {} is measured without any prior manipulation",
                                            i
                                        ),
                                        range_zero_indexed: qubit_range.clone(),
                                        related_informations: vec![],
                                    };
                                    diags.push(diag);
                                }
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
}
