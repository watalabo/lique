use std::collections::HashSet;

use oq3_syntax::{
    ast::{AstChildren, Expr, GateOperand, Stmt},
    AstNode,
};

use crate::Diagnostic;

pub fn lint_measurement_twice(stmts: AstChildren<Stmt>) -> Vec<Diagnostic> {
    let mut diags = Vec::new();
    let mut measurement_operands: HashSet<GateOperand> = HashSet::new();
    for stmt in stmts {
        if let Stmt::AssignmentStmt(assignment) = stmt
            && let Some(rhs) = assignment.rhs()
            && let Expr::MeasureExpression(measurement) = rhs
            && let Some(operand) = measurement.gate_operand()
        {
            if measurement_operands.contains(&operand) {
                let range = operand.syntax().text_range();
                let start: usize = range.start().into();
                let end: usize = range.end().into();
                let diag = Diagnostic {
                    message: format!("Measurement of the same qubit twice: {}", operand),
                    range_zero_indexed: start..end,
                };
                diags.push(diag);
            }
            measurement_operands.insert(operand);
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
    fn test_statements_module_root() {
        let source = r#"OPENQASM 3.0;
include "stdgates.inc";
bit[3] c;
qubit[3] q;
h q[0];
cx q[0], q[1];
cx q[1], q[2];
c[0] = measure q[0];
c[1] = measure q[1];
c[2] = measure q[0]"#;
        let result =
            syntax_to_semantics::parse_source_string(source, Some("test.qasm"), None::<&[String]>);
        let stmts = result.syntax_result().syntax_ast().tree().statements();
        let diags = lint_measurement_twice(stmts);

        let range = &diags[0].range_zero_indexed;
        let start = range.start;
        let end = range.end;
        assert_eq!(start, 155);
        assert_eq!(end, 159);
    }
}
