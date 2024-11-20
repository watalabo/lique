use oq3_syntax::{
    ast::{AstChildren, Expr, GateOperand, Stmt},
    AstNode,
};

use super::contains_or_equal;
use crate::{Diagnostic, RelatedInformation};

pub fn lint_op_after_measurement(stmts: AstChildren<Stmt>) -> Vec<Diagnostic> {
    let mut diags = Vec::new();
    let mut measurement_operands: Vec<GateOperand> = Vec::new();
    for stmt in stmts {
        match stmt {
            Stmt::AssignmentStmt(assignment) => {
                if let Some(rhs) = assignment.rhs()
                    && let Expr::MeasureExpression(measurement) = rhs
                    && let Some(operand) = measurement.gate_operand()
                {
                    measurement_operands.push(operand);
                }
            }
            Stmt::ExprStmt(expr_stmt) => {
                if let Some(expr) = expr_stmt.expr()
                    && let Expr::GateCallExpr(gate_call) = expr
                    && let Some(qubit_list) = gate_call.qubit_list()
                {
                    let gate_operands = qubit_list.gate_operands();
                    for operand in gate_operands {
                        if let Some(earlier) = measurement_operands
                            .iter()
                            .find(|o| contains_or_equal(o, &operand))
                        {
                            let diag = Diagnostic {
                                message: "Operation after measurement of the same qubit"
                                    .to_string(),
                                range_zero_indexed: expr_stmt.syntax().text_range().into(),
                                related_informations: vec![
                                    RelatedInformation {
                                        message: "Earlier measurement".to_string(),
                                        range_zero_indexed: earlier.syntax().text_range().into(),
                                    },
                                    RelatedInformation {
                                        message: "For this qubit".to_string(),
                                        range_zero_indexed: operand.syntax().text_range().into(),
                                    },
                                ],
                            };
                            diags.push(diag);
                        }
                    }
                }
            }
            _ => {
                continue;
            }
        }
    }
    diags
}

#[cfg(test)]
mod tests {
    use oq3_semantics::syntax_to_semantics;
    use oq3_source_file::SourceTrait;

    use super::*;

    #[test]
    fn test_statements_root_same_index() {
        let source = r#"OPENQASM 3.0;
include "stdgates.inc";
bit[3] c;
qubit[3] q;
h q[0];
c[0] = measure q[0];
h q[1];
h q[0];"#;
        let result =
            syntax_to_semantics::parse_source_string(source, Some("test.qasm"), None::<&[String]>);
        let stmts = result.syntax_result().syntax_ast().tree().statements();
        let diags = lint_op_after_measurement(stmts);

        let range = &diags[0].range_zero_indexed;
        let start = range.start;
        let end = range.end;
        assert_eq!(start, 97);
        assert_eq!(end, 104);
    }

    #[test]
    fn test_statements_root_contains() {
        let source = r#"OPENQASM 3.0;
include "stdgates.inc";
bit[3] c;
qubit[3] q;
h q[0];
c = measure q;
h q[0];"#;
        let result =
            syntax_to_semantics::parse_source_string(source, Some("test.qasm"), None::<&[String]>);
        let stmts = result.syntax_result().syntax_ast().tree().statements();
        let diags = lint_op_after_measurement(stmts);

        let range = &diags[0].range_zero_indexed;
        let start = range.start;
        let end = range.end;
        assert_eq!(start, 83);
        assert_eq!(end, 90);
    }
}
