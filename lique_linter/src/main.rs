use ariadne::{ColorGenerator, Label, Report, ReportKind, Source};
use lique_core::run_lints;
use oq3_semantics::syntax_to_semantics;
use oq3_source_file::SourceTrait;

fn main() {
    let path = "./test.qasm";
    let result = syntax_to_semantics::parse_source_file(path, None::<&[String]>);
    let source_text = result.syntax_result().syntax_ast().tree().to_string();
    let mut colors = ColorGenerator::new();
    let color = colors.next();
    for diag in run_lints(result) {
        Report::build(ReportKind::Warning, (path, diag.range_zero_indexed.clone()))
            .with_message(diag.message)
            .with_labels(diag.related_informations.iter().map(|info| {
                Label::new((path, info.range_zero_indexed.clone()))
                    .with_message(info.message.clone())
                    .with_color(color)
            }))
            .finish()
            .print((path, Source::from(&source_text)))
            .unwrap();
    }
}
