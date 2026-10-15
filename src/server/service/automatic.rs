// AI-authored opt-in save scheduling through the manual action entrypoints.
use super::super::{
    action::model::{ActionKind, ActionRequest},
    model::*,
};
use crate::{config::RuntimeMode, runtime_cmake::model::ExecutionMode};
use lspally::server::model::*;
use std::{collections::BTreeSet, path::Path};
impl Service {
    pub(super) fn invalidate_runtime_file(&mut self, path: &Path) {
        let roots: BTreeSet<_> = self
            .projects
            .iter()
            .filter(|project| {
                project.members.contains(path)
                    || project.bindings.contains(path)
                    || project.workspace.root().file == path
            })
            .map(|project| project.workspace.root().file.clone())
            .collect();
        for root in &roots {
            if let Some(job) = self.running.remove(root) {
                job.cancel.cancel();
            }
        }
        for project in &mut self.projects {
            if roots.contains(&project.workspace.root().file) {
                project.workspace.clear_runtime();
                project.paths_dirty = true;
            }
        }
    }
    pub(super) async fn start_saved_runs(&mut self) {
        for (root, saved) in std::mem::take(&mut self.saved_runs) {
            let Some(document) = self.documents.get(&saved.file) else {
                continue;
            };
            let owner = self.owners(&saved.file).into_iter().find(|owner| {
                let project = &self.projects[*owner];
                project.workspace.root().file == root
                    && project.settings.runtime.enable
                    && project.settings.runtime.mode == RuntimeMode::OnSave
                    && project.execution.as_ref() == Some(&saved.mode)
            });
            let Some(owner) = owner else { continue };
            if self.running.contains_key(&root)
                || self.jobs.len() >= self.config.limits.max_workspaces
            {
                self.saved_runs.insert(root, saved);
                continue;
            }
            let mode = self.projects[owner].execution.clone();
            let request = ActionRequest {
                kind: if matches!(mode, Some(ExecutionMode::Script { .. })) {
                    ActionKind::RunScript
                } else {
                    ActionKind::Configure
                },
                uri: Uri(document.uri.0.clone()),
                root,
                line: 0,
                cancellation: Cancellation::default(),
                execution: mode,
            };
            if let Err(error) = self.run_action(request).await {
                let _ = self
                    .client
                    .log_message(LogMessageParams {
                        typ: MessageType(2),
                        message: format!("Automatic CMake run skipped: {error}"),
                    })
                    .await;
            }
        }
    }
}
