//! Find static Find-module or config entrypoints with caller values and configured prefixes.
//! Searches files and environment hints, not package versions/components, providers, registry,
//! CPS metadata or toolchain policy. Never executes package code.
// Contains AI-generated documentation additions.
mod candidates;
mod model;
mod request;
use super::{Workspace, model::DirectoryContext};
use crate::parser::{build_struct::model::Dependency, semanticer::model::ValueBindings};
use std::path::{Path, PathBuf};

/// Split one known semicolon-separated search hint into nonempty paths. No filesystem access or
/// mutation.
///
/// # Arguments
/// - `values` `[in]`: Optional conservative bindings.
/// - `name` `[in]`: Case-sensitive variable name to query.
///
/// # Returns
/// Paths in value order, or empty for absent/unknown bindings.
///
/// # Examples
/// ```text
/// let prefixes = value_paths(Some(values), "CMAKE_PREFIX_PATH");
/// ```
fn value_paths(values: Option<&ValueBindings>, name: &str) -> Vec<PathBuf> {
    values
        .and_then(|values| values.get(name, None))
        .map(|value| {
            value
                .text()
                .split(';')
                .filter(|part| !part.is_empty())
                .map(PathBuf::from)
                .collect()
        })
        .unwrap_or_default()
}
/// Assemble ordered config roots from known values, request flags and environment. Reads process
/// environment but performs no file I/O or mutation.
///
/// # Arguments
/// - `dependency` `[in]`: Package name used for NAME_DIR/NAME_ROOT hints.
/// - `request` `[in]`: Parsed flags and explicit hints.
/// - `values` `[in]`: Optional known package/prefix bindings.
/// - `configured` `[in]` : Configured default prefixes, used only when system/default paths are
///   enabled.
///
/// # Returns
/// Roots ordered as package DIR, explicit hints, then enabled known/environment/default families.
/// Relative paths are retained for later source-context joining.
///
/// # Algorithm
/// Start with the known package DIR and explicit hints regardless of default-path flags. If
/// defaults remain enabled, append enabled known ROOT/prefix families, permitted environment hints
/// and configured system prefixes. Preserve order and duplicates for later candidate-level
/// deduplication.
///
/// # Flow
/// ```text
/// [known Package_DIR]       [explicit hints]
///           |                      |
///           +----> [ordered roots]-+
///                          |
///                  <defaults enabled?>
///                    /             \
///                  no              yes
///                   |     [known ROOT/prefix families]
///                   |              |
///                   |     [enabled environment hints]
///                   |              |
///                   |     [configured system prefixes]
///                   +--------------+
///                          |
///                    [return roots]
/// ```
///
/// # Examples
/// ```text
/// let search_roots = roots(dependency, request, Some(values), &configured);
/// ```
fn roots(
    dependency: &Dependency,
    request: &model::Request,
    values: Option<&ValueBindings>,
    configured: &[PathBuf],
) -> Vec<PathBuf> {
    let mut roots = value_paths(values, &format!("{}_DIR", dependency.path.text));
    roots.extend(request.hints.iter().cloned());
    if request.defaults {
        if request.package_roots {
            roots.extend(value_paths(
                values,
                &format!("{}_ROOT", dependency.path.text),
            ));
        }
        if request.cmake_paths {
            roots.extend(value_paths(values, "CMAKE_PREFIX_PATH"));
        }
        if request.environment {
            if let Some(paths) = std::env::var_os("CMAKE_PREFIX_PATH") {
                roots.extend(std::env::split_paths(&paths));
            }
            if let Some(path) = std::env::var_os(format!("{}_DIR", dependency.path.text)) {
                roots.push(path.into());
            }
            if request.package_roots {
                if let Some(path) = std::env::var_os(format!("{}_ROOT", dependency.path.text)) {
                    roots.push(path.into());
                }
            }
        }
        if request.system_paths {
            roots.extend(configured.iter().cloned());
        }
    }
    roots
}
impl Workspace {
    /// Resolve a supported Find-module/config entrypoint without evaluating package code. Probes
    /// shallow directories, metadata, overlays and permitted environment hints.
    ///
    /// # Arguments
    /// - `self` `[in]`: Workspace search configuration and overlay identities; unchanged.
    /// - `dependency` `[in]` : Request with a literal first operand; dynamic requests decline
    ///   lookup.
    /// - `caller` `[in]`: List-file identity used to select builtin-module precedence.
    /// - `directory` `[in]`: Current source directory and optional binary context.
    /// - `values` `[in]`: Known package hints and CMAKE_FIND_PACKAGE_PREFER_CONFIG preference.
    ///
    /// # Returns
    /// (chosen entrypoint, checked candidates), or (None, empty) for unsupported operands;
    /// unresolved supported search retains checked candidates.
    ///
    /// # Algorithm
    /// Parse literal flags. Unless config is preferred, try FindName.cmake first when allowed.
    /// Search enabled config roots exactly, then conventional prefix layouts. If config is
    /// preferred, allowed module fallback runs last.
    /// Version/component/toolchain/provider/registry/CPS evaluation is deliberately absent.
    ///
    /// # Flow
    /// ```text
    /// [parse request] ---> <supported?> -- no --> [none]
    ///        | yes
    ///  <prefer config?>
    ///     /           \
    ///  [Find module] [config roots]
    ///     | miss       | miss
    ///  [config roots] [Find module]
    ///     \           /
    ///       [first match + checked paths]
    /// ```
    ///
    /// # Examples
    /// ```text
    /// let (entry, checked) = workspace.resolve_package(dependency, caller, directory, Some(values)).await;
    /// ```
    pub(super) async fn resolve_package(
        &self,
        dependency: &Dependency,
        caller: &Path,
        directory: &DirectoryContext,
        values: Option<&ValueBindings>,
    ) -> (Option<PathBuf>, Vec<PathBuf>) {
        let Some(request) = request::parse(dependency) else {
            return (None, Vec::new());
        };
        let prefer_config = values
            .and_then(|values| values.get("CMAKE_FIND_PACKAGE_PREFER_CONFIG", None))
            .is_some_and(|value| {
                matches!(
                    value.text().to_ascii_uppercase().as_str(),
                    "1" | "ON" | "TRUE" | "YES" | "Y"
                )
            });
        let mut module = dependency.clone();
        module.path.text = format!("Find{}", dependency.path.text);
        let mut candidates = Vec::new();
        if !request.config_only && !prefer_config {
            let (resolved, checked) = self
                .resolve_module(&module, caller, directory, values)
                .await;
            candidates.extend(checked);
            if resolved.is_some() {
                return (resolved, candidates);
            }
        }
        if !request.module_only {
            let roots = roots(dependency, &request, values, &self.package_search_paths);
            if let Some(path) =
                candidates::find(self, roots, &directory.source, &request, &mut candidates).await
            {
                return (Some(path), candidates);
            }
        }
        if !request.config_only && prefer_config {
            let (resolved, checked) = self
                .resolve_module(&module, caller, directory, values)
                .await;
            candidates.extend(checked);
            return (resolved, candidates);
        }
        (None, candidates)
    }
}
