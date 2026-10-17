// Script-generated routing by scripts/gen/lspally-schema/src/main.rs.
// Contains AI-authored additions or edits.
//! Route standard methods into typed handlers and forward only non-reserved extensions.
//! Dispatch awaits business code; it does not itself write framed transport output.
use super::handlers;
use crate::{
    rpc::{Notification, Request, Response, ResponsePayload},
    server::model::*,
};

/// Classify a typed parameter-decoding failure as InvalidParams; no I/O.
///
/// # Arguments
/// - `error` `[in]`: decoder failure whose display text is retained.
/// # Returns
/// Error -32602 with the decoder's explanation; no output parameters.
/// # Examples
/// ```text
/// let error = invalid_params(decode_error);
/// assert_eq!(error.code, -32602);
/// ```
pub(super) fn invalid_params(error: crate::rpc::DecodeError) -> ServerError {
    ServerError::invalid(error.to_string())
}
/// Dispatch an incoming request and build a response retaining its correlation ID.
/// Standard handlers decode typed inputs; only unknown non-reserved methods reach
/// the extension hook. Business code may change state or perform I/O.
///
/// # Arguments
/// - `server` `[in,out]`: shared implementation receiving the selected handler call;
///   any business-state mutation is controlled by that implementation.
/// - `context` `[in,out]`: call services and shared session state passed to the handler.
/// - `request` `[in]`: owned envelope; parameters are consumed during typed decoding.
/// # Returns
/// `Ok(response)` with the original ID and encoded result, including JSON null;
/// `Err(error)` if decoding, business handling or result encoding fails. The worker
/// converts errors into RPC error replies; this function does not write either reply.
/// # Errors
/// Invalid standard parameters yield InvalidParams without extension fallback.
/// Reserved `$/` methods and known server-to-client request names yield MethodNotFound.
/// Business errors and outgoing conversion/serialization errors propagate.
/// # Algorithm
/// Match exact standard method constants first. Delegate to a feature adapter,
/// reject reserved/wrong-direction names, or pass the extension's raw parameters
/// to `custom_request` and wrap its JSON result with the retained ID.
/// # Flow
/// ```text
///              [request method]
///                     |
///                <standard?>
///                /        \
///              yes        no
///               |          |
///       [typed handler] <reserved/client-only?>
///               |        /             \
///               |      yes             no
///               |       |               |
///               |   [unsupported] [custom_request]
///               |                       |
///               +-----------+-----------+
///                           v
///                 [response or error]
/// ```
/// # Examples
/// ```text
/// let response = request(&server, context, rpc_request).await?;
/// assert_eq!(response.id, Some(original_id));
/// ```
pub(super) async fn request<S: ILanguageServer>(
    server: &S,
    context: Context,
    mut request: Request,
) -> ServerResult<Response> {
    match request.method.as_str(){
name if name == <lsp_types::request::Initialize as lsp_types::request::Request>::METHOD => handlers::lifecycle::initialize(server,context,request).await,
name if name == <lsp_types::request::Shutdown as lsp_types::request::Request>::METHOD => handlers::lifecycle::shutdown(server,context,request).await,
name if name == <lsp_types::request::Completion as lsp_types::request::Request>::METHOD => handlers::completion::completion(server,context,request).await,
name if name == <lsp_types::request::ResolveCompletionItem as lsp_types::request::Request>::METHOD => handlers::completion::resolve_completion_item(server,context,request).await,
name if name == <lsp_types::request::HoverRequest as lsp_types::request::Request>::METHOD => handlers::completion::hover_request(server,context,request).await,
name if name == <lsp_types::request::SignatureHelpRequest as lsp_types::request::Request>::METHOD => handlers::completion::signature_help_request(server,context,request).await,
name if name == <lsp_types::request::GotoDeclaration as lsp_types::request::Request>::METHOD => handlers::navigation::goto_declaration(server,context,request).await,
name if name == <lsp_types::request::GotoDefinition as lsp_types::request::Request>::METHOD => handlers::navigation::goto_definition(server,context,request).await,
name if name == <lsp_types::request::References as lsp_types::request::Request>::METHOD => handlers::navigation::references(server,context,request).await,
name if name == <lsp_types::request::GotoTypeDefinition as lsp_types::request::Request>::METHOD => handlers::navigation::goto_type_definition(server,context,request).await,
name if name == <lsp_types::request::GotoImplementation as lsp_types::request::Request>::METHOD => handlers::navigation::goto_implementation(server,context,request).await,
name if name == <lsp_types::request::DocumentHighlightRequest as lsp_types::request::Request>::METHOD => handlers::navigation::document_highlight_request(server,context,request).await,
name if name == <lsp_types::request::DocumentSymbolRequest as lsp_types::request::Request>::METHOD => handlers::navigation::document_symbol_request(server,context,request).await,
name if name == <lsp_types::request::WorkspaceSymbolRequest as lsp_types::request::Request>::METHOD => handlers::navigation::workspace_symbol_request(server,context,request).await,
name if name == <lsp_types::request::WorkspaceSymbolResolve as lsp_types::request::Request>::METHOD => handlers::navigation::workspace_symbol_resolve(server,context,request).await,
name if name == <lsp_types::request::ExecuteCommand as lsp_types::request::Request>::METHOD => handlers::actions::execute_command(server,context,request).await,
name if name == <lsp_types::request::WillSaveWaitUntil as lsp_types::request::Request>::METHOD => handlers::documents::will_save_wait_until(server,context,request).await,
name if name == <lsp_types::request::CodeActionRequest as lsp_types::request::Request>::METHOD => handlers::actions::code_action_request(server,context,request).await,
name if name == <lsp_types::request::CodeActionResolveRequest as lsp_types::request::Request>::METHOD => handlers::actions::code_action_resolve_request(server,context,request).await,
name if name == <lsp_types::request::CodeLensRequest as lsp_types::request::Request>::METHOD => handlers::actions::code_lens_request(server,context,request).await,
name if name == <lsp_types::request::CodeLensResolve as lsp_types::request::Request>::METHOD => handlers::actions::code_lens_resolve(server,context,request).await,
name if name == <lsp_types::request::DocumentLinkRequest as lsp_types::request::Request>::METHOD => handlers::actions::document_link_request(server,context,request).await,
name if name == <lsp_types::request::DocumentLinkResolve as lsp_types::request::Request>::METHOD => handlers::actions::document_link_resolve(server,context,request).await,
name if name == <lsp_types::request::Formatting as lsp_types::request::Request>::METHOD => handlers::editing::formatting(server,context,request).await,
name if name == <lsp_types::request::RangeFormatting as lsp_types::request::Request>::METHOD => handlers::editing::range_formatting(server,context,request).await,
name if name == <lsp_types::request::OnTypeFormatting as lsp_types::request::Request>::METHOD => handlers::editing::on_type_formatting(server,context,request).await,
name if name == <lsp_types::request::LinkedEditingRange as lsp_types::request::Request>::METHOD => handlers::navigation::linked_editing_range(server,context,request).await,
name if name == <lsp_types::request::Rename as lsp_types::request::Request>::METHOD => handlers::editing::rename(server,context,request).await,
name if name == <lsp_types::request::DocumentColor as lsp_types::request::Request>::METHOD => handlers::editing::document_color(server,context,request).await,
name if name == <lsp_types::request::ColorPresentationRequest as lsp_types::request::Request>::METHOD => handlers::editing::color_presentation_request(server,context,request).await,
name if name == <lsp_types::request::FoldingRangeRequest as lsp_types::request::Request>::METHOD => handlers::navigation::folding_range_request(server,context,request).await,
name if name == <lsp_types::request::PrepareRenameRequest as lsp_types::request::Request>::METHOD => handlers::editing::prepare_rename_request(server,context,request).await,
name if name == <lsp_types::request::SelectionRangeRequest as lsp_types::request::Request>::METHOD => handlers::navigation::selection_range_request(server,context,request).await,
name if name == <lsp_types::request::CallHierarchyPrepare as lsp_types::request::Request>::METHOD => handlers::hierarchies::call_hierarchy_prepare(server,context,request).await,
name if name == <lsp_types::request::CallHierarchyIncomingCalls as lsp_types::request::Request>::METHOD => handlers::hierarchies::call_hierarchy_incoming_calls(server,context,request).await,
name if name == <lsp_types::request::CallHierarchyOutgoingCalls as lsp_types::request::Request>::METHOD => handlers::hierarchies::call_hierarchy_outgoing_calls(server,context,request).await,
name if name == <lsp_types::request::SemanticTokensFullRequest as lsp_types::request::Request>::METHOD => handlers::annotations::semantic_tokens_full_request(server,context,request).await,
name if name == <lsp_types::request::SemanticTokensFullDeltaRequest as lsp_types::request::Request>::METHOD => handlers::annotations::semantic_tokens_full_delta_request(server,context,request).await,
name if name == <lsp_types::request::SemanticTokensRangeRequest as lsp_types::request::Request>::METHOD => handlers::annotations::semantic_tokens_range_request(server,context,request).await,
name if name == <lsp_types::request::WillCreateFiles as lsp_types::request::Request>::METHOD => handlers::editing::will_create_files(server,context,request).await,
name if name == <lsp_types::request::WillRenameFiles as lsp_types::request::Request>::METHOD => handlers::editing::will_rename_files(server,context,request).await,
name if name == <lsp_types::request::WillDeleteFiles as lsp_types::request::Request>::METHOD => handlers::editing::will_delete_files(server,context,request).await,
name if name == <lsp_types::request::MonikerRequest as lsp_types::request::Request>::METHOD => handlers::hierarchies::moniker_request(server,context,request).await,
name if name == <lsp_types::request::InlayHintRequest as lsp_types::request::Request>::METHOD => handlers::annotations::inlay_hint_request(server,context,request).await,
name if name == <lsp_types::request::InlayHintResolveRequest as lsp_types::request::Request>::METHOD => handlers::annotations::inlay_hint_resolve_request(server,context,request).await,
name if name == <lsp_types::request::InlineValueRequest as lsp_types::request::Request>::METHOD => handlers::annotations::inline_value_request(server,context,request).await,
name if name == <lsp_types::request::DocumentDiagnosticRequest as lsp_types::request::Request>::METHOD => handlers::diagnostics::document_diagnostic_request(server,context,request).await,
name if name == <lsp_types::request::WorkspaceDiagnosticRequest as lsp_types::request::Request>::METHOD => handlers::diagnostics::workspace_diagnostic_request(server,context,request).await,
name if name == <lsp_types::request::TypeHierarchyPrepare as lsp_types::request::Request>::METHOD => handlers::hierarchies::type_hierarchy_prepare(server,context,request).await,
name if name == <lsp_types::request::TypeHierarchySupertypes as lsp_types::request::Request>::METHOD => handlers::hierarchies::type_hierarchy_supertypes(server,context,request).await,
name if name == <lsp_types::request::TypeHierarchySubtypes as lsp_types::request::Request>::METHOD => handlers::hierarchies::type_hierarchy_subtypes(server,context,request).await,
        name if name.starts_with("$/") || is_client_request(name)=>Err(ServerError::unsupported()),
        _=>{let params=Extension{method:request.method,params:extension_params(request.params)};let result=server.custom_request(context,params).await?;Ok(Response{jsonrpc:crate::rpc::Version::V2,id:Some(request.id),payload:ResponsePayload::Result(result)})}
    }
}
/// Dispatch an incoming notification without constructing a reply.
/// Standard handlers decode typed input; unknown non-reserved methods reach the
/// extension hook. Business code may mutate state or perform I/O.
///
/// # Arguments
/// - `server` `[in,out]`: shared implementation receiving the selected hook;
///   business-state changes belong to that implementation.
/// - `context` `[in,out]`: per-call services and shared session passed to the hook.
/// - `notification` `[in]`: owned method and parameters consumed by dispatch.
/// # Returns
/// `Ok(())` after a handler succeeds or a reserved/wrong-direction notification is
/// ignored; `Err(error)` for invalid standard parameters or a handler failure.
/// No output parameters or response envelopes are produced.
/// # Errors
/// Invalid standard parameters yield InvalidParams, not an extension fallback.
/// Business errors propagate to the worker, which logs notification failures on stderr.
/// # Algorithm
/// Match standard notification constants, ignore unknown reserved `$/` names and
/// known server-to-client notification names, then forward other names and raw
/// parameters to `custom_notification`.
/// # Flow
/// ```text
///            [notification method]
///                     |
///                <standard?>
///                /        \
///              yes        no
///               |          |
///       [typed handler] <reserved/client-only?>
///               |        /             \
///               |      yes             no
///               |       |               |
///               |    [Ok(())] [custom_notification]
///               |                       |
///               +-----------+-----------+
///                           v
///                    [() or error]
/// ```
/// # Examples
/// ```text
/// notification(&server, context, rpc_notification).await?;
/// ```
pub(super) async fn notification<S: ILanguageServer>(
    server: &S,
    context: Context,
    mut notification: Notification,
) -> ServerResult<()> {
    match notification.method.as_str(){
name if name == <lsp_types::notification::Cancel as lsp_types::notification::Notification>::METHOD => handlers::lifecycle::cancel(server,context,notification).await,
name if name == <lsp_types::notification::SetTrace as lsp_types::notification::Notification>::METHOD => handlers::lifecycle::set_trace(server,context,notification).await,
name if name == <lsp_types::notification::Initialized as lsp_types::notification::Notification>::METHOD => handlers::lifecycle::initialized(server,context,notification).await,
name if name == <lsp_types::notification::Exit as lsp_types::notification::Notification>::METHOD => handlers::lifecycle::exit(server,context,notification).await,
name if name == <lsp_types::notification::DidChangeConfiguration as lsp_types::notification::Notification>::METHOD => handlers::documents::did_change_configuration(server,context,notification).await,
name if name == <lsp_types::notification::DidOpenTextDocument as lsp_types::notification::Notification>::METHOD => handlers::documents::did_open_text_document(server,context,notification).await,
name if name == <lsp_types::notification::DidChangeTextDocument as lsp_types::notification::Notification>::METHOD => handlers::documents::did_change_text_document(server,context,notification).await,
name if name == <lsp_types::notification::WillSaveTextDocument as lsp_types::notification::Notification>::METHOD => handlers::documents::will_save_text_document(server,context,notification).await,
name if name == <lsp_types::notification::DidCloseTextDocument as lsp_types::notification::Notification>::METHOD => handlers::documents::did_close_text_document(server,context,notification).await,
name if name == <lsp_types::notification::DidSaveTextDocument as lsp_types::notification::Notification>::METHOD => handlers::documents::did_save_text_document(server,context,notification).await,
name if name == <lsp_types::notification::DidOpenNotebookDocument as lsp_types::notification::Notification>::METHOD => handlers::documents::did_open_notebook_document(server,context,notification).await,
name if name == <lsp_types::notification::DidChangeNotebookDocument as lsp_types::notification::Notification>::METHOD => handlers::documents::did_change_notebook_document(server,context,notification).await,
name if name == <lsp_types::notification::DidSaveNotebookDocument as lsp_types::notification::Notification>::METHOD => handlers::documents::did_save_notebook_document(server,context,notification).await,
name if name == <lsp_types::notification::DidCloseNotebookDocument as lsp_types::notification::Notification>::METHOD => handlers::documents::did_close_notebook_document(server,context,notification).await,
name if name == <lsp_types::notification::DidChangeWatchedFiles as lsp_types::notification::Notification>::METHOD => handlers::documents::did_change_watched_files(server,context,notification).await,
name if name == <lsp_types::notification::DidChangeWorkspaceFolders as lsp_types::notification::Notification>::METHOD => handlers::documents::did_change_workspace_folders(server,context,notification).await,
name if name == <lsp_types::notification::WorkDoneProgressCancel as lsp_types::notification::Notification>::METHOD => handlers::annotations::work_done_progress_cancel(server,context,notification).await,
name if name == <lsp_types::notification::DidCreateFiles as lsp_types::notification::Notification>::METHOD => handlers::documents::did_create_files(server,context,notification).await,
name if name == <lsp_types::notification::DidRenameFiles as lsp_types::notification::Notification>::METHOD => handlers::documents::did_rename_files(server,context,notification).await,
name if name == <lsp_types::notification::DidDeleteFiles as lsp_types::notification::Notification>::METHOD => handlers::documents::did_delete_files(server,context,notification).await,
        name if name.starts_with("$/") || is_client_notification(name)=>Ok(()),
        _=>server.custom_notification(context,Extension{method:notification.method,params:extension_params(notification.params)}).await,
    }
}
/// Move object/array RPC parameters into raw extension JSON; no I/O or cloning.
///
/// # Arguments
/// - `params` `[in]`: optional owned parameter object or array.
/// # Returns
/// `Some(Object)` or `Some(Array)` retaining the payload; `None` stays absent.
/// No output parameters or shared state are changed.
/// # Examples
/// ```text
/// assert!(extension_params(None).is_none());
/// let value = extension_params(Some(crate::rpc::Params::Right(vec![])));
/// assert_eq!(value, Some(serde_json::json!([])));
/// ```
fn extension_params(params: Option<crate::rpc::Params>) -> Option<serde_json::Value> {
    params.map(|p| match p {
        crate::rpc::Params::Left(v) => serde_json::Value::Object(v),
        crate::rpc::Params::Right(v) => serde_json::Value::Array(v),
    })
}
/// Recognize standard requests whose receiver must be the client; no side effects.
/// Used to prevent wrong-direction calls from falling through to extensions.
///
/// # Arguments
/// - `method` `[in]`: exact case-sensitive method spelling.
/// # Returns
/// `true` for one of the listed server-to-client requests; `false` otherwise.
/// This is not a test for every standard LSP request.
/// # Examples
/// ```text
/// assert!(is_client_request("workspace/applyEdit"));
/// assert!(!is_client_request("textDocument/hover"));
/// ```
fn is_client_request(method: &str) -> bool {
    matches!(
        method,
        "window/showMessageRequest"
            | "client/registerCapability"
            | "client/unregisterCapability"
            | "workspace/applyEdit"
            | "workspace/configuration"
            | "workspace/workspaceFolders"
            | "window/workDoneProgress/create"
            | "workspace/semanticTokens/refresh"
            | "workspace/codeLens/refresh"
            | "window/showDocument"
            | "workspace/inlayHint/refresh"
            | "workspace/inlineValue/refresh"
            | "workspace/diagnostic/refresh"
    )
}
/// Recognize standard notifications whose receiver must be the client; no side effects.
/// Used to ignore wrong-direction notifications rather than call extension hooks.
///
/// # Arguments
/// - `method` `[in]`: exact case-sensitive method spelling.
/// # Returns
/// `true` for one of the listed server-to-client notifications; `false` otherwise.
/// This is not a test for every standard LSP notification.
/// # Examples
/// ```text
/// assert!(is_client_notification("textDocument/publishDiagnostics"));
/// assert!(!is_client_notification("textDocument/didOpen"));
/// ```
fn is_client_notification(method: &str) -> bool {
    matches!(
        method,
        "window/showMessage"
            | "window/logMessage"
            | "telemetry/event"
            | "textDocument/publishDiagnostics"
            | "$/progress"
            | "$/logTrace"
    )
}
