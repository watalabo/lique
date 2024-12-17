import argparse
import os
import subprocess


def create_database() -> int:
    if not os.path.exists("evaluation/data/lintq/codeql_db"):
        os.makedirs("evaluation/data/lintq/codeql_db")
    current_dir = os.getcwd()
    args = [
        "docker", "run", 
        "-v", f"{current_dir}/evaluation/:/home/codeql/project/data", 
        "-it", "--rm", "lintq", 
        "codeql", "database", "create", f"data/lintq/codeql_db", 
        "--language=python", "--source-root", "data/dataset/python", "--overwrite"
    ]
    process = subprocess.run(args)
    return process.returncode


def run_lintq() -> int:
    current_dir = os.getcwd()
    process = subprocess.run([
        "docker", "run",
        "-v", f"{current_dir}/evaluation/:/home/codeql/project/data",
        "-it", "--rm", "lintq",
        "codeql", "database", "analyze", "--rerun", "--format=sarifv2.1.0", "--threads=10",
        f"--output=data/lintq/lintq_results.json",
        "--", f"data/lintq/codeql_db", "data/lintq/lintq.qls",
    ])
    if process.returncode != 0:
        return process.returncode

    # Separate this command to avoid codeql database analyse parse this as inputs
    process = subprocess.run([
        "docker", "run",
        "-v", f"{current_dir}/evaluation/:/home/codeql/project/data",
        "-it", "--rm", "lintq",
        "chown", f"{os.getuid()}:{os.getgid()}", f"/home/codeql/project/data/lintq/lintq_results.json"
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

    if args.overwrite_db:
        if code := create_database():
            print(f"Failed to create database")
            exit(code)
    if code := run_lintq():
        print(f"Failed to run lintq")
        exit(code)
