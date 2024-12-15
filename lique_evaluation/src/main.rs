use std::{
    fs::{File, create_dir_all},
    path::Path,
};

use lique_core::{LintReport, resolve_qasm_range, rule::Rule, run_lints, source_map::SourceMap};
use oq3_semantics::syntax_to_semantics;
use serde::Deserialize;

#[derive(Clone, Deserialize)]
struct DatasetCase {
    // rule_id: Vec<String>,
    file_name: String,
}

fn main() -> anyhow::Result<()> {
    let evaluation_dir = Path::new("./evaluation");
    let dataset_dir = evaluation_dir.join("dataset");
    let qasm_dir = dataset_dir.join("qasm");
    let source_map_dir = dataset_dir.join("source_map");
    let dataset_file = dataset_dir.join("dataset.json");
    create_dir_all(&qasm_dir).unwrap();
    create_dir_all(&source_map_dir).unwrap();

    let dataset_file = File::open(dataset_file).unwrap();
    let dataset_cases: Vec<DatasetCase> = serde_json::from_reader(dataset_file).unwrap();

    let results = dataset_cases
        .into_iter()
        .flat_map(|case| {
            Rule::all()
                .into_iter()
                .map(move |rule| (case.clone(), rule))
        })
        .flat_map(|(case, rule)| {
            let qasm_path = qasm_dir.join(format!("{}.qasm", &case.file_name));
            let source_map_path = source_map_dir.join(format!("{}.json", &case.file_name));
            run_lint(rule, qasm_path, source_map_path, case.file_name)
        })
        .collect::<anyhow::Result<Vec<_>>>()?;

    let json_file = File::create(evaluation_dir.join("lique_results2.json"))?;
    serde_json::to_writer(json_file, &results)?;
    Ok(())
}

fn run_lint<P: AsRef<Path>>(
    rule: Rule,
    qasm_path: P,
    source_map_path: P,
    base_file_name: String,
) -> impl Iterator<Item = anyhow::Result<LintReport>> {
    let parsed_qasm = syntax_to_semantics::parse_source_file(qasm_path.as_ref(), None::<&[String]>);
    let rules = vec![rule];
    let diagnostics = run_lints(parsed_qasm, &rules);

    let source_map_file = File::open(source_map_path).unwrap();
    let source_map: SourceMap = serde_json::from_reader(source_map_file).unwrap();
    diagnostics.into_iter().map(move |diagnostic| {
        let source_range = resolve_qasm_range(&diagnostic.range_zero_indexed, &source_map)?;
        Ok(LintReport {
            rule_id: diagnostic.rule_id.clone(),
            line_number: source_range,
            file_name: base_file_name.clone(),
        })
    })
}
