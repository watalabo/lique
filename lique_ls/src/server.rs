use std::collections::HashMap;

use lique_core::lints;
use log::{debug, info};
use lsp_types::{
    notification::{DidSaveTextDocument, Notification, PublishDiagnostics},
    request::{Initialize, Request},
    Diagnostic, DiagnosticOptions, DiagnosticRelatedInformation, DiagnosticServerCapabilities,
    DiagnosticSeverity, InitializeResult, Location, PublishDiagnosticsParams, SemanticTokensParams,
    ServerCapabilities, TextDocumentSyncCapability, TextDocumentSyncKind, Uri,
};
use oq3_semantics::syntax_to_semantics;
use oq3_source_file::SourceTrait;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
};

use crate::{
    locator::Locator,
    protocol::{NotificationMessage, OutgoingMessage, RpcMessageRequest, RpcMessageResponse},
};

type Header = HashMap<String, String>;

#[derive(Clone)]
pub struct Server {
    file_locators: HashMap<Uri, Locator>,
}

impl Server {
    pub fn new() -> Self {
        Self {
            file_locators: Default::default(),
        }
    }

    pub async fn start(self, port: u16) -> anyhow::Result<()> {
        let listener = TcpListener::bind(("127.0.0.1", port)).await?;
        info!("Listening on port {}", port);
        loop {
            let (stream, _) = listener.accept().await?;
            debug!("Accepted connection");
            self.clone().handle(stream).await?;
        }
    }

    async fn handle(mut self, stream: TcpStream) -> anyhow::Result<()> {
        let (mut reader, mut writer) = tokio::io::split(stream);
        tokio::spawn(async move {
            loop {
                if let Err(e) = {
                    let headers = Self::read_headers(&mut reader).await?;
                    let content_length = headers
                        .get("Content-Length")
                        .unwrap()
                        .parse::<usize>()
                        .unwrap();
                    let mut content = vec![0; content_length];
                    reader.read_exact(&mut content).await?;
                    let content = String::from_utf8(content)?;
                    let req = serde_json::from_str::<RpcMessageRequest>(&content)?;
                    debug!("{:?}", &req);

                    let Some(res) = self.handle_request(req).await? else {
                        continue;
                    };
                    let res = match res {
                        OutgoingMessage::RpcMessageResponse(res) => serde_json::to_string(&res)?,
                        OutgoingMessage::NotificationMessage(res) => serde_json::to_string(&res)?,
                    };
                    let res = format!(
                        "Content-Length: {}\r\nContent-Type: application/json\r\n\r\n{}",
                        res.len(),
                        res
                    );
                    writer.write_all(res.as_bytes()).await?;
                    anyhow::Ok(())
                } {
                    eprintln!("Error reading from stream: {:?}", e);
                    break;
                }
            }
            anyhow::Ok(())
        });
        Ok(())
    }

