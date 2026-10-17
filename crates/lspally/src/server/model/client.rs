// AI-generated implementation.
//! Business payloads for diagnostics and arbitrary progress; construction performs no I/O.
use super::{Data, Diagnostic, NumberOrString, Uri};

/// Replacement diagnostic report sent with [`super::ClientHandle::publish_diagnostics`].
/// An empty diagnostic vector clears the client's previous report for this URI.
/// Construction does not send anything; publishing does not apply document edits.
///
/// # Examples
/// ```
/// use lspally::server::model::{DiagnosticReport, Uri};
/// let clear = DiagnosticReport {
///     uri: Uri("file:///tmp/CMakeLists.txt".into()),
///     version: Some(3), diagnostics: vec![],
/// };
/// ```
#[derive(Debug)]
pub struct DiagnosticReport {
    /// Document whose report is replaced. Example: `report.uri.0.starts_with("file:")`.
    pub uri: Uri,
    /// Analyzed document version, or `None` when not version-qualified.
    /// Example: `report.version = Some(3)`.
    pub version: Option<i32>,
    /// Complete replacement list, not an append operation. Example: `report.diagnostics.clear()`.
    pub diagnostics: Vec<Diagnostic>,
}
/// Arbitrary progress payload sent with [`super::ClientHandle::progress`].
/// Unlike [`super::ProgressParams`], this also supports method-specific partial results.
/// Construction has no side effects; the caller must choose the appropriate token.
///
/// # Examples
/// ```
/// use lspally::server::model::{Data, NumberOrString, ProgressUpdate};
/// let update = ProgressUpdate {
///     token: NumberOrString::String("symbols".into()),
///     value: Data::List(vec![]),
/// };
/// ```
#[derive(Debug)]
pub struct ProgressUpdate {
    /// Client-supplied or previously created progress identity.
    /// Example: `update.token = NumberOrString::Number(1)`.
    pub token: NumberOrString,
    /// Progress body, including arbitrary partial-result arrays.
    /// Example: `update.value = Data::List(vec![])`.
    pub value: Data,
}
