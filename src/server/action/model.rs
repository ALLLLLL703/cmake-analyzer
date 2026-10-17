//! Command identities and owned action payloads used by the action dispatcher.
//! Constructing these records has no side effects; execution validates project ownership separately.

// AI-authored action command identities and validated request payloads.
use lspally::server::model::{Cancellation, Uri};
use std::path::PathBuf;
#[derive(Clone, Copy, Debug)]
/// One advertised server command, consumed by [`ActionRequest`] and the action dispatcher.
/// Selecting a variant has no effect; configure and script execution can change arbitrary project files.
/// [`Self::command`] supplies the wire spelling; [`Self::parse`] rejects unknown spellings.
///
/// # Examples
/// ```text
/// let kind = ActionKind::Configure;
/// assert_eq!(ActionKind::parse(kind.command()).unwrap().command(), kind.command());
/// ```
pub(in crate::server) enum ActionKind {
    /// Configure the selected project with -S/-B when dispatched. Example:
    /// `ActionKind::Configure.command()` selects cmake-analyzer.configure.
    Configure,
    /// Run the explicitly selected -P script when dispatched. Example: `ActionKind::RunScript.command()`
    /// selects cmake-analyzer.runScript.
    RunScript,
    /// Read recorded executions without rerunning CMake. Example:
    /// `ActionKind::ExpandedArguments.command()` selects the inspection command.
    ExpandedArguments,
    /// Cancel a queued or live matching profile. Example: `ActionKind::CancelConfigure.command()`
    /// selects cancellation for either execution mode.
    CancelConfigure,
}
impl ActionKind {
    /// Return the protocol command spelling without side effects.
    ///
    /// # Arguments
    /// - `self` `[in]`: command identity; consumed by value (the enum is Copy).
    ///
    /// # Returns
    /// Static command name advertised to clients.
    ///
    /// # Examples
    /// Local server context:
    /// ```text
    /// assert_eq!(ActionKind::RunScript.command(), "cmake-analyzer.runScript");
    /// ```
    pub fn command(self) -> &'static str {
        match self {
            Self::Configure => "cmake-analyzer.configure",
            Self::RunScript => "cmake-analyzer.runScript",
            Self::ExpandedArguments => "cmake-analyzer.expandedArguments",
            Self::CancelConfigure => "cmake-analyzer.cancelConfigure",
        }
    }
    /// Recognize one exact protocol command name without side effects.
    ///
    /// # Arguments
    /// - `value` `[in]`: case-sensitive command spelling.
    ///
    /// # Returns
    /// Some identity for an advertised command; None for every other spelling.
    ///
    /// # Examples
    /// Local server context:
    /// ```text
    /// assert!(ActionKind::parse("cmake-analyzer.configure").is_some());
    /// ```
    pub fn parse(value: &str) -> Option<Self> {
        [
            Self::Configure,
            Self::RunScript,
            Self::ExpandedArguments,
            Self::CancelConfigure,
        ]
        .into_iter()
        .find(|kind| kind.command() == value)
    }
}
/// Owned action arguments sent through the service queue by manual commands or authorized saves.
/// Construction does not authorize or execute code. [`ActionKind`] chooses the effect; the service
/// rechecks that the URI, root and optional execution profile belong together.
///
/// # Examples
/// ```text
/// let request = ActionRequest { kind: ActionKind::Configure, uri, root,
///     line: 0, cancellation: Cancellation::default(), execution: None };
/// ```
pub(in crate::server) struct ActionRequest {
    /// Requested operation. Example: `request.kind = ActionKind::CancelConfigure` selects cancellation.
    pub kind: ActionKind,
    /// Originating document URI used for ownership. Example: `let uri = &request.uri` reads the selected
    /// document.
    pub uri: Uri,
    /// Exact active project entry path, not a source directory. Example: `let entry = &request.root`
    /// reads the entry file.
    pub root: PathBuf,
    /// Zero-based source line for expanded-argument inspection. Example: `request.line = 4` selects the
    /// fifth line.
    pub line: u64,
    /// Request token, not the independent live-job token. Example: `request.cancellation.cancel()`
    /// signals cancellation; runtime start checks it before spawning, inspection/cancel actions do not.
    pub cancellation: Cancellation,
    /// Optional profile discriminator for shared entry paths; None permits mode inference. Example:
    /// `request.execution = Some(crate::runtime_cmake::model::ExecutionMode::Configure)` selects
    /// configuration.
    pub execution: Option<crate::runtime_cmake::model::ExecutionMode>,
}