    async fn handle_request(
        &mut self,
        req: RpcMessageRequest,
    ) -> anyhow::Result<Option<OutgoingMessage>> {
        match req.method.as_str() {
            Initialize::METHOD => Ok(Some(OutgoingMessage::RpcMessageResponse(
                RpcMessageResponse::new(
                    req.id,
                    InitializeResult {
                        server_info: None,
                        capabilities: ServerCapabilities {
                            text_document_sync: Some(TextDocumentSyncCapability::Kind(
                                TextDocumentSyncKind::FULL,
                            )),
                            declaration_provider: None,
                            definition_provider: None,
                            semantic_tokens_provider: None,
                            diagnostic_provider: Some(DiagnosticServerCapabilities::Options(
                                DiagnosticOptions {
                                    identifier: None,
                                    inter_file_dependencies: false,
                                    workspace_diagnostics: false,
                                    work_done_progress_options: Default::default(),
                                },
                            )),
                            position_encoding: None,
                            selection_range_provider: None,
                            hover_provider: None,
                            completion_provider: None,
                            signature_help_provider: None,
                            type_definition_provider: None,
                            implementation_provider: None,
                            references_provider: None,
                            document_highlight_provider: None,
                            document_symbol_provider: None,
                            workspace_symbol_provider: None,
                            code_action_provider: None,
                            code_lens_provider: None,
                            document_formatting_provider: None,
                            document_range_formatting_provider: None,
                            document_on_type_formatting_provider: None,
                            rename_provider: None,
                            document_link_provider: None,
                            color_provider: None,
                            folding_range_provider: None,
                            execute_command_provider: None,
                            workspace: None,
                            call_hierarchy_provider: None,
                            moniker_provider: None,
                            linked_editing_range_provider: None,
                            inline_value_provider: None,
                            inlay_hint_provider: None,
                            notebook_document_sync: None,
                            experimental: None,
                        },
                    },
                )?,
            ))),
            DidSaveTextDocument::METHOD => {
                let params = serde_json::from_value::<SemanticTokensParams>(req.params)?;
                let uri = params.text_document.uri;
                let path = uri.path().to_string();
                debug!("Handling save for {}", path);
                if !self.file_locators.contains_key(&uri) {
                    let locator = Locator::read_file(&path);
                    self.file_locators.insert(uri.clone(), locator);
                }
                let result = syntax_to_semantics::parse_source_file(&path, None::<&[String]>);
                let diags = vec![
                    lints::measurement_twice::lint_measurement_twice(
                        result.syntax_result().syntax_ast().tree().statements(),
                    ),
                    lints::op_after_measurement::lint_op_after_measurement(
                        result.syntax_result().syntax_ast().tree().statements(),
                    ),
                ]
                .into_iter()
                .flatten()
                .map(|diag| self.convert_diagnostic(diag, &uri))
                .collect::<Vec<_>>();
                debug!("#diags: {}", diags.len());
                let notification =
                    NotificationMessage::new::<PublishDiagnostics>(PublishDiagnosticsParams {
                        uri,
                        version: None,
                        diagnostics: diags,
                    })?;
                Ok(Some(OutgoingMessage::NotificationMessage(notification)))
            }
            _ => Ok(None),
        }
    }

    async fn read_headers(reader: &mut (impl AsyncReadExt + Unpin)) -> anyhow::Result<Header> {
        let mut data = vec![];
        while !data.ends_with("\r\n\r\n".as_bytes()) {
            let mut buf = vec![0; 1];
            let n = reader.read(&mut buf).await?;
            if n == 0 {
                anyhow::bail!("UnexpectedEof");
            }

            data.push(buf[0]);
        }
        let data = String::from_utf8(data)?;
        Self::parse_headers(&data)
    }

    fn parse_headers(headers: &str) -> anyhow::Result<Header> {
        let headers = headers
            .split("\r\n")
            .filter(|line| !line.is_empty())
            .map(|line| {
                let mut parts = line.splitn(2, ": ");
                (
                    parts.next().unwrap().to_string(),
                    parts.next().unwrap().to_string(),
                )
            })
            .collect::<HashMap<_, _>>();
        if headers.contains_key("Content-Length") {
            Ok(headers)
        } else {
            Err(anyhow::anyhow!("No Content-Length"))
        }
    }

    fn convert_diagnostic(&self, diagnostic: lique_core::Diagnostic, uri: &Uri) -> Diagnostic {
        let locator = self.file_locators.get(uri).unwrap();
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_payload() {
        let packet = "Content-Length: 57\r\nContent-Type: application/json\r\n\r\n";
        let header = Server::parse_headers(packet).unwrap();
        assert_eq!(
            header,
            vec![
                ("Content-Length".to_string(), "57".to_string()),
                ("Content-Type".to_string(), "application/json".to_string())
            ]
            .into_iter()
            .collect()
        );
    }

    #[test]
    fn test_read_payload_without_content_length() {
        let packet = "Content-Type: application/json\r\n\r\n";
        assert!(Server::parse_headers(packet).is_err());
    }
}
