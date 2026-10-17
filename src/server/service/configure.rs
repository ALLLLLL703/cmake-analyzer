//! Coordinate background CMake execution without giving jobs mutable service state.
//! Saved-source checks guard both launch and installation. Run IDs reject completions
//! from removed or superseded jobs. Cancellation removes the live marker immediately;
//! the task remains in the join set until reaped. Only this opt-in path runs project
//! CMake code; shutdown cancels and joins every tracked task.
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
    /// Read all known source inputs and require exact equality with their analyzed/editor text.
    /// Ignores runtime documents inside the observed build directory; never saves buffers or changes caches.
    ///
    /// # Arguments
    /// - `self` `[in]`: Known snapshots and owner-qualified open texts read without modifying caches.
    /// - `owner` `[in]`: Valid current project index whose static/runtime/open inputs are checked.
    ///
    /// # Returns
    /// `Ok(())` when every collected input matches disk; `Err` on unreadable or different text.
    ///
    /// # Errors
    /// Disk read/UTF-8 errors become internal errors; differing text produces an invalid-parameters error asking for save/refresh.
    ///
    /// # Panics
    /// Panics if owner is not a current projects index.
    ///
    /// # Algorithm
    /// Collect static visits, then non-build runtime documents, then owner-qualified open texts into a path map, with later sources replacing earlier ones. Read each file asynchronously and stop on the first mismatch.
    ///
    /// # Flow
    /// ```text
    ///  [Static snapshots]     [Runtime outside build]
    ///          |                        |
    ///          +-----------+------------+
    ///                      v
    ///               [Path/text map] <---- [Owned open buffers override]
    ///                      |
    ///               [Read each file]
    ///                      |
    ///            <Exact text match?>
    ///              /             \
    ///             no             yes
    ///             |               |
    ///          [Error]       [Next file / Ok]
    /// ```
    ///
    /// # Examples
    /// ```text
    /// service.require_saved(owner).await?;
    /// ```
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
    /// Reject unsaved owned buffers, invalidate changed known disk inputs and flush refreshed analysis.
    /// Reads disk and mutates Workspace caches; does not launch CMake.
    ///
    /// # Arguments
    /// - `self` `[in,out]`: Workspace disk caches invalidated and dirty analysis flushed; editor buffers remain unchanged.
    /// - `owner` `[in]`: Valid current project index whose cached static/runtime sources are refreshed.
    ///
    /// # Returns
    /// `Ok(())` after the flush; `Err` for unsaved text, read/cache errors or flush admission/synchronization errors.
    ///
    /// # Errors
    /// Unreadable/non-UTF-8 inputs, open text differing from disk, Workspace invalidation errors, or flush failures. Some earlier invalidations may remain.
    ///
    /// # Panics
    /// Panics if owner is not a current projects index.
    ///
    /// # Algorithm
    /// Snapshot known static/runtime texts, verify all owned open buffers are saved, compare known files with disk, mark changed inputs invalid/dirty, then flush all service projects.
    ///
    /// # Flow
    /// ```text
    ///  [Known static/runtime text]      [Owned opens]
    ///               |                        |
    ///               |                <Saved on disk?>
    ///               |                 /           \
    ///               |                no           yes
    ///               |                |             |
    ///               |             [Error]          |
    ///               +------------------------------+
    ///               v
    ///        [Invalidate changed disk inputs]
    ///               |
    ///            [Flush]
    /// ```
    ///
    /// # Examples
    /// ```text
    /// service.refresh_configure_inputs(owner).await?;
    /// ```
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
    /// Validate an action and launch a tracked background configure/script process.
    /// Reads and refreshes inputs, reserves a run ID/build directory, starts a job and notifies the client; does not await process completion.
    ///
    /// # Arguments
    /// - `self` `[in,out]`: Analysis caches may refresh; run counter increments and job/running records are added before the start notification.
    /// - `request` `[in]`: Consumed action selection with root/profile/URI and launch-time cancellation.
    ///
    /// # Returns
    /// `Ok(Some(Data::Object))` with started=true and numeric run ID after the start message succeeds; never Ok(None). `Err` on validation/setup/message failure. A message error can occur after the job has already started.
    ///
    /// # Errors
    /// Stale/invalid action project, disabled runtime, missing mode, invalid script arguments, occupied root/job limit, unsaved/unreadable inputs, invalid or reused build directory, cancellation before launch, exhausted run IDs, or client message failure. Process/trace failures arrive later through configure_finished.
    ///
    /// # Panics
    /// Tokio job spawning requires an active runtime; internal project-index invariants must hold.
    ///
    /// # Algorithm
    /// Resolve owner/settings, validate mode and job slots, refresh inputs then resolve the owner again and require saved sources. Scripts use startup cwd; configure uses the entry parent and a separate unused build directory. Check request cancellation once before incrementing the run counter, spawn execute with an independent job signal, record the marker and send the started notification.
    ///
    /// # Flow
    /// ```text
    ///  [Action + effective runtime settings]
    ///                  |
    ///         [Validate + refresh inputs]
    ///                  |
    ///              <Mode?>
    ///             /       \
    ///         script     configure
    ///           |           |
    ///      [Startup cwd] [Check separate build]
    ///           +-----------+
    ///                  v
    ///       [Check cancellation + allocate ID]
    ///                  |
    ///       [Spawn task + insert running marker]
    ///                  |
    ///          [Notify client + return ID]
    /// ```
    ///
    /// # Examples
    /// ```text
    /// let started = service.start_runtime(request).await?;
    /// ```
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
    /// Cancel every accepted runtime job and remove all installed runtime layers.
    /// Does not await tasks; static analysis remains and all path caches are marked dirty.
    ///
    /// # Arguments
    /// - `self` `[in,out]`: Running markers drained/cancelled, runtime layers removed and path caches marked dirty; join-set tasks remain.
    ///
    /// # Returns
    /// `()` after draining running markers and clearing observations; jobs remain in the join set until reaped.
    ///
    /// # Examples
    /// ```text
    /// service.invalidate_runtime();
    /// ```
    pub(super) fn invalidate_runtime(&mut self) {
        for (_, task) in std::mem::take(&mut self.running) {
            task.cancel.cancel();
        }
        for project in &mut self.projects {
            project.workspace.clear_runtime();
            project.paths_dirty = true;
        }
    }
    /// Invalidate runtime work and await every outstanding task, including already stale jobs.
    /// Process cancellation/reaping is delegated to each job; join results are intentionally discarded.
    ///
    /// # Arguments
    /// - `self` `[in,out]`: Runtime markers/layers invalidated and the job set drained; static projects/documents are retained.
    ///
    /// # Returns
    /// `()` once the join set is empty.
    ///
    /// # Algorithm
    /// Signal all running markers through invalidate_runtime, then drain join_next until no tracked task remains.
    ///
    /// # Flow
    /// ```text
    ///  [Running markers]          [Stale/unreaped jobs]
    ///          |                          |
    ///      [Cancel all]                   |
    ///          +------------+-------------+
    ///                       v
    ///                 [join_next] <--------+
    ///                       |              |
    ///                 <Got a task?> -- yes-+
    ///                       | no
    ///                    [Return]
    /// ```
    ///
    /// # Examples
    /// ```text
    /// service.stop_configures().await;
    /// ```
    pub(super) async fn stop_configures(&mut self) {
        self.invalidate_runtime();
        while self.jobs.join_next().await.is_some() {}
    }
    /// Accept only current job results, recheck saved sources and install observations into the matching profile.
    /// Mutates runtime membership/path state and clears publication keys on success; logs join failures or sends a completion/failure message.
    ///
    /// # Arguments
    /// - `self` `[in,out]`: Matching marker removed; accepted observations, membership/path caches and publication keys updated.
    /// - `completion` `[in]`: Consumed join result containing the root, execution mode, run ID and process trace/error.
    ///
    /// # Returns
    /// `()`; stale/unowned completions are silently discarded, task failures log to stderr, and install/process failures are reported to the client. Client-message failure is ignored.
    ///
    /// # Algorithm
    /// Reject failed joins or mismatched run IDs, then remove the marker and find the nonstandalone matching profile. Require known inputs still saved, check newly observed open sources against disk, install the trace with the job signal, extend membership and clear diagnostic keys. Report success or the validation/process/install error.
    ///
    /// # Flow
    /// ```text
    ///  [Joined completion]
    ///           |
    ///      <Current run ID?> -- no --> [Discard]
    ///           | yes
    ///  [Remove marker + find profile]
    ///           |
    ///  [Recheck saved + observed inputs]
    ///           |
    ///      <Install succeeds?>
    ///        /           \
    ///       no           yes
    ///       |             |
    ///  [Error message] [Members/path dirty + clear keys]
    ///                     |
    ///               [Success message]
    /// ```
    ///
    /// # Examples
    /// ```text
    /// service.configure_finished(completion).await;
    /// ```
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
            project.paths_dirty = true;
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
