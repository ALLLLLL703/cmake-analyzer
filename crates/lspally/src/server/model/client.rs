// AI-generated implementation.
use super::{Data, Diagnostic, NumberOrString, Uri};

/// Publish replaces the previous report; an empty vector clears it.
#[derive(Debug)]
pub struct DiagnosticReport {
    pub uri: Uri,
    pub version: Option<i32>,
    pub diagnostics: Vec<Diagnostic>,
}
#[derive(Debug)]
pub struct ProgressUpdate {
    pub token: NumberOrString,
    pub value: Data,
}
