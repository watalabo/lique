use ariadne::{ColorGenerator, Label, Report, ReportKind, Source};
use clap::Parser;
use lique_core::byte_offset::{ByteOffsetError, ByteOffsetLocator};
use lique_core::locate_in_source_file;
use lique_core::source_map::SourceMap;
use lique_core::{rule::Rule, run_lints};
use oq3_semantics::syntax_to_semantics;
use oq3_source_file::SourceTrait;
use std::fs::File;
use std::io::Read;
use std::process::ExitCode;

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
}

fn enumerate_rules(command: &Command) -> Vec<Rule> {
    if !command.enabled_rules.is_empty() {
        command.enabled_rules.clone()
    } else {
        Rule::all()
    }
}

fn main() -> Result<ExitCode, Box<dyn std::error::Error>> {
    let command = Command::parse();

    let path = &command.file;
    let result = syntax_to_semantics::parse_source_file(path, None::<&[String]>);
    let qasm_source_text = result.syntax_result().syntax_ast().tree().to_string();
    let mut colors = ColorGenerator::new();
    let color = colors.next();
    let rules = enumerate_rules(&command);

    let diagnostics = run_lints(result, &rules);
    let is_diagnostics_empty = diagnostics.is_empty();
    match (command.source_map, command.source_file) {
        (Some(source_map_path), Some(source_file_path)) => {
            let mut source_map_file = File::open(source_map_path)?;
            let mut source_map_content = String::new();
            source_map_file.read_to_string(&mut source_map_content)?;
            let source_map: SourceMap = serde_json::from_str(&source_map_content)?;

            let source_file_locator = ByteOffsetLocator::read_from_file(&source_file_path)?;

            for diag in diagnostics {
                let converted_range = locate_in_source_file(
                    &diag.range_zero_indexed,
                    &source_map,
                    &source_file_locator,
                )?;

                let labels = diag
                    .related_informations
                    .iter()
                    .map(|info| {
                        let converted_range = locate_in_source_file(
                            &info.range_zero_indexed,
                            &source_map,
                            &source_file_locator,
                        )?;
                        Ok(Label::new((&source_file_path, converted_range))
                            .with_message(info.message.clone())
                            .with_color(color))
                    })
                    .collect::<Result<Vec<_>, ByteOffsetError>>()?;
                Report::build(ReportKind::Warning, (&source_file_path, converted_range))
                    .with_message(diag.message)
                    .with_labels(labels)
                    .finish()
                    .print((
                        &source_file_path,
                        Source::from(&source_file_locator.contents),
                    ))
                    .unwrap();
            }
        }
        (None, None) => {
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
