// AI-generated implementation.
use super::model::{Argument, TargetDeclaration, TargetKind};
pub(super) fn target(command: &str, name: Argument, arguments: Vec<Argument>) -> TargetDeclaration {
    let mut kind = match command {
        "add_executable" => TargetKind::Executable,
        "add_custom_target" => TargetKind::Custom,
        _ => TargetKind::Library,
    };
    let mut imported = false;
    let mut global = false;
    let mut alias = None;
    let mut start = 0;
    // Flags stop at the first source; library-only flags may be executable filenames.
    if command != "add_custom_target" {
        for (index, arg) in arguments.iter().enumerate() {
            if !arg.literal {
                break;
            }
            match arg.text.as_str() {
                "STATIC" if command == "add_library" => kind = TargetKind::StaticLibrary,
                "SHARED" if command == "add_library" => kind = TargetKind::SharedLibrary,
                "MODULE" if command == "add_library" => kind = TargetKind::ModuleLibrary,
                "OBJECT" if command == "add_library" => kind = TargetKind::ObjectLibrary,
                "INTERFACE" if command == "add_library" => kind = TargetKind::InterfaceLibrary,
                "UNKNOWN" if command == "add_library" => kind = TargetKind::UnknownLibrary,
                "IMPORTED" => imported = true,
                "GLOBAL" => global = true,
                "ALIAS" => {
                    // An alias points to a target; its argument is not a source file.
                    kind = TargetKind::Alias;
                    alias = arguments.get(index + 1).cloned();
                    start = arguments.len();
                    break;
                }
                "WIN32" | "MACOSX_BUNDLE" if command == "add_executable" => {}
                "EXCLUDE_FROM_ALL" => {}
                _ => break,
            }
            start = index + 1;
        }
    }
    // Custom-target command arguments are not sources; only SOURCES contributes files.
    // the human maintainer did not use this before, so users,good luck,,,(its fully made in openai)
    let sources = if kind == TargetKind::Custom {
        arguments
            .iter()
            .position(|arg| arg.literal && arg.text == "SOURCES")
            .map(|start| {
                arguments[start + 1..]
                    .iter()
                    .take_while(|arg| {
                        !arg.literal
                            || !matches!(
                                arg.text.as_str(),
                                "COMMAND"
                                    | "DEPENDS"
                                    | "BYPRODUCTS"
                                    | "WORKING_DIRECTORY"
                                    | "COMMENT"
                                    | "JOB_POOL"
                                    | "VERBATIM"
                                    | "USES_TERMINAL"
                                    | "COMMAND_EXPAND_LISTS"
                                    | "JOB_SERVER_AWARE"
                            )
                    })
                    .cloned()
                    .collect()
            })
            .unwrap_or_default()
    } else if imported || kind == TargetKind::Alias {
        Vec::new()
    } else {
        arguments[start..].to_vec()
    };
    TargetDeclaration {
        references: Vec::new(),
        name,
        kind,
        imported,
        global,
        alias,
        sources: sources.into(),
        arguments: arguments.into(),
    }
}
