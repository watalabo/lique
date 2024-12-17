mod calculate_metrics;
mod run_lique;
mod types;

use std::path::Path;

use clap::Parser;

#[derive(Parser)]
struct Cli {
    #[arg(long)]
    lique: bool,
    #[arg(long)]
    metrics: bool,
}

fn main() -> anyhow::Result<()> {
    let args = Cli::parse();
    if args.lique {
        run_lique::run_lique()?;
    }
    if args.metrics {
        let evaluation_dir = Path::new("./evaluation");
        let dataset_file_path = evaluation_dir.join("dataset.json");
        let lique_results_file_path = evaluation_dir.join("lique_results.json");
        let lique_metrics_file_path = evaluation_dir.join("lique_metrics.json");
        calculate_metrics::calculate_metrics(
            dataset_file_path,
            lique_results_file_path,
            lique_metrics_file_path,
        );
        let dataset_file_path = evaluation_dir.join("dataset.json");
        let lintq_results_file_path = evaluation_dir.join("lintq_results.json");
        let lintq_metrics_file_path = evaluation_dir.join("lintq_metrics.json");
        calculate_metrics::calculate_metrics(
            dataset_file_path,
            lintq_results_file_path,
            lintq_metrics_file_path,
        );
    }
    Ok(())
}
