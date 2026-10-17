mod candidates;
mod model;
mod request;
use super::{Workspace, model::DirectoryContext};
use crate::parser::{build_struct::model::Dependency, semanticer::model::ValueBindings};
use std::path::{Path, PathBuf};

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
