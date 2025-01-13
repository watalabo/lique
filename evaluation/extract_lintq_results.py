from dataclasses import asdict
from json import dump, load
import os
from dataset.common import DatasetCase


def extract_results() -> list[DatasetCase]:
    cases = []
    with open(f"./evaluation/lintq/lintq_results.json") as f:
        original_results = load(f)
    for result in original_results["runs"][0]["results"]:
        file_name_ext = result["locations"][0]["physicalLocation"]["artifactLocation"]["uri"]
        case = DatasetCase(
            file_name=os.path.splitext(file_name_ext)[0],
            # LintQ uses 1-indexed line numbers
            line_number=result["locations"][0]["physicalLocation"]["region"]["startLine"] - 1,
            rule_id=result["ruleId"],
        )
        cases.append(case)
    return cases

if __name__ == "__main__":
    evaluation_dir = "./evaluation"

    cases = extract_results()
    cases.sort(key=lambda case: (case.file_name, case.line_number, case.rule_id))
    cases = [asdict(case) for case in cases]
    with open(f"{evaluation_dir}/results/lintq_results.json", "w") as f:
        dump(cases, f, indent=4)
