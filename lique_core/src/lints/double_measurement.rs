use core::convert::Into;

use oq3_syntax::{
    ast::{AstChildren, Expr, GateOperand, Stmt},
    AstNode,
};

use crate::{rule::Rule, Diagnostic, RelatedInformation};

use super::contains_or_equal;

pub fn lint_double_measurement(stmts: AstChildren<Stmt>) -> Vec<Diagnostic> {
    let mut diags = Vec::new();
    let mut measurement_operands: Vec<GateOperand> = Vec::new();
    for stmt in stmts {
        if let Stmt::AssignmentStmt(assignment) = stmt
            && let Some(rhs) = assignment.rhs()
            && let Expr::MeasureExpression(measurement) = rhs
            && let Some(operand) = measurement.gate_operand()
        {
            if let Some(earlier) = measurement_operands
                .iter()
                .find(|o| contains_or_equal(o, &operand))
            {
                let diag = Diagnostic {
                    rule_id: Rule::DoubleMeasurement.into(),
                    message: "Measurement of the same qubit twice".to_string(),
                    range_zero_indexed: assignment.syntax().text_range().into(),
                    related_informations: vec![
                        RelatedInformation {
                            message: "Earlier measurement here".to_string(),
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
            measurement_operands.push(operand);
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
    fn test_statements_root() {
        let source = r#"OPENQASM 3.0;
include "stdgates.inc";
bit[3] c;
qubit[3] q;
h q[0];
cx q[0], q[1];
cx q[1], q[2];
c[0] = measure q[0];
c[1] = measure q[1];
c[2] = measure q[0];"#;
        let result =
            syntax_to_semantics::parse_source_string(source, Some("test.qasm"), None::<&[String]>);
        let stmts = result.syntax_result().syntax_ast().tree().statements();
        let diags = lint_double_measurement(stmts);

        let range = &diags[0].range_zero_indexed;
        let start = range.start;
        let end = range.end;
        assert_eq!(start, 140);
        assert_eq!(end, 160);
    }

    #[test]
    fn test_statements_root_same_qubits() {
        let source = r#"OPENQASM 3.0;
include "stdgates.inc";
bit[3] c;
qubit[3] q;
h q[0];
cx q[0], q[1];
cx q[1], q[2];
c = measure q;
c = measure q;"#;
        let result =
            syntax_to_semantics::parse_source_string(source, Some("test.qasm"), None::<&[String]>);
        let stmts = result.syntax_result().syntax_ast().tree().statements();
        let diags = lint_double_measurement(stmts);

        let range = &diags[0].range_zero_indexed;
        let start = range.start;
        let end = range.end;
        assert_eq!(start, 113);
        assert_eq!(end, 127);
    }
}
