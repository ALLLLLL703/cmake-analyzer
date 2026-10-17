// AI-generated implementation.
//! Unrecognized, non-reserved protocol methods forwarded without their RPC envelope.
/// Raw extension input for [`super::ILanguageServer::custom_request`] and
/// [`super::ILanguageServer::custom_notification`]. Only extensions expose JSON directly.
/// The wrapper retains the request ID and envelope; constructing this record has no I/O.
/// Known standard methods with invalid parameters are rejected, not forwarded here.
///
/// # Examples
/// ```
/// use lspally::server::model::Extension;
/// let extension = Extension { method: "cmake-analyzer/reloadConfig".into(), params: None };
/// ```
#[derive(Debug)]
pub struct Extension {
    /// Method spelling supplied by the peer. Example: `extension.method == "cmake-analyzer/reloadConfig"`.
    pub method: String,
    /// Object/array parameters, or `None` for omitted/null parameters.
    /// Example: `extension.params = Some(serde_json::json!({"path": "/tmp/config.toml"}))`.
    pub params: Option<serde_json::Value>,
}
