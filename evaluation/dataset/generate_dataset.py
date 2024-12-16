import os
import re
from dataclasses import asdict
from json import dump

from common import DatasetCase


def parse_comment(comment: str) -> list[str]:
    comment_substr = comment.split('#', 1)[-1]
    return re.findall(r'ql-[\w-]+', comment_substr)


def parse_file(file_path: str) -> list[DatasetCase]:
    dataset_cases = []
    with open(file_path, 'r') as f:
        file_name = os.path.splitext(os.path.basename(file_path))[0]
        for line_number, line in enumerate(f):
            rule_ids = parse_comment(line)
            for rule_id in rule_ids:
                dataset_cases.append(DatasetCase(file_name, line_number, rule_id))
    return dataset_cases


def parse_files(python_dir: str) -> list[DatasetCase]:
    dataset_cases = []
    for root, _, files in os.walk(python_dir):
        for file in files:
            if file.endswith(".py"):
                file_path = os.path.join(root, file)
                dataset_cases.extend(parse_file(file_path))
    return dataset_cases


if __name__ == "__main__":
    python_dir = "./evaluation/dataset/python"
    dataset_cases = parse_files(python_dir)
    dataset_cases.sort(key=lambda dataset_case: (dataset_case.file_name, dataset_case.line_number, dataset_case.rule_id))
    dataset_cases = [asdict(dataset_case) for dataset_case in dataset_cases]
    with open("./evaluation/dataset.json", "w") as f:
        dump(dataset_cases, f, indent=4)
