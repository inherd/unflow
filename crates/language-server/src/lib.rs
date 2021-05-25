use std::collections::HashMap;

use lsp_text_document::FullTextDocument;
use serde_json::Value;
use tokio::sync::Mutex;
use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::*;
use tower_lsp::{Client, LanguageServer, LspService, Server};
#[derive(Debug)]
pub struct FileOffsets {
    files: Vec<Vec<usize>>,
}

#[derive(Debug)]
pub struct Hovers {
    offsets: FileOffsets,
    lookup: Vec<(usize, usize, String)>,
}

struct UnflowServer {
    client: Client,
    files: Mutex<HashMap<Url, FullTextDocument>>,
}

#[tokio::main(flavor = "current_thread")]
pub async fn start() {
    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();

    let (service, messages) = LspService::new(|client| UnflowServer {
        client,
        files: Mutex::new(HashMap::new()),
    });

    Server::new(stdin, stdout)
        .interleave(messages)
        .serve(service)
        .await;

    std::process::exit(1);
}

impl UnflowServer {
    async fn parse_file(&self, uri: Url) {
        if let Ok(_path) = uri.to_file_path() {
            let mut diags = vec![];
            let diagnostic = Diagnostic::default();
            diags.push(diagnostic);

            let res = self.client.publish_diagnostics(uri, diags, None);

            res.await;
        }
    }
}

#[tower_lsp::async_trait]
impl LanguageServer for UnflowServer {
    async fn initialize(&self, _: InitializeParams) -> Result<InitializeResult> {
        Ok(InitializeResult {
            server_info: None,
            capabilities: ServerCapabilities {
                text_document_sync: Some(TextDocumentSyncCapability::Kind(
                    TextDocumentSyncKind::Incremental,
                )),
                ..ServerCapabilities::default()
            },
        })
    }

    async fn initialized(&self, _: InitializedParams) {
        self.client
            .log_message(MessageType::Info, "server fucked!")
            .await;
    }

    async fn shutdown(&self) -> Result<()> {
        Ok(())
    }

    async fn did_change_workspace_folders(&self, _: DidChangeWorkspaceFoldersParams) {
        self.client
            .log_message(MessageType::Info, "workspace folders changed!")
            .await;
    }

    async fn did_change_configuration(&self, _: DidChangeConfigurationParams) {
        self.client
            .log_message(MessageType::Info, "configuration changed!")
            .await;
    }

    async fn did_change_watched_files(&self, _: DidChangeWatchedFilesParams) {
        self.client
            .log_message(MessageType::Info, "watched files have changed!")
            .await;
    }

    async fn execute_command(&self, _: ExecuteCommandParams) -> Result<Option<Value>> {
        self.client
            .log_message(MessageType::Info, "command executed!")
            .await;
        Ok(None)
    }

    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        let TextDocumentItem {
            uri,
            language_id,
            version,
            text,
        } = params.text_document;
        self.files.lock().await.insert(
            uri.clone(),
            FullTextDocument::new(uri, language_id, version as i64, text),
        );
    }

    async fn did_change(&self, params: DidChangeTextDocumentParams) {
        let DidChangeTextDocumentParams {
            content_changes,
            text_document,
        } = params;
        if let Some(document) = self.files.lock().await.get_mut(&text_document.uri) {
            let changes = content_changes
                .into_iter()
                .map(|change| {
                    let range = change.range.and_then(|range| {
                        Some(lsp_types::Range {
                            start: lsp_types::Position::new(
                                range.start.line as u32,
                                range.start.character as u32,
                            ),
                            end: lsp_types::Position::new(
                                range.end.line as u32,
                                range.end.character as u32,
                            ),
                        })
                    });
                    lsp_types::TextDocumentContentChangeEvent {
                        range,
                        range_length: change.range_length.and_then(|v| Some(v as u32)),
                        text: change.text,
                    }
                })
                .collect();
            document.update(changes, text_document.version as i64);
        }
    }

    async fn did_save(&self, params: DidSaveTextDocumentParams) {
        if let Some(document) = self.files.lock().await.get(&params.text_document.uri) {
            // 从这里直接拿到编辑uri 对应的text string, 可以做一些具体的parsing,目前只是输出到 client 端的console
            self.client.log_message(MessageType::Info, format!("{}", document.rope.to_string())).await;
        }
    }

    async fn did_close(&self, params: DidCloseTextDocumentParams) {
        let uri = params.text_document.uri;
        self.files.lock().await.remove(&uri);
    }

    async fn completion(&self, _: CompletionParams) -> Result<Option<CompletionResponse>> {
        Ok(None)
    }
}
