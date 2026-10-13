// Contains AI-authored additions or edits.
use std::{
    collections::{BTreeMap, HashMap, HashSet},
    future::Future,
    sync::{
        Arc,
        atomic::{AtomicBool, AtomicI32, AtomicU8},
    },
};
use tokio::sync::{Mutex, mpsc, oneshot};
mod client;
pub use client::*;
mod context;
mod data;
mod extension;
pub use context::*;
pub use data::*;
pub use extension::*;
mod call_hierarchy;
pub use call_hierarchy::*;
mod code_action;
pub use code_action::*;
mod code_lens;
pub use code_lens::*;
mod color;
pub use color::*;
mod document;
pub use document::*;
mod diagnostic;
pub use diagnostic::*;
mod lifecycle;
pub use lifecycle::*;
mod capabilities;
pub use capabilities::*;
mod edit;
pub use edit::*;
mod workspace;
pub use workspace::*;
mod common;
pub use common::*;
mod completion;
pub use completion::*;
mod document_diagnostic;
pub use document_diagnostic::*;
mod document_highlight;
pub use document_highlight::*;
mod document_link;
pub use document_link::*;
mod document_symbols;
pub use document_symbols::*;
mod file_operations;
pub use file_operations::*;
mod folding_range;
pub use folding_range::*;
mod formatting;
pub use formatting::*;
mod hover;
pub use hover::*;
mod inlay_hint;
pub use inlay_hint::*;
mod inline_value;
pub use inline_value::*;
mod linked_editing;
pub use linked_editing::*;
mod moniker;
pub use moniker::*;
mod notebook;
pub use notebook::*;
mod progress;
pub use progress::*;
mod references;
pub use references::*;
mod rename;
pub use rename::*;
mod selection_range;
pub use selection_range::*;
mod semantic_tokens;
pub use semantic_tokens::*;
mod signature_help;
pub use signature_help::*;
mod trace;
pub use trace::*;
mod type_hierarchy;
pub use type_hierarchy::*;
mod window;
pub use window::*;
mod workspace_diagnostic;
pub use workspace_diagnostic::*;
mod workspace_folders;
pub use workspace_folders::*;
mod workspace_symbols;
pub use workspace_symbols::*;

