use oq3_syntax::ast::{AstChildren, Stmt};

use crate::{lints, Diagnostic};

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Rule {
    DoubleMeasurement,
    OpAfterMeasurement,
}

impl From<&str> for Rule {
    fn from(s: &str) -> Self {
        match s {
            "double-measurement" => Rule::DoubleMeasurement,
            "operation-after-measurement" => Rule::OpAfterMeasurement,
            _ => panic!("Unknown rule: {}", s),
        }
    }
}

impl From<Rule> for String {
    fn from(rule: Rule) -> String {
        match rule {
            Rule::DoubleMeasurement => "double-measurement".to_string(),
            Rule::OpAfterMeasurement => "operation-after-measurement".to_string(),
        }
    }
}

impl Rule {
    pub fn all() -> Vec<Self> {
        vec![Rule::DoubleMeasurement, Rule::OpAfterMeasurement]
    }

    pub fn lint(&self, stmts: AstChildren<Stmt>) -> Vec<Diagnostic> {
        match self {
            Rule::DoubleMeasurement => lints::double_measurement::lint_double_measurement(stmts),
            Rule::OpAfterMeasurement => {
                lints::op_after_measurement::lint_op_after_measurement(stmts)
            }
        }
    }
}
