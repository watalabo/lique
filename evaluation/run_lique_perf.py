import csv
import os
import subprocess
import timeit


def run_lique_each(rule: str | None, target: str) -> None:
    args = [
        "cargo", "run", "-p", "lique_linter", "--release",
        "--", "--source-file", f"evaluation/dataset/python/{target}.py",
        "--source-map", f"evaluation/dataset/source_map/{target}.json",
        f"evaluation/dataset/qasm/{target}.qasm"
    ]
    if rule is not None:
        args.append("--enabled-rules")
        args.append(rule)
    process = subprocess.run(args, stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL)
    return process.returncode


def run_lique_perf() -> None:
    result = []
    rules = ["all", "conditional-without-measurement", "double-measurement", "operation-after-measurement", "oversized-circuit", "unmanipulated-qubits", "unmeasurable-qubits"]
    for rule in rules:
        execution_time = 0
        for target in os.listdir("evaluation/dataset/python/"):
            if target == "__pycache__":
                continue
            # Extract the filename without extension
            target = os.path.splitext(os.path.basename(target))[0]
            if rule == "all":
                rule_ = None
            execution_time += timeit.timeit(lambda: run_lique_each(rule_, target), number=1)
        rule = rule.replace("-", "_")
        result.append([rule, execution_time, 0, 0])

    with open("evaluation/results/lique_perf.csv", "w") as f:
        writer = csv.writer(f)
        writer.writerow(["rule", "query_time", "database_time", "compile_time"])
        writer.writerows(result)


if __name__ == "__main__":
    run_lique_perf()
