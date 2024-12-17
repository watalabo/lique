import argparse
import os
import subprocess

from dataset.common import rules_all


def create_database(rule: str) -> int:
    current_dir = os.getcwd()
    args = [
        "docker", "run", 
        "-v", f"{current_dir}/evaluation/:/home/codeql/project/data", 
        "-it", "--rm", "lintq", 
        "codeql", "database", "create", f"data/lintq/{rule}/codeql_db", 
        "--language=python", "--source-root", "data/dataset/python", "--overwrite"
    ]
    process = subprocess.run(args)
    return process.returncode


def run_lintq(rule: str) -> int:
    current_dir = os.getcwd()
    process = subprocess.run([
        "docker", "run",
        "-v", f"{current_dir}/evaluation/:/home/codeql/project/data",
        "-v", f"{current_dir}/evaluation/lintq/{rule}/lintq.qls:/home/codeql/project/lintq.qls",
        "-it", "--rm", "lintq",
        "codeql", "database", "analyze", "--rerun", "--format=sarifv2.1.0", "--threads=10",
        f"--output=data/lintq/{rule}/lintq_results.json",
        "--", f"data/lintq/{rule}/codeql_db", "lintq.qls",
    ])
    if process.returncode != 0:
        return process.returncode

    # Separate this command to avoid codeql database analyse parse this as inputs
    process = subprocess.run([
        "docker", "run",
        "-v", f"{current_dir}/evaluation/:/home/codeql/project/data",
        "-it", "--rm", "lintq",
        "chown", f"{os.getuid()}:{os.getgid()}", f"/home/codeql/project/data/lintq/{rule}/lintq_results.json"
    ])
    return process.returncode


if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument("--overwrite-db", action="store_true")
    parser.add_argument("--rules", type=str)

    args = parser.parse_args()

    evaluation_dir = "./evaluation"
    dataset_dir = f"{evaluation_dir}/dataset"
    codeql_db_dir = f"{dataset_dir}/codeql_db"
    python_dir = f"{dataset_dir}/python"

    if args.rules is not None:
        rules = [s.strip() for s in args.rules.split(",")]
        if not all(rule in rules_all for rule in rules):
            print(f"Invalid rules: {rules}")
            exit(1)
    else:
        rules = rules_all
    for rule in rules:
        if args.overwrite_db:
            if code := create_database(rule):
                print(f"Failed to create database for rule {rule}")
                exit(code)
        if code := run_lintq(rule):
            print(f"Failed to run lintq for rule {rule}")
            exit(code)
