// AI-authored static CMake invocation parser.
mod model;
pub use model::InterpreterServices;
use std::{
    io,
    path::{Path, PathBuf},
};

/// Resolve against the server startup directory, without requiring the path to exist.
pub fn resolve_path(cwd: &Path, value: &Path) -> io::Result<PathBuf> {
    crate::workspace::absolute(&cwd.join(value))
}

/// Accept CMake project (-S/-B) or script (-P) invocations, without shell expansion or execution.
pub fn parse(command: &str, cwd: &Path) -> io::Result<InterpreterServices> {
    let invalid = |message| io::Error::new(io::ErrorKind::InvalidInput, message);
    let words = shell_words::split(command).map_err(|error| invalid(error.to_string()))?;
    let mut words = words.iter();
    let program = words
        .next()
        .ok_or_else(|| invalid("Empty interpreter command".into()))?;
    if !matches!(
        Path::new(program)
            .file_name()
            .and_then(|name| name.to_str()),
        Some("cmake" | "cmake.exe")
    ) {
        return Err(invalid("Interpreter program must be cmake".into()));
    }
    let (mut source, mut binary, mut script) = (None, None, None);
    while let Some(word) = words.next() {
        let (option, value) =
            if word.len() > 2 && matches!(&word.as_bytes()[..2], b"-S" | b"-B" | b"-P") {
                (&word[..2], &word[2..])
            } else {
                (
                    word.as_str(),
                    words
                        .next()
                        .ok_or_else(|| invalid(format!("Missing value for {word}")))?
                        .as_str(),
                )
            };
        if value.is_empty() || value.starts_with('-') {
            return Err(invalid(format!("Invalid path for {option}")));
        }
        let target = match option {
            "-S" => &mut source,
            "-B" => &mut binary,
            "-P" => &mut script,
            _ => {
                return Err(invalid(format!(
                    "Unsupported interpreter option {option}; expected -S, -B or -P"
                )));
            }
        };
        if target.is_some() {
            return Err(invalid(format!("Repeated interpreter option {option}")));
        }
        *target = Some(resolve_path(cwd, Path::new(value))?);
    }
    match (source, binary, script) {
        (Some(source_directory), binary_directory, None) => Ok(InterpreterServices::Project {
            source_directory,
            binary_directory,
        }),
        (None, None, Some(file)) => Ok(InterpreterServices::Script { file }),
        _ => Err(invalid(
            "Use either cmake -S <source> [-B <build>] or cmake -P <script>".into(),
        )),
    }
}
