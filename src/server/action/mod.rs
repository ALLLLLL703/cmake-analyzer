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
