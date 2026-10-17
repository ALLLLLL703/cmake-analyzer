//! Decode dependency operands shared by build extraction, completion and navigation.
//! This records requests only; it neither searches files nor executes CMake.
// Contains AI-generated documentation additions.
use super::model::{Argument, Dependency, DependencyKind};
use crate::parser::semanticer::model::SemanticCommandRepresent;

/// Decode dependency category and literal flags without searching files or expanding expressions.
///
/// # Arguments
/// - `name` `[in]`: Lowercase command name; unknown names take include semantics.
/// - `first` `[in]`: Owned path/package/source operand.
/// - `rest` `[in]`: Owned remaining arguments, retained unchanged in the result.
///
/// # Returns
/// Request with optional, policy and possible binary-directory metadata.
///
/// # Examples
/// ```text
/// let dependency = from_arguments("include", path_argument, vec![]);
/// ```
pub(crate) fn from_arguments(name: &str, first: Argument, rest: Vec<Argument>) -> Dependency {
    let kind = match name {
        "find_package" => DependencyKind::Package,
        "add_subdirectory" => DependencyKind::Subdirectory,
        _ => DependencyKind::Include,
    };
    Dependency {
        kind,
        path: first,
        binary: if kind == DependencyKind::Subdirectory {
            rest.first()
                .filter(|arg| !matches!(arg.text.as_str(), "EXCLUDE_FROM_ALL" | "SYSTEM"))
                .cloned()
        } else {
            None
        },
        optional: if kind == DependencyKind::Package {
            !rest.iter().any(|arg| arg.literal && arg.text == "REQUIRED")
        } else {
            rest.iter().any(|arg| arg.literal && arg.text == "OPTIONAL")
        },
        no_policy_scope: rest
            .iter()
            .any(|arg| arg.literal && arg.text == "NO_POLICY_SCOPE"),
        arguments: rest,
    }
}
/// Recover dependency operands from a semantic command without validating schema provenance. No I/O
/// or mutation.
///
/// # Arguments
/// - `command` `[in]`: Command to classify case-insensitively.
///
/// # Returns
/// Some request for include/add_subdirectory/find_package with a first operand; None for other
/// names or no operands.
///
/// # Examples
/// ```text
/// let dependency = from_command(command);
/// ```
pub(crate) fn from_command(command: &SemanticCommandRepresent) -> Option<Dependency> {
    let name = command.name.to_ascii_lowercase();
    if !matches!(
        name.as_str(),
        "include" | "add_subdirectory" | "find_package"
    ) {
        return None;
    }
    let mut args = command.args.iter().map(|arg| Argument {
        text: arg.content.clone(),
        literal: arg.literal_text().is_some(),
    });
    Some(from_arguments(&name, args.next()?, args.collect()))
}
