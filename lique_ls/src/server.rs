use lique_core::lints;
use lsp_types::{
    notification::{DidChangeTextDocument, Notification},
    Diagnostic, DiagnosticRelatedInformation, DiagnosticSeverity, DidChangeTextDocumentParams,
    Location, PublishDiagnosticsParams, Uri,
};
use oq3_semantics::syntax_to_semantics;
use oq3_source_file::SourceTrait;
use serde_wasm_bindgen::{from_value, to_value};
use wasm_bindgen::prelude::*;

use crate::locator::Locator;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

fn console_log(s: &str) {
    log(&format!("lique-ls: {}", s));
}

#[derive(Clone)]
#[wasm_bindgen]
pub struct Server {
    send_diagnostics_js: js_sys::Function,
}

#[wasm_bindgen]
impl Server {
    #[wasm_bindgen(constructor)]
    pub fn new(send_diagnostics_js: js_sys::Function) -> Self {
        Self {
            send_diagnostics_js,
        }
    }

    #[wasm_bindgen(js_name=onNotification)]
    pub fn on_notification(&self, method: &str, params: JsValue) {
        console_log(&format!("Received notification: {}", method));
        match method {
            DidChangeTextDocument::METHOD => {
                let params = from_value::<DidChangeTextDocumentParams>(params).unwrap();
                let uri = params.text_document.uri;
                let source = params.content_changes[0].text.clone();
                let locator = Locator::read_string(&source);
                let result =
                    syntax_to_semantics::parse_source_string(source, None, None::<&[String]>);
                let diagnostics = vec![
                    lints::measurement_twice::lint_measurement_twice(
                        result.syntax_result().syntax_ast().tree().statements(),
                    ),
                    lints::op_after_measurement::lint_op_after_measurement(
                        result.syntax_result().syntax_ast().tree().statements(),
                    ),
                ];
                let diagnostics = diagnostics
                    .into_iter()
                    .flatten()
                    .map(|diag| self.convert_diagnostic(diag, &uri, &locator))
                    .collect::<Vec<_>>();
                self.send_diagnostics(uri, diagnostics);
            }
            _ => {}
        }
    }

    fn convert_diagnostic(
        &self,
        diagnostic: lique_core::Diagnostic,
        uri: &Uri,
        locator: &Locator,
    ) -> Diagnostic {
        // let locator = self.file_locators.get(uri).unwrap();
        let range = diagnostic.range_zero_indexed;
        Diagnostic::new(
            locator.locate(range),
            Some(DiagnosticSeverity::WARNING),
            None,
            Some(diagnostic.message),
            "lique".to_string(),
            Some(
                diagnostic
                    .related_informations
                    .into_iter()
                    .map(|info| DiagnosticRelatedInformation {
                        location: Location {
                            uri: uri.clone(),
                            range: locator.locate(info.range_zero_indexed.clone()),
                        },
                        message: info.message,
                    })
                    .collect(),
            ),
            None,
        )
    }

    fn send_diagnostics(&self, uri: Uri, diagnostics: Vec<Diagnostic>) {
        let this = JsValue::null();
        let params = PublishDiagnosticsParams {
            uri,
            version: None,
            diagnostics,
        };
        let params = to_value(&params).unwrap();
        if let Err(e) = self.send_diagnostics_js.call1(&this, &params) {
            console_log(&format!("Error sending diagnostics: {:?}", e));
        }
    }
}
