mod convert;
mod semantic_tokens;

use dashmap::DashMap;
use semantic_tokens::{semantic_tokens, semantic_tokens_legend};
use tower_lsp::{
    jsonrpc::{Error, Result},
    lsp_types::{
        DidChangeTextDocumentParams, DidOpenTextDocumentParams, InitializeParams, InitializeResult,
        SemanticTokens, SemanticTokensFullOptions, SemanticTokensOptions, SemanticTokensParams,
        SemanticTokensResult, SemanticTokensServerCapabilities, ServerCapabilities,
        TextDocumentSyncCapability, TextDocumentSyncKind, Url,
    },
    Client, LanguageServer, LspService, Server,
};
use wotw_seedgen_seed_language::ast::{self, Snippet};

struct Backend {
    client: Client,
    text_documents: DashMap<Url, String>,
}

impl Backend {
    fn new(client: Client) -> Self {
        Self {
            client,
            text_documents: Default::default(),
        }
    }
}

#[tower_lsp::async_trait]
impl LanguageServer for Backend {
    async fn initialize(&self, _: InitializeParams) -> Result<InitializeResult> {
        Ok(InitializeResult {
            capabilities: ServerCapabilities {
                text_document_sync: Some(TextDocumentSyncCapability::Kind(
                    TextDocumentSyncKind::INCREMENTAL,
                )),
                semantic_tokens_provider: Some(
                    SemanticTokensServerCapabilities::SemanticTokensOptions(
                        SemanticTokensOptions {
                            legend: semantic_tokens_legend(),
                            full: Some(SemanticTokensFullOptions::Bool(true)),
                            ..Default::default()
                        },
                    ),
                ),
                ..Default::default()
            },
            server_info: None,
        })
    }

    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        self.text_documents
            .insert(params.text_document.uri, params.text_document.text);
    }
    async fn did_change(&self, params: DidChangeTextDocumentParams) {
        if let Some(mut text_document) = self.text_documents.get_mut(&params.text_document.uri) {
            for content_change in params.content_changes {
                match content_change
                    .range
                    .and_then(|range| convert::range(range, &text_document))
                {
                    None => *text_document.value_mut() = content_change.text,
                    Some(range) => text_document.replace_range(range, &content_change.text),
                }
            }
        }
    }

    async fn semantic_tokens_full(
        &self,
        params: SemanticTokensParams,
    ) -> Result<Option<SemanticTokensResult>> {
        let source =
            self.text_documents
                .get(&params.text_document.uri)
                .ok_or(Error::invalid_params(format!(
                    "unknown text document {}",
                    params.text_document.uri
                )))?;

        let ast = ast::parse::<Snippet>(&source);
        let data = semantic_tokens(&source, ast.parsed);

        Ok(Some(SemanticTokensResult::Tokens(SemanticTokens {
            data,
            ..Default::default()
        })))
    }

    async fn shutdown(&self) -> Result<()> {
        Ok(())
    }
}

#[tokio::main]
async fn main() {
    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();

    let (service, socket) = LspService::build(|client| Backend::new(client)).finish();

    Server::new(stdin, stdout, socket).serve(service).await;
}
