//! Pure parsing of configured CMake invocation strings into [`InterpreterServices`].
//! No shell expansion, project execution, existence checks or disk writes. Relative
//! bases may require reading the process current directory during path normalization.
//!
//! Contains AI-generated documentation additions.
// AI-authored static CMake invocation parser.
mod model;
pub use model::InterpreterServices;
use std::{
    io,
    path::{Path, PathBuf},
};

/// Resolve against the server startup directory, without requiring the path to exist.
/// Join and lexically normalize a path without checking existence or resolving symlinks.
/// No filesystem writes or processes; reads current directory only if the joined path is relative.
///
/// # Arguments
/// - `cwd` `[in]`: base directory, normally absolute startup cwd.
/// - `value` `[in]`: path to resolve; an absolute value replaces the base.
///
/// # Returns
/// `Ok(path)` is an absolute path with dot/parent components cleaned; `Err` means cwd lookup failed for a relative base.
///
/// # Errors
/// Process current-directory lookup may fail when cwd/value do not already produce an absolute path.
///
/// # Examples
/// ```
/// # use cmake_analyzer::parser::interpreter_commands::resolve_path;
/// # use std::path::Path;
/// let cwd = std::env::current_dir()?;
/// assert_eq!(resolve_path(&cwd, Path::new("cmake/../tool.cmake"))?, cwd.join("tool.cmake"));
/// # Ok::<(), std::io::Error>(())
/// ```
pub fn resolve_path(cwd: &Path, value: &Path) -> io::Result<PathBuf> {
    crate::workspace::absolute(&cwd.join(value))
}

/// Accept CMake project (-S/-B) or script (-P) invocations, without shell expansion or execution.
/// Tokenize a literal CMake -S/-B or -P command into static service requirements.
/// No shell/environment/tilde expansion, existence checks, file writes or process execution.
/// May query current directory only when the supplied base is relative.
///
/// # Arguments
/// - `command` `[in]`: quoted command string; executable basename must be cmake or cmake.exe.
/// - `cwd` `[in]`: startup path base for every option, not the configuration-file directory.
///
/// # Returns
/// `Ok(Project)` for -S with optional -B; `Ok(Script)` for -P alone. `Err` rejects all other forms.
///
/// # Errors
/// InvalidInput for empty/malformed shell-word quoting, other programs/options, missing/invalid paths,
/// repeated options or mixed/incomplete modes; path-normalization errors propagate.
///
/// # Algorithm
/// Split quoted words without invoking a shell, validate the executable basename, consume separate
/// or joined -S/-B/-P options exactly once, normalize values, then accept one complete mode.
///
/// # Flow
/// ```text
///  [quoted command] ---> [split words]
///                             |
///                      [validate program]
///                             |
///                      [consume options] --bad--> [Err]
///                             |
///                         <mode?>
///                        /       \
///               [-S with -B?]   [-P only]
///                     |             |
///                 [Project]      [Script]
/// ```
///
/// # Examples
/// ```
/// # use cmake_analyzer::parser::interpreter_commands::{parse, InterpreterServices};
/// let cwd = std::env::current_dir()?;
/// let services = parse("cmake -S . -B build", &cwd)?;
/// assert_eq!(services.entry(), cwd.join("CMakeLists.txt"));
/// assert!(matches!(parse("cmake -P tools/check.cmake", &cwd)?, InterpreterServices::Script { .. }));
/// # Ok::<(), std::io::Error>(())
/// ```
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
