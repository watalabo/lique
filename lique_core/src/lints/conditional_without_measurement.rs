use crate::{rule::Rule, Diagnostic};
use core::convert::Into;
use std::collections::HashSet;

use oq3_syntax::{
    ast::{AstChildren, Expr, Stmt},
    AstNode,
};

pub fn lint_conditional_without_measurement(stmts: AstChildren<Stmt>) -> Vec<Diagnostic> {
    let mut diags = Vec::new();
    let mut measured_bits_names = HashSet::new();

    for stmt in stmts.clone() {
        if let Stmt::AssignmentStmt(assignment) = stmt.clone() {
            if let Some(identifier) = assignment.identifier() {
                measured_bits_names.insert(identifier.syntax().to_string());
            } else if let Some(indexed_identifier) = assignment.indexed_identifier()
                && let Some(identifier) = indexed_identifier.identifier()
            {
                measured_bits_names.insert(identifier.syntax().to_string());
            }
        }

        if let Stmt::IfStmt(if_stmt) = stmt
            && let Some(condition) = if_stmt.condition()
            && let Expr::BinExpr(binary_expr) = condition.clone()
            && let Some(lhs) = binary_expr.lhs()
            && let Expr::Identifier(identifier) = lhs
        {
            let measured_bits = identifier.syntax().to_string();
            if !measured_bits_names.contains(&measured_bits) {
                let diag = Diagnostic {
                    rule_id: Rule::ConditionalWithoutMeasurement.into(),
                    message: "Conditional gates without preceedint measurement".to_string(),
                    range_zero_indexed: condition.syntax().text_range().into(),
                    related_informations: vec![],
                };
                measured_bits_names.remove(&measured_bits);
                diags.push(diag);
            }
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
    fn test_conditional_without_measurement() {
        let source = r#"OPENQASM 3.0;
include "stdgates.inc";
bit[1] c;
qubit[1] q0;
if (c == 0) {
  h q0[0];
}
c[0] = measure q0[0];
if (c == 0) {
  h q0[0];
}"#;
        let result =
            syntax_to_semantics::parse_source_string(source, Some("test.qasm"), None::<&[String]>);
        let stmts = result.syntax_result().syntax_ast().tree().statements();
        let diags = lint_conditional_without_measurement(stmts);

        assert_eq!(diags.len(), 1);
        let range = &diags[0].range_zero_indexed;
        assert_eq!(range.start, 65);
        assert_eq!(range.end, 71);
    }

    #[test]
    fn test_conditional_without_measurement_multiple_classical_bits() {
        let source = r#"OPENQASM 3.0;
include "stdgates.inc";
bit[1] c0;
bit[1] c1;
qubit[2] q0;
c0[0] = measure q0[0];
if (c0 == 0) {
  h q0[0];
}
if (c1 == 0) {
  h q0[0]
}"#;
        let result =
            syntax_to_semantics::parse_source_string(source, Some("test.qasm"), None::<&[String]>);
        let stmts = result.syntax_result().syntax_ast().tree().statements();
        let diags = lint_conditional_without_measurement(stmts);

        assert_eq!(diags.len(), 1);
        let range = &diags[0].range_zero_indexed;
        assert_eq!(range.start, 128);
        assert_eq!(range.end, 135);
    }

    #[test]
    fn test_conditional_with_measurement() {
        let source = r#"OPENQASM 3.0;
include "stdgates.inc";
bit[1] c;
qubit[1] q0;
c[0] = measure q0[0];
if (c == 0) {
  h q0[0];
}"#;
        let result =
            syntax_to_semantics::parse_source_string(source, Some("test.qasm"), None::<&[String]>);
        let stmts = result.syntax_result().syntax_ast().tree().statements();
        let diags = lint_conditional_without_measurement(stmts);

        assert_eq!(diags.len(), 0);
    }
}
