use oq3_syntax::ast::{AstChildren, Stmt};

use crate::{lints, Diagnostic};

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Rule {
    ConditionalWithoutMeasurement,
    DoubleMeasurement,
    OpAfterMeasurement,
    UnmanipulatedQubit,
    UnmeasurableQubits,
}

impl From<&str> for Rule {
    fn from(s: &str) -> Self {
        match s {
            "conditional-without-measurement" => Rule::ConditionalWithoutMeasurement,
            "double-measurement" => Rule::DoubleMeasurement,
            "operation-after-measurement" => Rule::OpAfterMeasurement,
            "unmanipulated-qubit" => Rule::UnmanipulatedQubit,
            "unmeasurable-qubits" => Rule::UnmeasurableQubits,
            _ => panic!("Unknown rule: {}", s),
        }
    }
}

impl From<Rule> for String {
    fn from(rule: Rule) -> String {
        match rule {
            Rule::ConditionalWithoutMeasurement => "ql-conditional-without-measurement".to_string(),
            Rule::DoubleMeasurement => "ql-double-measurement".to_string(),
            Rule::OpAfterMeasurement => "ql-operation-after-measurement".to_string(),
            Rule::UnmanipulatedQubit => "ql-constant-classic-bit".to_string(),
            Rule::UnmeasurableQubits => "ql-unmeasurable-qubits".to_string(),
        }
    }
}

impl Rule {
    pub fn all() -> Vec<Self> {
        vec![
            Rule::ConditionalWithoutMeasurement,
            Rule::DoubleMeasurement,
            Rule::OpAfterMeasurement,
            Rule::UnmanipulatedQubit,
            Rule::UnmeasurableQubits,
        ]
    }

    pub fn lint(&self, stmts: AstChildren<Stmt>) -> Vec<Diagnostic> {
        match self {
            Rule::ConditionalWithoutMeasurement => {
                lints::conditional_without_measurement::lint_conditional_without_measurement(stmts)
            }
            Rule::DoubleMeasurement => lints::double_measurement::lint_double_measurement(stmts),
            Rule::OpAfterMeasurement => {
                lints::op_after_measurement::lint_op_after_measurement(stmts)
            }
            Rule::UnmanipulatedQubit => {
                lints::unmanipulated_qubits::lint_unmanipulated_qubits(stmts)
            }
            Rule::UnmeasurableQubits => lints::unmeasurable_qubits::lint_unmeasurable_qubits(stmts),
        }
    }
}
