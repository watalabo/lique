use core::convert::Into;

use oq3_syntax::ast::{AstChildren, Stmt};

use crate::{rule::Rule, Diagnostic};

use super::{count_clbits, count_qubits};

pub fn lint_unmeasurable_qubits(stmts: AstChildren<Stmt>) -> Vec<Diagnostic> {
    let mut diags = Vec::new();
    let (num_qubits, last_qubits_range) = count_qubits(stmts.clone());
    let num_clbits = count_clbits(stmts.clone()).values().map(|v| v.0).sum::<usize>();

    if num_clbits < num_qubits {
        let diag = Diagnostic {
                    rule_id: Rule::UnmeasurableQubits.into(),
                    message: format!("Number of classical registers({}) is fewer than the number of quantum registers({})", num_clbits, num_qubits),
                    range_zero_indexed: last_qubits_range,
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
        assert_eq!(range.start, 48);
        assert_eq!(range.end, 59);
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
