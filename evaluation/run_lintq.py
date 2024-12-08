from dataclasses import dataclass
import json
import subprocess

from dataset.common import DatasetCase


@dataclass
class LiqueResult:
    rule_id: str
    expected_label: bool
    actual_label: bool
    file: str


if __name__ == "__main__":
    base_dir = "./evaluation"
    with open(f"{base_dir}/dataset/dataset.json", "r") as f:
        dataset = json.load(f)
    dataset = [DatasetCase(**case) for case in dataset]

    results = []
    for case in dataset:
        file_path = f"{base_dir}/dataset/qasm/{case.file_name}.qasm"
        process = subprocess.run(
            ["cargo", "run", "-p", "lique_linter", "--release", "--", file_path],
        )
        result = LiqueResult(
            rule_id=case.rule_id,
            expected_label=case.label,
            actual_label=process.returncode == 1,
            file=case.file_name,
        )
        results.append(result)

    with open(f"{base_dir}/lique_results.json", "w") as f:
        json.dump([result.__dict__ for result in results], f, indent=4)
