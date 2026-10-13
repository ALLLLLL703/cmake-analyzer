// Script-generated test scaffolding by scripts/gen/lspally-schema/trait_fixture.py.
// Contains AI-generated test edits.
#![allow(unused_variables)]
use lspally::server::model::*;
use std::sync::{Arc, Mutex};
pub(crate) struct Server {
    pub(crate) events: Arc<Mutex<Vec<String>>>,
}
impl ILanguageServer for Server {
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
    async fn shutdown(&self, context: Context, params: ()) -> ServerResult<()> {
        Ok(())
    }
    async fn completion(
        &self,
        context: Context,
        params: CompletionParams,
    ) -> ServerResult<Option<CompletionResponse>> {
        let _ = (context, params);
        Err(ServerError::unsupported())
    }
    async fn resolve_completion_item(
        &self,
        context: Context,
        params: CompletionItem,
    ) -> ServerResult<CompletionItem> {
        let _ = (context, params);
        Err(ServerError::unsupported())
    }
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
    async fn signature_help_request(
        &self,
        context: Context,
        params: SignatureHelpParams,
    ) -> ServerResult<Option<SignatureHelp>> {
        let _ = (context, params);
        Err(ServerError::unsupported())
    }
    async fn goto_declaration(
        &self,
        context: Context,
        params: GotoDeclarationParams,
    ) -> ServerResult<Option<GotoDeclarationResponse>> {
        let _ = (context, params);
        Err(ServerError::unsupported())
    }
    async fn goto_definition(
        &self,
        context: Context,
        params: GotoDefinitionParams,
    ) -> ServerResult<Option<GotoDefinitionResponse>> {
        let _ = (context, params);
        Err(ServerError::unsupported())
    }
    async fn references(
        &self,
        context: Context,
        params: ReferenceParams,
    ) -> ServerResult<Option<Vec<Location>>> {
        let _ = (context, params);
        Err(ServerError::unsupported())
    }
    async fn goto_type_definition(
        &self,
        context: Context,
        params: GotoTypeDefinitionParams,
    ) -> ServerResult<Option<GotoTypeDefinitionResponse>> {
        let _ = (context, params);
        Err(ServerError::unsupported())
    }
    async fn goto_implementation(
        &self,
        context: Context,
        params: GotoImplementationParams,
    ) -> ServerResult<Option<GotoImplementationResponse>> {
        let _ = (context, params);
        Err(ServerError::unsupported())
    }
    async fn document_highlight_request(
        &self,
        context: Context,
        params: DocumentHighlightParams,
    ) -> ServerResult<Option<Vec<DocumentHighlight>>> {
        let _ = (context, params);
        Err(ServerError::unsupported())
    }
    async fn document_symbol_request(
        &self,
        context: Context,
        params: DocumentSymbolParams,
    ) -> ServerResult<Option<DocumentSymbolResponse>> {
        let _ = (context, params);
        Err(ServerError::unsupported())
    }
    async fn workspace_symbol_request(
        &self,
        context: Context,
        params: WorkspaceSymbolParams,
    ) -> ServerResult<Option<WorkspaceSymbolResponse>> {
        let _ = (context, params);
        Err(ServerError::unsupported())
    }
    async fn workspace_symbol_resolve(
        &self,
        context: Context,
        params: WorkspaceSymbol,
    ) -> ServerResult<WorkspaceSymbol> {
        let _ = (context, params);
        Err(ServerError::unsupported())
    }
    async fn execute_command(
        &self,
        context: Context,
        params: ExecuteCommandParams,
    ) -> ServerResult<Option<Data>> {
        let _ = (context, params);
        Err(ServerError::unsupported())
    }
    async fn will_save_wait_until(
        &self,
        context: Context,
        params: WillSaveTextDocumentParams,
    ) -> ServerResult<Option<Vec<TextEdit>>> {
        let _ = (context, params);
        Err(ServerError::unsupported())
    }
    async fn code_action_request(
        &self,
        context: Context,
        params: CodeActionParams,
    ) -> ServerResult<Option<CodeActionResponse>> {
        let _ = (context, params);
        Err(ServerError::unsupported())
    }
    async fn code_action_resolve_request(
        &self,
        context: Context,
        params: CodeAction,
    ) -> ServerResult<CodeAction> {
        let _ = (context, params);
        Err(ServerError::unsupported())
    }
    async fn code_lens_request(
        &self,
        context: Context,
        params: CodeLensParams,
    ) -> ServerResult<Option<Vec<CodeLens>>> {
        let _ = (context, params);
        Err(ServerError::unsupported())
    }
    async fn code_lens_resolve(
        &self,
        context: Context,
        params: CodeLens,
    ) -> ServerResult<CodeLens> {
        let _ = (context, params);
        Err(ServerError::unsupported())
    }
    async fn document_link_request(
        &self,
        context: Context,
        params: DocumentLinkParams,
    ) -> ServerResult<Option<Vec<DocumentLink>>> {
        let _ = (context, params);
        Err(ServerError::unsupported())
    }
    async fn document_link_resolve(
        &self,
        context: Context,
        params: DocumentLink,
    ) -> ServerResult<DocumentLink> {
        let _ = (context, params);
        Err(ServerError::unsupported())
    }
    async fn formatting(
        &self,
        context: Context,
        params: DocumentFormattingParams,
    ) -> ServerResult<Option<Vec<TextEdit>>> {
        let _ = (context, params);
        Err(ServerError::unsupported())
    }
    async fn range_formatting(
        &self,
        context: Context,
        params: DocumentRangeFormattingParams,
    ) -> ServerResult<Option<Vec<TextEdit>>> {
        let _ = (context, params);
        Err(ServerError::unsupported())
    }
    async fn on_type_formatting(
        &self,
        context: Context,
        params: DocumentOnTypeFormattingParams,
    ) -> ServerResult<Option<Vec<TextEdit>>> {
        let _ = (context, params);
        Err(ServerError::unsupported())
    }
    async fn linked_editing_range(
        &self,
        context: Context,
        params: LinkedEditingRangeParams,
    ) -> ServerResult<Option<LinkedEditingRanges>> {
        let _ = (context, params);
        Err(ServerError::unsupported())
    }
    async fn rename(
        &self,
        context: Context,
        params: RenameParams,
    ) -> ServerResult<Option<WorkspaceEdit>> {
        let _ = (context, params);
        Err(ServerError::unsupported())
    }
    async fn document_color(
        &self,
        context: Context,
        params: DocumentColorParams,
    ) -> ServerResult<Vec<ColorInformation>> {
        let _ = (context, params);
        Err(ServerError::unsupported())
    }
    async fn color_presentation_request(
        &self,
        context: Context,
        params: ColorPresentationParams,
    ) -> ServerResult<Vec<ColorPresentation>> {
        let _ = (context, params);
        Err(ServerError::unsupported())
    }
    async fn folding_range_request(
        &self,
        context: Context,
        params: FoldingRangeParams,
    ) -> ServerResult<Option<Vec<FoldingRange>>> {
        let _ = (context, params);
        Err(ServerError::unsupported())
    }
    async fn prepare_rename_request(
        &self,
        context: Context,
        params: TextDocumentPositionParams,
    ) -> ServerResult<Option<PrepareRenameResponse>> {
        let _ = (context, params);
        Err(ServerError::unsupported())
    }
    async fn selection_range_request(
        &self,
        context: Context,
        params: SelectionRangeParams,
    ) -> ServerResult<Option<Vec<SelectionRange>>> {
        let _ = (context, params);
        Err(ServerError::unsupported())
    }
    async fn call_hierarchy_prepare(
        &self,
        context: Context,
        params: CallHierarchyPrepareParams,
    ) -> ServerResult<Option<Vec<CallHierarchyItem>>> {
        let _ = (context, params);
        Err(ServerError::unsupported())
    }
    async fn call_hierarchy_incoming_calls(
        &self,
        context: Context,
        params: CallHierarchyIncomingCallsParams,
    ) -> ServerResult<Option<Vec<CallHierarchyIncomingCall>>> {
        let _ = (context, params);
        Err(ServerError::unsupported())
    }
    async fn call_hierarchy_outgoing_calls(
        &self,
        context: Context,
        params: CallHierarchyOutgoingCallsParams,
    ) -> ServerResult<Option<Vec<CallHierarchyOutgoingCall>>> {
        let _ = (context, params);
        Err(ServerError::unsupported())
    }
    async fn semantic_tokens_full_request(
        &self,
        context: Context,
        params: SemanticTokensParams,
    ) -> ServerResult<Option<SemanticTokensResult>> {
        let _ = (context, params);
        Err(ServerError::unsupported())
    }
    async fn semantic_tokens_full_delta_request(
        &self,
        context: Context,
        params: SemanticTokensDeltaParams,
    ) -> ServerResult<Option<SemanticTokensFullDeltaResult>> {
        let _ = (context, params);
        Err(ServerError::unsupported())
    }
    async fn semantic_tokens_range_request(
        &self,
        context: Context,
        params: SemanticTokensRangeParams,
    ) -> ServerResult<Option<SemanticTokensRangeResult>> {
        let _ = (context, params);
        Err(ServerError::unsupported())
    }
    async fn will_create_files(
        &self,
        context: Context,
        params: CreateFilesParams,
    ) -> ServerResult<Option<WorkspaceEdit>> {
        let _ = (context, params);
        Err(ServerError::unsupported())
    }
    async fn will_rename_files(
        &self,
        context: Context,
        params: RenameFilesParams,
    ) -> ServerResult<Option<WorkspaceEdit>> {
        let _ = (context, params);
        Err(ServerError::unsupported())
    }
    async fn will_delete_files(
        &self,
        context: Context,
        params: DeleteFilesParams,
    ) -> ServerResult<Option<WorkspaceEdit>> {
        let _ = (context, params);
        Err(ServerError::unsupported())
    }
    async fn moniker_request(
        &self,
        context: Context,
        params: MonikerParams,
    ) -> ServerResult<Option<Vec<Moniker>>> {
        let _ = (context, params);
        Err(ServerError::unsupported())
    }
    async fn inlay_hint_request(
        &self,
        context: Context,
        params: InlayHintParams,
    ) -> ServerResult<Option<Vec<InlayHint>>> {
        let _ = (context, params);
        Err(ServerError::unsupported())
    }
    async fn inlay_hint_resolve_request(
        &self,
        context: Context,
        params: InlayHint,
    ) -> ServerResult<InlayHint> {
        let _ = (context, params);
        Err(ServerError::unsupported())
    }
    async fn inline_value_request(
        &self,
        context: Context,
        params: InlineValueParams,
    ) -> ServerResult<Option<Vec<InlineValue>>> {
        let _ = (context, params);
        Err(ServerError::unsupported())
    }
    async fn document_diagnostic_request(
        &self,
        context: Context,
        params: DocumentDiagnosticParams,
    ) -> ServerResult<DocumentDiagnosticReportResult> {
        let _ = (context, params);
        Err(ServerError::unsupported())
    }
    async fn workspace_diagnostic_request(
        &self,
        context: Context,
        params: WorkspaceDiagnosticParams,
    ) -> ServerResult<WorkspaceDiagnosticReportResult> {
        let _ = (context, params);
        Err(ServerError::unsupported())
    }
    async fn type_hierarchy_prepare(
        &self,
        context: Context,
        params: TypeHierarchyPrepareParams,
    ) -> ServerResult<Option<Vec<TypeHierarchyItem>>> {
        let _ = (context, params);
        Err(ServerError::unsupported())
    }
    async fn type_hierarchy_supertypes(
        &self,
        context: Context,
        params: TypeHierarchySupertypesParams,
    ) -> ServerResult<Option<Vec<TypeHierarchyItem>>> {
        let _ = (context, params);
        Err(ServerError::unsupported())
    }
    async fn type_hierarchy_subtypes(
        &self,
        context: Context,
        params: TypeHierarchySubtypesParams,
    ) -> ServerResult<Option<Vec<TypeHierarchyItem>>> {
        let _ = (context, params);
        Err(ServerError::unsupported())
    }
    async fn cancel(&self, context: Context, params: CancelParams) -> ServerResult<()> {
        Ok(())
    }
    async fn set_trace(&self, context: Context, params: SetTraceParams) -> ServerResult<()> {
        Ok(())
    }
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
    async fn exit(&self, context: Context, params: ()) -> ServerResult<()> {
        self.events.lock().unwrap().push("exit".into());
        Ok(())
    }
    async fn did_change_configuration(
        &self,
        context: Context,
        params: DidChangeConfigurationParams,
    ) -> ServerResult<()> {
        let _ = (context, params);
        Err(ServerError::unsupported())
    }
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
    async fn will_save_text_document(
        &self,
        context: Context,
        params: WillSaveTextDocumentParams,
    ) -> ServerResult<()> {
        let _ = (context, params);
        Err(ServerError::unsupported())
    }
    async fn did_close_text_document(
        &self,
        context: Context,
        params: DidCloseTextDocumentParams,
    ) -> ServerResult<()> {
        let _ = (context, params);
        Err(ServerError::unsupported())
    }
    async fn did_save_text_document(
        &self,
        context: Context,
        params: DidSaveTextDocumentParams,
    ) -> ServerResult<()> {
        let _ = (context, params);
        Err(ServerError::unsupported())
    }
    async fn did_open_notebook_document(
        &self,
        context: Context,
        params: DidOpenNotebookDocumentParams,
    ) -> ServerResult<()> {
        let _ = (context, params);
        Err(ServerError::unsupported())
    }
    async fn did_change_notebook_document(
        &self,
        context: Context,
        params: DidChangeNotebookDocumentParams,
    ) -> ServerResult<()> {
        let _ = (context, params);
        Err(ServerError::unsupported())
    }
    async fn did_save_notebook_document(
        &self,
        context: Context,
        params: DidSaveNotebookDocumentParams,
    ) -> ServerResult<()> {
        let _ = (context, params);
        Err(ServerError::unsupported())
    }
    async fn did_close_notebook_document(
        &self,
        context: Context,
        params: DidCloseNotebookDocumentParams,
    ) -> ServerResult<()> {
        let _ = (context, params);
        Err(ServerError::unsupported())
    }
    async fn did_change_watched_files(
        &self,
        context: Context,
        params: DidChangeWatchedFilesParams,
    ) -> ServerResult<()> {
        let _ = (context, params);
        Err(ServerError::unsupported())
    }
    async fn did_change_workspace_folders(
        &self,
        context: Context,
        params: DidChangeWorkspaceFoldersParams,
    ) -> ServerResult<()> {
        let _ = (context, params);
        Err(ServerError::unsupported())
    }
    async fn work_done_progress_cancel(
        &self,
        context: Context,
        params: WorkDoneProgressCancelParams,
    ) -> ServerResult<()> {
        let _ = (context, params);
        Err(ServerError::unsupported())
    }
    async fn did_create_files(
        &self,
        context: Context,
        params: CreateFilesParams,
    ) -> ServerResult<()> {
        let _ = (context, params);
        Err(ServerError::unsupported())
    }
    async fn did_rename_files(
        &self,
        context: Context,
        params: RenameFilesParams,
    ) -> ServerResult<()> {
        let _ = (context, params);
        Err(ServerError::unsupported())
    }
    async fn did_delete_files(
        &self,
        context: Context,
        params: DeleteFilesParams,
    ) -> ServerResult<()> {
        let _ = (context, params);
        Err(ServerError::unsupported())
    }
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
    async fn custom_notification(&self, context: Context, params: Extension) -> ServerResult<()> {
        self.events.lock().unwrap().push(params.method);
        Ok(())
    }
}
