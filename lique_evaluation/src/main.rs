mod calculate_metrics;
mod run_lique;

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
        calculate_metrics::calculate_metrics_lique();
    }
    Ok(())
}
