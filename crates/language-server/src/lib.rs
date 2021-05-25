use std::collections::HashMap;
use std::path::PathBuf;

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

#[derive(Debug)]
struct UnflowServer {
    client: Client,
    files: Mutex<HashMap<PathBuf, Hovers>>,
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
        Ok(InitializeResult::default())
    }

    async fn initialized(&self, _: InitializedParams) {
        self.client
            .log_message(MessageType::Info, "server initialized!")
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
        let uri = params.text_document.uri;

        self.parse_file(uri).await;
    }

    async fn did_change(&self, params: DidChangeTextDocumentParams) {
        let uri = params.text_document.uri;

        self.parse_file(uri).await;
    }

    async fn did_save(&self, params: DidSaveTextDocumentParams) {
        let uri = params.text_document.uri;

        self.parse_file(uri).await;
    }

    async fn did_close(&self, params: DidCloseTextDocumentParams) {
        let uri = params.text_document.uri;

        if let Ok(path) = uri.to_file_path() {
            self.files.lock().await.remove(&path);
        }
    }

    async fn completion(&self, _: CompletionParams) -> Result<Option<CompletionResponse>> {
        Ok(None)
    }
}
