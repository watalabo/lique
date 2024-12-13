import os
import shutil
import polars as pl

if __name__ == "__main__":
    df = pl.read_csv("lintq/bug_reports/Inspection_LintQ_on_LintQ_dataset.csv")

    rule_ids = df.get_column("rule_id").unique().to_list()
    for rule_id in rule_ids:
        for label_resolution in ["TP", "FP"]:
            files = df.filter(pl.col("rule_id") == rule_id).filter(pl.col("label_resolution") == label_resolution).get_column("file").to_list()
            dst = f"evaluation/dataset/unresolved/{rule_id}/{label_resolution}/"
            os.makedirs(dst, exist_ok=True)
            for file in files:
                src = f"lintq/data/datasets/exp_v08/files_selected/{file}"
                shutil.copy(src, dst)
