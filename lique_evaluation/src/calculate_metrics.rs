use core::convert::AsRef;
use std::io::BufReader;
use std::path::Path;
use std::{collections::HashMap, fs::File};

use lique_core::rule::Rule;

use crate::types::{Dataset, DatasetCase, DatasetCasesClassified, Metrics};

pub fn calculate_metrics<P: AsRef<Path>>(
    dataset_file_path: P,
    results_file_path: P,
    metrics_file_path: P,
) {
    let dataset = load_dataset(dataset_file_path);
    let lique_results = load_dataset(results_file_path);
    let cases = calculate_metrics_inner(dataset, lique_results);
    let json_file = File::create(metrics_file_path).unwrap();
    serde_json::to_writer_pretty(json_file, &cases).unwrap();
}

fn load_dataset<P: AsRef<Path>>(file_path: P) -> Vec<DatasetCase> {
    let file = File::open(file_path).expect("Unable to open file");
    let reader = BufReader::new(file);
    serde_json::from_reader(reader).expect("Unable to parse JSON")
}

fn calculate_metrics_inner(
    dataset: Vec<DatasetCase>,
    lique_results: Vec<DatasetCase>,
) -> DatasetCasesClassified {
    let mut metrics_map = HashMap::new();
    let mut tp_cases = Dataset::new();
    let mut fp_cases = Dataset::new();
    let mut fn_cases = Dataset::new();
    for rule in Rule::all() {
        let rule: String = rule.into();
        let (metrics, tp_cases_per_rule, fp_cases_per_rule, fn_cases_per_rule) =
            calculate_metrics_per_rule(dataset.clone(), lique_results.clone(), &rule);
        metrics_map.insert(rule.clone(), metrics);
        tp_cases.insert(rule.clone(), tp_cases_per_rule);
        fp_cases.insert(rule.clone(), fp_cases_per_rule);
        fn_cases.insert(rule.clone(), fn_cases_per_rule);
    }
    let tp = metrics_map.values().map(|m| m.tp).sum();
    let fp = metrics_map.values().map(|m| m.fp).sum();
    let r#fn = metrics_map.values().map(|m| m.r#fn).sum();
    let metrics_overall = Metrics {
        tp,
        fp,
        r#fn,
        precision: tp / (tp + fp),
        recall: tp / (tp + r#fn),
        f1: 2.0 * tp / (2.0 * tp + fp + r#fn),
    };
    metrics_map.insert("all".to_string(), metrics_overall);

    DatasetCasesClassified {
        metrics: metrics_map,
        tp_cases,
        fp_cases,
        fn_cases,
    }
}

fn calculate_metrics_per_rule(
    dataset: Vec<DatasetCase>,
    lique_results: Vec<DatasetCase>,
    rule_id: &str,
) -> (
    Metrics,
    Vec<DatasetCase>,
    Vec<DatasetCase>,
    Vec<DatasetCase>,
) {
    let (tp_cases, fp_cases, fn_cases) = count_tp_etc(dataset, lique_results, rule_id);

    let tp = tp_cases.len() as f64;
    let fp = fp_cases.len() as f64;
    let r#fn = fn_cases.len() as f64;
    let precision = tp / (tp + fp);
    let recall = tp / (tp + r#fn);
    let f1 = 2.0 * tp / (2.0 * tp + fp + r#fn);

    (
        Metrics {
            tp,
            fp,
            r#fn,
            precision,
            recall,
            f1,
        },
        tp_cases,
        fp_cases,
        fn_cases,
    )
}

fn count_tp_etc(
    dataset: Vec<DatasetCase>,
    lique_results: Vec<DatasetCase>,
    rule_id: &str,
) -> (Vec<DatasetCase>, Vec<DatasetCase>, Vec<DatasetCase>) {
    let mut tp_cases = Vec::new();
    let mut fp_cases = Vec::new();
    let mut fn_cases = Vec::new();

    for dataset_case in dataset.iter() {
        if dataset_case.rule_id != rule_id {
            continue;
        }

        let mut found = false;
        for lique_result in lique_results.iter() {
            if lique_result.rule_id == dataset_case.rule_id
                && lique_result.file_name == dataset_case.file_name
                && lique_result.line_number == dataset_case.line_number
            {
                tp_cases.push(dataset_case.clone());
                found = true;
                break;
            }
        }
        if !found {
            fn_cases.push(dataset_case.clone());
        }
    }

    for lique_result in lique_results.iter() {
        if lique_result.rule_id == rule_id {
            let mut found = false;
            for dataset_case in dataset.iter() {
                if lique_result.file_name == dataset_case.file_name
                    && lique_result.line_number == dataset_case.line_number
                {
                    found = true;
                    break;
                }
            }
            if !found {
                fp_cases.push(lique_result.clone());
            }
        }
    }

    (tp_cases, fp_cases, fn_cases)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_tp_etc() {
        let dataset = vec![
            DatasetCase {
                rule_id: "rule1".to_string(),
                file_name: "file1".to_string(),
                line_number: 1,
            },
            DatasetCase {
                rule_id: "rule1".to_string(),
                file_name: "file2".to_string(),
                line_number: 1,
            },
            DatasetCase {
                rule_id: "rule1".to_string(),
                file_name: "file1".to_string(),
                line_number: 10,
            },
            DatasetCase {
                rule_id: "rule2".to_string(),
                file_name: "file1".to_string(),
                line_number: 10,
            },
            DatasetCase {
                rule_id: "rule1".to_string(),
                file_name: "file3".to_string(),
                line_number: 1,
            },
        ];
        let lique_results = vec![
            DatasetCase {
                rule_id: "rule1".to_string(),
                file_name: "file1".to_string(),
                line_number: 1,
            },
            DatasetCase {
                rule_id: "rule1".to_string(),
                file_name: "file2".to_string(),
                line_number: 1,
            },
            DatasetCase {
                rule_id: "rule1".to_string(),
                file_name: "file1".to_string(),
                line_number: 10,
            },
            DatasetCase {
                rule_id: "rule2".to_string(),
                file_name: "file1".to_string(),
                line_number: 10,
            },
            DatasetCase {
                rule_id: "rule1".to_string(),
                file_name: "file3".to_string(),
                line_number: 5,
            },
        ];
        let (tp_cases, fp_cases, fn_cases) = count_tp_etc(dataset, lique_results, "rule1");
        assert_eq!(tp_cases.len(), 3);
        assert_eq!(fp_cases.len(), 1);
        assert_eq!(fn_cases.len(), 1);
    }
}
