use oq3_syntax::{
    ast::{AstChildren, Expr, GateOperand, Stmt},
    AstNode,
};

use super::contains_or_equal;
use crate::{rule::Rule, Diagnostic, RelatedInformation};

pub fn lint_op_after_measurement(stmts: AstChildren<Stmt>) -> Vec<Diagnostic> {
    let mut diags = Vec::new();
    let mut measurement_operands: Vec<GateOperand> = Vec::new();
    lint_op_after_measurement_inner(stmts, &mut measurement_operands, &mut diags);
    diags
}

fn lint_op_after_measurement_inner(
    stmts: AstChildren<Stmt>,
    measured_operands: &mut Vec<GateOperand>,
    diags: &mut Vec<Diagnostic>,
) {
    for stmt in stmts {
        match stmt {
            Stmt::AssignmentStmt(assignment) => {
                if let Some(rhs) = assignment.rhs()
                    && let Expr::MeasureExpression(measurement) = rhs
                    && let Some(operand) = measurement.gate_operand()
                {
                    measured_operands.push(operand);
                }
            }
            Stmt::IfStmt(if_stmt) => {
                if let Some(then_stmt) = if_stmt.then_branch() {
                    let then_stmts = then_stmt.statements();
                    lint_op_after_measurement_inner(then_stmts, measured_operands, diags);
                }
                if let Some(else_stmt) = if_stmt.else_branch() {
                    let else_stmts = else_stmt.statements();
                    lint_op_after_measurement_inner(else_stmts, measured_operands, diags);
                }
            }
            Stmt::ExprStmt(expr_stmt) => {
                if let Some(expr) = expr_stmt.expr()
                    && let Expr::GateCallExpr(gate_call) = expr
                    && let Some(qubit_list) = gate_call.qubit_list()
                {
                    let gate_operands = qubit_list.gate_operands();
                    for operand in gate_operands {
                        if let Some(earlier) = measured_operands
                            .iter()
                            .find(|o| contains_or_equal(o, &operand))
                        {
                            let diag = Diagnostic {
                                rule_id: Rule::OpAfterMeasurement.into(),
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

    #[test]
    fn test_statements_in_if() {
        let source = r#"OPENQASM 3.0;
include "stdgates.inc";
bit[3] c;
qubit[3] q;
h q[0];
c = measure q;
if (c == 0) {
  h q[0];
}"#;
        let result =
            syntax_to_semantics::parse_source_string(source, Some("test.qasm"), None::<&[String]>);
        let stmts = result.syntax_result().syntax_ast().tree().statements();
        let diags = lint_op_after_measurement(stmts);

        let range = &diags[0].range_zero_indexed;
        let start = range.start;
        let end = range.end;
        assert_eq!(start, 99);
        assert_eq!(end, 106);
    }
}
