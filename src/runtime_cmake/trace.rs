// AI-authored bounded trace decoding. Frames identify execution nesting, not lexical scopes.
use super::model::*;
use crate::parser::build_struct::model::DependencyKind;
use std::{
    io,
    path::{Path, PathBuf},
    sync::Arc,
};
pub(super) fn parse(
    run: RunId,
    bytes: &[u8],
    cwd: &Path,
    build: Option<&Path>,
    cancel: &lspally::server::model::Cancellation,
) -> io::Result<Trace> {
    let initial_directory = Arc::new(cwd.to_owned());
    let mut events = Vec::new();
    let mut declared = std::collections::HashSet::new();
    for line in bytes.split(|byte| *byte == b'\n') {
        if cancel.is_cancelled() {
            return Err(io::Error::new(
                io::ErrorKind::Interrupted,
                "CMake trace parsing cancelled",
            ));
        }
        let Ok(value) = serde_json::from_slice::<serde_json::Value>(line) else {
            continue;
        };
        let Some(command) = value.get("cmd").and_then(|value| value.as_str()) else {
            continue;
        };
        let invalid = || io::Error::new(io::ErrorKind::InvalidData, "Malformed CMake trace event");
        let file = value
            .get("file")
            .and_then(|value| value.as_str())
            .ok_or_else(invalid)?;
        let line = value
            .get("line")
            .and_then(|value| value.as_u64())
            .and_then(|line| u32::try_from(line).ok())
            .filter(|line| *line > 0)
            .ok_or_else(invalid)?;
        let arguments = value
            .get("args")
            .and_then(|value| value.as_array())
            .ok_or_else(invalid)?
            .iter()
            .map(|value| value.as_str().map(str::to_owned).ok_or_else(invalid))
            .collect::<io::Result<Vec<_>>>()?;
        let file = crate::parser::interpreter_commands::resolve_path(cwd, Path::new(file))?;
        let frame = value
            .get("global_frame")
            .and_then(|value| value.as_u64())
            .ok_or_else(|| {
                io::Error::new(
                    io::ErrorKind::InvalidData,
                    "CMake runtime analysis requires JSON trace format v1 minor 2 (global_frame)",
                )
            })?;
        let command = command.to_ascii_lowercase();
        let builtin = !declared.contains(&command);
        if builtin && matches!(command.as_str(), "function" | "macro") {
            if let Some(name) = arguments.first() {
                declared.insert(name.to_ascii_lowercase());
            }
        }
        events.push(TraceEvent {
            source_directory: initial_directory.clone(),
            id: EventId::new(events.len()),
            file,
            line,
            command,
            arguments,
            frame: RuntimeFrame::new(frame),
            builtin,
        });
    }
    if events.is_empty() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "CMake produced no JSON trace events",
        ));
    }
    let mut dependencies = Vec::new();
    let mut source_directories = vec![(RuntimeFrame::new(0), initial_directory.clone())];
    for index in 0..events.len() {
        let frame = events[index].frame;
        while source_directories
            .last()
            .is_some_and(|(previous, _)| previous.depth() > frame.depth())
        {
            source_directories.pop();
        }
        events[index].source_directory = source_directories
            .last()
            .map_or_else(|| initial_directory.clone(), |(_, path)| path.clone());
        let event = &events[index];
        if !event.builtin {
            continue;
        }
        let kind = match event.command.as_str() {
            "include" => DependencyKind::Include,
            "add_subdirectory" => DependencyKind::Subdirectory,
            _ => continue,
        };
        // The first deeper event belongs to the loaded file; no path-variable evaluation is needed.
        let resolved = events
            .get(index + 1)
            .filter(|next| next.frame.depth() > event.frame.depth())
            .map(|next| next.file.clone())
            .or_else(|| {
                if kind == DependencyKind::Include
                    && event
                        .arguments
                        .iter()
                        .skip(1)
                        .any(|argument| argument == "OPTIONAL")
                {
                    return None;
                }
                let argument = event.arguments.first()?;
                if argument.is_empty() || argument.contains([';', '\\']) {
                    return None;
                }
                let path = PathBuf::from(argument);
                let source = source_directories
                    .last()
                    .map(|(_, path)| path.as_path())
                    .unwrap_or(cwd);
                let file = crate::parser::interpreter_commands::resolve_path(source, &path).ok()?;
                if kind == DependencyKind::Subdirectory {
                    Some(file.join("CMakeLists.txt"))
                } else if path.is_absolute()
                    || path.extension().is_some()
                    || path.components().count() > 1
                {
                    Some(file)
                } else {
                    None
                }
            });
        if kind == DependencyKind::Subdirectory {
            if let (Some(file), Some(next)) = (
                &resolved,
                events
                    .get(index + 1)
                    .filter(|next| next.frame.depth() > event.frame.depth()),
            ) {
                if let Some(directory) = file.parent() {
                    source_directories.push((next.frame, Arc::new(directory.to_owned())));
                }
            }
        }
        dependencies.push(ObservedDependency {
            event: event.id,
            kind,
            resolved,
        });
    }
    Ok(Trace {
        run,
        build_directory: build.map(Path::to_owned),
        events,
        dependencies,
    })
}
