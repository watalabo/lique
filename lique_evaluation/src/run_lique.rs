use std::{
    fs::{File, create_dir_all},
    path::Path,
};

use lique_core::{resolve_qasm_range, rule::Rule, run_lints, source_map::SourceMap};
use oq3_semantics::syntax_to_semantics;

use crate::types::DatasetCase;

pub fn run_lique() -> anyhow::Result<()> {
    let evaluation_dir = Path::new("./evaluation");
    let dataset_dir = evaluation_dir.join("dataset");
    let qasm_dir = dataset_dir.join("qasm");
    let source_map_dir = dataset_dir.join("source_map");
    let python_dir = dataset_dir.join("python");
    create_dir_all(&qasm_dir).unwrap();
    create_dir_all(&source_map_dir).unwrap();

    let dataset_cases = enumerate_file_base_names(&python_dir);
    let mut results = dataset_cases
        .into_iter()
        .flat_map(|case| {
            Rule::all()
                .into_iter()
                .map(move |rule| (case.clone(), rule))
        })
        .flat_map(|(case, rule)| {
            let qasm_path = qasm_dir.join(format!("{}.qasm", &case));
            let source_map_path = source_map_dir.join(format!("{}.json", &case));
            run_lint(rule, qasm_path, source_map_path, case)
        })
        .collect::<anyhow::Result<Vec<_>>>()?;
    results.sort_by_key(|report| {
        (
            report.file_name.clone(),
            report.line_number,
            report.rule_id.clone(),
        )
    });
    results.dedup();

    let json_file = File::create(evaluation_dir.join("lique_results.json"))?;
    serde_json::to_writer_pretty(json_file, &results)?;
    Ok(())
}

/// Enumerate file base names in the directory.
/// e.g. `qasm/{a,b,c.x}.py` -> `["a", "b", "c.x"]`
fn enumerate_file_base_names<P: AsRef<Path>>(dir: P) -> Vec<String> {
    std::fs::read_dir(dir)
        .unwrap()
        .filter_map(|entry| {
            let entry = entry.unwrap();
            if entry.file_type().unwrap().is_dir() {
                return None;
            }
            let file_name = entry.file_name();
            let file_name_str = file_name.to_string_lossy();
            file_name_str.rsplit_once('.').map(|x| x.0.to_string())
        })
        .collect()
}

fn run_lint<P: AsRef<Path>>(
    rule: Rule,
    qasm_path: P,
    source_map_path: P,
    base_file_name: String,
) -> impl Iterator<Item = anyhow::Result<DatasetCase>> {
    let parsed_qasm = syntax_to_semantics::parse_source_file(qasm_path.as_ref(), None::<&[String]>);
    let rules = vec![rule];
    let diagnostics = run_lints(parsed_qasm, &rules);

    let source_map_file = File::open(source_map_path).unwrap();
    let source_map: SourceMap = serde_json::from_reader(source_map_file).unwrap();
    diagnostics.into_iter().map(move |diagnostic| {
        let source_range = resolve_qasm_range(&diagnostic.range_zero_indexed, &source_map)?;
        Ok(DatasetCase {
            file_name: base_file_name.clone(),
            line_number: source_range,
            rule_id: diagnostic.rule_id.clone(),
        })
    })
}
