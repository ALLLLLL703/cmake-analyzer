//! Rebuild configuration and projects before replacing live analysis state.
//! Reload stages open overlays under new settings and checks live-count limits.
//! Loading/admission errors preserve the live service. Workspace flush failures that
//! are logged internally are not promoted to reload errors. Success cancels old
//! runtime observations and pending saves, but retains publication keys for comparison.
// AI-authored transactional configuration reload.
use super::{super::model::*, internal};
use crate::config::ConfigStore;
use lspally::server::model::*;
use std::{collections::BTreeMap, path::PathBuf};

impl Service {
    /// Stage new sources, settings and projects with existing editor overlays, then replace live analysis.
    /// Successful replacement cancels runtime markers and pending saves; open buffers, client and publication keys remain live.
    ///
    /// # Arguments
    /// - `self` `[in,out]`: Live help/configuration/roots/projects replaced only after staging; runtime markers and saved runs cleared on success.
    /// - `path` `[in]`: Optional consumed override for the explicit configuration file; None retains the previous source.
    ///
    /// # Returns
    /// `Ok(count)` of rebuilt projects on success; `Err` leaves live configuration/projects/runtime intact, though filesystem reads already happened.
    ///
    /// # Errors
    /// Source read/parse/validation errors, new limits below live open/project counts, root or owner admission failures, and overlay synchronization failures. Logged internal Workspace flush failures do not abort staging.
    ///
    /// # Algorithm
    /// Load new sources and reject below-live limits, build a temporary Service with cloned documents/roots, resolve cwd and project settings, admit all open files and flush. Only then invalidate old runtime, clear save requests and replace help/config/roots/projects. Jobs remain tracked until their stale completions are joined; publication keys compare against new snapshots.
    ///
    /// # Flow
    /// ```text
    ///  [New sources + live counts]
    ///               |
    ///       <Load/limits valid?>
    ///         /             \
    ///        no             yes
    ///        |               |
    ///  [Keep live + error] [Stage projects + overlays]
    ///                        |
    ///                   <Stage succeeds?>
    ///                    /          \
    ///                   no          yes
    ///                   |            |
    ///             [Keep live] [Cancel old runtime]
    ///                                |
    ///                         [Replace analysis + count]
    /// ```
    ///
    /// # Examples
    /// ```text
    /// let count = service.reload_config(Some("/work/settings.toml".into())).await?;
    /// ```
    pub(super) async fn reload_config(&mut self, path: Option<PathBuf>) -> ServerResult<usize> {
        let mut sources = self.config.sources.clone();
        if let Some(path) = path {
            sources.explicit_file = Some(path);
        }
        let config = ConfigStore::load(sources).await.map_err(internal)?;
        if self.documents.len() > config.limits.max_open_documents
            || self.projects.len() > config.limits.max_workspaces
        {
            return Err(ServerError::invalid(
                "Reload limits are below the current open document or Workspace count; close documents/remove Workspaces first",
            ));
        }
        // Stage configuration, ownership, and analysis before replacing the live state.
        let mut staged = Service {
            local_help: super::super::hovers::LocalHelp::new(
                config.sources.cwd.clone(),
                config.global_runtime(),
            ),
            hover_markdown: self.hover_markdown,
            config,
            roots: self.roots.clone(),
            projects: Vec::new(),
            documents: self.documents.clone(),
            clears: Vec::new(),
            published: BTreeMap::new(),
            client: self.client.clone(),
            encoding: self.encoding,
            hierarchical: self.hierarchical,
            jobs: tokio::task::JoinSet::new(),
            running: BTreeMap::new(),
            run_counter: self.run_counter,
            saved_runs: Default::default(),
        };
        let cwd = staged.config.sources.cwd.clone();
        staged.config.for_directory(&cwd).await.map_err(internal)?;
        for root in staged.roots.clone() {
            staged.add_project(root, false, None).await?;
        }
        let paths: Vec<_> = staged.documents.keys().cloned().collect();
        for path in paths {
            staged.ensure_owners(&path).await?;
        }
        staged.flush().await?;
        self.invalidate_runtime();
        self.saved_runs.clear();
        self.local_help = staged.local_help;
        self.config = staged.config;
        self.roots = staged.roots;
        self.projects = staged.projects;
        Ok(self.projects.len())
    }
}
