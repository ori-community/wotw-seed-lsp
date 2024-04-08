use tower_lsp::{
    jsonrpc::Result,
    lsp_types::{
        Hover, HoverContents, HoverParams, HoverProviderCapability, InitializeParams,
        InitializeResult, MarkedString, MessageType, Position, Range, ServerCapabilities,
    },
    Client, LanguageServer, LspService, Server,
};

struct Backend {
    client: Client,
}

#[tower_lsp::async_trait]
impl LanguageServer for Backend {
    async fn initialize(&self, _: InitializeParams) -> Result<InitializeResult> {
        Ok(InitializeResult {
            capabilities: ServerCapabilities {
                hover_provider: Some(HoverProviderCapability::Simple(true)),
                ..Default::default()
            },
            server_info: None,
        })
    }

    async fn hover(&self, _: HoverParams) -> Result<Option<Hover>> {
        self.client.log_message(MessageType::INFO, "hover").await;

        Ok(Some(Hover {
            contents: HoverContents::Scalar(MarkedString::String("teehee".to_string())),
            range: Some(Range::new(Position::new(0, 0), Position::new(10, 0))),
        }))
    }

    async fn shutdown(&self) -> Result<()> {
        Ok(())
    }
}

#[tokio::main]
async fn main() {
    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();

    let (service, socket) = LspService::build(|client| Backend { client }).finish();

    Server::new(stdin, stdout, socket).serve(service).await;
}