// Script-generated standard method declarations by scripts/gen/lspally-schema/src/main.rs; custom hooks are AI-authored.
/// All incoming methods require an explicit implementation. No capability is implied by this trait.
pub trait ILanguageServer: Send + Sync + 'static {
    fn initialize(
        &self,
        context: Context,
        params: InitializeParams,
    ) -> impl Future<Output = ServerResult<InitializeResult>> + Send;
    fn shutdown(
        &self,
        context: Context,
        params: (),
    ) -> impl Future<Output = ServerResult<()>> + Send;
    fn completion(
        &self,
        context: Context,
        params: CompletionParams,
    ) -> impl Future<Output = ServerResult<Option<CompletionResponse>>> + Send;
    fn resolve_completion_item(
        &self,
        context: Context,
        params: CompletionItem,
    ) -> impl Future<Output = ServerResult<CompletionItem>> + Send;
    fn hover_request(
        &self,
        context: Context,
        params: HoverParams,
    ) -> impl Future<Output = ServerResult<Option<Hover>>> + Send;
    fn signature_help_request(
        &self,
        context: Context,
        params: SignatureHelpParams,
    ) -> impl Future<Output = ServerResult<Option<SignatureHelp>>> + Send;
    fn goto_declaration(
        &self,
        context: Context,
        params: GotoDeclarationParams,
    ) -> impl Future<Output = ServerResult<Option<GotoDeclarationResponse>>> + Send;
    fn goto_definition(
        &self,
        context: Context,
        params: GotoDefinitionParams,
    ) -> impl Future<Output = ServerResult<Option<GotoDefinitionResponse>>> + Send;
    fn references(
        &self,
        context: Context,
        params: ReferenceParams,
    ) -> impl Future<Output = ServerResult<Option<Vec<Location>>>> + Send;
    fn goto_type_definition(
        &self,
        context: Context,
        params: GotoTypeDefinitionParams,
    ) -> impl Future<Output = ServerResult<Option<GotoTypeDefinitionResponse>>> + Send;
    fn goto_implementation(
        &self,
        context: Context,
        params: GotoImplementationParams,
    ) -> impl Future<Output = ServerResult<Option<GotoImplementationResponse>>> + Send;
    fn document_highlight_request(
        &self,
        context: Context,
        params: DocumentHighlightParams,
    ) -> impl Future<Output = ServerResult<Option<Vec<DocumentHighlight>>>> + Send;
    fn document_symbol_request(
        &self,
        context: Context,
        params: DocumentSymbolParams,
    ) -> impl Future<Output = ServerResult<Option<DocumentSymbolResponse>>> + Send;
    fn workspace_symbol_request(
        &self,
        context: Context,
        params: WorkspaceSymbolParams,
    ) -> impl Future<Output = ServerResult<Option<WorkspaceSymbolResponse>>> + Send;
    fn workspace_symbol_resolve(
        &self,
        context: Context,
        params: WorkspaceSymbol,
    ) -> impl Future<Output = ServerResult<WorkspaceSymbol>> + Send;
    fn execute_command(
        &self,
        context: Context,
        params: ExecuteCommandParams,
    ) -> impl Future<Output = ServerResult<Option<Data>>> + Send;
    fn will_save_wait_until(
        &self,
        context: Context,
        params: WillSaveTextDocumentParams,
    ) -> impl Future<Output = ServerResult<Option<Vec<TextEdit>>>> + Send;
    fn code_action_request(
        &self,
        context: Context,
        params: CodeActionParams,
    ) -> impl Future<Output = ServerResult<Option<CodeActionResponse>>> + Send;
    fn code_action_resolve_request(
        &self,
        context: Context,
        params: CodeAction,
    ) -> impl Future<Output = ServerResult<CodeAction>> + Send;
    fn code_lens_request(
        &self,
        context: Context,
        params: CodeLensParams,
    ) -> impl Future<Output = ServerResult<Option<Vec<CodeLens>>>> + Send;
    fn code_lens_resolve(
        &self,
        context: Context,
        params: CodeLens,
    ) -> impl Future<Output = ServerResult<CodeLens>> + Send;
    fn document_link_request(
        &self,
        context: Context,
        params: DocumentLinkParams,
    ) -> impl Future<Output = ServerResult<Option<Vec<DocumentLink>>>> + Send;
    fn document_link_resolve(
        &self,
        context: Context,
        params: DocumentLink,
    ) -> impl Future<Output = ServerResult<DocumentLink>> + Send;
    fn formatting(
        &self,
        context: Context,
        params: DocumentFormattingParams,
    ) -> impl Future<Output = ServerResult<Option<Vec<TextEdit>>>> + Send;
    fn range_formatting(
        &self,
        context: Context,
        params: DocumentRangeFormattingParams,
    ) -> impl Future<Output = ServerResult<Option<Vec<TextEdit>>>> + Send;
    fn on_type_formatting(
        &self,
        context: Context,
        params: DocumentOnTypeFormattingParams,
    ) -> impl Future<Output = ServerResult<Option<Vec<TextEdit>>>> + Send;
    fn linked_editing_range(
        &self,
        context: Context,
        params: LinkedEditingRangeParams,
    ) -> impl Future<Output = ServerResult<Option<LinkedEditingRanges>>> + Send;
    fn rename(
        &self,
        context: Context,
        params: RenameParams,
    ) -> impl Future<Output = ServerResult<Option<WorkspaceEdit>>> + Send;
    fn document_color(
        &self,
        context: Context,
        params: DocumentColorParams,
    ) -> impl Future<Output = ServerResult<Vec<ColorInformation>>> + Send;
    fn color_presentation_request(
        &self,
        context: Context,
        params: ColorPresentationParams,
    ) -> impl Future<Output = ServerResult<Vec<ColorPresentation>>> + Send;
    fn folding_range_request(
        &self,
        context: Context,
        params: FoldingRangeParams,
    ) -> impl Future<Output = ServerResult<Option<Vec<FoldingRange>>>> + Send;
    fn prepare_rename_request(
        &self,
        context: Context,
        params: TextDocumentPositionParams,
    ) -> impl Future<Output = ServerResult<Option<PrepareRenameResponse>>> + Send;
    fn selection_range_request(
        &self,
        context: Context,
        params: SelectionRangeParams,
    ) -> impl Future<Output = ServerResult<Option<Vec<SelectionRange>>>> + Send;
    fn call_hierarchy_prepare(
        &self,
        context: Context,
        params: CallHierarchyPrepareParams,
    ) -> impl Future<Output = ServerResult<Option<Vec<CallHierarchyItem>>>> + Send;
    fn call_hierarchy_incoming_calls(
        &self,
        context: Context,
        params: CallHierarchyIncomingCallsParams,
    ) -> impl Future<Output = ServerResult<Option<Vec<CallHierarchyIncomingCall>>>> + Send;
    fn call_hierarchy_outgoing_calls(
        &self,
        context: Context,
        params: CallHierarchyOutgoingCallsParams,
    ) -> impl Future<Output = ServerResult<Option<Vec<CallHierarchyOutgoingCall>>>> + Send;
    fn semantic_tokens_full_request(
        &self,
        context: Context,
        params: SemanticTokensParams,
    ) -> impl Future<Output = ServerResult<Option<SemanticTokensResult>>> + Send;
    fn semantic_tokens_full_delta_request(
        &self,
        context: Context,
        params: SemanticTokensDeltaParams,
    ) -> impl Future<Output = ServerResult<Option<SemanticTokensFullDeltaResult>>> + Send;
    fn semantic_tokens_range_request(
        &self,
        context: Context,
        params: SemanticTokensRangeParams,
    ) -> impl Future<Output = ServerResult<Option<SemanticTokensRangeResult>>> + Send;
    fn will_create_files(
        &self,
        context: Context,
        params: CreateFilesParams,
    ) -> impl Future<Output = ServerResult<Option<WorkspaceEdit>>> + Send;
    fn will_rename_files(
        &self,
        context: Context,
        params: RenameFilesParams,
    ) -> impl Future<Output = ServerResult<Option<WorkspaceEdit>>> + Send;
    fn will_delete_files(
        &self,
        context: Context,
        params: DeleteFilesParams,
    ) -> impl Future<Output = ServerResult<Option<WorkspaceEdit>>> + Send;
    fn moniker_request(
        &self,
        context: Context,
        params: MonikerParams,
    ) -> impl Future<Output = ServerResult<Option<Vec<Moniker>>>> + Send;
    fn inlay_hint_request(
        &self,
        context: Context,
        params: InlayHintParams,
    ) -> impl Future<Output = ServerResult<Option<Vec<InlayHint>>>> + Send;
    fn inlay_hint_resolve_request(
        &self,
        context: Context,
        params: InlayHint,
    ) -> impl Future<Output = ServerResult<InlayHint>> + Send;
    fn inline_value_request(
        &self,
        context: Context,
        params: InlineValueParams,
    ) -> impl Future<Output = ServerResult<Option<Vec<InlineValue>>>> + Send;
    fn document_diagnostic_request(
        &self,
        context: Context,
        params: DocumentDiagnosticParams,
    ) -> impl Future<Output = ServerResult<DocumentDiagnosticReportResult>> + Send;
    fn workspace_diagnostic_request(
        &self,
        context: Context,
        params: WorkspaceDiagnosticParams,
    ) -> impl Future<Output = ServerResult<WorkspaceDiagnosticReportResult>> + Send;
    fn type_hierarchy_prepare(
        &self,
        context: Context,
        params: TypeHierarchyPrepareParams,
    ) -> impl Future<Output = ServerResult<Option<Vec<TypeHierarchyItem>>>> + Send;
    fn type_hierarchy_supertypes(
        &self,
        context: Context,
        params: TypeHierarchySupertypesParams,
    ) -> impl Future<Output = ServerResult<Option<Vec<TypeHierarchyItem>>>> + Send;
    fn type_hierarchy_subtypes(
        &self,
        context: Context,
        params: TypeHierarchySubtypesParams,
    ) -> impl Future<Output = ServerResult<Option<Vec<TypeHierarchyItem>>>> + Send;
    fn cancel(
        &self,
        context: Context,
        params: CancelParams,
    ) -> impl Future<Output = ServerResult<()>> + Send;
    fn set_trace(
        &self,
        context: Context,
        params: SetTraceParams,
    ) -> impl Future<Output = ServerResult<()>> + Send;
    fn initialized(
        &self,
        context: Context,
        params: InitializedParams,
    ) -> impl Future<Output = ServerResult<()>> + Send;
    fn exit(&self, context: Context, params: ()) -> impl Future<Output = ServerResult<()>> + Send;
    fn did_change_configuration(
        &self,
        context: Context,
        params: DidChangeConfigurationParams,
    ) -> impl Future<Output = ServerResult<()>> + Send;
    fn did_open_text_document(
        &self,
        context: Context,
        params: DidOpenTextDocumentParams,
    ) -> impl Future<Output = ServerResult<()>> + Send;
    fn did_change_text_document(
        &self,
        context: Context,
        params: DidChangeTextDocumentParams,
    ) -> impl Future<Output = ServerResult<()>> + Send;
    fn will_save_text_document(
        &self,
        context: Context,
        params: WillSaveTextDocumentParams,
    ) -> impl Future<Output = ServerResult<()>> + Send;
    fn did_close_text_document(
        &self,
        context: Context,
        params: DidCloseTextDocumentParams,
    ) -> impl Future<Output = ServerResult<()>> + Send;
    fn did_save_text_document(
        &self,
        context: Context,
        params: DidSaveTextDocumentParams,
    ) -> impl Future<Output = ServerResult<()>> + Send;
    fn did_open_notebook_document(
        &self,
        context: Context,
        params: DidOpenNotebookDocumentParams,
    ) -> impl Future<Output = ServerResult<()>> + Send;
    fn did_change_notebook_document(
        &self,
        context: Context,
        params: DidChangeNotebookDocumentParams,
    ) -> impl Future<Output = ServerResult<()>> + Send;
    fn did_save_notebook_document(
        &self,
        context: Context,
        params: DidSaveNotebookDocumentParams,
    ) -> impl Future<Output = ServerResult<()>> + Send;
    fn did_close_notebook_document(
        &self,
        context: Context,
        params: DidCloseNotebookDocumentParams,
    ) -> impl Future<Output = ServerResult<()>> + Send;
    fn did_change_watched_files(
        &self,
        context: Context,
        params: DidChangeWatchedFilesParams,
    ) -> impl Future<Output = ServerResult<()>> + Send;
    fn did_change_workspace_folders(
        &self,
        context: Context,
        params: DidChangeWorkspaceFoldersParams,
    ) -> impl Future<Output = ServerResult<()>> + Send;
    fn work_done_progress_cancel(
        &self,
        context: Context,
        params: WorkDoneProgressCancelParams,
    ) -> impl Future<Output = ServerResult<()>> + Send;
    fn did_create_files(
        &self,
        context: Context,
        params: CreateFilesParams,
    ) -> impl Future<Output = ServerResult<()>> + Send;
    fn did_rename_files(
        &self,
        context: Context,
        params: RenameFilesParams,
    ) -> impl Future<Output = ServerResult<()>> + Send;
    fn did_delete_files(
        &self,
        context: Context,
        params: DeleteFilesParams,
    ) -> impl Future<Output = ServerResult<()>> + Send;

    fn custom_request(
        &self,
        context: Context,
        params: Extension,
    ) -> impl Future<Output = ServerResult<serde_json::Value>> + Send;
    fn custom_notification(
        &self,
        context: Context,
        params: Extension,
    ) -> impl Future<Output = ServerResult<()>> + Send;
}
#[derive(Debug)]
pub struct Limits {
    pub input_queue: usize,
    pub output_queue: usize,
    pub pending_requests: usize,
    pub header_bytes: usize,
    pub frame_bytes: usize,
}
impl Default for Limits {
    fn default() -> Self {
        Self {
            input_queue: 128,
            output_queue: 128,
            pending_requests: 128,
            header_bytes: 8192,
            frame_bytes: 16 * 1024 * 1024,
        }
    }
}
pub struct Wrapper<S: ILanguageServer> {
    pub(super) server: Arc<S>,
    pub(super) limits: Limits,
}
impl<S: ILanguageServer> Wrapper<S> {
    pub fn new(server: S) -> Self {
        Self {
            server: Arc::new(server),
            limits: Limits::default(),
        }
    }
    pub fn with_limits(server: S, limits: Limits) -> Self {
        Self {
            server: Arc::new(server),
            limits,
        }
    }
}
#[derive(Clone)]
pub struct ClientHandle {
    pub(super) session: Arc<Session>,
}
#[derive(Debug, PartialEq, Eq)]
pub enum ExitStatus {
    Graceful,
    Abnormal,
    Disconnected,
}
pub(super) struct WritePacket {
    pub message: crate::rpc::Message,
    pub done: oneshot::Sender<Result<(), String>>,
}
pub(super) enum Inbound {
    Request(crate::rpc::Request, Cancellation),
    Notification(crate::rpc::Notification),
}
pub(super) struct Session {
    pub writer: mpsc::Sender<WritePacket>,
    pub phase: AtomicU8,
    pub shutdown_requested: AtomicBool,
    pub trace: AtomicU8,
    pub encoding: AtomicU8,
    pub next_id: AtomicI32,
    pub outgoing:
        std::sync::Mutex<HashMap<crate::rpc::RequestId, oneshot::Sender<crate::rpc::Response>>>,
    pub incoming: Mutex<HashMap<crate::rpc::RequestId, Cancellation>>,
    pub supported: Mutex<HashSet<&'static str>>,
    pub registerable: Mutex<HashSet<&'static str>>,
    pub initialization_progress: Mutex<Option<lsp_types::ProgressToken>>,
    pub offered_encodings: Mutex<Vec<String>>,
    pub closed: Cancellation,
    pub pending_limit: usize,
}
