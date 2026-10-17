use super::model::Request;
use crate::parser::build_struct::model::Dependency;
use std::path::{Path, PathBuf};

pub(super) fn parse(dependency: &Dependency) -> Option<Request> {
    let mut names = vec![dependency.path.text.clone()];
    let mut hints = Vec::new();
    let mut configs = Vec::new();
    let mut suffixes = vec![PathBuf::new()];
    let mut section = "";
    let (mut config_only, mut module_only, mut defaults) = (false, false, true);
    let (mut package_roots, mut cmake_paths, mut environment, mut system_paths) =
        (true, true, true, true);
    for arg in &dependency.arguments {
        if !arg.literal {
            return None;
        }
        match arg.text.as_str() {
            "MODULE" => {
                module_only = true;
                section = "";
            }
            "CONFIG" | "NO_MODULE" => {
                config_only = true;
                section = "";
            }
            "NO_DEFAULT_PATH" => {
                defaults = false;
                section = "";
                config_only = true;
            }
            "NO_PACKAGE_ROOT_PATH" => {
                package_roots = false;
                section = "";
                config_only = true;
            }
            "NO_CMAKE_PATH" => {
                cmake_paths = false;
                section = "";
                config_only = true;
            }
            "NO_CMAKE_ENVIRONMENT_PATH" => {
                environment = false;
                section = "";
                config_only = true;
            }
            "NO_CMAKE_SYSTEM_PATH" => {
                system_paths = false;
                section = "";
                config_only = true;
            }
            "NAMES" => {
                names.clear();
                section = "NAMES";
                config_only = true;
            }
            "CONFIGS" => {
                section = "CONFIGS";
                config_only = true;
            }
            "HINTS" | "PATHS" | "PATH_SUFFIXES" => {
                section = &arg.text;
                config_only = true;
            }
            "REQUIRED"
            | "QUIET"
            | "EXACT"
            | "GLOBAL"
            | "NO_POLICY_SCOPE"
            | "COMPONENTS"
            | "OPTIONAL_COMPONENTS"
            | "BYPASS_PROVIDER"
            | "UNWIND_INCLUDE" => section = "",
            // Registry and root-path policies cannot be reproduced without a toolchain context.
            "REGISTRY_VIEW"
            | "ONLY_CMAKE_FIND_ROOT_PATH"
            | "CMAKE_FIND_ROOT_PATH_BOTH"
            | "NO_CMAKE_FIND_ROOT_PATH" => return None,
            flag if flag.starts_with("NO_") => section = "",
            _ => match section {
                "NAMES" => names.push(arg.text.clone()),
                "CONFIGS" => configs.push(arg.text.clone()),
                "HINTS" | "PATHS" => hints.push(PathBuf::from(&arg.text)),
                "PATH_SUFFIXES" => suffixes.push(PathBuf::from(&arg.text)),
                _ => {}
            },
        }
    }
    if names.is_empty()
        || config_only && module_only
        || configs
            .iter()
            .any(|name| Path::new(name).components().count() != 1 || !name.ends_with(".cmake"))
    {
        return None;
    }
    let filenames = if configs.is_empty() {
        names
            .iter()
            .flat_map(|name| {
                [
                    format!("{name}Config.cmake"),
                    format!("{}-config.cmake", name.to_ascii_lowercase()),
                ]
            })
            .collect()
    } else {
        configs
    };
    Some(Request {
        names,
        hints,
        suffixes,
        filenames,
        config_only,
        module_only,
        defaults,
        package_roots,
        cmake_paths,
        environment,
        system_paths,
    })
}
