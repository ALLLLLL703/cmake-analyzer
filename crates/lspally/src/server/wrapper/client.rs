// Script-generated standard client methods by scripts/gen/lspally-schema/src/main.rs.
// Contains AI-authored additions or edits.
//! Typed outgoing calls, lifecycle/capability checks and response correlation.
//! Public sends enqueue framed messages and await writer flush; requests also await a peer reply.
use super::convert::*;
use crate::server::model as d;
use crate::{rpc, server::model::*};
use lsp_types::{notification::Notification as LspNotification, request::Request as LspRequest};
impl ClientHandle {
    /// Ask the client to display a message and optional action choices.
    /// Enqueues output to the shared transport; awaits flush and a matching client reply.
    /// This is the only outgoing request allowed before RUNNING, once initialization has cached its support.
    ///
    /// # Arguments
    /// - `self` `[in,out]`: connected session; output queue and pending-response bookkeeping are changed.
    /// - `params` `[in]`: message severity, display text and available actions.
    /// # Returns
    /// `Ok(Some(action))` for the chosen action; `Ok(None)` when dismissed; `Err(error)` on send, reply or conversion failure.
    /// # Errors
    /// Returns errors for disallowed lifecycle/capability, exhausted IDs or pending capacity, disconnection, peer errors, or malformed replies.
    /// Nested adapters may also reject invalid business values before any message is sent.
    /// # Examples
    /// A running wrapper must supply the connected handle and negotiated parameters.
    /// ```no_run
    /// use lspally::server::model::*;
    /// async fn send(client: &ClientHandle, params: ShowMessageRequestParams) -> ServerResult<Option<MessageActionItem>> {
    ///     client.show_message_request(params).await
    /// }
    /// ```
    pub async fn show_message_request(
        &self,
        params: ShowMessageRequestParams,
    ) -> ServerResult<Option<MessageActionItem>> {
        let params = encode_show_message_request_params(params)?;
        let result = self
            .request::<lsp_types::request::ShowMessageRequest>(params)
            .await?;
        Ok::<_, ServerError>((result).map(decode_message_action_item).transpose()?)
    }
    /// Ask the client to register feature handlers dynamically.
    /// Enqueues output to the shared transport; awaits flush and a matching client reply.
    /// Checks every method against the client's dynamic-registration permissions before sending; one disallowed entry rejects the entire call.
    ///
    /// # Arguments
    /// - `self` `[in,out]`: connected session; output queue and pending-response bookkeeping are changed.
    /// - `params` `[in]`: registration IDs, method names and method-specific options.
    /// # Returns
    /// `Ok(())` when the client acknowledges registration; `Err(error)` when rejected or delivery/decoding fails.
    /// # Errors
    /// Returns MethodNotFound if any registration method was not permitted by client capabilities. Returns errors for disallowed lifecycle/capability, exhausted IDs or pending capacity, disconnection, peer errors, or malformed replies.
    /// Nested adapters may also reject invalid business values before any message is sent.
    /// # Examples
    /// A running wrapper must supply the connected handle and negotiated parameters.
    /// ```no_run
    /// use lspally::server::model::*;
    /// async fn send(client: &ClientHandle, params: RegistrationParams) -> ServerResult<()> {
    ///     client.register_capability(params).await
    /// }
    /// ```
    /// # Algorithm
    /// Lock the allowed-method set and reject the batch if any method is absent.
    /// Release that lock before encoding or awaiting the client response.
    /// # Flow
    /// ```text
    /// [registrations] ---> [allowed-method set]
    ///                             |
    ///                       <all allowed?>
    ///                        /         \
    ///                      no          yes
    ///                       |           |
    ///                    [error] [release lock]
    ///                                   |
    ///                            [encode + request]
    /// ```
    pub async fn register_capability(&self, params: RegistrationParams) -> ServerResult<()> {
        let allowed = self.session.registerable.lock().await;
        if params
            .registrations
            .iter()
            .any(|registration| !allowed.contains(registration.method.as_str()))
        {
            return Err(ServerError::unsupported());
        }
        drop(allowed);
        let params = encode_registration_params(params)?;
        let result = self
            .request::<lsp_types::request::RegisterCapability>(params)
            .await?;
        Ok::<_, ServerError>(result)
    }
    /// Ask the client to remove previously registered feature handlers.
    /// Enqueues output to the shared transport; awaits flush and a matching client reply.
    /// Does not maintain a local registry or validate IDs against earlier successful registrations.
    ///
    /// # Arguments
    /// - `self` `[in,out]`: connected session; output queue and pending-response bookkeeping are changed.
    /// - `params` `[in]`: registration IDs and method names to remove.
    /// # Returns
    /// `Ok(())` when the client acknowledges removal; `Err(error)` on rejection or delivery/decoding failure.
    /// # Errors
    /// Returns errors for disallowed lifecycle/capability, exhausted IDs or pending capacity, disconnection, peer errors, or malformed replies.
    /// Nested adapters may also reject invalid business values before any message is sent.
    /// # Examples
    /// A running wrapper must supply the connected handle and negotiated parameters.
    /// ```no_run
    /// use lspally::server::model::*;
    /// async fn send(client: &ClientHandle, params: UnregistrationParams) -> ServerResult<()> {
    ///     client.unregister_capability(params).await
    /// }
    /// ```
    pub async fn unregister_capability(&self, params: UnregistrationParams) -> ServerResult<()> {
        let params = encode_unregistration_params(params)?;
        let result = self
            .request::<lsp_types::request::UnregisterCapability>(params)
            .await?;
        Ok::<_, ServerError>(result)
    }
    /// Ask the client to apply a labeled workspace edit; this server does not apply it.
    /// Enqueues output to the shared transport; awaits flush and a matching client reply.
    /// A successful RPC can still return `applied == false`; inspect the response.
    ///
    /// # Arguments
    /// - `self` `[in,out]`: connected session; output queue and pending-response bookkeeping are changed.
    /// - `params` `[in]`: optional display label and client-applied text/resource changes.
    /// # Returns
    /// `Ok(response)` describing whether edits were applied and any failure reason/index; `Err(error)` for request or conversion failure.
    /// # Errors
    /// Returns errors for disallowed lifecycle/capability, exhausted IDs or pending capacity, disconnection, peer errors, or malformed replies.
    /// Nested adapters may also reject invalid business values before any message is sent.
    /// # Examples
    /// A running wrapper must supply the connected handle and negotiated parameters.
    /// ```no_run
    /// use lspally::server::model::*;
    /// async fn send(client: &ClientHandle, params: ApplyWorkspaceEditParams) -> ServerResult<ApplyWorkspaceEditResponse> {
    ///     client.apply_workspace_edit(params).await
    /// }
    /// ```
    pub async fn apply_workspace_edit(
        &self,
        params: ApplyWorkspaceEditParams,
    ) -> ServerResult<ApplyWorkspaceEditResponse> {
        let params = encode_apply_workspace_edit_params(params)?;
        let result = self
            .request::<lsp_types::request::ApplyWorkspaceEdit>(params)
            .await?;
        Ok::<_, ServerError>(decode_apply_workspace_edit_response(result)?)
    }
    /// Read requested configuration sections from the client.
    /// Enqueues output to the shared transport; awaits flush and a matching client reply.
    /// Converts the returned JSON trees to business data without changing server configuration itself.
    ///
    /// # Arguments
    /// - `self` `[in,out]`: connected session; output queue and pending-response bookkeeping are changed.
    /// - `params` `[in]`: ordered configuration queries, each with a section and optional scope URI.
    /// # Returns
    /// `Ok(values)` in query order, including nulls for unavailable sections; `Err(error)` on request or value conversion failure.
    /// # Errors
    /// Returns errors for disallowed lifecycle/capability, exhausted IDs or pending capacity, disconnection, peer errors, or malformed replies.
    /// Nested adapters may also reject invalid business values before any message is sent.
    /// # Examples
    /// A running wrapper must supply the connected handle and negotiated parameters.
    /// ```no_run
    /// use lspally::server::model::*;
    /// async fn send(client: &ClientHandle, params: ConfigurationParams) -> ServerResult<Vec<Data>> {
    ///     client.workspace_configuration(params).await
    /// }
    /// ```
    pub async fn workspace_configuration(
        &self,
        params: ConfigurationParams,
    ) -> ServerResult<Vec<Data>> {
        let params = encode_configuration_params(params)?;
        let result = self
            .request::<lsp_types::request::WorkspaceConfiguration>(params)
            .await?;
        Ok::<_, ServerError>(
            (result)
                .into_iter()
                .map(data_from_wire)
                .collect::<Result<Vec<_>, ServerError>>()?,
        )
    }
    /// Read the client's current workspace-folder list.
    /// Enqueues output to the shared transport; awaits flush and a matching client reply.
    /// Does not scan directories or add folders to the business server.
    ///
    /// # Arguments
    /// - `self` `[in,out]`: connected session; output queue and pending-response bookkeeping are changed.
    /// - `params` `[in]`: unit payload required by the protocol; it contains no query options.
    /// # Returns
    /// `Ok(Some(folders))` when a list is supplied (possibly empty); `Ok(None)` when unavailable; `Err(error)` on request/conversion failure.
    /// # Errors
    /// Returns errors for disallowed lifecycle/capability, exhausted IDs or pending capacity, disconnection, peer errors, or malformed replies.
    /// Nested adapters may also reject invalid business values before any message is sent.
    /// # Examples
    /// A running wrapper must supply the connected handle and negotiated parameters.
    /// ```no_run
    /// use lspally::server::model::*;
    /// async fn send(client: &ClientHandle, params: ()) -> ServerResult<Option<Vec<WorkspaceFolder>>> {
    ///     client.workspace_folders_request(params).await
    /// }
    /// ```
    pub async fn workspace_folders_request(
        &self,
        params: (),
    ) -> ServerResult<Option<Vec<WorkspaceFolder>>> {
        let params = params;
        let result = self
            .request::<lsp_types::request::WorkspaceFoldersRequest>(params)
            .await?;
        Ok::<_, ServerError>(
            (result)
                .map(|v| {
                    Ok::<_, ServerError>(
                        (v).into_iter()
                            .map(decode_workspace_folder)
                            .collect::<Result<Vec<_>, ServerError>>()?,
                    )
                })
                .transpose()?,
        )
    }
    /// Ask the client to create a server-initiated work-done progress token.
    /// Enqueues output to the shared transport; awaits flush and a matching client reply.
    /// Subsequent updates are sent separately through [`Self::work_done_progress`].
    ///
    /// # Arguments
    /// - `self` `[in,out]`: connected session; output queue and pending-response bookkeeping are changed.
    /// - `params` `[in]`: token to create before sending its work-done updates.
    /// # Returns
    /// `Ok(())` after client acknowledgement; `Err(error)` on rejection or delivery failure.
    /// # Errors
    /// Returns errors for disallowed lifecycle/capability, exhausted IDs or pending capacity, disconnection, peer errors, or malformed replies.
    /// Nested adapters may also reject invalid business values before any message is sent.
    /// # Examples
    /// A running wrapper must supply the connected handle and negotiated parameters.
    /// ```no_run
    /// use lspally::server::model::*;
    /// async fn send(client: &ClientHandle, params: WorkDoneProgressCreateParams) -> ServerResult<()> {
    ///     client.work_done_progress_create(params).await
    /// }
    /// ```
    pub async fn work_done_progress_create(
        &self,
        params: WorkDoneProgressCreateParams,
    ) -> ServerResult<()> {
        let params = encode_work_done_progress_create_params(params)?;
        let result = self
            .request::<lsp_types::request::WorkDoneProgressCreate>(params)
            .await?;
        Ok::<_, ServerError>(result)
    }
    /// Ask the client to refresh displayed semantic tokens.
    /// Enqueues output to the shared transport; awaits flush and a matching client reply.
    /// Does not compute tokens; the client may issue new token requests.
    ///
    /// # Arguments
    /// - `self` `[in,out]`: connected session; output queue and pending-response bookkeeping are changed.
    /// - `params` `[in]`: unit payload; refresh covers the client's relevant documents.
    /// # Returns
    /// `Ok(())` after client acknowledgement, or `Err(error)` on request failure.
    /// # Errors
    /// Returns errors for disallowed lifecycle/capability, exhausted IDs or pending capacity, disconnection, peer errors, or malformed replies.
    /// Nested adapters may also reject invalid business values before any message is sent.
    /// # Examples
    /// A running wrapper must supply the connected handle and negotiated parameters.
    /// ```no_run
    /// use lspally::server::model::*;
    /// async fn send(client: &ClientHandle, params: ()) -> ServerResult<()> {
    ///     client.semantic_tokens_refresh(params).await
    /// }
    /// ```
    pub async fn semantic_tokens_refresh(&self, params: ()) -> ServerResult<()> {
        let params = params;
        let result = self
            .request::<lsp_types::request::SemanticTokensRefresh>(params)
            .await?;
        Ok::<_, ServerError>(result)
    }
    /// Ask the client to refresh displayed code lenses.
    /// Enqueues output to the shared transport; awaits flush and a matching client reply.
    /// Does not resolve lenses; the client may request them again.
    ///
    /// # Arguments
    /// - `self` `[in,out]`: connected session; output queue and pending-response bookkeeping are changed.
    /// - `params` `[in]`: unit payload; there is no document filter in this request.
    /// # Returns
    /// `Ok(())` after client acknowledgement, or `Err(error)` on request failure.
    /// # Errors
    /// Returns errors for disallowed lifecycle/capability, exhausted IDs or pending capacity, disconnection, peer errors, or malformed replies.
    /// Nested adapters may also reject invalid business values before any message is sent.
    /// # Examples
    /// A running wrapper must supply the connected handle and negotiated parameters.
    /// ```no_run
    /// use lspally::server::model::*;
    /// async fn send(client: &ClientHandle, params: ()) -> ServerResult<()> {
    ///     client.code_lens_refresh(params).await
    /// }
    /// ```
    pub async fn code_lens_refresh(&self, params: ()) -> ServerResult<()> {
        let params = params;
        let result = self
            .request::<lsp_types::request::CodeLensRefresh>(params)
            .await?;
        Ok::<_, ServerError>(result)
    }
    /// Ask the client to open or focus a resource and optional selection.
    /// Enqueues output to the shared transport; awaits flush and a matching client reply.
    /// The client, not this wrapper, opens the resource; a false success flag is not an RPC error.
    ///
    /// # Arguments
    /// - `self` `[in,out]`: connected session; output queue and pending-response bookkeeping are changed.
    /// - `params` `[in]`: URI, external/focus policy and optional selected span.
    /// # Returns
    /// `Ok(result)` whose `success` flag reports whether the client could show it; `Err(error)` on request/conversion failure.
    /// # Errors
    /// Returns errors for disallowed lifecycle/capability, exhausted IDs or pending capacity, disconnection, peer errors, or malformed replies.
    /// Nested adapters may also reject invalid business values before any message is sent.
    /// # Examples
    /// A running wrapper must supply the connected handle and negotiated parameters.
    /// ```no_run
    /// use lspally::server::model::*;
    /// async fn send(client: &ClientHandle, params: ShowDocumentParams) -> ServerResult<ShowDocumentResult> {
    ///     client.show_document(params).await
    /// }
    /// ```
    pub async fn show_document(
        &self,
        params: ShowDocumentParams,
    ) -> ServerResult<ShowDocumentResult> {
        let params = encode_show_document_params(params)?;
        let result = self
            .request::<lsp_types::request::ShowDocument>(params)
            .await?;
        Ok::<_, ServerError>(decode_show_document_result(result)?)
    }
    /// Ask the client to refresh inlay hints.
    /// Enqueues output to the shared transport; awaits flush and a matching client reply.
    /// The client decides when to request replacement hints.
    ///
    /// # Arguments
    /// - `self` `[in,out]`: connected session; output queue and pending-response bookkeeping are changed.
    /// - `params` `[in]`: unit payload; no range is supplied by a refresh request.
    /// # Returns
    /// `Ok(())` after client acknowledgement, or `Err(error)` on request failure.
    /// # Errors
    /// Returns errors for disallowed lifecycle/capability, exhausted IDs or pending capacity, disconnection, peer errors, or malformed replies.
    /// Nested adapters may also reject invalid business values before any message is sent.
    /// # Examples
    /// A running wrapper must supply the connected handle and negotiated parameters.
    /// ```no_run
    /// use lspally::server::model::*;
    /// async fn send(client: &ClientHandle, params: ()) -> ServerResult<()> {
    ///     client.inlay_hint_refresh_request(params).await
    /// }
    /// ```
    pub async fn inlay_hint_refresh_request(&self, params: ()) -> ServerResult<()> {
        let params = params;
        let result = self
            .request::<lsp_types::request::InlayHintRefreshRequest>(params)
            .await?;
        Ok::<_, ServerError>(result)
    }
    /// Ask the client to refresh inline debugger values.
    /// Enqueues output to the shared transport; awaits flush and a matching client reply.
    /// Does not evaluate expressions; the client may issue inline-value requests.
    ///
    /// # Arguments
    /// - `self` `[in,out]`: connected session; output queue and pending-response bookkeeping are changed.
    /// - `params` `[in]`: unit payload; no document or stopped location is supplied.
    /// # Returns
    /// `Ok(())` after client acknowledgement, or `Err(error)` on request failure.
    /// # Errors
    /// Returns errors for disallowed lifecycle/capability, exhausted IDs or pending capacity, disconnection, peer errors, or malformed replies.
    /// Nested adapters may also reject invalid business values before any message is sent.
    /// # Examples
    /// A running wrapper must supply the connected handle and negotiated parameters.
    /// ```no_run
    /// use lspally::server::model::*;
    /// async fn send(client: &ClientHandle, params: ()) -> ServerResult<()> {
    ///     client.inline_value_refresh_request(params).await
    /// }
    /// ```
    pub async fn inline_value_refresh_request(&self, params: ()) -> ServerResult<()> {
        let params = params;
        let result = self
            .request::<lsp_types::request::InlineValueRefreshRequest>(params)
            .await?;
        Ok::<_, ServerError>(result)
    }
    /// Ask the client to refresh pull diagnostics.
    /// Enqueues output to the shared transport; awaits flush and a matching client reply.
    /// Use [`Self::publish_diagnostics`] for push replacement reports instead.
    ///
    /// # Arguments
    /// - `self` `[in,out]`: connected session; output queue and pending-response bookkeeping are changed.
    /// - `params` `[in]`: unit payload; the refresh is not a pushed diagnostic report.
    /// # Returns
    /// `Ok(())` after client acknowledgement, or `Err(error)` on request failure.
    /// # Errors
    /// Returns errors for disallowed lifecycle/capability, exhausted IDs or pending capacity, disconnection, peer errors, or malformed replies.
    /// Nested adapters may also reject invalid business values before any message is sent.
    /// # Examples
    /// A running wrapper must supply the connected handle and negotiated parameters.
    /// ```no_run
    /// use lspally::server::model::*;
    /// async fn send(client: &ClientHandle, params: ()) -> ServerResult<()> {
    ///     client.workspace_diagnostic_refresh(params).await
    /// }
    /// ```
    pub async fn workspace_diagnostic_refresh(&self, params: ()) -> ServerResult<()> {
        let params = params;
        let result = self
            .request::<lsp_types::request::WorkspaceDiagnosticRefresh>(params)
            .await?;
        Ok::<_, ServerError>(result)
    }
    /// Send a trace message according to the session's current trace mode.
    /// If enabled, enqueues output and awaits flush, not a client reply.
    /// Off sends nothing. Messages mode removes verbose details from the owned payload; verbose mode keeps them. No caller-owned output is changed.
    ///
    /// # Arguments
    /// - `self` `[in,out]`: connected session; output queue is changed.
    /// - `params` `[in]`: message and optional verbose details; consumed, not mutated in the caller.
    /// # Returns
    /// `Ok(())` when tracing is off or the filtered message is flushed; `Err(error)` on conversion or delivery failure.
    /// # Errors
    /// Returns errors for disallowed lifecycle, closed transport or writer failure.
    /// Nested adapters may also reject invalid business values before any message is sent.
    /// # Examples
    /// A running wrapper must supply the connected handle and negotiated parameters.
    /// ```no_run
    /// use lspally::server::model::*;
    /// async fn send(client: &ClientHandle, params: LogTraceParams) -> ServerResult<()> {
    ///     client.log_trace(params).await
    /// }
    /// ```
    /// # Algorithm
    /// Read the trace atomic. Off returns immediately; messages-only mode removes
    /// verbose details from the owned payload; all other nonzero modes keep them.
    /// Encode the remaining payload and send through normal notification checks.
    /// # Flow
    /// ```text
    ///           <trace mode?>
    ///          /      |       \
    ///        off   messages   verbose
    ///         |       |          |
    ///     [Ok(())] [drop detail] [keep detail]
    ///                 |          |
    ///                 +-----+----+
    ///                       v
    ///                 [encode + notify]
    /// ```
    pub async fn log_trace(&self, mut params: LogTraceParams) -> ServerResult<()> {
        use std::sync::atomic::Ordering;
        match self.session.trace.load(Ordering::Acquire) {
            0 => return Ok(()),
            1 => params.verbose = None,
            _ => {}
        }
        let params = encode_log_trace_params(params)?;
        self.notify::<lsp_types::notification::LogTrace>(params)
            .await
    }
    /// Send a user-facing message without waiting for a user choice.
    /// Enqueues output to the shared transport; awaits flush, not a client reply.
    /// This notification is allowed during initialization; [`Self::show_message_request`] offers action choices.
    ///
    /// # Arguments
    /// - `self` `[in,out]`: connected session; output queue is changed.
    /// - `params` `[in]`: message severity and display text.
    /// # Returns
    /// `Ok(())` after writer flush; `Err(error)` on conversion or delivery failure. There is no peer acknowledgement.
    /// # Errors
    /// Returns errors for disallowed lifecycle, closed transport or writer failure.
    /// Nested adapters may also reject invalid business values before any message is sent.
    /// # Examples
    /// A running wrapper must supply the connected handle and negotiated parameters.
    /// ```no_run
    /// use lspally::server::model::*;
    /// async fn send(client: &ClientHandle, params: ShowMessageParams) -> ServerResult<()> {
    ///     client.show_message(params).await
    /// }
    /// ```
    pub async fn show_message(&self, params: ShowMessageParams) -> ServerResult<()> {
        let params = encode_show_message_params(params)?;
        self.notify::<lsp_types::notification::ShowMessage>(params)
            .await
    }
    /// Send a message for the client's log, not a modal user prompt.
    /// Enqueues output to the shared transport; awaits flush, not a client reply.
    /// Allowed during initialization; no server stdout logging is used.
    ///
    /// # Arguments
    /// - `self` `[in,out]`: connected session; output queue is changed.
    /// - `params` `[in]`: severity and log text.
    /// # Returns
    /// `Ok(())` after writer flush; `Err(error)` on conversion or delivery failure. There is no peer acknowledgement.
    /// # Errors
    /// Returns errors for disallowed lifecycle, closed transport or writer failure.
    /// Nested adapters may also reject invalid business values before any message is sent.
    /// # Examples
    /// A running wrapper must supply the connected handle and negotiated parameters.
    /// ```no_run
    /// use lspally::server::model::*;
    /// async fn send(client: &ClientHandle, params: LogMessageParams) -> ServerResult<()> {
    ///     client.log_message(params).await
    /// }
    /// ```
    pub async fn log_message(&self, params: LogMessageParams) -> ServerResult<()> {
        let params = encode_log_message_params(params)?;
        self.notify::<lsp_types::notification::LogMessage>(params)
            .await
    }
    /// Send an application-defined telemetry object or array to the client.
    /// Enqueues output to the shared transport; awaits flush, not a client reply.
    /// Allowed during initialization; does not add a telemetry schema or collect telemetry itself.
    ///
    /// # Arguments
    /// - `self` `[in,out]`: connected session; output queue is changed.
    /// - `params` `[in]`: object (`Left`) or array (`Right`) whose nested business data is converted to JSON.
    /// # Returns
    /// `Ok(())` after writer flush; `Err(error)` on conversion or delivery failure. There is no peer acknowledgement.
    /// # Errors
    /// Non-finite floating-point payloads return InternalError before sending. Returns errors for disallowed lifecycle, closed transport or writer failure.
    /// Nested adapters may also reject invalid business values before any message is sent.
    /// # Examples
    /// A running wrapper must supply the connected handle and negotiated parameters.
    /// ```no_run
    /// use lspally::server::model::*;
    /// async fn send(client: &ClientHandle, params: OneOf<LSPObject, LSPArray>) -> ServerResult<()> {
    ///     client.telemetry_event(params).await
    /// }
    /// ```
    /// # Algorithm
    /// Preserve the object/array choice, recursively convert each data value and
    /// stop on the first invalid number before sending the complete notification.
    /// # Flow
    /// ```text
    ///            <payload?>
    ///           /          \
    ///        object        array
    ///          |             |
    ///     [map entries] [map elements]
    ///          |             |
    ///          +------+------+
    ///                 v
    ///        [converted or error]
    ///                 |
    ///          [notify if valid]
    /// ```
    pub async fn telemetry_event(&self, params: OneOf<LSPObject, LSPArray>) -> ServerResult<()> {
        let params = match params {
            d::OneOf::Left(v) => lsp_types::OneOf::Left(
                (v).into_iter()
                    .map(|(k, v)| Ok::<_, ServerError>((k, data_to_wire(v)?)))
                    .collect::<Result<_, ServerError>>()?,
            ),
            d::OneOf::Right(v) => lsp_types::OneOf::Right(
                (v).into_iter()
                    .map(data_to_wire)
                    .collect::<Result<Vec<_>, ServerError>>()?,
            ),
        };
        self.notify::<lsp_types::notification::TelemetryEvent>(params)
            .await
    }
    /// Replace the client's diagnostic report for one URI.
    /// Enqueues output to the shared transport; awaits flush, not a client reply.
    /// Does not analyze the document, merge prior diagnostics or write its contents.
    ///
    /// # Arguments
    /// - `self` `[in,out]`: connected session; output queue is changed.
    /// - `report` `[in]`: complete replacement list and optional document version; an empty list clears the report.
    /// # Returns
    /// `Ok(())` after writer flush; `Err(error)` on conversion or delivery failure. There is no peer acknowledgement.
    /// # Errors
    /// Invalid output URIs and non-finite diagnostic data return InternalError before sending. Returns errors for disallowed lifecycle, closed transport or writer failure.
    /// Nested adapters may also reject invalid business values before any message is sent.
    /// # Examples
    /// A running wrapper must supply the connected handle and negotiated parameters.
    /// ```no_run
    /// use lspally::server::model::*;
    /// async fn send(client: &ClientHandle, report: DiagnosticReport) -> ServerResult<()> {
    ///     client.publish_diagnostics(report).await
    /// }
    /// ```
    pub async fn publish_diagnostics(&self, report: DiagnosticReport) -> ServerResult<()> {
        let params = encode_publish_diagnostics_params(PublishDiagnosticsParams {
            uri: report.uri,
            version: report.version,
            diagnostics: report.diagnostics,
        })?;
        self.notify::<lsp_types::notification::PublishDiagnostics>(params)
            .await
    }
    /// Send arbitrary partial-result or work-done progress without narrowing its JSON shape.
    /// Enqueues output to the shared transport; awaits flush, not a client reply.
    /// Builds a raw `$/progress` object to preserve partial results unsupported by the work-done enum. Before RUNNING only the initialize request's token is accepted.
    ///
    /// # Arguments
    /// - `self` `[in,out]`: connected session; output queue is changed.
    /// - `params` `[in]`: progress identity and arbitrary value, including method-specific partial-result arrays.
    /// # Returns
    /// `Ok(())` after writer flush; `Err(error)` on payload conversion or lifecycle/delivery failure. There is no peer acknowledgement.
    /// # Errors
    /// Non-finite payload numbers return InternalError; a missing/mismatched initialization token returns InvalidParams. Returns errors for disallowed lifecycle, closed transport or writer failure.
    /// Nested adapters may also reject invalid business values before any message is sent.
    /// # Examples
    /// A running wrapper must supply the connected handle and negotiated parameters.
    /// ```no_run
    /// use lspally::server::model::*;
    /// async fn send(client: &ClientHandle, params: ProgressUpdate) -> ServerResult<()> {
    ///     client.progress(params).await
    /// }
    /// ```
    /// # Algorithm
    /// Convert the numeric/string token without changing its representation. Build
    /// an object with `token` and recursively converted `value`, then send its raw
    /// notification through the normal lifecycle and initialization-token checks.
    /// # Flow
    /// ```text
    /// [numeric token]    [string token]    [Data value]
    ///        |                 |               |
    ///        +--------+--------+       [convert recursively]
    ///                 |                        |
    ///                 +-----------+------------+
    ///                             v
    ///                 [object: token + value]
    ///                             |
    ///                    [notify_message]
    /// ```
    pub async fn progress(&self, params: ProgressUpdate) -> ServerResult<()> {
        let token = match params.token {
            NumberOrString::Number(value) => serde_json::Value::from(value),
            NumberOrString::String(value) => serde_json::Value::from(value),
        };
        let mut object = serde_json::Map::new();
        object.insert("token".into(), token);
        object.insert("value".into(), data_to_wire(params.value)?);
        self.notify_message(rpc::Notification {
            jsonrpc: rpc::Version::V2,
            method: "$/progress".into(),
            params: Some(rpc::Params::Left(object)),
        })
        .await
    }
    /// Send a typed begin/report/end work-done progress notification.
    /// Enqueues output to the shared transport; awaits flush, not a client reply.
    /// Before RUNNING, only updates using the initialize request's progress token are allowed.
    ///
    /// # Arguments
    /// - `self` `[in,out]`: connected session; output queue is changed.
    /// - `params` `[in]`: progress token and work-done update; use [`Self::progress`] for arbitrary partial results.
    /// # Returns
    /// `Ok(())` after writer flush; `Err(error)` on conversion or lifecycle/delivery failure. There is no peer acknowledgement.
    /// # Errors
    /// A missing/mismatched initialization token returns InvalidParams. Returns errors for disallowed lifecycle, closed transport or writer failure.
    /// Nested adapters may also reject invalid business values before any message is sent.
    /// # Examples
    /// A running wrapper must supply the connected handle and negotiated parameters.
    /// ```no_run
    /// use lspally::server::model::*;
    /// async fn send(client: &ClientHandle, params: ProgressParams) -> ServerResult<()> {
    ///     client.work_done_progress(params).await
    /// }
    /// ```
    pub async fn work_done_progress(&self, params: ProgressParams) -> ServerResult<()> {
        self.notify::<lsp_types::notification::Progress>(encode_progress_params(params)?)
            .await
    }
    /// Send a typed client request and correlate its response while preserving cancellation cleanup.
    /// Performs transport I/O through the session writer; no direct filesystem access.
    ///
    /// # Arguments
    /// - `self` `[in,out]`: session whose next ID, pending map and writer queue are changed.
    /// - `params` `[in]`: owned parameters for the request marker `R`.
    /// # Returns
    /// `Ok(result)` decoded as `R::Result`; `Err(error)` on admission, transport, peer or decoding failure.
    /// # Errors
    /// Disallowed lifecycle returns -32002; missing capability returns MethodNotFound. ID exhaustion, poisoned pending lock, pending limit, closure, serialization, remote errors and invalid replies also fail.
    /// # Algorithm
    /// Check lifecycle/capability, allocate an ID and insert its response sender under a short lock.
    /// Recheck shutdown under that lock. Keep a drop guard for abandoned waits, send and flush the
    /// request, then race the reply against session closure. Decode against the original ID/method
    /// envelope without retaining its parameter tree.
    /// # Flow
    /// ```text
    /// [check phase + capability]
    ///                 |
    ///          <admitted?>
    ///           /       \
    ///         no        yes
    ///          |         |
    ///       [error] [allocate ID + pending sender]
    ///                    |
    ///              [send and flush]
    ///                    |
    ///             +------+-------+
    ///             v              v
    ///        [peer reply]    [session closed]
    ///             |              |
    ///      [finish guard]     [drop guard]
    ///             |              |
    ///       [decode result]    [error]
    /// ```
    /// # Examples
    /// ```text
    /// let action = client.request::<lsp_types::request::ShowMessageRequest>(wire_params).await?;
    /// ```
    async fn request<R: LspRequest>(&self, params: R::Params) -> ServerResult<R::Result> {
        use super::{
            lifecycle,
            pending::{PendingGuard, disconnected},
        };
        use std::sync::atomic::Ordering;
        use tokio::sync::oneshot;
        let phase = self.session.phase.load(Ordering::Acquire);
        if phase >= lifecycle::SHUTTING_DOWN
            || (phase < lifecycle::RUNNING && R::METHOD != "window/showMessageRequest")
        {
            return Err(ServerError::new(
                -32002,
                "Client request is not allowed in this lifecycle phase",
            ));
        }
        if !self.session.supported.lock().await.contains(R::METHOD) {
            return Err(ServerError::unsupported());
        }
        let id = self.allocate_id()?;
        let request = rpc::Request::new::<R>(id.clone(), params).map_err(codec_error)?;
        let (sender, received) = oneshot::channel();
        {
            let mut pending = self.session.outgoing.lock().map_err(|_| disconnected())?;
            // Shutdown retires this map; a capability-lock wait must not insert a new waiter afterward.
            if self.session.phase.load(Ordering::Acquire) >= lifecycle::SHUTTING_DOWN {
                return Err(ServerError::new(-32002, "Server is shutting down"));
            }
            if pending.len() >= self.session.pending_limit {
                return Err(ServerError::new(
                    -32000,
                    "Pending client request limit exceeded",
                ));
            }
            pending.insert(id.clone(), sender);
        }
        let guard = PendingGuard {
            session: self.session.clone(),
            id: Some(id),
        };
        // Keep the envelope for ID/method correlation, but do not retain its parameter tree.
        let correlation = rpc::Request {
            jsonrpc: rpc::Version::V2,
            id: request.id.clone(),
            method: request.method.clone(),
            params: None,
        };
        self.session.send(rpc::Message::Request(request)).await?;
        let response = tokio::select! {
            _ = self.session.closed.cancelled() => return Err(disconnected()),
            response = received => response.map_err(|_| disconnected())?,
        };
        guard.finish();
        correlation
            .decode_response::<R>(response)
            .map_err(|error| match error {
                rpc::DecodeError::Remote(error) => ServerError::new(error.code, error.message),
                error => codec_error(error),
            })
    }
    /// Construct a typed notification and send it through the lifecycle-checked transport.
    /// Performs transport I/O through the session writer; no direct filesystem access.
    ///
    /// # Arguments
    /// - `self` `[in,out]`: session whose writer queue receives the notification.
    /// - `params` `[in]`: owned parameters for notification marker `N`.
    /// # Returns
    /// `Ok(())` after writer flush; `Err(error)` if encoding, lifecycle validation or delivery fails.
    /// # Errors
    /// Serialization errors become InternalError; lifecycle and transport failures propagate from `notify_message`.
    /// # Algorithm
    /// Serialize the typed notification once, then delegate to `notify_message`; no peer response is
    /// registered or awaited.
    /// # Flow
    /// ```text
    /// [typed parameters]
    ///               |
    ///         <serialize?>
    ///          /        \
    ///       failure    success
    ///          |          |
    ///       [error] [notify_message]
    ///                     |
    ///              [flush or error]
    /// ```
    /// # Examples
    /// ```text
    /// client.notify::<lsp_types::notification::LogMessage>(wire_params).await?;
    /// ```
    async fn notify<N: LspNotification>(&self, params: N::Params) -> ServerResult<()> {
        self.notify_message(rpc::Notification::new::<N>(params).map_err(codec_error)?)
            .await
    }
    /// Enforce session output restrictions, then enqueue and flush an RPC notification.
    /// Performs transport I/O through the session writer; no direct filesystem access.
    ///
    /// # Arguments
    /// - `self` `[in,out]`: session whose output queue is changed; initialization token is read under its lock.
    /// - `message` `[in]`: owned notification envelope to send unchanged.
    /// # Returns
    /// `Ok(())` after writer flush; `Err(error)` if the message is forbidden or transport fails.
    /// # Errors
    /// Shut-down sessions return InternalError. Before RUNNING, only show/log messages, telemetry and progress are permitted (-32002 otherwise); progress needs the initialization token (-32602 otherwise). Writer/closure errors propagate.
    /// # Algorithm
    /// Read phase first. Before RUNNING allow the restricted method set and verify progress identity
    /// against the retained initialization token. Hand accepted messages to the single session writer
    /// and await its flush acknowledgement.
    /// # Flow
    /// ```text
    /// [notification + phase]
    ///                  |
    ///            <shut down?>
    ///              /      \
    ///            yes      no
    ///             |        |
    ///          [error] <before RUNNING?>
    ///                     /      \
    ///                   yes      no
    ///                    |        |
    ///         [check method/token] |
    ///                    |        |
    ///                    +----+---+
    ///                         v
    ///                  [send + flush]
    /// ```
    /// # Examples
    /// ```text
    /// client.notify_message(notification).await?;
    /// ```
    async fn notify_message(&self, message: rpc::Notification) -> ServerResult<()> {
        use super::lifecycle;
        use std::sync::atomic::Ordering;
        let phase = self.session.phase.load(Ordering::Acquire);
        if phase >= lifecycle::SHUT_DOWN {
            return Err(ServerError::new(-32603, "Server is shut down"));
        }
        if phase < lifecycle::RUNNING {
            if !matches!(
                message.method.as_str(),
                "window/showMessage" | "window/logMessage" | "telemetry/event" | "$/progress"
            ) {
                return Err(ServerError::new(
                    -32002,
                    "Notification is not allowed before initialization",
                ));
            }
            if message.method == "$/progress" {
                let token = match &message.params {
                    Some(rpc::Params::Left(object)) => object.get("token"),
                    _ => None,
                };
                let expected = self.session.initialization_progress.lock().await;
                if token
                    .and_then(|v| {
                        serde_json::from_value::<lsp_types::ProgressToken>(v.clone()).ok()
                    })
                    .as_ref()
                    != expected.as_ref()
                    || expected.is_none()
                {
                    return Err(ServerError::invalid(
                        "Initialization progress needs the client's initialization token",
                    ));
                }
            }
        }
        self.session.send(rpc::Message::Notification(message)).await
    }
}
