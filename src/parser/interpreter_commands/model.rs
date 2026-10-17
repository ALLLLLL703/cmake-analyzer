//! Service requirements from [`super::parse`], separate from runtime execution options.
//! Paths describe static analysis roots; constructing/reading them never runs a command.
//!
//! Contains AI-generated documentation additions.
// AI-authored interpreter service requirements.
use std::path::PathBuf;

/// Parsed analysis/entry mode; parsing never executes the command.
#[derive(Clone, Debug, PartialEq, Eq)]
/// Static analysis entry requirements selected by [`super::parse`]. Server ownership
/// uses [`Self::entry`] and [`Self::binary_directory`]; this is not an execution request.
/// Construction and accessors have no file/process effects.
///
/// # Examples
/// ```
/// # use cmake_analyzer::parser::interpreter_commands::InterpreterServices;
/// let services = InterpreterServices::Script { file: "check.cmake".into() };
/// assert_eq!(services.entry(), std::path::PathBuf::from("check.cmake"));
/// ```
pub enum InterpreterServices {
    /// Project graph rooted at source_directory/CMakeLists.txt. Example: `InterpreterServices::Project { source_directory: "src".into(), binary_directory: None }`.
    Project {
        /// Source tree directory, normalized by parse. Example: `InterpreterServices::Project { source_directory: "src".into(), binary_directory: None }`.
        source_directory: PathBuf,
        /// Optional directory context for static directory builtins, not a created build. Example: `InterpreterServices::Project { source_directory: "src".into(), binary_directory: Some("build".into()) }`.
        binary_directory: Option<PathBuf>,
    },
    /// Script graph rooted directly at file. Example: `InterpreterServices::Script { file: "check.cmake".into() }`.
    Script {
        /// Script path normalized by parse; never executed by this model. Example: `InterpreterServices::Script { file: "tools/check.cmake".into() }`.
        file: PathBuf,
    },
}
impl InterpreterServices {
    /// Build the static root file path in memory. No existence checks, mutation or process effects.
    ///
    /// # Arguments
    /// - `self` `[in]`: project/script requirement borrowed unchanged.
    ///
    /// # Returns
    /// Project source joined with CMakeLists.txt, or a cloned script path; always an owned PathBuf.
    ///
    /// # Examples
    /// ```
    /// # use cmake_analyzer::parser::interpreter_commands::InterpreterServices;
    /// let mode = InterpreterServices::Project { source_directory: "src".into(), binary_directory: None };
    /// assert_eq!(mode.entry(), std::path::PathBuf::from("src/CMakeLists.txt"));
    /// ```
    pub fn entry(&self) -> PathBuf {
        match self {
            Self::Project {
                source_directory, ..
            } => source_directory.join("CMakeLists.txt"),
            Self::Script { file } => file.clone(),
        }
    }
    /// Copy optional project binary context without reading or creating directories.
    ///
    /// # Arguments
    /// - `self` `[in]`: requirement borrowed unchanged.
    ///
    /// # Returns
    /// `Some(path)` for a project with an explicit binary directory; `None` for omitted project context or any script.
    ///
    /// # Examples
    /// ```
    /// # use cmake_analyzer::parser::interpreter_commands::InterpreterServices;
    /// let mode = InterpreterServices::Script { file: "check.cmake".into() };
    /// assert_eq!(mode.binary_directory(), None);
    /// ```
    pub fn binary_directory(&self) -> Option<PathBuf> {
        match self {
            Self::Project {
                binary_directory, ..
            } => binary_directory.clone(),
            Self::Script { .. } => None,
        }
    }
}
