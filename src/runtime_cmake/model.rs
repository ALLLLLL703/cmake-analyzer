// AI-authored runtime observations, distinct from static snapshots.
use crate::parser::build_struct::model::DependencyKind;
use std::{path::PathBuf, time::Duration};
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct RunId(u64);
impl RunId {
    pub fn new(value: u64) -> Self {
        Self(value)
    }
    pub fn value(self) -> u64 {
        self.0
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EventId(usize);
impl EventId {
    pub(super) fn new(value: usize) -> Self {
        Self(value)
    }
    pub fn index(self) -> usize {
        self.0
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RuntimeFrame(u64);
impl RuntimeFrame {
    pub(super) fn new(value: u64) -> Self {
        Self(value)
    }
    pub fn depth(self) -> u64 {
        self.0
    }
}
#[derive(Clone, Debug)]
pub struct TraceEvent {
    pub id: EventId,
    pub file: PathBuf,
    pub line: u32,
    pub command: String,
    pub arguments: Vec<String>,
    pub frame: RuntimeFrame,
    pub builtin: bool,
}
#[derive(Clone, Debug)]
pub struct ObservedDependency {
    pub event: EventId,
    pub kind: DependencyKind,
    pub resolved: Option<PathBuf>,
}
#[derive(Clone, Debug)]
pub struct Trace {
    pub run: RunId,
    /// None for script runs; no build directory is created or exempted from input checks.
    pub build_directory: Option<PathBuf>,
    pub events: Vec<TraceEvent>,
    pub dependencies: Vec<ObservedDependency>,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ExecutionMode {
    Configure,
    Script { file: PathBuf },
}
#[derive(Clone, Debug)]
/// Shared execution options. Script mode uses source_directory as cwd and ignores build_directory.
pub struct ConfigureOptions {
    pub mode: ExecutionMode,
    pub executable: PathBuf,
    pub source_directory: PathBuf,
    pub build_directory: PathBuf,
    pub arguments: Vec<String>,
    pub timeout: Duration,
    pub max_output_bytes: usize,
    pub max_error_bytes: usize,
}
