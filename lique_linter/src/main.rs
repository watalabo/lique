use std::process::ExitCode;

use ariadne::{ColorGenerator, Label, Report, ReportKind, Source};
use clap::Parser;
use lique_core::{rule::Rule, run_lints};
use oq3_semantics::syntax_to_semantics;
use oq3_source_file::SourceTrait;

#[derive(Parser)]
struct Command {
    file: String,
    #[arg(long, help = "Eanble all lint rules")]
    all: bool,
    #[arg(long, value_parser = clap::value_parser!(Rule), value_delimiter = ',', help = "Enable specific lint rules. Prioritized over --all")]
    enabled_rules: Vec<Rule>,
}

fn enumerate_rules(command: &Command) -> Vec<Rule> {
    if !command.enabled_rules.is_empty() {
        command.enabled_rules.clone()
    } else {
        Rule::all()
    }
}

fn main() -> ExitCode {
    let command = Command::parse();

    let path = &command.file;
    let result = syntax_to_semantics::parse_source_file(path, None::<&[String]>);
    let source_text = result.syntax_result().syntax_ast().tree().to_string();
    let mut colors = ColorGenerator::new();
    let color = colors.next();
    let rules = enumerate_rules(&command);

    let diagnostics = run_lints(result, &rules);
    let is_diagnostics_empty = diagnostics.is_empty();
    for diag in diagnostics {
        Report::build(
            ReportKind::Warning,
            (&path, diag.range_zero_indexed.clone()),
        )
        .with_message(diag.message)
        .with_labels(diag.related_informations.iter().map(|info| {
            Label::new((&path, info.range_zero_indexed.clone()))
                .with_message(info.message.clone())
                .with_color(color)
        }))
        .finish()
        .print((&path, Source::from(&source_text)))
        .unwrap();
    }

    if is_diagnostics_empty {
        ExitCode::SUCCESS
    } else {
        ExitCode::from(1)
    }
}
