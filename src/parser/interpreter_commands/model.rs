// AI-authored interpreter service requirements.
use std::path::PathBuf;

/// Parsed analysis/entry mode; parsing never executes the command.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum InterpreterServices {
    Project {
        source_directory: PathBuf,
        binary_directory: Option<PathBuf>,
    },
    Script {
        file: PathBuf,
    },
}
impl InterpreterServices {
    pub fn entry(&self) -> PathBuf {
        match self {
            Self::Project {
                source_directory, ..
            } => source_directory.join("CMakeLists.txt"),
            Self::Script { file } => file.clone(),
        }
    }
    pub fn binary_directory(&self) -> Option<PathBuf> {
        match self {
            Self::Project {
                binary_directory, ..
            } => binary_directory.clone(),
            Self::Script { .. } => None,
        }
    }
}
