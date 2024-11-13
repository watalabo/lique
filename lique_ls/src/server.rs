use rustpython_parser::{ast::Fold, source_code::RandomLocator};
use std::collections::HashMap;

use lique_core::{lints, SourceCode};
use log::{debug, info};
use lsp_types::{
    notification::{DidSaveTextDocument, Notification, PublishDiagnostics},
    request::{Initialize, Request},
    Diagnostic, DiagnosticOptions, DiagnosticServerCapabilities, InitializeResult, Position,
    PublishDiagnosticsParams, Range, SemanticTokensParams, ServerCapabilities,
    TextDocumentSyncCapability, TextDocumentSyncKind,
};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
};

use crate::protocol::{
    NotificationMessage, OutgoingMessage, RpcMessageRequest, RpcMessageResponse,
};

type Header = HashMap<String, String>;

#[derive(Clone)]
pub struct Server;

impl Server {
    pub async fn start(self, port: u16) -> anyhow::Result<()> {
        let listener = TcpListener::bind(("127.0.0.1", port)).await?;
        info!("Listening on port {}", port);
        loop {
            let (stream, _) = listener.accept().await?;
            debug!("Accepted connection");
            self.clone().handle(stream).await?;
        }
    }

    async fn handle(self, stream: TcpStream) -> anyhow::Result<()> {
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
        &self,
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
                let code = SourceCode::read_from_path(path);
                let mut locator = RandomLocator::new(&code);
                if let Ok(module) = code.parse() {
                    let module = locator.fold(module).unwrap();
                    let diags = vec![
                        lints::measurement_twice::lint_measurement_twice(&module.body),
                        lints::measurement_twice::lint_measurement_twice(&module.body),
                    ]
                    .into_iter()
                    .flatten()
                    .map(convert_diagnostic)
                    .collect::<Vec<_>>();
                    let notification =
                        NotificationMessage::new::<PublishDiagnostics>(PublishDiagnosticsParams {
                            uri,
                            version: None,
                            diagnostics: diags,
                        })?;
                    Ok(Some(OutgoingMessage::NotificationMessage(notification)))
                } else {
                    Ok(None)
                }
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
}

fn convert_diagnostic(diagnostic: lique_core::Diagnostic) -> Diagnostic {
    let range = diagnostic.range;
    let start = range.start;
    let end = range.end.unwrap();
    Diagnostic::new_simple(
        Range {
            start: Position {
                line: start.row.to_zero_indexed_usize() as u32,
                character: start.column.to_zero_indexed_usize() as u32,
            },
            end: Position {
                line: end.row.to_zero_indexed_usize() as u32,
                character: end.column.to_zero_indexed_usize() as u32,
            },
        },
        diagnostic.message,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_payload() {
        let packet = "Content-Length: 57\r\nContent-Type: application/json\r\n\r\n";
        let header = Server::parse_headers(&packet).unwrap();
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
        assert!(Server::parse_headers(&packet).is_err());
    }
}
