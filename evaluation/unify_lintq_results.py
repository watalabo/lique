from dataclasses import asdict
from json import dump, load
import os
from dataset.common import DatasetCase, rules_all


def unify_results(rules: list[str]) -> list[DatasetCase]:
    cases = []
    for rule in rules:
        with open(f"./evaluation/lintq/{rule}/lintq_results.json") as f:
            original_results = load(f)
        for result in original_results["runs"][0]["results"]:
            file_name_ext = result["locations"][0]["physicalLocation"]["artifactLocation"]["uri"]
            case = DatasetCase(
                file_name=os.path.splitext(file_name_ext)[0],
                line_number=result["locations"][0]["physicalLocation"]["region"]["startLine"],
                rule_id=rule,
            )
            cases.append(case)
    return cases

if __name__ == "__main__":
    evaluation_dir = "./evaluation"

    cases = unify_results(rules_all)
    cases.sort(key=lambda case: (case.file_name, case.line_number, case.rule_id))
    cases = [asdict(case) for case in cases]
    with open(f"{evaluation_dir}/lintq_results.json", "w") as f:
        dump(cases, f, indent=4)
