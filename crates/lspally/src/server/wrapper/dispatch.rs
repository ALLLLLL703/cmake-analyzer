// Script-generated routing by scripts/gen/lspally-schema/src/main.rs.
// Contains AI-authored additions or edits.
use super::handlers;
use crate::{
    rpc::{Notification, Request, Response, ResponsePayload},
    server::model::*,
};

pub(super) fn invalid_params(error: crate::rpc::DecodeError) -> ServerError {
    ServerError::invalid(error.to_string())
}
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
fn extension_params(params: Option<crate::rpc::Params>) -> Option<serde_json::Value> {
    params.map(|p| match p {
        crate::rpc::Params::Left(v) => serde_json::Value::Object(v),
        crate::rpc::Params::Right(v) => serde_json::Value::Array(v),
    })
}
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
