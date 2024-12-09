use oq3_syntax::ast::{AstChildren, Stmt};

use crate::{lints, Diagnostic};

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Rule {
    MeasurementTwice,
    OpAfterMeasurement,
}

impl From<&str> for Rule {
    fn from(s: &str) -> Self {
        match s {
            "measurement-twice" => Rule::MeasurementTwice,
            "op-after-measurement" => Rule::OpAfterMeasurement,
            _ => panic!("Unknown rule: {}", s),
        }
    }
}

impl Rule {
    pub fn all() -> Vec<Self> {
        vec![Rule::MeasurementTwice, Rule::OpAfterMeasurement]
    }

    pub fn lint(&self, stmts: AstChildren<Stmt>) -> Vec<Diagnostic> {
        match self {
            Rule::MeasurementTwice => lints::measurement_twice::lint_measurement_twice(stmts),
            Rule::OpAfterMeasurement => {
                lints::op_after_measurement::lint_op_after_measurement(stmts)
            }
        }
    }
}
