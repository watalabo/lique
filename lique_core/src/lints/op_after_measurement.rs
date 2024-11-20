use oq3_syntax::{
    ast::{AstChildren, Expr, GateOperand, Stmt},
    AstNode,
};

use crate::{lints::contains_or_equal, Diagnostic};

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
                    dbg!(&assignment, &operand);
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
                        dbg!(&expr_stmt, &operand);
                        if measurement_operands
                            .iter()
                            .any(|o| contains_or_equal(o, &operand))
                        {
                            let range = operand.syntax().text_range();
                            let start: usize = range.start().into();
                            let end: usize = range.end().into();
                            let diag = Diagnostic {
                                message: format!(
                                    "Operation after measurement of the same qubit: {}",
                                    operand
                                ),
                                range_zero_indexed: start..end,
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
        assert_eq!(start, 99);
        assert_eq!(end, 103);
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
        assert_eq!(start, 85);
        assert_eq!(end, 89);
    }
}
