import argparse
import json
import os
import shutil
import subprocess
import timeit


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


def run_lintq(rule: str) -> int:
    current_dir = os.getcwd()
    process = subprocess.run([
        "docker", "run",
        "-v", f"{current_dir}/evaluation/:/home/codeql/project/data",
        "-it", "--rm", "lintq",
        "codeql", "database", "analyze", "--rerun", "--format=sarifv2.1.0", "--threads=10",
        f"--output=data/lintq/lintq_results.json",
        "--", f"data/lintq/codeql_db", f"data/lintq/{rule}.qls",
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


def create_database_each(target: str) -> int:
    if not os.path.exists("evaluation/data/lintq/codeql_db"):
        os.makedirs("evaluation/data/lintq/codeql_db")
    current_dir = os.getcwd()
    process = subprocess.run([
        "docker", "run",
        "-v", f"{current_dir}/evaluation/:/home/codeql/project/data",
        "-it", "--rm", "lintq",
        "codeql", "database", "create", f"data/lintq/codeql_db",
        "--language=python", "--source-root", f"data/dataset/separated/{target}",
        "--overwrite"
    ])
    return process.returncode


def compile_query_each(rule: str) -> int:
    compile_cache = "data/lintq/query_cache"
    # Remove the cache in Docker container to avoid permission issue
    process = subprocess.run([
        "docker", "run",
        "-v", f"{os.getcwd()}/evaluation/:/home/codeql/project/data",
        "-it", "--rm", "lintq", "rm", "-rf", compile_cache
    ])
    if process.returncode != 0:
        return process.returncode

    current_dir = os.getcwd()
    process = subprocess.run([
        "docker", "run",
        "-v", f"{current_dir}/evaluation/:/home/codeql/project/data",
        "-it", "--rm", "lintq",
        "codeql", "query", "compile", "--compilation-cache", f"data/lintq/query_cache", f"data/lintq/{rule}.qls"
    ])
    return process.returncode


def run_lintq_each(rule: str) -> int:
    current_dir = os.getcwd()
    process = subprocess.run([
        "docker", "run",
        "-v", f"{current_dir}/evaluation/:/home/codeql/project/data",
        "-it", "--rm", "lintq",
        "codeql", "database", "analyze", "--rerun", "--format=sarifv2.1.0", "--threads=10",
        f"--output=data/lintq/lintq_perf_results.json",
        "--compilation-cache", f"data/lintq/query_cache",
        "--", f"data/lintq/codeql_db", f"data/lintq/{rule}.qls",
    ])
    return process.returncode


def run_lintq_perf() -> None:
    result = {}
    database_time = timeit.timeit(lambda: create_database(), number=1)
    result["database_time"] = database_time
    for rule in ["conditional_without_measurement", "constant_classic_bit", "double_measurement", "operation_after_measurement", "oversized_circuit", "unmeasurable_qubits"]:
        compile_time = timeit.timeit(lambda: compile_query_each(rule), number=1)
        query_time = timeit.timeit(lambda: run_lintq_each(rule), number=1)
        result[rule] = {"compile_time": compile_time, "query_time": query_time}

    with open("evaluation/lintq_perf.json", "w") as f:
        json.dump(result, f, indent=4)


if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument("--overwrite-db", action="store_true")
    parser.add_argument("--perf", action="store_true")

    args = parser.parse_args()

    evaluation_dir = "./evaluation"
    dataset_dir = f"{evaluation_dir}/dataset"
    codeql_db_dir = f"{dataset_dir}/codeql_db"
    python_dir = f"{dataset_dir}/python"

    if args.perf:
        run_lintq_perf()
        exit(0)
    if args.overwrite_db:
        if code := create_database():
            print(f"Failed to create database")
            exit(code)
    if code := run_lintq("all"):
        print(f"Failed to run lintq")
        exit(code)
