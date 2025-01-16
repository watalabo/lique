use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct DatasetCase {
    pub file_name: String,
    pub line_number: usize,
    pub rule_id: String,
}

pub type Dataset = HashMap<String, Vec<DatasetCase>>;

#[derive(Debug, Serialize)]
pub struct Metrics {
    pub tp: f64,
    pub fp: f64,
    pub r#fn: f64,
    pub precision: f64,
    pub recall: f64,
    pub f1: f64,
}

#[derive(Debug, Serialize)]
pub struct DatasetCasesClassified {
    pub metrics: HashMap<String, Metrics>,
    pub tp_cases: Dataset,
    pub fp_cases: Dataset,
    pub fn_cases: Dataset,
}
