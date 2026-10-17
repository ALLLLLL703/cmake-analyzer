// Script-generated test scaffolding by scripts/gen/lspally-schema/trait_fixture.py.
// Contains AI-generated test edits.
// Contains AI-generated documentation additions.

//! Provide the shared language-server fixture for duplex transport tests.
//! Supported hooks record events or exchange client messages; other hooks return unsupported.
//! No filesystem or subprocess access; the generator header describes the original scaffolding.

#![allow(unused_variables)]
use lspally::server::model::*;
use std::sync::{Arc, Mutex};
/// Shared event-recording server used by duplex integration tests.
/// [`ILanguageServer`] hooks exercise [`Context`] encoding, cancellation, and client callbacks.
/// [`super::session::Session`] owns the transport; constructing this value does no I/O.
/// The mutex permits the test driver to inspect ordered business events.
///
/// # Flow
/// ```text
/// [Session frames]         [test assertions]
///        |                        ^
///        v                        |
/// [ILanguageServer hooks] --> [events mutex]
///        |
///        v
/// [Context client callbacks]
/// ```
///
/// # Examples
/// Local fixture-module context:
/// ```text
/// let server = Server { events: Arc::new(Mutex::new(Vec::new())) };
/// ```
pub(crate) struct Server {
    /// Ordered event log shared with the session driver; hooks append, never clear it.
    /// Example: `server.events.lock().unwrap().push("observed".into())` appends an event;
    /// `server.events.lock().unwrap().len()` reads the count.
    pub(crate) events: Arc<Mutex<Vec<String>>>,
}
impl ILanguageServer for Server {
    /// Validate workspace capabilities and advertise UTF-8 with fixture server information. No I/O or event mutation.
    ///
    /// # Arguments
    /// - `self` `[in]`: fixture server, not modified.
    /// - `context` `[in]`: unused initialization context.
    /// - `params` `[in]`: initialization input; workspace capabilities must be present.
    ///
    /// # Returns
    /// `Ok(InitializeResult)` with UTF-8 encoding, server name fixture, and no version. No error is returned.
    ///
    /// # Panics
    /// If workspace capabilities are absent.
    ///
    /// # Examples
    /// Local fixture context with a prepared `Context` and `InitializeParams` payload:
    /// ```text
    /// let result = server.initialize(context, params).await;
    /// ```
    async fn initialize(
        &self,
        context: Context,
        params: InitializeParams,
    ) -> ServerResult<InitializeResult> {
        assert!(params.capabilities.workspace.is_some());
        let mut capabilities = ServerCapabilities::default();
        capabilities.position_encoding =
            Some(PositionEncodingKind(std::borrow::Cow::Borrowed("utf-8")));
        Ok(InitializeResult {
            capabilities,
            server_info: Some(ServerInfo {
                name: "fixture".into(),
                version: None,
            }),
        })
    }
    /// Accept shutdown without changing events or performing I/O.
    ///
    /// # Arguments
    /// - `self` `[in]`: fixture server, not modified.
    /// - `context` `[in]`: unused shutdown context.
    /// - `params` `[in]`: unused unit payload.
    ///
    /// # Returns
    /// `Ok(())` unconditionally; no error is returned.
    ///
    /// # Examples
    /// Local fixture context with a prepared `Context` and `()` payload:
    /// ```text
    /// let result = server.shutdown(context, params).await;
    /// ```
    async fn shutdown(&self, context: Context, params: ()) -> ServerResult<()> {
        Ok(())
    }
    /// Reject the completion hook as unsupported; discard its CompletionParams payload.
    /// Does not read/change events, send callbacks, or perform external I/O.
    ///
    /// # Arguments
    /// - `self` `[in]`: fixture server, not modified.
    /// - `context` `[in]`: unused request/notification context; no callbacks are made.
    /// - `params` `[in]`: consumed CompletionParams fixture input, deliberately ignored.
    ///
    /// # Returns
    /// `Err(ServerError::unsupported())` always; no success payload within `ServerResult<Option<CompletionResponse>>` is produced.
    ///
    /// # Errors
    /// Always returns the unsupported-method error.
    ///
    /// # Examples
    /// Local fixture context with a prepared `Context` and `CompletionParams` payload:
    /// ```text
    /// let result = server.completion(context, params).await;
    /// ```
    async fn completion(
        &self,
        context: Context,
        params: CompletionParams,
    ) -> ServerResult<Option<CompletionResponse>> {
        let _ = (context, params);
        Err(ServerError::unsupported())
    }
    /// Reject the resolve_completion_item hook as unsupported; discard its CompletionItem payload.
    /// Does not read/change events, send callbacks, or perform external I/O.
    ///
    /// # Arguments
    /// - `self` `[in]`: fixture server, not modified.
    /// - `context` `[in]`: unused request/notification context; no callbacks are made.
    /// - `params` `[in]`: consumed CompletionItem fixture input, deliberately ignored.
    ///
    /// # Returns
    /// `Err(ServerError::unsupported())` always; no success payload within `ServerResult<CompletionItem>` is produced.
    ///
    /// # Errors
    /// Always returns the unsupported-method error.
    ///
    /// # Examples
    /// Local fixture context with a prepared `Context` and `CompletionItem` payload:
    /// ```text
    /// let result = server.resolve_completion_item(context, params).await;
    /// ```
    async fn resolve_completion_item(
        &self,
        context: Context,
        params: CompletionItem,
    ) -> ServerResult<CompletionItem> {
        let _ = (context, params);
        Err(ServerError::unsupported())
    }
    /// Check negotiated encoding and position, append hover to events, and return no hover. No external I/O.
    ///
    /// # Arguments
    /// - `self` `[in,out]`: shared event log; appends hover through its mutex.
    /// - `context` `[in]`: request context whose encoding must be UTF-8.
    /// - `params` `[in]`: hover position whose character must equal 4; document URI is not opened.
    ///
    /// # Returns
    /// `Ok(None)`; this fixture never returns hover contents or a server error.
    ///
    /// # Panics
    /// If encoding/character assertions fail or the event mutex is poisoned.
    ///
    /// # Examples
    /// Local fixture context with a prepared `Context` and `HoverParams` payload:
    /// ```text
    /// let result = server.hover_request(context, params).await;
    /// ```
    async fn hover_request(
        &self,
        context: Context,
        params: HoverParams,
    ) -> ServerResult<Option<Hover>> {
        assert_eq!(context.encoding, CoordinateEncoding::Utf8);
        assert_eq!(params.text_document_position_params.position.character, 4);
        self.events.lock().unwrap().push("hover".into());
        Ok(None)
    }
    /// Reject the signature_help_request hook as unsupported; discard its SignatureHelpParams payload.
    /// Does not read/change events, send callbacks, or perform external I/O.
    ///
    /// # Arguments
    /// - `self` `[in]`: fixture server, not modified.
    /// - `context` `[in]`: unused request/notification context; no callbacks are made.
    /// - `params` `[in]`: consumed SignatureHelpParams fixture input, deliberately ignored.
    ///
    /// # Returns
    /// `Err(ServerError::unsupported())` always; no success payload within `ServerResult<Option<SignatureHelp>>` is produced.
    ///
    /// # Errors
    /// Always returns the unsupported-method error.
    ///
    /// # Examples
    /// Local fixture context with a prepared `Context` and `SignatureHelpParams` payload:
    /// ```text
    /// let result = server.signature_help_request(context, params).await;
    /// ```
    async fn signature_help_request(
        &self,
        context: Context,
        params: SignatureHelpParams,
    ) -> ServerResult<Option<SignatureHelp>> {
        let _ = (context, params);
        Err(ServerError::unsupported())
    }
    /// Reject the goto_declaration hook as unsupported; discard its GotoDeclarationParams payload.
    /// Does not read/change events, send callbacks, or perform external I/O.
    ///
    /// # Arguments
    /// - `self` `[in]`: fixture server, not modified.
    /// - `context` `[in]`: unused request/notification context; no callbacks are made.
    /// - `params` `[in]`: consumed GotoDeclarationParams fixture input, deliberately ignored.
    ///
    /// # Returns
    /// `Err(ServerError::unsupported())` always; no success payload within `ServerResult<Option<GotoDeclarationResponse>>` is produced.
    ///
    /// # Errors
    /// Always returns the unsupported-method error.
    ///
    /// # Examples
    /// Local fixture context with a prepared `Context` and `GotoDeclarationParams` payload:
    /// ```text
    /// let result = server.goto_declaration(context, params).await;
    /// ```
    async fn goto_declaration(
        &self,
        context: Context,
        params: GotoDeclarationParams,
    ) -> ServerResult<Option<GotoDeclarationResponse>> {
        let _ = (context, params);
        Err(ServerError::unsupported())
    }
    /// Reject the goto_definition hook as unsupported; discard its GotoDefinitionParams payload.
    /// Does not read/change events, send callbacks, or perform external I/O.
    ///
    /// # Arguments
    /// - `self` `[in]`: fixture server, not modified.
    /// - `context` `[in]`: unused request/notification context; no callbacks are made.
    /// - `params` `[in]`: consumed GotoDefinitionParams fixture input, deliberately ignored.
    ///
    /// # Returns
    /// `Err(ServerError::unsupported())` always; no success payload within `ServerResult<Option<GotoDefinitionResponse>>` is produced.
    ///
    /// # Errors
    /// Always returns the unsupported-method error.
    ///
    /// # Examples
    /// Local fixture context with a prepared `Context` and `GotoDefinitionParams` payload:
    /// ```text
    /// let result = server.goto_definition(context, params).await;
    /// ```
    async fn goto_definition(
        &self,
        context: Context,
        params: GotoDefinitionParams,
    ) -> ServerResult<Option<GotoDefinitionResponse>> {
        let _ = (context, params);
        Err(ServerError::unsupported())
    }
    /// Reject the references hook as unsupported; discard its ReferenceParams payload.
    /// Does not read/change events, send callbacks, or perform external I/O.
    ///
    /// # Arguments
    /// - `self` `[in]`: fixture server, not modified.
    /// - `context` `[in]`: unused request/notification context; no callbacks are made.
    /// - `params` `[in]`: consumed ReferenceParams fixture input, deliberately ignored.
    ///
    /// # Returns
    /// `Err(ServerError::unsupported())` always; no success payload within `ServerResult<Option<Vec<Location>>>` is produced.
    ///
    /// # Errors
    /// Always returns the unsupported-method error.
    ///
    /// # Examples
    /// Local fixture context with a prepared `Context` and `ReferenceParams` payload:
    /// ```text
    /// let result = server.references(context, params).await;
    /// ```
    async fn references(
        &self,
        context: Context,
        params: ReferenceParams,
    ) -> ServerResult<Option<Vec<Location>>> {
        let _ = (context, params);
        Err(ServerError::unsupported())
    }
    /// Reject the goto_type_definition hook as unsupported; discard its GotoTypeDefinitionParams payload.
    /// Does not read/change events, send callbacks, or perform external I/O.
    ///
    /// # Arguments
    /// - `self` `[in]`: fixture server, not modified.
    /// - `context` `[in]`: unused request/notification context; no callbacks are made.
    /// - `params` `[in]`: consumed GotoTypeDefinitionParams fixture input, deliberately ignored.
    ///
    /// # Returns
    /// `Err(ServerError::unsupported())` always; no success payload within `ServerResult<Option<GotoTypeDefinitionResponse>>` is produced.
    ///
    /// # Errors
    /// Always returns the unsupported-method error.
    ///
    /// # Examples
    /// Local fixture context with a prepared `Context` and `GotoTypeDefinitionParams` payload:
    /// ```text
    /// let result = server.goto_type_definition(context, params).await;
    /// ```
    async fn goto_type_definition(
        &self,
        context: Context,
        params: GotoTypeDefinitionParams,
    ) -> ServerResult<Option<GotoTypeDefinitionResponse>> {
        let _ = (context, params);
        Err(ServerError::unsupported())
    }
    /// Reject the goto_implementation hook as unsupported; discard its GotoImplementationParams payload.
    /// Does not read/change events, send callbacks, or perform external I/O.
    ///
    /// # Arguments
    /// - `self` `[in]`: fixture server, not modified.
    /// - `context` `[in]`: unused request/notification context; no callbacks are made.
    /// - `params` `[in]`: consumed GotoImplementationParams fixture input, deliberately ignored.
    ///
    /// # Returns
    /// `Err(ServerError::unsupported())` always; no success payload within `ServerResult<Option<GotoImplementationResponse>>` is produced.
    ///
    /// # Errors
    /// Always returns the unsupported-method error.
    ///
    /// # Examples
    /// Local fixture context with a prepared `Context` and `GotoImplementationParams` payload:
    /// ```text
    /// let result = server.goto_implementation(context, params).await;
    /// ```
    async fn goto_implementation(
        &self,
        context: Context,
        params: GotoImplementationParams,
    ) -> ServerResult<Option<GotoImplementationResponse>> {
        let _ = (context, params);
        Err(ServerError::unsupported())
    }
    /// Reject the document_highlight_request hook as unsupported; discard its DocumentHighlightParams payload.
    /// Does not read/change events, send callbacks, or perform external I/O.
    ///
    /// # Arguments
    /// - `self` `[in]`: fixture server, not modified.
    /// - `context` `[in]`: unused request/notification context; no callbacks are made.
    /// - `params` `[in]`: consumed DocumentHighlightParams fixture input, deliberately ignored.
    ///
    /// # Returns
    /// `Err(ServerError::unsupported())` always; no success payload within `ServerResult<Option<Vec<DocumentHighlight>>>` is produced.
    ///
    /// # Errors
    /// Always returns the unsupported-method error.
    ///
    /// # Examples
    /// Local fixture context with a prepared `Context` and `DocumentHighlightParams` payload:
    /// ```text
    /// let result = server.document_highlight_request(context, params).await;
    /// ```
    async fn document_highlight_request(
        &self,
        context: Context,
        params: DocumentHighlightParams,
    ) -> ServerResult<Option<Vec<DocumentHighlight>>> {
        let _ = (context, params);
        Err(ServerError::unsupported())
    }
    /// Reject the document_symbol_request hook as unsupported; discard its DocumentSymbolParams payload.
    /// Does not read/change events, send callbacks, or perform external I/O.
    ///
    /// # Arguments
    /// - `self` `[in]`: fixture server, not modified.
    /// - `context` `[in]`: unused request/notification context; no callbacks are made.
    /// - `params` `[in]`: consumed DocumentSymbolParams fixture input, deliberately ignored.
    ///
    /// # Returns
    /// `Err(ServerError::unsupported())` always; no success payload within `ServerResult<Option<DocumentSymbolResponse>>` is produced.
    ///
    /// # Errors
    /// Always returns the unsupported-method error.
    ///
    /// # Examples
    /// Local fixture context with a prepared `Context` and `DocumentSymbolParams` payload:
    /// ```text
    /// let result = server.document_symbol_request(context, params).await;
    /// ```
    async fn document_symbol_request(
        &self,
        context: Context,
        params: DocumentSymbolParams,
    ) -> ServerResult<Option<DocumentSymbolResponse>> {
        let _ = (context, params);
        Err(ServerError::unsupported())
    }
    /// Reject the workspace_symbol_request hook as unsupported; discard its WorkspaceSymbolParams payload.
    /// Does not read/change events, send callbacks, or perform external I/O.
    ///
    /// # Arguments
    /// - `self` `[in]`: fixture server, not modified.
    /// - `context` `[in]`: unused request/notification context; no callbacks are made.
    /// - `params` `[in]`: consumed WorkspaceSymbolParams fixture input, deliberately ignored.
    ///
    /// # Returns
    /// `Err(ServerError::unsupported())` always; no success payload within `ServerResult<Option<WorkspaceSymbolResponse>>` is produced.
    ///
    /// # Errors
    /// Always returns the unsupported-method error.
    ///
    /// # Examples
    /// Local fixture context with a prepared `Context` and `WorkspaceSymbolParams` payload:
    /// ```text
    /// let result = server.workspace_symbol_request(context, params).await;
    /// ```
    async fn workspace_symbol_request(
        &self,
        context: Context,
        params: WorkspaceSymbolParams,
    ) -> ServerResult<Option<WorkspaceSymbolResponse>> {
        let _ = (context, params);
        Err(ServerError::unsupported())
    }
    /// Reject the workspace_symbol_resolve hook as unsupported; discard its WorkspaceSymbol payload.
    /// Does not read/change events, send callbacks, or perform external I/O.
    ///
    /// # Arguments
    /// - `self` `[in]`: fixture server, not modified.
    /// - `context` `[in]`: unused request/notification context; no callbacks are made.
    /// - `params` `[in]`: consumed WorkspaceSymbol fixture input, deliberately ignored.
    ///
    /// # Returns
    /// `Err(ServerError::unsupported())` always; no success payload within `ServerResult<WorkspaceSymbol>` is produced.
    ///
    /// # Errors
    /// Always returns the unsupported-method error.
    ///
    /// # Examples
    /// Local fixture context with a prepared `Context` and `WorkspaceSymbol` payload:
    /// ```text
    /// let result = server.workspace_symbol_resolve(context, params).await;
    /// ```
    async fn workspace_symbol_resolve(
        &self,
        context: Context,
        params: WorkspaceSymbol,
    ) -> ServerResult<WorkspaceSymbol> {
        let _ = (context, params);
        Err(ServerError::unsupported())
    }
    /// Reject the execute_command hook as unsupported; discard its ExecuteCommandParams payload.
    /// Does not read/change events, send callbacks, or perform external I/O.
    ///
    /// # Arguments
    /// - `self` `[in]`: fixture server, not modified.
    /// - `context` `[in]`: unused request/notification context; no callbacks are made.
    /// - `params` `[in]`: consumed ExecuteCommandParams fixture input, deliberately ignored.
    ///
    /// # Returns
    /// `Err(ServerError::unsupported())` always; no success payload within `ServerResult<Option<Data>>` is produced.
    ///
    /// # Errors
    /// Always returns the unsupported-method error.
    ///
    /// # Examples
    /// Local fixture context with a prepared `Context` and `ExecuteCommandParams` payload:
    /// ```text
    /// let result = server.execute_command(context, params).await;
    /// ```
    async fn execute_command(
        &self,
        context: Context,
        params: ExecuteCommandParams,
    ) -> ServerResult<Option<Data>> {
        let _ = (context, params);
        Err(ServerError::unsupported())
    }
    /// Reject the will_save_wait_until hook as unsupported; discard its WillSaveTextDocumentParams payload.
    /// Does not read/change events, send callbacks, or perform external I/O.
    ///
    /// # Arguments
    /// - `self` `[in]`: fixture server, not modified.
    /// - `context` `[in]`: unused request/notification context; no callbacks are made.
    /// - `params` `[in]`: consumed WillSaveTextDocumentParams fixture input, deliberately ignored.
    ///
    /// # Returns
    /// `Err(ServerError::unsupported())` always; no success payload within `ServerResult<Option<Vec<TextEdit>>>` is produced.
    ///
    /// # Errors
    /// Always returns the unsupported-method error.
    ///
    /// # Examples
    /// Local fixture context with a prepared `Context` and `WillSaveTextDocumentParams` payload:
    /// ```text
    /// let result = server.will_save_wait_until(context, params).await;
    /// ```
    async fn will_save_wait_until(
        &self,
        context: Context,
        params: WillSaveTextDocumentParams,
    ) -> ServerResult<Option<Vec<TextEdit>>> {
        let _ = (context, params);
        Err(ServerError::unsupported())
    }
    /// Reject the code_action_request hook as unsupported; discard its CodeActionParams payload.
    /// Does not read/change events, send callbacks, or perform external I/O.
    ///
    /// # Arguments
    /// - `self` `[in]`: fixture server, not modified.
    /// - `context` `[in]`: unused request/notification context; no callbacks are made.
    /// - `params` `[in]`: consumed CodeActionParams fixture input, deliberately ignored.
    ///
    /// # Returns
    /// `Err(ServerError::unsupported())` always; no success payload within `ServerResult<Option<CodeActionResponse>>` is produced.
    ///
    /// # Errors
    /// Always returns the unsupported-method error.
    ///
    /// # Examples
    /// Local fixture context with a prepared `Context` and `CodeActionParams` payload:
    /// ```text
    /// let result = server.code_action_request(context, params).await;
    /// ```
    async fn code_action_request(
        &self,
        context: Context,
        params: CodeActionParams,
    ) -> ServerResult<Option<CodeActionResponse>> {
        let _ = (context, params);
        Err(ServerError::unsupported())
    }
    /// Reject the code_action_resolve_request hook as unsupported; discard its CodeAction payload.
    /// Does not read/change events, send callbacks, or perform external I/O.
    ///
    /// # Arguments
    /// - `self` `[in]`: fixture server, not modified.
    /// - `context` `[in]`: unused request/notification context; no callbacks are made.
    /// - `params` `[in]`: consumed CodeAction fixture input, deliberately ignored.
    ///
    /// # Returns
    /// `Err(ServerError::unsupported())` always; no success payload within `ServerResult<CodeAction>` is produced.
    ///
    /// # Errors
    /// Always returns the unsupported-method error.
    ///
    /// # Examples
    /// Local fixture context with a prepared `Context` and `CodeAction` payload:
    /// ```text
    /// let result = server.code_action_resolve_request(context, params).await;
    /// ```
    async fn code_action_resolve_request(
        &self,
        context: Context,
        params: CodeAction,
    ) -> ServerResult<CodeAction> {
        let _ = (context, params);
        Err(ServerError::unsupported())
    }
    /// Reject the code_lens_request hook as unsupported; discard its CodeLensParams payload.
    /// Does not read/change events, send callbacks, or perform external I/O.
    ///
    /// # Arguments
    /// - `self` `[in]`: fixture server, not modified.
    /// - `context` `[in]`: unused request/notification context; no callbacks are made.
    /// - `params` `[in]`: consumed CodeLensParams fixture input, deliberately ignored.
    ///
    /// # Returns
    /// `Err(ServerError::unsupported())` always; no success payload within `ServerResult<Option<Vec<CodeLens>>>` is produced.
    ///
    /// # Errors
    /// Always returns the unsupported-method error.
    ///
    /// # Examples
    /// Local fixture context with a prepared `Context` and `CodeLensParams` payload:
    /// ```text
    /// let result = server.code_lens_request(context, params).await;
    /// ```
    async fn code_lens_request(
        &self,
        context: Context,
        params: CodeLensParams,
    ) -> ServerResult<Option<Vec<CodeLens>>> {
        let _ = (context, params);
        Err(ServerError::unsupported())
    }
    /// Reject the code_lens_resolve hook as unsupported; discard its CodeLens payload.
    /// Does not read/change events, send callbacks, or perform external I/O.
    ///
    /// # Arguments
    /// - `self` `[in]`: fixture server, not modified.
    /// - `context` `[in]`: unused request/notification context; no callbacks are made.
    /// - `params` `[in]`: consumed CodeLens fixture input, deliberately ignored.
    ///
    /// # Returns
    /// `Err(ServerError::unsupported())` always; no success payload within `ServerResult<CodeLens>` is produced.
    ///
    /// # Errors
    /// Always returns the unsupported-method error.
    ///
    /// # Examples
    /// Local fixture context with a prepared `Context` and `CodeLens` payload:
    /// ```text
    /// let result = server.code_lens_resolve(context, params).await;
    /// ```
    async fn code_lens_resolve(
        &self,
        context: Context,
        params: CodeLens,
    ) -> ServerResult<CodeLens> {
        let _ = (context, params);
        Err(ServerError::unsupported())
    }
    /// Reject the document_link_request hook as unsupported; discard its DocumentLinkParams payload.
    /// Does not read/change events, send callbacks, or perform external I/O.
    ///
    /// # Arguments
    /// - `self` `[in]`: fixture server, not modified.
    /// - `context` `[in]`: unused request/notification context; no callbacks are made.
    /// - `params` `[in]`: consumed DocumentLinkParams fixture input, deliberately ignored.
    ///
    /// # Returns
    /// `Err(ServerError::unsupported())` always; no success payload within `ServerResult<Option<Vec<DocumentLink>>>` is produced.
    ///
    /// # Errors
    /// Always returns the unsupported-method error.
    ///
    /// # Examples
    /// Local fixture context with a prepared `Context` and `DocumentLinkParams` payload:
    /// ```text
    /// let result = server.document_link_request(context, params).await;
    /// ```
    async fn document_link_request(
        &self,
        context: Context,
        params: DocumentLinkParams,
    ) -> ServerResult<Option<Vec<DocumentLink>>> {
        let _ = (context, params);
        Err(ServerError::unsupported())
    }
    /// Reject the document_link_resolve hook as unsupported; discard its DocumentLink payload.
    /// Does not read/change events, send callbacks, or perform external I/O.
    ///
    /// # Arguments
    /// - `self` `[in]`: fixture server, not modified.
    /// - `context` `[in]`: unused request/notification context; no callbacks are made.
    /// - `params` `[in]`: consumed DocumentLink fixture input, deliberately ignored.
    ///
    /// # Returns
    /// `Err(ServerError::unsupported())` always; no success payload within `ServerResult<DocumentLink>` is produced.
    ///
    /// # Errors
    /// Always returns the unsupported-method error.
    ///
    /// # Examples
    /// Local fixture context with a prepared `Context` and `DocumentLink` payload:
    /// ```text
    /// let result = server.document_link_resolve(context, params).await;
    /// ```
    async fn document_link_resolve(
        &self,
        context: Context,
        params: DocumentLink,
    ) -> ServerResult<DocumentLink> {
        let _ = (context, params);
        Err(ServerError::unsupported())
    }
    /// Reject the formatting hook as unsupported; discard its DocumentFormattingParams payload.
    /// Does not read/change events, send callbacks, or perform external I/O.
    ///
    /// # Arguments
    /// - `self` `[in]`: fixture server, not modified.
    /// - `context` `[in]`: unused request/notification context; no callbacks are made.
    /// - `params` `[in]`: consumed DocumentFormattingParams fixture input, deliberately ignored.
    ///
    /// # Returns
    /// `Err(ServerError::unsupported())` always; no success payload within `ServerResult<Option<Vec<TextEdit>>>` is produced.
    ///
    /// # Errors
    /// Always returns the unsupported-method error.
    ///
    /// # Examples
    /// Local fixture context with a prepared `Context` and `DocumentFormattingParams` payload:
    /// ```text
    /// let result = server.formatting(context, params).await;
    /// ```
    async fn formatting(
        &self,
        context: Context,
        params: DocumentFormattingParams,
    ) -> ServerResult<Option<Vec<TextEdit>>> {
        let _ = (context, params);
        Err(ServerError::unsupported())
    }
    /// Reject the range_formatting hook as unsupported; discard its DocumentRangeFormattingParams payload.
    /// Does not read/change events, send callbacks, or perform external I/O.
    ///
    /// # Arguments
    /// - `self` `[in]`: fixture server, not modified.
    /// - `context` `[in]`: unused request/notification context; no callbacks are made.
    /// - `params` `[in]`: consumed DocumentRangeFormattingParams fixture input, deliberately ignored.
    ///
    /// # Returns
    /// `Err(ServerError::unsupported())` always; no success payload within `ServerResult<Option<Vec<TextEdit>>>` is produced.
    ///
    /// # Errors
    /// Always returns the unsupported-method error.
    ///
    /// # Examples
    /// Local fixture context with a prepared `Context` and `DocumentRangeFormattingParams` payload:
    /// ```text
    /// let result = server.range_formatting(context, params).await;
    /// ```
    async fn range_formatting(
        &self,
        context: Context,
        params: DocumentRangeFormattingParams,
    ) -> ServerResult<Option<Vec<TextEdit>>> {
        let _ = (context, params);
        Err(ServerError::unsupported())
    }
    /// Reject the on_type_formatting hook as unsupported; discard its DocumentOnTypeFormattingParams payload.
    /// Does not read/change events, send callbacks, or perform external I/O.
    ///
    /// # Arguments
    /// - `self` `[in]`: fixture server, not modified.
    /// - `context` `[in]`: unused request/notification context; no callbacks are made.
    /// - `params` `[in]`: consumed DocumentOnTypeFormattingParams fixture input, deliberately ignored.
    ///
    /// # Returns
    /// `Err(ServerError::unsupported())` always; no success payload within `ServerResult<Option<Vec<TextEdit>>>` is produced.
    ///
    /// # Errors
    /// Always returns the unsupported-method error.
    ///
    /// # Examples
    /// Local fixture context with a prepared `Context` and `DocumentOnTypeFormattingParams` payload:
    /// ```text
    /// let result = server.on_type_formatting(context, params).await;
    /// ```
    async fn on_type_formatting(
        &self,
        context: Context,
        params: DocumentOnTypeFormattingParams,
    ) -> ServerResult<Option<Vec<TextEdit>>> {
        let _ = (context, params);
        Err(ServerError::unsupported())
    }
    /// Reject the linked_editing_range hook as unsupported; discard its LinkedEditingRangeParams payload.
    /// Does not read/change events, send callbacks, or perform external I/O.
    ///
    /// # Arguments
    /// - `self` `[in]`: fixture server, not modified.
    /// - `context` `[in]`: unused request/notification context; no callbacks are made.
    /// - `params` `[in]`: consumed LinkedEditingRangeParams fixture input, deliberately ignored.
    ///
    /// # Returns
    /// `Err(ServerError::unsupported())` always; no success payload within `ServerResult<Option<LinkedEditingRanges>>` is produced.
    ///
    /// # Errors
    /// Always returns the unsupported-method error.
    ///
    /// # Examples
    /// Local fixture context with a prepared `Context` and `LinkedEditingRangeParams` payload:
    /// ```text
    /// let result = server.linked_editing_range(context, params).await;
    /// ```
    async fn linked_editing_range(
        &self,
        context: Context,
        params: LinkedEditingRangeParams,
    ) -> ServerResult<Option<LinkedEditingRanges>> {
        let _ = (context, params);
        Err(ServerError::unsupported())
    }
    /// Reject the rename hook as unsupported; discard its RenameParams payload.
    /// Does not read/change events, send callbacks, or perform external I/O.
    ///
    /// # Arguments
    /// - `self` `[in]`: fixture server, not modified.
    /// - `context` `[in]`: unused request/notification context; no callbacks are made.
    /// - `params` `[in]`: consumed RenameParams fixture input, deliberately ignored.
    ///
    /// # Returns
    /// `Err(ServerError::unsupported())` always; no success payload within `ServerResult<Option<WorkspaceEdit>>` is produced.
    ///
    /// # Errors
    /// Always returns the unsupported-method error.
    ///
    /// # Examples
    /// Local fixture context with a prepared `Context` and `RenameParams` payload:
    /// ```text
    /// let result = server.rename(context, params).await;
    /// ```
    async fn rename(
        &self,
        context: Context,
        params: RenameParams,
    ) -> ServerResult<Option<WorkspaceEdit>> {
        let _ = (context, params);
        Err(ServerError::unsupported())
    }
    /// Reject the document_color hook as unsupported; discard its DocumentColorParams payload.
    /// Does not read/change events, send callbacks, or perform external I/O.
    ///
    /// # Arguments
    /// - `self` `[in]`: fixture server, not modified.
    /// - `context` `[in]`: unused request/notification context; no callbacks are made.
    /// - `params` `[in]`: consumed DocumentColorParams fixture input, deliberately ignored.
    ///
    /// # Returns
    /// `Err(ServerError::unsupported())` always; no success payload within `ServerResult<Vec<ColorInformation>>` is produced.
    ///
    /// # Errors
    /// Always returns the unsupported-method error.
    ///
    /// # Examples
    /// Local fixture context with a prepared `Context` and `DocumentColorParams` payload:
    /// ```text
    /// let result = server.document_color(context, params).await;
    /// ```
    async fn document_color(
        &self,
        context: Context,
        params: DocumentColorParams,
    ) -> ServerResult<Vec<ColorInformation>> {
        let _ = (context, params);
        Err(ServerError::unsupported())
    }
    /// Reject the color_presentation_request hook as unsupported; discard its ColorPresentationParams payload.
    /// Does not read/change events, send callbacks, or perform external I/O.
    ///
    /// # Arguments
    /// - `self` `[in]`: fixture server, not modified.
    /// - `context` `[in]`: unused request/notification context; no callbacks are made.
    /// - `params` `[in]`: consumed ColorPresentationParams fixture input, deliberately ignored.
    ///
    /// # Returns
    /// `Err(ServerError::unsupported())` always; no success payload within `ServerResult<Vec<ColorPresentation>>` is produced.
    ///
    /// # Errors
    /// Always returns the unsupported-method error.
    ///
    /// # Examples
    /// Local fixture context with a prepared `Context` and `ColorPresentationParams` payload:
    /// ```text
    /// let result = server.color_presentation_request(context, params).await;
    /// ```
    async fn color_presentation_request(
        &self,
        context: Context,
        params: ColorPresentationParams,
    ) -> ServerResult<Vec<ColorPresentation>> {
        let _ = (context, params);
        Err(ServerError::unsupported())
    }
    /// Reject the folding_range_request hook as unsupported; discard its FoldingRangeParams payload.
    /// Does not read/change events, send callbacks, or perform external I/O.
    ///
    /// # Arguments
    /// - `self` `[in]`: fixture server, not modified.
    /// - `context` `[in]`: unused request/notification context; no callbacks are made.
    /// - `params` `[in]`: consumed FoldingRangeParams fixture input, deliberately ignored.
    ///
    /// # Returns
    /// `Err(ServerError::unsupported())` always; no success payload within `ServerResult<Option<Vec<FoldingRange>>>` is produced.
    ///
    /// # Errors
    /// Always returns the unsupported-method error.
    ///
    /// # Examples
    /// Local fixture context with a prepared `Context` and `FoldingRangeParams` payload:
    /// ```text
    /// let result = server.folding_range_request(context, params).await;
    /// ```
    async fn folding_range_request(
        &self,
        context: Context,
        params: FoldingRangeParams,
    ) -> ServerResult<Option<Vec<FoldingRange>>> {
        let _ = (context, params);
        Err(ServerError::unsupported())
    }
    /// Reject the prepare_rename_request hook as unsupported; discard its TextDocumentPositionParams payload.
    /// Does not read/change events, send callbacks, or perform external I/O.
    ///
    /// # Arguments
    /// - `self` `[in]`: fixture server, not modified.
    /// - `context` `[in]`: unused request/notification context; no callbacks are made.
    /// - `params` `[in]`: consumed TextDocumentPositionParams fixture input, deliberately ignored.
    ///
    /// # Returns
    /// `Err(ServerError::unsupported())` always; no success payload within `ServerResult<Option<PrepareRenameResponse>>` is produced.
    ///
    /// # Errors
    /// Always returns the unsupported-method error.
    ///
    /// # Examples
    /// Local fixture context with a prepared `Context` and `TextDocumentPositionParams` payload:
    /// ```text
    /// let result = server.prepare_rename_request(context, params).await;
    /// ```
    async fn prepare_rename_request(
        &self,
        context: Context,
        params: TextDocumentPositionParams,
    ) -> ServerResult<Option<PrepareRenameResponse>> {
        let _ = (context, params);
        Err(ServerError::unsupported())
    }
    /// Reject the selection_range_request hook as unsupported; discard its SelectionRangeParams payload.
    /// Does not read/change events, send callbacks, or perform external I/O.
    ///
    /// # Arguments
    /// - `self` `[in]`: fixture server, not modified.
    /// - `context` `[in]`: unused request/notification context; no callbacks are made.
    /// - `params` `[in]`: consumed SelectionRangeParams fixture input, deliberately ignored.
    ///
    /// # Returns
    /// `Err(ServerError::unsupported())` always; no success payload within `ServerResult<Option<Vec<SelectionRange>>>` is produced.
    ///
    /// # Errors
    /// Always returns the unsupported-method error.
    ///
    /// # Examples
    /// Local fixture context with a prepared `Context` and `SelectionRangeParams` payload:
    /// ```text
    /// let result = server.selection_range_request(context, params).await;
    /// ```
    async fn selection_range_request(
        &self,
        context: Context,
        params: SelectionRangeParams,
    ) -> ServerResult<Option<Vec<SelectionRange>>> {
        let _ = (context, params);
        Err(ServerError::unsupported())
    }
    /// Reject the call_hierarchy_prepare hook as unsupported; discard its CallHierarchyPrepareParams payload.
    /// Does not read/change events, send callbacks, or perform external I/O.
    ///
    /// # Arguments
    /// - `self` `[in]`: fixture server, not modified.
    /// - `context` `[in]`: unused request/notification context; no callbacks are made.
    /// - `params` `[in]`: consumed CallHierarchyPrepareParams fixture input, deliberately ignored.
    ///
    /// # Returns
    /// `Err(ServerError::unsupported())` always; no success payload within `ServerResult<Option<Vec<CallHierarchyItem>>>` is produced.
    ///
    /// # Errors
    /// Always returns the unsupported-method error.
    ///
    /// # Examples
    /// Local fixture context with a prepared `Context` and `CallHierarchyPrepareParams` payload:
    /// ```text
    /// let result = server.call_hierarchy_prepare(context, params).await;
    /// ```
    async fn call_hierarchy_prepare(
        &self,
        context: Context,
        params: CallHierarchyPrepareParams,
    ) -> ServerResult<Option<Vec<CallHierarchyItem>>> {
        let _ = (context, params);
        Err(ServerError::unsupported())
    }
    /// Reject the call_hierarchy_incoming_calls hook as unsupported; discard its CallHierarchyIncomingCallsParams payload.
    /// Does not read/change events, send callbacks, or perform external I/O.
    ///
    /// # Arguments
    /// - `self` `[in]`: fixture server, not modified.
    /// - `context` `[in]`: unused request/notification context; no callbacks are made.
    /// - `params` `[in]`: consumed CallHierarchyIncomingCallsParams fixture input, deliberately ignored.
    ///
    /// # Returns
    /// `Err(ServerError::unsupported())` always; no success payload within `ServerResult<Option<Vec<CallHierarchyIncomingCall>>>` is produced.
    ///
    /// # Errors
    /// Always returns the unsupported-method error.
    ///
    /// # Examples
    /// Local fixture context with a prepared `Context` and `CallHierarchyIncomingCallsParams` payload:
    /// ```text
    /// let result = server.call_hierarchy_incoming_calls(context, params).await;
    /// ```
    async fn call_hierarchy_incoming_calls(
        &self,
        context: Context,
        params: CallHierarchyIncomingCallsParams,
    ) -> ServerResult<Option<Vec<CallHierarchyIncomingCall>>> {
        let _ = (context, params);
        Err(ServerError::unsupported())
    }
    /// Reject the call_hierarchy_outgoing_calls hook as unsupported; discard its CallHierarchyOutgoingCallsParams payload.
    /// Does not read/change events, send callbacks, or perform external I/O.
    ///
    /// # Arguments
    /// - `self` `[in]`: fixture server, not modified.
    /// - `context` `[in]`: unused request/notification context; no callbacks are made.
    /// - `params` `[in]`: consumed CallHierarchyOutgoingCallsParams fixture input, deliberately ignored.
    ///
    /// # Returns
    /// `Err(ServerError::unsupported())` always; no success payload within `ServerResult<Option<Vec<CallHierarchyOutgoingCall>>>` is produced.
    ///
    /// # Errors
    /// Always returns the unsupported-method error.
    ///
    /// # Examples
    /// Local fixture context with a prepared `Context` and `CallHierarchyOutgoingCallsParams` payload:
    /// ```text
    /// let result = server.call_hierarchy_outgoing_calls(context, params).await;
    /// ```
    async fn call_hierarchy_outgoing_calls(
        &self,
        context: Context,
        params: CallHierarchyOutgoingCallsParams,
    ) -> ServerResult<Option<Vec<CallHierarchyOutgoingCall>>> {
        let _ = (context, params);
        Err(ServerError::unsupported())
    }
    /// Reject the semantic_tokens_full_request hook as unsupported; discard its SemanticTokensParams payload.
    /// Does not read/change events, send callbacks, or perform external I/O.
    ///
    /// # Arguments
    /// - `self` `[in]`: fixture server, not modified.
    /// - `context` `[in]`: unused request/notification context; no callbacks are made.
    /// - `params` `[in]`: consumed SemanticTokensParams fixture input, deliberately ignored.
    ///
    /// # Returns
    /// `Err(ServerError::unsupported())` always; no success payload within `ServerResult<Option<SemanticTokensResult>>` is produced.
    ///
    /// # Errors
    /// Always returns the unsupported-method error.
    ///
    /// # Examples
    /// Local fixture context with a prepared `Context` and `SemanticTokensParams` payload:
    /// ```text
    /// let result = server.semantic_tokens_full_request(context, params).await;
    /// ```
    async fn semantic_tokens_full_request(
        &self,
        context: Context,
        params: SemanticTokensParams,
    ) -> ServerResult<Option<SemanticTokensResult>> {
        let _ = (context, params);
        Err(ServerError::unsupported())
    }
    /// Reject the semantic_tokens_full_delta_request hook as unsupported; discard its SemanticTokensDeltaParams payload.
    /// Does not read/change events, send callbacks, or perform external I/O.
    ///
    /// # Arguments
    /// - `self` `[in]`: fixture server, not modified.
    /// - `context` `[in]`: unused request/notification context; no callbacks are made.
    /// - `params` `[in]`: consumed SemanticTokensDeltaParams fixture input, deliberately ignored.
    ///
    /// # Returns
    /// `Err(ServerError::unsupported())` always; no success payload within `ServerResult<Option<SemanticTokensFullDeltaResult>>` is produced.
    ///
    /// # Errors
    /// Always returns the unsupported-method error.
    ///
    /// # Examples
    /// Local fixture context with a prepared `Context` and `SemanticTokensDeltaParams` payload:
    /// ```text
    /// let result = server.semantic_tokens_full_delta_request(context, params).await;
    /// ```
    async fn semantic_tokens_full_delta_request(
        &self,
        context: Context,
        params: SemanticTokensDeltaParams,
    ) -> ServerResult<Option<SemanticTokensFullDeltaResult>> {
        let _ = (context, params);
        Err(ServerError::unsupported())
    }
    /// Reject the semantic_tokens_range_request hook as unsupported; discard its SemanticTokensRangeParams payload.
    /// Does not read/change events, send callbacks, or perform external I/O.
    ///
    /// # Arguments
    /// - `self` `[in]`: fixture server, not modified.
    /// - `context` `[in]`: unused request/notification context; no callbacks are made.
    /// - `params` `[in]`: consumed SemanticTokensRangeParams fixture input, deliberately ignored.
    ///
    /// # Returns
    /// `Err(ServerError::unsupported())` always; no success payload within `ServerResult<Option<SemanticTokensRangeResult>>` is produced.
    ///
    /// # Errors
    /// Always returns the unsupported-method error.
    ///
    /// # Examples
    /// Local fixture context with a prepared `Context` and `SemanticTokensRangeParams` payload:
    /// ```text
    /// let result = server.semantic_tokens_range_request(context, params).await;
    /// ```
    async fn semantic_tokens_range_request(
        &self,
        context: Context,
        params: SemanticTokensRangeParams,
    ) -> ServerResult<Option<SemanticTokensRangeResult>> {
        let _ = (context, params);
        Err(ServerError::unsupported())
    }
    /// Reject the will_create_files hook as unsupported; discard its CreateFilesParams payload.
    /// Does not read/change events, send callbacks, or perform external I/O.
    ///
    /// # Arguments
    /// - `self` `[in]`: fixture server, not modified.
    /// - `context` `[in]`: unused request/notification context; no callbacks are made.
    /// - `params` `[in]`: consumed CreateFilesParams fixture input, deliberately ignored.
    ///
    /// # Returns
    /// `Err(ServerError::unsupported())` always; no success payload within `ServerResult<Option<WorkspaceEdit>>` is produced.
    ///
    /// # Errors
    /// Always returns the unsupported-method error.
    ///
    /// # Examples
    /// Local fixture context with a prepared `Context` and `CreateFilesParams` payload:
    /// ```text
    /// let result = server.will_create_files(context, params).await;
    /// ```
    async fn will_create_files(
        &self,
        context: Context,
        params: CreateFilesParams,
    ) -> ServerResult<Option<WorkspaceEdit>> {
        let _ = (context, params);
        Err(ServerError::unsupported())
    }
    /// Reject the will_rename_files hook as unsupported; discard its RenameFilesParams payload.
    /// Does not read/change events, send callbacks, or perform external I/O.
    ///
    /// # Arguments
    /// - `self` `[in]`: fixture server, not modified.
    /// - `context` `[in]`: unused request/notification context; no callbacks are made.
    /// - `params` `[in]`: consumed RenameFilesParams fixture input, deliberately ignored.
    ///
    /// # Returns
    /// `Err(ServerError::unsupported())` always; no success payload within `ServerResult<Option<WorkspaceEdit>>` is produced.
    ///
    /// # Errors
    /// Always returns the unsupported-method error.
    ///
    /// # Examples
    /// Local fixture context with a prepared `Context` and `RenameFilesParams` payload:
    /// ```text
    /// let result = server.will_rename_files(context, params).await;
    /// ```
    async fn will_rename_files(
        &self,
        context: Context,
        params: RenameFilesParams,
    ) -> ServerResult<Option<WorkspaceEdit>> {
        let _ = (context, params);
        Err(ServerError::unsupported())
    }
    /// Reject the will_delete_files hook as unsupported; discard its DeleteFilesParams payload.
    /// Does not read/change events, send callbacks, or perform external I/O.
    ///
    /// # Arguments
    /// - `self` `[in]`: fixture server, not modified.
    /// - `context` `[in]`: unused request/notification context; no callbacks are made.
    /// - `params` `[in]`: consumed DeleteFilesParams fixture input, deliberately ignored.
    ///
    /// # Returns
    /// `Err(ServerError::unsupported())` always; no success payload within `ServerResult<Option<WorkspaceEdit>>` is produced.
    ///
    /// # Errors
    /// Always returns the unsupported-method error.
    ///
    /// # Examples
    /// Local fixture context with a prepared `Context` and `DeleteFilesParams` payload:
    /// ```text
    /// let result = server.will_delete_files(context, params).await;
    /// ```
    async fn will_delete_files(
        &self,
        context: Context,
        params: DeleteFilesParams,
    ) -> ServerResult<Option<WorkspaceEdit>> {
        let _ = (context, params);
        Err(ServerError::unsupported())
    }
    /// Reject the moniker_request hook as unsupported; discard its MonikerParams payload.
    /// Does not read/change events, send callbacks, or perform external I/O.
    ///
    /// # Arguments
    /// - `self` `[in]`: fixture server, not modified.
    /// - `context` `[in]`: unused request/notification context; no callbacks are made.
    /// - `params` `[in]`: consumed MonikerParams fixture input, deliberately ignored.
    ///
    /// # Returns
    /// `Err(ServerError::unsupported())` always; no success payload within `ServerResult<Option<Vec<Moniker>>>` is produced.
    ///
    /// # Errors
    /// Always returns the unsupported-method error.
    ///
    /// # Examples
    /// Local fixture context with a prepared `Context` and `MonikerParams` payload:
    /// ```text
    /// let result = server.moniker_request(context, params).await;
    /// ```
    async fn moniker_request(
        &self,
        context: Context,
        params: MonikerParams,
    ) -> ServerResult<Option<Vec<Moniker>>> {
        let _ = (context, params);
        Err(ServerError::unsupported())
    }
    /// Reject the inlay_hint_request hook as unsupported; discard its InlayHintParams payload.
    /// Does not read/change events, send callbacks, or perform external I/O.
    ///
    /// # Arguments
    /// - `self` `[in]`: fixture server, not modified.
    /// - `context` `[in]`: unused request/notification context; no callbacks are made.
    /// - `params` `[in]`: consumed InlayHintParams fixture input, deliberately ignored.
    ///
    /// # Returns
    /// `Err(ServerError::unsupported())` always; no success payload within `ServerResult<Option<Vec<InlayHint>>>` is produced.
    ///
    /// # Errors
    /// Always returns the unsupported-method error.
    ///
    /// # Examples
    /// Local fixture context with a prepared `Context` and `InlayHintParams` payload:
    /// ```text
    /// let result = server.inlay_hint_request(context, params).await;
    /// ```
    async fn inlay_hint_request(
        &self,
        context: Context,
        params: InlayHintParams,
    ) -> ServerResult<Option<Vec<InlayHint>>> {
        let _ = (context, params);
        Err(ServerError::unsupported())
    }
    /// Reject the inlay_hint_resolve_request hook as unsupported; discard its InlayHint payload.
    /// Does not read/change events, send callbacks, or perform external I/O.
    ///
    /// # Arguments
    /// - `self` `[in]`: fixture server, not modified.
    /// - `context` `[in]`: unused request/notification context; no callbacks are made.
    /// - `params` `[in]`: consumed InlayHint fixture input, deliberately ignored.
    ///
    /// # Returns
    /// `Err(ServerError::unsupported())` always; no success payload within `ServerResult<InlayHint>` is produced.
    ///
    /// # Errors
    /// Always returns the unsupported-method error.
    ///
    /// # Examples
    /// Local fixture context with a prepared `Context` and `InlayHint` payload:
    /// ```text
    /// let result = server.inlay_hint_resolve_request(context, params).await;
    /// ```
    async fn inlay_hint_resolve_request(
        &self,
        context: Context,
        params: InlayHint,
    ) -> ServerResult<InlayHint> {
        let _ = (context, params);
        Err(ServerError::unsupported())
    }
    /// Reject the inline_value_request hook as unsupported; discard its InlineValueParams payload.
    /// Does not read/change events, send callbacks, or perform external I/O.
    ///
    /// # Arguments
    /// - `self` `[in]`: fixture server, not modified.
    /// - `context` `[in]`: unused request/notification context; no callbacks are made.
    /// - `params` `[in]`: consumed InlineValueParams fixture input, deliberately ignored.
    ///
    /// # Returns
    /// `Err(ServerError::unsupported())` always; no success payload within `ServerResult<Option<Vec<InlineValue>>>` is produced.
    ///
    /// # Errors
    /// Always returns the unsupported-method error.
    ///
    /// # Examples
    /// Local fixture context with a prepared `Context` and `InlineValueParams` payload:
    /// ```text
    /// let result = server.inline_value_request(context, params).await;
    /// ```
    async fn inline_value_request(
        &self,
        context: Context,
        params: InlineValueParams,
    ) -> ServerResult<Option<Vec<InlineValue>>> {
        let _ = (context, params);
        Err(ServerError::unsupported())
    }
    /// Reject the document_diagnostic_request hook as unsupported; discard its DocumentDiagnosticParams payload.
    /// Does not read/change events, send callbacks, or perform external I/O.
    ///
    /// # Arguments
    /// - `self` `[in]`: fixture server, not modified.
    /// - `context` `[in]`: unused request/notification context; no callbacks are made.
    /// - `params` `[in]`: consumed DocumentDiagnosticParams fixture input, deliberately ignored.
    ///
    /// # Returns
    /// `Err(ServerError::unsupported())` always; no success payload within `ServerResult<DocumentDiagnosticReportResult>` is produced.
    ///
    /// # Errors
    /// Always returns the unsupported-method error.
    ///
    /// # Examples
    /// Local fixture context with a prepared `Context` and `DocumentDiagnosticParams` payload:
    /// ```text
    /// let result = server.document_diagnostic_request(context, params).await;
    /// ```
    async fn document_diagnostic_request(
        &self,
        context: Context,
        params: DocumentDiagnosticParams,
    ) -> ServerResult<DocumentDiagnosticReportResult> {
        let _ = (context, params);
        Err(ServerError::unsupported())
    }
    /// Reject the workspace_diagnostic_request hook as unsupported; discard its WorkspaceDiagnosticParams payload.
    /// Does not read/change events, send callbacks, or perform external I/O.
    ///
    /// # Arguments
    /// - `self` `[in]`: fixture server, not modified.
    /// - `context` `[in]`: unused request/notification context; no callbacks are made.
    /// - `params` `[in]`: consumed WorkspaceDiagnosticParams fixture input, deliberately ignored.
    ///
    /// # Returns
    /// `Err(ServerError::unsupported())` always; no success payload within `ServerResult<WorkspaceDiagnosticReportResult>` is produced.
    ///
    /// # Errors
    /// Always returns the unsupported-method error.
    ///
    /// # Examples
    /// Local fixture context with a prepared `Context` and `WorkspaceDiagnosticParams` payload:
    /// ```text
    /// let result = server.workspace_diagnostic_request(context, params).await;
    /// ```
    async fn workspace_diagnostic_request(
        &self,
        context: Context,
        params: WorkspaceDiagnosticParams,
    ) -> ServerResult<WorkspaceDiagnosticReportResult> {
        let _ = (context, params);
        Err(ServerError::unsupported())
    }
    /// Reject the type_hierarchy_prepare hook as unsupported; discard its TypeHierarchyPrepareParams payload.
    /// Does not read/change events, send callbacks, or perform external I/O.
    ///
    /// # Arguments
    /// - `self` `[in]`: fixture server, not modified.
    /// - `context` `[in]`: unused request/notification context; no callbacks are made.
    /// - `params` `[in]`: consumed TypeHierarchyPrepareParams fixture input, deliberately ignored.
    ///
    /// # Returns
    /// `Err(ServerError::unsupported())` always; no success payload within `ServerResult<Option<Vec<TypeHierarchyItem>>>` is produced.
    ///
    /// # Errors
    /// Always returns the unsupported-method error.
    ///
    /// # Examples
    /// Local fixture context with a prepared `Context` and `TypeHierarchyPrepareParams` payload:
    /// ```text
    /// let result = server.type_hierarchy_prepare(context, params).await;
    /// ```
    async fn type_hierarchy_prepare(
        &self,
        context: Context,
        params: TypeHierarchyPrepareParams,
    ) -> ServerResult<Option<Vec<TypeHierarchyItem>>> {
        let _ = (context, params);
        Err(ServerError::unsupported())
    }
    /// Reject the type_hierarchy_supertypes hook as unsupported; discard its TypeHierarchySupertypesParams payload.
    /// Does not read/change events, send callbacks, or perform external I/O.
    ///
    /// # Arguments
    /// - `self` `[in]`: fixture server, not modified.
    /// - `context` `[in]`: unused request/notification context; no callbacks are made.
    /// - `params` `[in]`: consumed TypeHierarchySupertypesParams fixture input, deliberately ignored.
    ///
    /// # Returns
    /// `Err(ServerError::unsupported())` always; no success payload within `ServerResult<Option<Vec<TypeHierarchyItem>>>` is produced.
    ///
    /// # Errors
    /// Always returns the unsupported-method error.
    ///
    /// # Examples
    /// Local fixture context with a prepared `Context` and `TypeHierarchySupertypesParams` payload:
    /// ```text
    /// let result = server.type_hierarchy_supertypes(context, params).await;
    /// ```
    async fn type_hierarchy_supertypes(
        &self,
        context: Context,
        params: TypeHierarchySupertypesParams,
    ) -> ServerResult<Option<Vec<TypeHierarchyItem>>> {
        let _ = (context, params);
        Err(ServerError::unsupported())
    }
    /// Reject the type_hierarchy_subtypes hook as unsupported; discard its TypeHierarchySubtypesParams payload.
    /// Does not read/change events, send callbacks, or perform external I/O.
    ///
    /// # Arguments
    /// - `self` `[in]`: fixture server, not modified.
    /// - `context` `[in]`: unused request/notification context; no callbacks are made.
    /// - `params` `[in]`: consumed TypeHierarchySubtypesParams fixture input, deliberately ignored.
    ///
    /// # Returns
    /// `Err(ServerError::unsupported())` always; no success payload within `ServerResult<Option<Vec<TypeHierarchyItem>>>` is produced.
    ///
    /// # Errors
    /// Always returns the unsupported-method error.
    ///
    /// # Examples
    /// Local fixture context with a prepared `Context` and `TypeHierarchySubtypesParams` payload:
    /// ```text
    /// let result = server.type_hierarchy_subtypes(context, params).await;
    /// ```
    async fn type_hierarchy_subtypes(
        &self,
        context: Context,
        params: TypeHierarchySubtypesParams,
    ) -> ServerResult<Option<Vec<TypeHierarchyItem>>> {
        let _ = (context, params);
        Err(ServerError::unsupported())
    }
    /// Accept the forwarded cancellation hook without changing events or performing I/O.
    ///
    /// # Arguments
    /// - `self` `[in]`: fixture server, not modified.
    /// - `context` `[in]`: unused context; the wrapper handles cancellation itself.
    /// - `params` `[in]`: unused cancellation identifier.
    ///
    /// # Returns
    /// `Ok(())` unconditionally; no error is returned.
    ///
    /// # Examples
    /// Local fixture context with a prepared `Context` and `CancelParams` payload:
    /// ```text
    /// let result = server.cancel(context, params).await;
    /// ```
    async fn cancel(&self, context: Context, params: CancelParams) -> ServerResult<()> {
        Ok(())
    }
    /// Accept trace settings without changing events or performing I/O.
    ///
    /// # Arguments
    /// - `self` `[in]`: fixture server, not modified.
    /// - `context` `[in]`: unused context.
    /// - `params` `[in]`: unused trace setting.
    ///
    /// # Returns
    /// `Ok(())` unconditionally; no error is returned.
    ///
    /// # Examples
    /// Local fixture context with a prepared `Context` and `SetTraceParams` payload:
    /// ```text
    /// let result = server.set_trace(context, params).await;
    /// ```
    async fn set_trace(&self, context: Context, params: SetTraceParams) -> ServerResult<()> {
        Ok(())
    }
    /// Request fixture configuration from the client, validate its first value, and append configured.
    /// Sends an in-memory protocol callback through the session transport; no filesystem or subprocess access.
    ///
    /// # Arguments
    /// - `self` `[in,out]`: event log; appends configured only after the callback succeeds.
    /// - `context` `[in]`: client handle used for workspace/configuration.
    /// - `params` `[in]`: unused initialized payload.
    ///
    /// # Returns
    /// `Ok(())` after recording configuration; `Err(ServerError)` if the callback fails.
    ///
    /// # Errors
    /// Propagates workspace_configuration errors before appending any event.
    ///
    /// # Panics
    /// If the reply is empty, its first value is not text configured, or the event mutex is poisoned.
    ///
    /// # Algorithm
    /// Ask for the fixture section, propagate callback failure, require the first returned value
    /// to be configured, then append the event. The caller must answer the outgoing request.
    ///
    /// # Flow
    /// ```text
    /// [workspace/configuration] --> < callback result >
    ///                               /              \
    ///                             Err              Ok
    ///                              |                |
    ///                              v                v
    ///                         [return error] [validate value]
    ///                                               |
    ///                                               v
    ///                                      [append configured]
    /// ```
    ///
    /// # Examples
    /// Local fixture context with a prepared `Context` and `InitializedParams` payload:
    /// ```text
    /// let result = server.initialized(context, params).await;
    /// ```
    async fn initialized(&self, context: Context, params: InitializedParams) -> ServerResult<()> {
        let values = context
            .client
            .workspace_configuration(ConfigurationParams {
                items: vec![ConfigurationItem {
                    scope_uri: None,
                    section: Some("fixture".into()),
                }],
            })
            .await?;
        assert!(matches!(&values[0],Data::Text(v)if v=="configured"));
        self.events.lock().unwrap().push("configured".into());
        Ok(())
    }
    /// Append exit to the shared event log; no external I/O.
    ///
    /// # Arguments
    /// - `self` `[in,out]`: log to which exit is appended.
    /// - `context` `[in]`: unused exit context.
    /// - `params` `[in]`: unused unit payload.
    ///
    /// # Returns
    /// `Ok(())` after appending; no server error is returned.
    ///
    /// # Panics
    /// If the event mutex is poisoned.
    ///
    /// # Examples
    /// Local fixture context with a prepared `Context` and `()` payload:
    /// ```text
    /// let result = server.exit(context, params).await;
    /// ```
    async fn exit(&self, context: Context, params: ()) -> ServerResult<()> {
        self.events.lock().unwrap().push("exit".into());
        Ok(())
    }
    /// Reject the did_change_configuration hook as unsupported; discard its DidChangeConfigurationParams payload.
    /// Does not read/change events, send callbacks, or perform external I/O.
    ///
    /// # Arguments
    /// - `self` `[in]`: fixture server, not modified.
    /// - `context` `[in]`: unused request/notification context; no callbacks are made.
    /// - `params` `[in]`: consumed DidChangeConfigurationParams fixture input, deliberately ignored.
    ///
    /// # Returns
    /// `Err(ServerError::unsupported())` always; no success payload within `ServerResult<()>` is produced.
    ///
    /// # Errors
    /// Always returns the unsupported-method error.
    ///
    /// # Examples
    /// Local fixture context with a prepared `Context` and `DidChangeConfigurationParams` payload:
    /// ```text
    /// let result = server.did_change_configuration(context, params).await;
    /// ```
    async fn did_change_configuration(
        &self,
        context: Context,
        params: DidChangeConfigurationParams,
    ) -> ServerResult<()> {
        let _ = (context, params);
        Err(ServerError::unsupported())
    }
    /// Validate UTF-8, record the opened version, publish empty versioned diagnostics, and send partial progress.
    /// Writes protocol notifications to the client transport; never opens the document URI on disk.
    ///
    /// # Arguments
    /// - `self` `[in,out]`: log; appends `open:<version>` before sending notifications.
    /// - `context` `[in]`: UTF-8 context/client handle used for publication.
    /// - `params` `[in]`: opened document; its URI/version identify diagnostics, while text is unused.
    ///
    /// # Returns
    /// `Ok(())` after both notifications; `Err(ServerError)` on a send failure.
    ///
    /// # Errors
    /// Propagates diagnostic/progress send errors; the open event and any earlier notification remain emitted.
    ///
    /// # Panics
    /// If encoding is not UTF-8 or the event mutex is poisoned.
    ///
    /// # Algorithm
    /// Check UTF-8 and record the open version, then send empty diagnostics followed by
    /// token-99 partial progress. A failed send returns early without rolling back prior effects.
    ///
    /// # Flow
    /// ```text
    /// [check UTF-8 / append open]
    ///                |
    ///                v
    /// [publish diagnostics] ----error----> [return error]
    ///                |                           ^
    ///                v                           |
    /// [send partial progress] ---error-----------+
    ///                |
    ///                v
    ///           [return Ok]
    /// ```
    ///
    /// # Examples
    /// Local fixture context with a prepared `Context` and `DidOpenTextDocumentParams` payload:
    /// ```text
    /// let result = server.did_open_text_document(context, params).await;
    /// ```
    async fn did_open_text_document(
        &self,
        context: Context,
        params: DidOpenTextDocumentParams,
    ) -> ServerResult<()> {
        assert_eq!(context.encoding, CoordinateEncoding::Utf8);
        self.events
            .lock()
            .unwrap()
            .push(format!("open:{}", params.text_document.version));
        context
            .client
            .publish_diagnostics(DiagnosticReport {
                uri: params.text_document.uri,
                version: Some(params.text_document.version),
                diagnostics: vec![],
            })
            .await?;
        context
            .client
            .progress(ProgressUpdate {
                token: NumberOrString::Number(99),
                value: Data::List(vec![Data::Text("partial".into())]),
            })
            .await?;
        Ok(())
    }
    /// Append `change:<version>` to the event log; ignore text changes and perform no external I/O.
    ///
    /// # Arguments
    /// - `self` `[in,out]`: log; appends one change event.
    /// - `context` `[in]`: unused change context.
    /// - `params` `[in]`: changed document version; content changes are not applied to stored text.
    ///
    /// # Returns
    /// `Ok(())` after appending; no server error is returned.
    ///
    /// # Panics
    /// If the event mutex is poisoned.
    ///
    /// # Examples
    /// Local fixture context with a prepared `Context` and `DidChangeTextDocumentParams` payload:
    /// ```text
    /// let result = server.did_change_text_document(context, params).await;
    /// ```
    async fn did_change_text_document(
        &self,
        context: Context,
        params: DidChangeTextDocumentParams,
    ) -> ServerResult<()> {
        self.events
            .lock()
            .unwrap()
            .push(format!("change:{}", params.text_document.version));
        Ok(())
    }
    /// Reject the will_save_text_document hook as unsupported; discard its WillSaveTextDocumentParams payload.
    /// Does not read/change events, send callbacks, or perform external I/O.
    ///
    /// # Arguments
    /// - `self` `[in]`: fixture server, not modified.
    /// - `context` `[in]`: unused request/notification context; no callbacks are made.
    /// - `params` `[in]`: consumed WillSaveTextDocumentParams fixture input, deliberately ignored.
    ///
    /// # Returns
    /// `Err(ServerError::unsupported())` always; no success payload within `ServerResult<()>` is produced.
    ///
    /// # Errors
    /// Always returns the unsupported-method error.
    ///
    /// # Examples
    /// Local fixture context with a prepared `Context` and `WillSaveTextDocumentParams` payload:
    /// ```text
    /// let result = server.will_save_text_document(context, params).await;
    /// ```
    async fn will_save_text_document(
        &self,
        context: Context,
        params: WillSaveTextDocumentParams,
    ) -> ServerResult<()> {
        let _ = (context, params);
        Err(ServerError::unsupported())
    }
    /// Reject the did_close_text_document hook as unsupported; discard its DidCloseTextDocumentParams payload.
    /// Does not read/change events, send callbacks, or perform external I/O.
    ///
    /// # Arguments
    /// - `self` `[in]`: fixture server, not modified.
    /// - `context` `[in]`: unused request/notification context; no callbacks are made.
    /// - `params` `[in]`: consumed DidCloseTextDocumentParams fixture input, deliberately ignored.
    ///
    /// # Returns
    /// `Err(ServerError::unsupported())` always; no success payload within `ServerResult<()>` is produced.
    ///
    /// # Errors
    /// Always returns the unsupported-method error.
    ///
    /// # Examples
    /// Local fixture context with a prepared `Context` and `DidCloseTextDocumentParams` payload:
    /// ```text
    /// let result = server.did_close_text_document(context, params).await;
    /// ```
    async fn did_close_text_document(
        &self,
        context: Context,
        params: DidCloseTextDocumentParams,
    ) -> ServerResult<()> {
        let _ = (context, params);
        Err(ServerError::unsupported())
    }
    /// Reject the did_save_text_document hook as unsupported; discard its DidSaveTextDocumentParams payload.
    /// Does not read/change events, send callbacks, or perform external I/O.
    ///
    /// # Arguments
    /// - `self` `[in]`: fixture server, not modified.
    /// - `context` `[in]`: unused request/notification context; no callbacks are made.
    /// - `params` `[in]`: consumed DidSaveTextDocumentParams fixture input, deliberately ignored.
    ///
    /// # Returns
    /// `Err(ServerError::unsupported())` always; no success payload within `ServerResult<()>` is produced.
    ///
    /// # Errors
    /// Always returns the unsupported-method error.
    ///
    /// # Examples
    /// Local fixture context with a prepared `Context` and `DidSaveTextDocumentParams` payload:
    /// ```text
    /// let result = server.did_save_text_document(context, params).await;
    /// ```
    async fn did_save_text_document(
        &self,
        context: Context,
        params: DidSaveTextDocumentParams,
    ) -> ServerResult<()> {
        let _ = (context, params);
        Err(ServerError::unsupported())
    }
    /// Reject the did_open_notebook_document hook as unsupported; discard its DidOpenNotebookDocumentParams payload.
    /// Does not read/change events, send callbacks, or perform external I/O.
    ///
    /// # Arguments
    /// - `self` `[in]`: fixture server, not modified.
    /// - `context` `[in]`: unused request/notification context; no callbacks are made.
    /// - `params` `[in]`: consumed DidOpenNotebookDocumentParams fixture input, deliberately ignored.
    ///
    /// # Returns
    /// `Err(ServerError::unsupported())` always; no success payload within `ServerResult<()>` is produced.
    ///
    /// # Errors
    /// Always returns the unsupported-method error.
    ///
    /// # Examples
    /// Local fixture context with a prepared `Context` and `DidOpenNotebookDocumentParams` payload:
    /// ```text
    /// let result = server.did_open_notebook_document(context, params).await;
    /// ```
    async fn did_open_notebook_document(
        &self,
        context: Context,
        params: DidOpenNotebookDocumentParams,
    ) -> ServerResult<()> {
        let _ = (context, params);
        Err(ServerError::unsupported())
    }
    /// Reject the did_change_notebook_document hook as unsupported; discard its DidChangeNotebookDocumentParams payload.
    /// Does not read/change events, send callbacks, or perform external I/O.
    ///
    /// # Arguments
    /// - `self` `[in]`: fixture server, not modified.
    /// - `context` `[in]`: unused request/notification context; no callbacks are made.
    /// - `params` `[in]`: consumed DidChangeNotebookDocumentParams fixture input, deliberately ignored.
    ///
    /// # Returns
    /// `Err(ServerError::unsupported())` always; no success payload within `ServerResult<()>` is produced.
    ///
    /// # Errors
    /// Always returns the unsupported-method error.
    ///
    /// # Examples
    /// Local fixture context with a prepared `Context` and `DidChangeNotebookDocumentParams` payload:
    /// ```text
    /// let result = server.did_change_notebook_document(context, params).await;
    /// ```
    async fn did_change_notebook_document(
        &self,
        context: Context,
        params: DidChangeNotebookDocumentParams,
    ) -> ServerResult<()> {
        let _ = (context, params);
        Err(ServerError::unsupported())
    }
    /// Reject the did_save_notebook_document hook as unsupported; discard its DidSaveNotebookDocumentParams payload.
    /// Does not read/change events, send callbacks, or perform external I/O.
    ///
    /// # Arguments
    /// - `self` `[in]`: fixture server, not modified.
    /// - `context` `[in]`: unused request/notification context; no callbacks are made.
    /// - `params` `[in]`: consumed DidSaveNotebookDocumentParams fixture input, deliberately ignored.
    ///
    /// # Returns
    /// `Err(ServerError::unsupported())` always; no success payload within `ServerResult<()>` is produced.
    ///
    /// # Errors
    /// Always returns the unsupported-method error.
    ///
    /// # Examples
    /// Local fixture context with a prepared `Context` and `DidSaveNotebookDocumentParams` payload:
    /// ```text
    /// let result = server.did_save_notebook_document(context, params).await;
    /// ```
    async fn did_save_notebook_document(
        &self,
        context: Context,
        params: DidSaveNotebookDocumentParams,
    ) -> ServerResult<()> {
        let _ = (context, params);
        Err(ServerError::unsupported())
    }
    /// Reject the did_close_notebook_document hook as unsupported; discard its DidCloseNotebookDocumentParams payload.
    /// Does not read/change events, send callbacks, or perform external I/O.
    ///
    /// # Arguments
    /// - `self` `[in]`: fixture server, not modified.
    /// - `context` `[in]`: unused request/notification context; no callbacks are made.
    /// - `params` `[in]`: consumed DidCloseNotebookDocumentParams fixture input, deliberately ignored.
    ///
    /// # Returns
    /// `Err(ServerError::unsupported())` always; no success payload within `ServerResult<()>` is produced.
    ///
    /// # Errors
    /// Always returns the unsupported-method error.
    ///
    /// # Examples
    /// Local fixture context with a prepared `Context` and `DidCloseNotebookDocumentParams` payload:
    /// ```text
    /// let result = server.did_close_notebook_document(context, params).await;
    /// ```
    async fn did_close_notebook_document(
        &self,
        context: Context,
        params: DidCloseNotebookDocumentParams,
    ) -> ServerResult<()> {
        let _ = (context, params);
        Err(ServerError::unsupported())
    }
    /// Reject the did_change_watched_files hook as unsupported; discard its DidChangeWatchedFilesParams payload.
    /// Does not read/change events, send callbacks, or perform external I/O.
    ///
    /// # Arguments
    /// - `self` `[in]`: fixture server, not modified.
    /// - `context` `[in]`: unused request/notification context; no callbacks are made.
    /// - `params` `[in]`: consumed DidChangeWatchedFilesParams fixture input, deliberately ignored.
    ///
    /// # Returns
    /// `Err(ServerError::unsupported())` always; no success payload within `ServerResult<()>` is produced.
    ///
    /// # Errors
    /// Always returns the unsupported-method error.
    ///
    /// # Examples
    /// Local fixture context with a prepared `Context` and `DidChangeWatchedFilesParams` payload:
    /// ```text
    /// let result = server.did_change_watched_files(context, params).await;
    /// ```
    async fn did_change_watched_files(
        &self,
        context: Context,
        params: DidChangeWatchedFilesParams,
    ) -> ServerResult<()> {
        let _ = (context, params);
        Err(ServerError::unsupported())
    }
    /// Reject the did_change_workspace_folders hook as unsupported; discard its DidChangeWorkspaceFoldersParams payload.
    /// Does not read/change events, send callbacks, or perform external I/O.
    ///
    /// # Arguments
    /// - `self` `[in]`: fixture server, not modified.
    /// - `context` `[in]`: unused request/notification context; no callbacks are made.
    /// - `params` `[in]`: consumed DidChangeWorkspaceFoldersParams fixture input, deliberately ignored.
    ///
    /// # Returns
    /// `Err(ServerError::unsupported())` always; no success payload within `ServerResult<()>` is produced.
    ///
    /// # Errors
    /// Always returns the unsupported-method error.
    ///
    /// # Examples
    /// Local fixture context with a prepared `Context` and `DidChangeWorkspaceFoldersParams` payload:
    /// ```text
    /// let result = server.did_change_workspace_folders(context, params).await;
    /// ```
    async fn did_change_workspace_folders(
        &self,
        context: Context,
        params: DidChangeWorkspaceFoldersParams,
    ) -> ServerResult<()> {
        let _ = (context, params);
        Err(ServerError::unsupported())
    }
    /// Reject the work_done_progress_cancel hook as unsupported; discard its WorkDoneProgressCancelParams payload.
    /// Does not read/change events, send callbacks, or perform external I/O.
    ///
    /// # Arguments
    /// - `self` `[in]`: fixture server, not modified.
    /// - `context` `[in]`: unused request/notification context; no callbacks are made.
    /// - `params` `[in]`: consumed WorkDoneProgressCancelParams fixture input, deliberately ignored.
    ///
    /// # Returns
    /// `Err(ServerError::unsupported())` always; no success payload within `ServerResult<()>` is produced.
    ///
    /// # Errors
    /// Always returns the unsupported-method error.
    ///
    /// # Examples
    /// Local fixture context with a prepared `Context` and `WorkDoneProgressCancelParams` payload:
    /// ```text
    /// let result = server.work_done_progress_cancel(context, params).await;
    /// ```
    async fn work_done_progress_cancel(
        &self,
        context: Context,
        params: WorkDoneProgressCancelParams,
    ) -> ServerResult<()> {
        let _ = (context, params);
        Err(ServerError::unsupported())
    }
    /// Reject the did_create_files hook as unsupported; discard its CreateFilesParams payload.
    /// Does not read/change events, send callbacks, or perform external I/O.
    ///
    /// # Arguments
    /// - `self` `[in]`: fixture server, not modified.
    /// - `context` `[in]`: unused request/notification context; no callbacks are made.
    /// - `params` `[in]`: consumed CreateFilesParams fixture input, deliberately ignored.
    ///
    /// # Returns
    /// `Err(ServerError::unsupported())` always; no success payload within `ServerResult<()>` is produced.
    ///
    /// # Errors
    /// Always returns the unsupported-method error.
    ///
    /// # Examples
    /// Local fixture context with a prepared `Context` and `CreateFilesParams` payload:
    /// ```text
    /// let result = server.did_create_files(context, params).await;
    /// ```
    async fn did_create_files(
        &self,
        context: Context,
        params: CreateFilesParams,
    ) -> ServerResult<()> {
        let _ = (context, params);
        Err(ServerError::unsupported())
    }
    /// Reject the did_rename_files hook as unsupported; discard its RenameFilesParams payload.
    /// Does not read/change events, send callbacks, or perform external I/O.
    ///
    /// # Arguments
    /// - `self` `[in]`: fixture server, not modified.
    /// - `context` `[in]`: unused request/notification context; no callbacks are made.
    /// - `params` `[in]`: consumed RenameFilesParams fixture input, deliberately ignored.
    ///
    /// # Returns
    /// `Err(ServerError::unsupported())` always; no success payload within `ServerResult<()>` is produced.
    ///
    /// # Errors
    /// Always returns the unsupported-method error.
    ///
    /// # Examples
    /// Local fixture context with a prepared `Context` and `RenameFilesParams` payload:
    /// ```text
    /// let result = server.did_rename_files(context, params).await;
    /// ```
    async fn did_rename_files(
        &self,
        context: Context,
        params: RenameFilesParams,
    ) -> ServerResult<()> {
        let _ = (context, params);
        Err(ServerError::unsupported())
    }
    /// Reject the did_delete_files hook as unsupported; discard its DeleteFilesParams payload.
    /// Does not read/change events, send callbacks, or perform external I/O.
    ///
    /// # Arguments
    /// - `self` `[in]`: fixture server, not modified.
    /// - `context` `[in]`: unused request/notification context; no callbacks are made.
    /// - `params` `[in]`: consumed DeleteFilesParams fixture input, deliberately ignored.
    ///
    /// # Returns
    /// `Err(ServerError::unsupported())` always; no success payload within `ServerResult<()>` is produced.
    ///
    /// # Errors
    /// Always returns the unsupported-method error.
    ///
    /// # Examples
    /// Local fixture context with a prepared `Context` and `DeleteFilesParams` payload:
    /// ```text
    /// let result = server.did_delete_files(context, params).await;
    /// ```
    async fn did_delete_files(
        &self,
        context: Context,
        params: DeleteFilesParams,
    ) -> ServerResult<()> {
        let _ = (context, params);
        Err(ServerError::unsupported())
    }
    /// Handle wait, events, echo, and panic extension fixtures; reject other methods.
    /// Wait observes cancellation, events serializes the log into JSON, and echo moves parameters back. No external I/O.
    ///
    /// # Arguments
    /// - `self` `[in]`: event log read only for custom/events.
    /// - `context` `[in]`: cancellation token awaited by custom/wait.
    /// - `params` `[in]`: extension method and optional JSON; echo returns null when parameters are absent.
    ///
    /// # Returns
    /// `Ok(JSON)` for custom/events (event array) or custom/echo (parameters or null);
    /// `Err(ServerError)` for cancellation or unsupported methods. custom/panic does not return.
    ///
    /// # Errors
    /// custom/wait returns cancelled after the token fires; unknown methods return unsupported.
    ///
    /// # Panics
    /// custom/panic intentionally panics; custom/events panics if the mutex is poisoned.
    ///
    /// # Algorithm
    /// Match the method string without decoding parameters. Wait blocks on the cancellation token;
    /// events serializes the locked log; echo moves the optional JSON. The panic branch is intentional.
    ///
    /// # Flow
    /// ```text
    ///                       < method >
    ///             /        /     |      \          \
    ///           wait    events  echo    panic      other
    ///            |         |      |       |          |
    ///            v         v      v       v          v
    ///      [await token] [log] [params] [panic] [unsupported]
    ///            |         |      |
    ///            v         +---+--+
    ///       [cancelled]        v
    ///                       [Ok JSON]
    /// ```
    ///
    /// # Examples
    /// Local fixture context with a prepared `Context` and `Extension` payload:
    /// ```text
    /// let result = server.custom_request(context, params).await;
    /// ```
    async fn custom_request(
        &self,
        context: Context,
        params: Extension,
    ) -> ServerResult<serde_json::Value> {
        if params.method == "custom/wait" {
            context.cancellation.cancelled().await;
            return Err(ServerError::cancelled());
        }
        if params.method == "custom/events" {
            return Ok(serde_json::json!(*self.events.lock().unwrap()));
        }
        if params.method == "custom/echo" {
            return Ok(params.params.unwrap_or(serde_json::Value::Null));
        }
        if params.method == "custom/panic" {
            panic!("fixture panic");
        }
        Err(ServerError::unsupported())
    }
    /// Append the extension method name to events; ignore its parameters and perform no external I/O.
    ///
    /// # Arguments
    /// - `self` `[in,out]`: shared log; appends one method name.
    /// - `context` `[in]`: unused notification context.
    /// - `params` `[in]`: extension whose method is moved into the log.
    ///
    /// # Returns
    /// `Ok(())` after appending; no server error is returned.
    ///
    /// # Panics
    /// If the event mutex is poisoned.
    ///
    /// # Examples
    /// Local fixture context with a prepared `Context` and `Extension` payload:
    /// ```text
    /// let result = server.custom_notification(context, params).await;
    /// ```
    async fn custom_notification(&self, context: Context, params: Extension) -> ServerResult<()> {
        self.events.lock().unwrap().push(params.method);
        Ok(())
    }
}
