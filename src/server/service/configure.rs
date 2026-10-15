// AI-authored background CMake coordination; the actor remains available for edits.
use super::{
    super::{action::model::ActionRequest, model::*},
    internal,
};
use crate::runtime_cmake::{
    self,
    model::{ConfigureOptions, ExecutionMode, RunId},
};
use lspally::server::model::*;
use std::{collections::BTreeMap, path::PathBuf, sync::Arc, time::Duration};
impl Service {
    pub(in crate::server) async fn require_saved(&self, owner: usize) -> ServerResult<()> {
        let project = &self.projects[owner];
        let mut inputs: BTreeMap<PathBuf, Arc<str>> = BTreeMap::new();
        if let Some(graph) = project.workspace.root_graph() {
            inputs.extend(
                graph
                    .invocations
                    .iter()
                    .map(|visit| (visit.file.clone(), visit.snapshot.text.clone())),
            );
        }
        if let Some(layer) = project.workspace.runtime() {
            inputs.extend(
                layer
                    .documents
                    .iter()
                    .filter(|(file, _)| {
                        !layer
                            .trace
                            .build_directory
                            .as_ref()
                            .is_some_and(|build| file.starts_with(build))
                    })
                    .map(|(file, snapshot)| (file.clone(), snapshot.text.clone())),
            );
        }
        for (path, document) in &self.documents {
            if self.owners(path).contains(&owner) {
                inputs.insert(path.clone(), document.text.clone());
            }
        }
        for (path, text) in inputs {
            let disk = tokio::fs::read_to_string(&path).await.map_err(internal)?;
            if disk != text.as_ref() {
                return Err(ServerError::invalid(
                    "Save related documents and refresh changed disk inputs before running/inspecting CMake",
                ));
            }
        }
        Ok(())
    }
    async fn refresh_configure_inputs(&mut self, owner: usize) -> ServerResult<()> {
        let mut known = BTreeMap::new();
        if let Some(graph) = self.projects[owner].workspace.root_graph() {
            known.extend(
                graph
                    .invocations
                    .iter()
                    .map(|visit| (visit.file.clone(), visit.snapshot.text.clone())),
            );
        }
        if let Some(layer) = self.projects[owner].workspace.runtime() {
            known.extend(
                layer
                    .documents
                    .iter()
                    .map(|(file, snapshot)| (file.clone(), snapshot.text.clone())),
            );
        }
        for (file, document) in &self.documents {
            if self.owners(file).contains(&owner) {
                if tokio::fs::read_to_string(file).await.map_err(internal)?
                    != document.text.as_ref()
                {
                    return Err(ServerError::invalid(
                        "Save related documents before running CMake",
                    ));
                }
            }
        }
        for (file, old) in known {
            if tokio::fs::read_to_string(&file).await.map_err(internal)? != old.as_ref() {
                self.projects[owner]
                    .workspace
                    .invalidate_disk(&file)
                    .map_err(internal)?;
                self.projects[owner].dirty = true;
            }
        }
        self.flush().await
    }
    pub(in crate::server) async fn start_runtime(
        &mut self,
        request: ActionRequest,
    ) -> ServerResult<Option<Data>> {
        let owner = self.action_project(&request)?;
        let project = &self.projects[owner];
        let settings = project.settings.runtime.clone();
        if !settings.enable {
            return Err(ServerError::invalid("Runtime actions are disabled"));
        }
        let mode = project
            .execution
            .clone()
            .ok_or_else(|| ServerError::invalid("No configured runtime entry"))?;
        let script = matches!(mode, ExecutionMode::Script { .. });
        if script {
            crate::config::validate_script_arguments(&settings.arguments)
                .map_err(|error| ServerError::invalid(error.to_string()))?;
        }
        if self.running.contains_key(&request.root)
            || self.jobs.len() >= self.config.limits.max_workspaces
        {
            return Err(ServerError::invalid(
                "A CMake run is already running or the Workspace job limit is reached",
            ));
        }
        self.refresh_configure_inputs(owner).await?;
        let owner = self.action_project(&request)?;
        self.require_saved(owner).await?;
        let source = if script {
            self.config.sources.cwd.clone()
        } else {
            request
                .root
                .parent()
                .ok_or_else(|| ServerError::invalid("Project has no source directory"))?
                .to_owned()
        };
        let build = settings
            .build_directory
            .clone()
            .unwrap_or_else(|| source.join(".cmake-analyzer/build"));
        if !script {
            let source_actual = tokio::fs::canonicalize(&source).await.map_err(internal)?;
            if tokio::fs::canonicalize(&build).await.ok().as_ref() == Some(&source_actual) {
                return Err(ServerError::invalid(
                    "Build directory resolves to the source directory",
                ));
            }
            if build == source
                || self
                    .running
                    .values()
                    .any(|job| job.build_directory.as_ref() == Some(&build))
            {
                return Err(ServerError::invalid(
                    "Configure requires a separate, currently unused build directory",
                ));
            }
        }
        if request.cancellation.is_cancelled() {
            return Err(ServerError::cancelled());
        }
        self.run_counter = self
            .run_counter
            .checked_add(1)
            .ok_or_else(|| internal("Runtime run IDs exhausted"))?;
        let run = RunId::new(self.run_counter);
        let cancel = Cancellation::default();
        let options = ConfigureOptions {
            mode: mode.clone(),
            executable: settings.executable.clone(),
            source_directory: source,
            build_directory: build.clone(),
            arguments: settings.arguments.clone(),
            timeout: Duration::from_secs(settings.timeout_seconds),
            max_output_bytes: settings.max_output_bytes,
            max_error_bytes: settings.max_view_bytes,
        };
        let root = request.root.clone();
        let cancellation = cancel.clone();
        let execution = mode.clone();
        self.jobs.spawn(async move {
            RuntimeCompletion {
                root,
                run,
                mode,
                result: runtime_cmake::execute(run, options, cancellation).await,
            }
        });
        self.running.insert(
            request.root,
            RunningConfigure {
                mode: execution,
                run,
                cancel,
                build_directory: (!script).then_some(build),
            },
        );
        self.client
            .show_message(ShowMessageParams {
                typ: MessageType(3),
                message: format!(
                    "CMake run {} started; use Cancel CMake run to stop it",
                    run.value()
                ),
            })
            .await?;
        Ok(Some(Data::Object(BTreeMap::from([
            ("started".into(), Data::Bool(true)),
            ("run".into(), Data::Unsigned(run.value())),
        ]))))
    }
    pub(super) fn invalidate_runtime(&mut self) {
        for (_, task) in std::mem::take(&mut self.running) {
            task.cancel.cancel();
        }
        for project in &mut self.projects {
            project.workspace.clear_runtime();
        }
    }
    pub(super) async fn stop_configures(&mut self) {
        self.invalidate_runtime();
        while self.jobs.join_next().await.is_some() {}
    }
    pub(super) async fn configure_finished(
        &mut self,
        completion: Result<RuntimeCompletion, tokio::task::JoinError>,
    ) {
        let completion = match completion {
            Ok(value) => value,
            Err(error) => {
                eprintln!("Configure task: {error}");
                return;
            }
        };
        if !self
            .running
            .get(&completion.root)
            .is_some_and(|task| task.run == completion.run)
        {
            return;
        }
        let Some(task) = self.running.remove(&completion.root) else {
            return;
        };
        let Some(owner) = self.projects.iter().position(|project| {
            project.workspace.root().file == completion.root
                && !project.standalone
                && project.execution.as_ref() == Some(&completion.mode)
        }) else {
            return;
        };
        let result = async {
            self.require_saved(owner).await?;
            let trace = completion.result.map_err(internal)?;
            for file in trace
                .events
                .iter()
                .map(|event| &event.file)
                .collect::<std::collections::BTreeSet<_>>()
            {
                if let Some(document) = self.documents.get(file) {
                    if tokio::fs::read_to_string(file).await.map_err(internal)?
                        != document.text.as_ref()
                    {
                        return Err(ServerError::invalid(
                            "An observed source has unsaved changes; runtime result discarded",
                        ));
                    }
                }
            }
            self.projects[owner]
                .workspace
                .install_runtime_cancellable(trace, task.cancel)
                .await
                .map_err(internal)?;
            let project = &mut self.projects[owner];
            if let Some(layer) = project.workspace.runtime() {
                project.members.extend(layer.documents.keys().cloned());
            }
            self.published.clear();
            Ok::<(), ServerError>(())
        }
        .await;
        let (typ, message) = match result {
            Ok(()) => (
                MessageType(3),
                format!(
                    "CMake run {} completed; runtime observations are available",
                    completion.run.value()
                ),
            ),
            Err(error) => (
                MessageType(1),
                format!("CMake run {}: {error}", completion.run.value()),
            ),
        };
        let _ = self
            .client
            .show_message(ShowMessageParams { typ, message })
            .await;
    }
}
