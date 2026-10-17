//! List and dispatch explicitly requested runtime actions; listing never runs CMake.
//!
//! Configure and script actions execute project code, unlike local hover help inspection.
//! Manual executeCommand and globally authorized on-save scheduling share the same entrypoints.
//! The service checks ownership, saved inputs and job budgets before starting a child. Edits,
//! saves, closes, folder changes and successful reloads invalidate results. Completion is installed
//! only for the current tracked run and matching profile after saved-input checks; stale runs are
//! discarded.
//! Cancellation signals direct-child cleanup; it is not a sandbox or a descendant-process guarantee.
//!
//! ```text
//!  [Code Action request]       [executeCommand / authorized save]
//!           |                              |
//!  [Return commands only]          [Validate owner/profile]
//!                                          |
//!                          +---------------+---------------+
//!                          v               v               v
//!                     [Start job]    [Inspect trace]   [Cancel job]
//!                          |
//!                  [Current run + saved?] --no--> [Discard]
//!                          |
//!                         yes
//!                          v
//!                  [Install runtime layer]
//! ```

// AI-authored action registry. Listing actions never executes CMake.
mod cancel_configure;
mod configure;
mod expanded_arguments;
mod run_script;
use crate::runtime_cmake::model::ExecutionMode;
pub(in crate::server) mod model;
use super::{model::*, uri};
use lspally::server::model::*;
use model::*;
use tokio::sync::oneshot;
impl CmakeAnalyzer {
    /// List explicit source.cmake commands through the ordered service queue; never execute CMake.
    ///
    /// # Arguments
    /// - `self` `[in]`: analyzer handle; enqueues a query without changing document text.
    /// - `context` `[in]`: request cancellation and client context.
    /// - `params` `[in]`: document, starting line, trigger and requested action kinds.
    ///
    /// # Returns
    /// Ok(Some(actions)), including an empty list for automatic triggers or excluded kinds; otherwise
    /// the service reply.
    ///
    /// # Errors
    /// Queue/service failure, cancellation or action-list validation errors.
    ///
    /// # Algorithm
    /// Return no actions for automatic triggers or filters excluding source.cmake. Otherwise send the
    /// document and starting line through the ordered queue and race its reply against cancellation.
    ///
    /// # Flow
    /// ```text
    ///  [Trigger + requested kinds]
    ///             |
    ///       [Explicit source request?] --no--> [Empty actions]
    ///             | yes
    ///       [Enqueue owner query]
    ///             |
    ///        +----+-----------+
    ///        v                v
    ///   [Service reply] [Cancellation error]
    /// ```
    ///
    /// # Examples
    /// Local server context:
    /// ```text
    /// let listed = analyzer.code_actions(context, params).await?;
    /// ```
    pub(in crate::server) async fn code_actions(
        &self,
        context: Context,
        params: CodeActionParams,
    ) -> ServerResult<Option<CodeActionResponse>> {
        if params.context.trigger_kind.is_some_and(|kind| kind.0 == 2) {
            return Ok(Some(Vec::new()));
        }
        if params.context.only.as_ref().is_some_and(|kinds| {
            !kinds
                .iter()
                .any(|kind| kind.0.is_empty() || kind.0 == "source" || kind.0 == "source.cmake")
        }) {
            return Ok(Some(Vec::new()));
        }
        let (reply, response) = oneshot::channel();
        self.send(ServiceCommand::Actions {
            uri: params.text_document.uri,
            line: u64::from(params.range.start.line),
            reply,
        })
        .await?;
        tokio::select! {_=context.cancellation.cancelled()=>Err(ServerError::cancelled()),result=response=>result.map_err(|_|ServerError::new(-32603,"Workspace service stopped"))?}
    }
    /// Decode an explicit command and enqueue its effect; configure/script commands can execute project
    /// code.
    ///
    /// # Arguments
    /// - `self` `[in]`: analyzer queue handle.
    /// - `context` `[in]`: request token cloned into the action payload.
    /// - `params` `[in]`: command and URI, root file, zero-based line, optional configure/script profile
    ///   arguments.
    ///
    /// # Returns
    /// Ok(Some(data)) with the action-specific result; Option is forwarded unchanged from the
    /// dispatcher.
    ///
    /// # Errors
    /// Unsupported command, malformed/extra arguments, line at or above u32::MAX, queue closure,
    /// cancellation or action failure. Cancelling the reply wait does not cancel an already admitted
    /// independent runtime job.
    ///
    /// # Algorithm
    /// Recognize the command, strictly decode positional arguments, then send one owned request and
    /// await reply or cancellation.
    ///
    /// # Flow
    /// ```text
    ///  [Command + arguments]
    ///           |
    ///      [Decode valid?] --no--> [Invalid/unsupported]
    ///           | yes
    ///      [Enqueue request]
    ///           |
    ///      [Reply or cancel] ----> [Action data / error]
    /// ```
    ///
    /// # Examples
    /// Local server context:
    /// ```text
    /// let result = analyzer.execute_action(context, params).await?;
    /// ```
    pub(in crate::server) async fn execute_action(
        &self,
        context: Context,
        params: ExecuteCommandParams,
    ) -> ServerResult<Option<Data>> {
        let kind = ActionKind::parse(&params.command).ok_or_else(ServerError::unsupported)?;
        let mut arguments = params.arguments.into_iter();
        let Some(Data::Text(uri)) = arguments.next() else {
            return Err(ServerError::invalid("Expected document URI"));
        };
        let Some(Data::Text(root)) = arguments.next() else {
            return Err(ServerError::invalid("Expected project root file"));
        };
        let line = match arguments.next() {
            Some(Data::Unsigned(value)) => value,
            Some(Data::Integer(value)) if value >= 0 => value as u64,
            _ => return Err(ServerError::invalid("Expected source line")),
        };
        if line >= u64::from(u32::MAX) {
            return Err(ServerError::invalid("Source line exceeds protocol limits"));
        }
        let execution = match arguments.next() {
            None => None,
            Some(Data::Text(mode)) if mode == "configure" => Some(ExecutionMode::Configure),
            Some(Data::Text(mode)) if mode == "script" => Some(ExecutionMode::Script {
                file: std::path::PathBuf::from(&root),
            }),
            _ => {
                return Err(ServerError::invalid(
                    "Expected configure or script execution profile",
                ));
            }
        };
        if arguments.next().is_some() {
            return Err(ServerError::invalid("Unexpected action arguments"));
        }
        let request = ActionRequest {
            kind,
            uri: Uri(uri),
            root: std::path::PathBuf::from(root),
            line,
            cancellation: context.cancellation.clone(),
            execution,
        };
        let (reply, response) = oneshot::channel();
        self.send(ServiceCommand::RunAction { request, reply })
            .await?;
        tokio::select! {_=context.cancellation.cancelled()=>Err(ServerError::cancelled()),result=response=>result.map_err(|_|ServerError::new(-32603,"Workspace service stopped"))?}
    }
}
impl Service {
    /// Find a non-standalone owner matching the request root and execution profile; no I/O or mutation.
    ///
    /// # Arguments
    /// - `self` `[in]`: active projects and ownership tables.
    /// - `request` `[in]`: document URI, root, action kind and optional profile.
    ///
    /// # Returns
    /// Ok(owner index) for the first matching project.
    ///
    /// # Errors
    /// Invalid local URI or no active owner with a compatible profile. Configure requires Configure
    /// mode; RunScript requires Script mode.
    ///
    /// # Algorithm
    /// Convert the URI to a local path, then choose its first non-standalone owner with the exact
    /// root and requested optional profile. Configure/script commands additionally require their
    /// corresponding execution mode; inspection and cancellation accept either mode.
    ///
    /// # Flow
    /// ```text
    ///  [URI -> local path]
    ///          |
    ///   [Next owning project] <-------------------+
    ///          |                                  |
    ///   [Root / profile / kind match?] --no--------+
    ///          | yes                        no owners left
    ///   [Return owner index]                      |
    ///                                      [Invalid action]
    /// ```
    ///
    /// # Examples
    /// Local server context (requires the initialized service and its client):
    /// ```text
    /// let owner = service.action_project(&request)?;
    /// ```
    pub(in crate::server) fn action_project(&self, request: &ActionRequest) -> ServerResult<usize> {
        let path = uri::to_path(&request.uri)?;
        self.owners(&path)
            .into_iter()
            .find(|index| {
                self.projects[*index].workspace.root().file == request.root
                    && !self.projects[*index].standalone
                    && request
                        .execution
                        .as_ref()
                        .is_none_or(|mode| self.projects[*index].execution.as_ref() == Some(mode))
                    && match request.kind {
                        ActionKind::Configure => matches!(
                            self.projects[*index].execution,
                            Some(ExecutionMode::Configure)
                        ),
                        ActionKind::RunScript => matches!(
                            self.projects[*index].execution,
                            Some(ExecutionMode::Script { .. })
                        ),
                        _ => true,
                    }
            })
            .ok_or_else(|| ServerError::invalid("Action does not belong to an active project"))
    }
    /// Build command-only Code Actions for enabled related projects; no edits, client calls or process
    /// execution.
    ///
    /// # Arguments
    /// - `self` `[in]`: projects, runtime layers and queued/live job tables.
    /// - `uri` `[in]`: document whose owners supply actions.
    /// - `line` `[in]`: zero-based line copied to command arguments.
    ///
    /// # Returns
    /// Ok(Some(actions)); empty if no enabled non-standalone execution profile owns the file.
    ///
    /// # Errors
    /// Invalid local URI or a non-UTF-8 project entry path.
    ///
    /// # Algorithm
    /// For each eligible owner, offer its run command, observed-argument inspection if a layer exists,
    /// and cancellation if a matching queued/live job exists. Include a profile argument to distinguish
    /// shared roots.
    ///
    /// # Flow
    /// ```text
    ///  [Document owners]
    ///          |
    ///    [Enabled profile?] --no--> [Skip owner]
    ///          | yes                   |
    ///    [Run + optional view/cancel]   |
    ///          +-----------------------+
    ///          | next owner
    ///    [Return command list]
    /// ```
    ///
    /// # Examples
    /// Local server context (requires the initialized service and its client):
    /// ```text
    /// let actions = service.actions(uri, 0)?;
    /// ```
    pub(in crate::server) fn actions(
        &self,
        uri: Uri,
        line: u64,
    ) -> ServerResult<Option<CodeActionResponse>> {
        let path = uri::to_path(&uri)?;
        let mut actions = Vec::new();
        for owner in self.owners(&path) {
            let project = &self.projects[owner];
            let root = &project.workspace.root().file;
            if project.standalone || !project.settings.runtime.enable || project.execution.is_none()
            {
                continue;
            }
            let root_text = root
                .to_str()
                .ok_or_else(|| ServerError::invalid("Project path is not UTF-8"))?;
            let (kind, label) = match &project.execution {
                Some(ExecutionMode::Script { .. }) => (ActionKind::RunScript, "Run CMake script"),
                _ => (ActionKind::Configure, "Run CMake"),
            };
            let mut entries = vec![(
                kind,
                format!("{label} for {root_text} (executes project code)"),
            )];
            if project.workspace.runtime().is_some() {
                entries.push((
                    ActionKind::ExpandedArguments,
                    "Show observed expanded arguments at this line".into(),
                ));
            }
            if self
                .running
                .get(root)
                .is_some_and(|job| project.execution.as_ref() == Some(&job.mode))
                || self
                    .saved_runs
                    .get(root)
                    .is_some_and(|saved| project.execution.as_ref() == Some(&saved.mode))
            {
                entries.push((ActionKind::CancelConfigure, "Cancel CMake run".into()));
            }
            for (kind, title) in entries {
                actions.push(CodeActionOrCommand::CodeAction(CodeAction {
                    title: title.clone(),
                    kind: Some(CodeActionKind("source.cmake".into())),
                    diagnostics: None,
                    edit: None,
                    command: Some(Command {
                        title,
                        command: kind.command().into(),
                        arguments: Some(vec![
                                Data::Text(uri.0.clone()),
                                Data::Text(root_text.into()),
                                Data::Unsigned(line),
                                Data::Text(
                                    if matches!(
                                        project.execution,
                                        Some(ExecutionMode::Script { .. })
                                    ) {
                                        "script"
                                    } else {
                                        "configure"
                                    }
                                    .into(),
                                ),
                            ]),
                    }),
                    is_preferred: None,
                    disabled: None,
                    data: None,
                }));
            }
        }
        Ok(Some(actions))
    }
    /// Dispatch one selected action; runtime actions execute code, inspection reads saved state,
    /// cancellation signals cleanup.
    ///
    /// # Arguments
    /// - `self` `[in,out]`: service whose jobs, saved-run queue, runtime inputs or client messages may
    ///   change; document text is not overwritten.
    /// - `request` `[in]`: owned action and project selector.
    ///
    /// # Returns
    /// Forwarded Ok(Some(data)): started/run for execution, run/events/truncated for inspection, or a
    /// boolean for cancellation. No branch currently returns Ok(None).
    ///
    /// # Errors
    /// The selected entrypoint's ownership, saved-input, budget, process-admission or client-message
    /// errors.
    ///
    /// # Examples
    /// Local server context (requires the initialized service and its client):
    /// ```text
    /// let result = service.run_action(request).await?;
    /// ```
    pub(in crate::server) async fn run_action(
        &mut self,
        request: ActionRequest,
    ) -> ServerResult<Option<Data>> {
        match request.kind {
            ActionKind::Configure => configure::run(self, request).await,
            ActionKind::RunScript => run_script::run(self, request).await,
            ActionKind::ExpandedArguments => expanded_arguments::run(self, request).await,
            ActionKind::CancelConfigure => cancel_configure::run(self, request).await,
        }
    }
}
