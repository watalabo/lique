use core::convert::Into;

use oq3_syntax::{
    ast::{AstChildren, Expr, Stmt},
    AstNode,
};

use crate::{rule::Rule, Diagnostic};

pub fn lint_unmeasurable_qubits(stmts: AstChildren<Stmt>) -> Vec<Diagnostic> {
    let mut diags = Vec::new();
    let mut classical_registers = 0;
    let mut quantum_registers = 0;

    for stmt in stmts.clone() {
        if let Stmt::QuantumDeclarationStatement(declaration) = stmt
            && let Some(qubit) = declaration.qubit_type()
            && let Some(designator) = qubit.designator()
            && let Some(expr) = designator.expr()
            && let Expr::Literal(bits) = expr
        {
            quantum_registers += bits.to_string().parse::<usize>().unwrap();
        }
    }

    let mut last_classical_registers_range = 0..0;
    for stmt in stmts {
        if let Stmt::ClassicalDeclarationStatement(declaration) = stmt.clone()
            && let Some(qubit) = declaration.scalar_type()
            && let Some(designator) = qubit.designator()
            && let Some(expr) = designator.expr()
            && let Expr::Literal(bits) = expr
        {
            classical_registers += bits.to_string().parse::<usize>().unwrap();
            last_classical_registers_range = stmt.syntax().text_range().into();
        }
    }
    if classical_registers < quantum_registers {
        let diag = Diagnostic {
                    rule_id: Rule::UnmeasurableQubits.into(),
                    message: format!("Number of classical registers({}) is fewer than the number of quantum registers({})", classical_registers, quantum_registers),
                    range_zero_indexed: last_classical_registers_range,
                    related_informations: vec![],
                };
        diags.push(diag);
    }

    diags
}

#[cfg(test)]
mod tests {
    use oq3_semantics::syntax_to_semantics;
    use oq3_source_file::SourceTrait;

    use super::*;

    #[test]
    fn test_unmeasurable_qubits() {
        let source = r#"OPENQASM 3.0;
include "stdgates.inc";
bit[2] c;
qubit[3] q;
h q[0];
cx q[0], q[1];
cx q[1], q[2];"#;
        let result =
            syntax_to_semantics::parse_source_string(source, Some("test.qasm"), None::<&[String]>);
        let stmts = result.syntax_result().syntax_ast().tree().statements();
        let diags = lint_unmeasurable_qubits(stmts);

        assert_eq!(diags.len(), 1);
        let range = &diags[0].range_zero_indexed;
        assert_eq!(range.start, 38);
        assert_eq!(range.end, 47);
    }

    #[test]
    fn test_sufficient_classical_registers() {
        let source = r#"OPENQASM 3.0;
include "stdgates.inc";
bit[3] c;
qubit[3] q;
h q[0];
cx q[0], q[1];
cx q[1], q[2];"#;
        let result =
            syntax_to_semantics::parse_source_string(source, Some("test.qasm"), None::<&[String]>);
        let stmts = result.syntax_result().syntax_ast().tree().statements();
        let diags = lint_unmeasurable_qubits(stmts);

        assert_eq!(diags.len(), 0);
    }

    #[test]
    fn test_sufficient_multiple_classical_registers() {
        let source = r#"OPENQASM 3.0;
include "stdgates.inc";
bit[1] c0;
bit[2] c1;
qubit[3] q;
h q[0];
cx q[0], q[1];
cx q[1], q[2];"#;
        let result =
            syntax_to_semantics::parse_source_string(source, Some("test.qasm"), None::<&[String]>);
        let stmts = result.syntax_result().syntax_ast().tree().statements();
        let diags = lint_unmeasurable_qubits(stmts);

        assert_eq!(diags.len(), 0);
    }
}
