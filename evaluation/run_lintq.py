from dataclasses import dataclass
import json
import subprocess
from concurrent.futures import ProcessPoolExecutor, as_completed
import os

from dataset.common import DatasetCase


@dataclass
class LiqueResult:
    expected_rule_id: str
    executed_rule_id: str
    found: bool
    file: str


def build_linter():
    process = subprocess.run(["cargo", "build", "-p", "lique_linter", "--release"])
    return process.returncode


def run_linter(file_path, rule):
    process = subprocess.run(
        [
            "./target/release/lique_linter",
            file_path,
            "--enabled-rules",
            rule,
        ],
    )
    return process.returncode


if __name__ == "__main__":
    base_dir = "./evaluation"
    with open(f"{base_dir}/dataset/dataset.json", "r") as f:
        dataset = json.load(f)
    dataset = [DatasetCase(**case) for case in dataset]

    if code := build_linter():
        print(f"Failed to build linter: {code}")
        exit(code)

    results = []
    num_cpus = os.cpu_count()
    with ProcessPoolExecutor(max_workers=num_cpus) as executor:
        future_to_case = {
            executor.submit(
                run_linter, f"{base_dir}/dataset/qasm/{case.file_name}.qasm", rule
            ): (case, rule)
            for rule in ["double-measurement", "operation-after-measurement"]
            for case in dataset
        }

        for future in as_completed(future_to_case):
            case, rule = future_to_case[future]
            returncode = future.result()
            result = LiqueResult(
                expected_rule_id=case.rule_id,
                executed_rule_id=f"ql-{rule}",
                found=returncode == 1,
                file=case.file_name,
            )
            results.append(result)

    with open(f"{base_dir}/lique_results.json", "w") as f:
        # Sort results by `file`
        results = sorted(results, key=lambda x: x.file)
        json.dump([result.__dict__ for result in results], f, indent=4)
