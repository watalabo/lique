use crate::{rule::Rule, Diagnostic};
use core::convert::Into;
use std::collections::HashMap;

use oq3_syntax::{
    ast::{AstChildren, Expr, IndexKind, Stmt},
    AstNode,
};

use super::count_clbits;

pub fn lint_conditional_without_measurement(stmts: AstChildren<Stmt>) -> Vec<Diagnostic> {
    let mut diags = Vec::new();
    // classical register name -> Bit mask if each bit is measured
    let mut measured_bits = HashMap::new();
    let num_clbits = count_clbits(stmts.clone());

    for stmt in stmts.clone() {
        if let Stmt::AssignmentStmt(assignment) = stmt.clone()
            && let Some(rhs) = assignment.rhs()
            && let Expr::MeasureExpression(_) = rhs
        {
            if let Some(identifier) = assignment.identifier() {
                let clbits_name = identifier.syntax().to_string();
                let mask = (1 << num_clbits[&clbits_name].0) - 1;
                measured_bits.insert(clbits_name, mask);
            } else if let Some(indexed_identifier) = assignment.indexed_identifier()
                && let Some(identifier) = indexed_identifier.identifier()
            {
                for operator in indexed_identifier.index_operators() {
                    if let Some(kind) = operator.index_kind()
                        && let IndexKind::ExpressionList(list) = kind
                    {
                        for expr in list.exprs() {
                            if let Expr::Literal(literal) = expr {
                                let qubit_index =
                                    literal.syntax().to_string().parse::<usize>().unwrap();
                                let mask = 1 << qubit_index;
                                measured_bits
                                    .entry(identifier.syntax().to_string())
                                    .and_modify(|e| *e |= mask)
                                    .or_insert(mask);
                            }
                        }
                    }
                }
            }
        }

        if let Stmt::IfStmt(if_stmt) = stmt
            && let Some(condition) = if_stmt.condition()
        // && let Expr::BinExpr(binary_expr) = condition.clone()
        // && let Some(lhs) = binary_expr.lhs()
        {
            let clbits = {
                if let Expr::BinExpr(binary_expr) = condition.clone()
                    && let Some(lhs) = binary_expr.lhs()
                {
                    lhs
                } else if let Expr::Identifier(_) = condition {
                    condition.clone()
                } else if let Expr::IndexedIdentifier(_) = condition {
                    condition.clone()
                } else {
                    continue;
                }
            };
            match clbits {
                Expr::Identifier(identifier) => {
                    let clbits_name = identifier.syntax().to_string();
                    if let Some(mask) = measured_bits.get(&clbits_name).or(Some(&0)) {
                        if mask ^ ((1 << num_clbits[&clbits_name].0) - 1) > 0 {
                            let diag = Diagnostic {
                                rule_id: Rule::ConditionalWithoutMeasurement.into(),
                                message: "Conditional gates without preceeding measurement"
                                    .to_string(),
                                range_zero_indexed: condition.syntax().text_range().into(),
                                related_informations: vec![],
                            };
                            diags.push(diag);
                        }
                    }
                }
                Expr::IndexedIdentifier(indexed_identifier) => {
                    if let Some(identifier) = indexed_identifier.identifier() {
                        let clbits_name = identifier.syntax().to_string();
                        let Some(mask) = measured_bits.get(&clbits_name).or(Some(&0)) else {
                            continue;
                        };
                        for operator in indexed_identifier.index_operators() {
                            if let Some(kind) = operator.index_kind()
                                && let IndexKind::ExpressionList(list) = kind
                            {
                                for expr in list.exprs() {
                                    if let Expr::Literal(literal) = expr {
                                        let qubit_index =
                                            literal.syntax().to_string().parse::<usize>().unwrap();
                                        if mask & (1 << qubit_index) == 0 {
                                            let diag = Diagnostic {
                                                    rule_id: Rule::ConditionalWithoutMeasurement.into(),
                                                    message: "Conditional gates without preceeding measurement"
                                                        .to_string(),
                                                    range_zero_indexed: condition.syntax().text_range().into(),
                                                    related_informations: vec![],
                                                };
                                            diags.push(diag);
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                _ => {}
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

        dbg!(&diags);
        assert_eq!(diags.len(), 1);
        let range = &diags[0].range_zero_indexed;
        assert_eq!(range.start, 128);
        assert_eq!(range.end, 135);
    }

    #[test]
    fn test_conditional_without_measurement_index_access() {
        let source = r#"OPENQASM 3.0;
include "stdgates.inc";
bit[2] c0;
qubit[2] q0;
c0[0] = measure q0[0];
if (c0[1] == 1) {
  h q0[0];
}"#;
        let result =
            syntax_to_semantics::parse_source_string(source, Some("test.qasm"), None::<&[String]>);
        let stmts = result.syntax_result().syntax_ast().tree().statements();
        let diags = lint_conditional_without_measurement(stmts);

        assert_eq!(diags.len(), 1);
        let range = &diags[0].range_zero_indexed;
        assert_eq!(range.start, 89);
        assert_eq!(range.end, 99);
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
