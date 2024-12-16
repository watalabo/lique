use core::convert::AsRef;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

#[derive(Clone, Debug, Deserialize, Serialize)]
struct DatasetCase {
    pub file_name: String,
    pub line_number: usize,
    pub rule_id: String,
}

#[derive(Debug, Serialize)]
struct DatasetCasesClassified {
    pub tp: f64,
    pub fp: f64,
    pub r#fn: f64,
    pub precision: f64,
    pub recall: f64,
    pub f1: f64,
    pub tp_cases: Vec<DatasetCase>,
    pub fp_cases: Vec<DatasetCase>,
    pub fn_cases: Vec<DatasetCase>,
}

pub fn calculate_metrics_lique() {
    let evaluation_dir = Path::new("./evaluation");
    let dataset = load_dataset(evaluation_dir.join("dataset.json"));
    let lique_results = load_lique_result(evaluation_dir.join("lique_results.json"));
    let cases = calculate_metrics(dataset, lique_results);
    let json_file = File::create(evaluation_dir.join("lique_metrics.json")).unwrap();
    serde_json::to_writer_pretty(json_file, &cases).unwrap();
}

fn load_dataset<P: AsRef<Path>>(dataset_file_path: P) -> Vec<DatasetCase> {
    let file = File::open(dataset_file_path).expect("Unable to open file");
    let reader = BufReader::new(file);
    serde_json::from_reader(reader).expect("Unable to parse JSON")
}

fn load_lique_result<P: AsRef<Path>>(lique_result_file_path: P) -> Vec<DatasetCase> {
    let file = File::open(lique_result_file_path).expect("Unable to open file");
    let reader = BufReader::new(file);
    serde_json::from_reader(reader).expect("Unable to parse JSON")
}

fn calculate_metrics(
    dataset: Vec<DatasetCase>,
    lique_results: Vec<DatasetCase>,
) -> DatasetCasesClassified {
    let mut tp_cases = Vec::new();
    let mut fp_cases = Vec::new();
    let mut fn_cases = Vec::new();

    for dataset_case in dataset {
        let mut found = false;
        for lique_result in &lique_results {
            if dataset_case.file_name == lique_result.file_name
                && dataset_case.line_number == lique_result.line_number
            {
                found = true;
                if dataset_case.rule_id == lique_result.rule_id {
                    tp_cases.push(dataset_case.clone());
                } else {
                    fp_cases.push(dataset_case.clone());
                }
            }
        }
        if !found {
            fn_cases.push(dataset_case.clone());
        }
    }

    let tp = tp_cases.len() as f64;
    let fp = fp_cases.len() as f64;
    let r#fn = fn_cases.len() as f64;
    let precision = tp / (tp + fp);
    let recall = tp / (tp + r#fn);
    let f1 = 2.0 * tp / (2.0 * tp + fp + r#fn);

    DatasetCasesClassified {
        tp,
        fp,
        r#fn,
        precision,
        recall,
        f1,
        tp_cases,
        fp_cases,
        fn_cases,
    }
}
