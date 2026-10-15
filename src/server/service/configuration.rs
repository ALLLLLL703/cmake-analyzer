// AI-authored transactional configuration reload.
use super::{super::model::*, internal};
use crate::config::ConfigStore;
use lspally::server::model::*;
use std::{collections::BTreeMap, path::PathBuf};

impl Service {
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
        self.config = staged.config;
        self.roots = staged.roots;
        self.projects = staged.projects;
        Ok(self.projects.len())
    }
}
