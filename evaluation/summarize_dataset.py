
import csv
import json


def summarize() -> None:
    with open("evaluation/dataset.json", "r") as f:
        dataset = json.load(f)

    counts = {}
    for target in dataset:
        rule = target["rule_id"]
        if counts.get(rule) is None:
            counts[rule] = 0
        counts[rule] += 1

    with open("evaluation/results/dataset_summary.csv", "w") as f:
        writer = csv.writer(f)
        writer.writerow(["rule", "count"])
        for rule, count in counts.items():
            writer.writerow([rule, count])


if __name__ == "__main__":
    summarize()
