// Script-generated standard client methods by scripts/gen/lspally-schema/src/main.rs.
// Contains AI-authored additions or edits.
use super::convert::*;
use crate::server::model as d;
use crate::{rpc, server::model::*};
use lsp_types::{notification::Notification as LspNotification, request::Request as LspRequest};
impl ClientHandle {
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
    pub async fn unregister_capability(&self, params: UnregistrationParams) -> ServerResult<()> {
        let params = encode_unregistration_params(params)?;
        let result = self
            .request::<lsp_types::request::UnregisterCapability>(params)
            .await?;
        Ok::<_, ServerError>(result)
    }
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
    pub async fn semantic_tokens_refresh(&self, params: ()) -> ServerResult<()> {
        let params = params;
        let result = self
            .request::<lsp_types::request::SemanticTokensRefresh>(params)
            .await?;
        Ok::<_, ServerError>(result)
    }
    pub async fn code_lens_refresh(&self, params: ()) -> ServerResult<()> {
        let params = params;
        let result = self
            .request::<lsp_types::request::CodeLensRefresh>(params)
            .await?;
        Ok::<_, ServerError>(result)
    }
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
    pub async fn inlay_hint_refresh_request(&self, params: ()) -> ServerResult<()> {
        let params = params;
        let result = self
            .request::<lsp_types::request::InlayHintRefreshRequest>(params)
            .await?;
        Ok::<_, ServerError>(result)
    }
    pub async fn inline_value_refresh_request(&self, params: ()) -> ServerResult<()> {
        let params = params;
        let result = self
            .request::<lsp_types::request::InlineValueRefreshRequest>(params)
            .await?;
        Ok::<_, ServerError>(result)
    }
    pub async fn workspace_diagnostic_refresh(&self, params: ()) -> ServerResult<()> {
        let params = params;
        let result = self
            .request::<lsp_types::request::WorkspaceDiagnosticRefresh>(params)
            .await?;
        Ok::<_, ServerError>(result)
    }
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
    pub async fn show_message(&self, params: ShowMessageParams) -> ServerResult<()> {
        let params = encode_show_message_params(params)?;
        self.notify::<lsp_types::notification::ShowMessage>(params)
            .await
    }
    pub async fn log_message(&self, params: LogMessageParams) -> ServerResult<()> {
        let params = encode_log_message_params(params)?;
        self.notify::<lsp_types::notification::LogMessage>(params)
            .await
    }
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
    pub async fn publish_diagnostics(&self, report: DiagnosticReport) -> ServerResult<()> {
        let params = encode_publish_diagnostics_params(PublishDiagnosticsParams {
            uri: report.uri,
            version: report.version,
            diagnostics: report.diagnostics,
        })?;
        self.notify::<lsp_types::notification::PublishDiagnostics>(params)
            .await
    }
    /// Partial-result progress is not limited to the work-done enum in lsp-types.
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
    pub async fn work_done_progress(&self, params: ProgressParams) -> ServerResult<()> {
        self.notify::<lsp_types::notification::Progress>(encode_progress_params(params)?)
            .await
    }
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
    async fn notify<N: LspNotification>(&self, params: N::Params) -> ServerResult<()> {
        self.notify_message(rpc::Notification::new::<N>(params).map_err(codec_error)?)
            .await
    }
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
