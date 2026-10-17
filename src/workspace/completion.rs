//! Assemble schema-requested module/package names and supplemental target-name sources.
//! Discovery can read directories, parse sources and inspect `cmake --version`. It never changes
//! graph bindings, configures projects or executes project code.
// Contains AI-generated documentation additions.
mod context;
mod discovery;
pub(crate) mod model;
mod targets;
use super::Workspace;
use std::{io, path::Path};

/// Translate the cancellation predicate into an interrupted I/O error. No I/O; invoking stop may
/// have caller-defined side effects.
///
/// # Arguments
/// - `stop` `[in]`: Predicate returning true once the request should stop.
///
/// # Returns
/// Ok(()) while active; Err(Interrupted) when cancelled.
///
/// # Errors
/// Returns io::ErrorKind::Interrupted when stop() is true.
///
/// # Examples
/// ```text
/// check(&|| false)?;
/// ```
fn check(stop: &impl Fn() -> bool) -> io::Result<()> {
    if stop() {
        Err(io::Error::new(
            io::ErrorKind::Interrupted,
            "Completion cancelled",
        ))
    } else {
        Ok(())
    }
}
impl Workspace {
    /// Collect only schema-requested names/supplemental targets, without changing ordinary
    /// graph/reference/value bindings. May scan directories, cache parsed sources and inspect
    /// `cmake --version`; never configures or executes project code.
    ///
    /// # Arguments
    /// - `self` `[in,out]` : Workspace; updates discovery and supplemental source caches, not
    ///   source snapshots.
    /// - `path` `[in]`: Normalized requesting file identity.
    /// - `request` `[in]`: Requested catalog categories and active command.
    /// - `stop` `[in]`: Cloneable thread-safe cancellation predicate checked throughout discovery.
    ///
    /// # Returns
    /// Ok catalog with unrequested categories empty; Err on cancellation or discovery/source worker
    /// failure. Disabled module lookup suppresses module/package discovery, not the independent
    /// target-source branch.
    ///
    /// # Errors
    /// Interrupted cancellation, directory iteration errors, context-worker join errors or
    /// supplemental source analysis failures. Caches may have been partially refreshed before
    /// failure.
    ///
    /// # Algorithm
    /// Check cancellation, capture value/directory contexts for requested module/package discovery,
    /// then use root-keyed shallow name caches. Independently collect supplemental target sources
    /// if requested, preserving external policies and traversal budgets.
    ///
    /// # Flow
    /// ```text
    /// [request]
    ///     |
    ///  <module/package + enabled?>
    ///     /                 \
    ///  [capture hints]        skip
    ///     |                    |
    ///  [name discovery] --------+
    ///               |
    ///         <targets requested?>
    ///           /           \
    ///  [supplemental sources] skip
    ///           \           /
    ///               [catalog]
    /// ```
    ///
    /// # Examples
    /// ```text
    /// let catalog = workspace.completion_catalog(path, &request, || false).await?;
    /// ```
    pub(crate) async fn completion_catalog(
        &mut self,
        path: &Path,
        request: &model::Request,
        stop: impl Fn() -> bool + Send + Sync + Clone + 'static,
    ) -> io::Result<model::Catalog> {
        check(&stop)?;
        let mut result = model::Catalog::default();
        if (request.modules || request.packages) && self.module_lookup_enabled {
            let contexts = context::capture(self, path, request.command, stop.clone()).await?;
            result = discovery::collect(self, path, &contexts, request, &stop).await?;
        }
        if request.targets {
            result.targets = targets::collect(self, path, &stop).await?;
        }
        Ok(result)
    }
}
