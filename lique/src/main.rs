use std::fs::File;
use std::io::Read;
use std::process::ExitCode;

use ariadne::{ColorGenerator, Label, Report, ReportKind, Source};
use clap::Parser;
use lique_core::byte_offset::ByteOffsetLocator;
use lique_core::source_map::SourceMap;
use lique_core::{resolve_qasm_range, Diagnostic};
use lique_core::{rule::Rule, run_lints};
use oq3_semantics::syntax_to_semantics;
use oq3_source_file::SourceTrait;

#[derive(Parser)]
struct Command {
    #[arg(help = "Path to the QASM file")]
    file: String,
    #[arg(
        long,
        required = false,
        requires = "source_file",
        help = "Path to the source map file"
    )]
    source_map: Option<String>,
    #[arg(
        long,
        required = false,
        requires = "source_map",
        help = "Path to the Python source file"
    )]
    source_file: Option<String>,
    #[arg(long, value_parser = clap::value_parser!(Rule), value_delimiter = ',', help = "Enable specific lint rules")]
    enabled_rules: Vec<Rule>,
    #[arg(
        long,
        required = false,
        help = "Specify the JSON output file for diagnostics"
    )]
    json: Option<String>,
}

fn main() -> anyhow::Result<ExitCode> {
    let command = Command::parse();

    let path = &command.file;
    let parsed_qasm = syntax_to_semantics::parse_source_file(path, None::<&[String]>);
    let qasm_source_text = parsed_qasm.syntax_result().syntax_ast().tree().to_string();
    let rules = if !command.enabled_rules.is_empty() {
        command.enabled_rules.clone()
    } else {
        Rule::all()
    };

    let diagnostics = run_lints(parsed_qasm, &rules);
    let is_diagnostics_empty = diagnostics.is_empty();
    match (command.source_map, command.source_file) {
        (Some(source_map_path), Some(source_file_path)) => {
            print_diagnostics_by_source_map(&source_map_path, &source_file_path, diagnostics)?;
        }
        (None, None) => {
            for diag in diagnostics {
                let mut colors = ColorGenerator::new();
                let color = colors.next();
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
                .print((&path, Source::from(&qasm_source_text)))
                .unwrap();
            }
        }
        _ => unreachable!("Clap ensures both source_map and source_file exist"),
    }

    if is_diagnostics_empty {
        Ok(ExitCode::SUCCESS)
    } else {
        Ok(ExitCode::FAILURE)
    }
}

fn print_diagnostics_by_source_map(
    source_map_path: &str,
    source_file_path: &str,
    diagnostics: Vec<Diagnostic>,
) -> anyhow::Result<()> {
    let mut source_map_file = File::open(source_map_path)?;
    let mut source_map_content = String::new();
    source_map_file.read_to_string(&mut source_map_content)?;
    let source_map: SourceMap = serde_json::from_str(&source_map_content)?;

    let source_file_locator = ByteOffsetLocator::read_from_file(source_file_path)?;

    let mut colors = ColorGenerator::new();
    let color = colors.next();

    for diag in diagnostics {
        let labels = diag
            .related_informations
            .iter()
            .map(|info| {
                let source_range = resolve_qasm_range(&info.range_zero_indexed, &source_map)?;
                let source_file_range = source_file_locator.locate_line(source_range)?;
                Ok(Label::new((&source_file_path, source_file_range))
                    .with_message(info.message.clone())
                    .with_color(color))
            })
            .collect::<anyhow::Result<Vec<_>>>()?;

        let source_range = resolve_qasm_range(&diag.range_zero_indexed, &source_map)?;
        let source_range_bytes = source_file_locator.locate_line(source_range)?;
        Report::build(
            ReportKind::Warning,
            (&source_file_path, source_range_bytes.clone()),
        )
        .with_message(diag.message.clone())
        .with_labels(labels)
        .with_label(
            Label::new((&source_file_path, source_range_bytes))
                .with_message(diag.message)
                .with_color(color),
        )
        .finish()
        .print((
            &source_file_path,
            Source::from(&source_file_locator.contents_lines.concat()),
        ))
        .unwrap();
    }
    Ok(())
}
