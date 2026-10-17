// AI-generated implementation.
//! Compact JSON and Content-Length framing without stream I/O or terminal styling.
//!
//! All display helpers return owned strings, propagate serialization failures and add
//! neither an extra trailing newline nor ANSI escapes. Lengths count UTF-8 bytes.
use super::model::{Message, Notification, Request, Response};
use serde::Serialize;

/// Build one UTF-8 LSP frame without writing it; no external side effects.
/// Content-Length counts compact JSON bytes, not Unicode characters.
///
/// # Arguments
/// - `message` `[in]`: borrowed serializable payload; not restricted to validated RPC envelopes.
///
/// # Returns
/// `Ok(frame)` containing `Content-Length: <bytes>\r\n\r\n<JSON>` with no trailing newline;
/// `Err` when JSON serialization fails.
///
/// # Errors
/// Propagates serialization failures, including invalid manually constructed responses.
///
/// # Examples
/// ```
/// use lspally::rpc::display::to_lsp_string;
/// let frame = to_lsp_string(&serde_json::json!({"text": "é"})).unwrap();
/// let (header, body) = frame.split_once("\r\n\r\n").unwrap();
/// assert_eq!(header, format!("Content-Length: {}", body.len()));
/// ```
pub fn to_lsp_string(message: &impl Serialize) -> serde_json::Result<String> {
    let body = serde_json::to_string(message)?;
    Ok(format!("Content-Length: {}\r\n\r\n{body}", body.len()))
}
impl Request {
    /// Frame this request through [`to_lsp_string`]; no stream I/O or mutation.
    ///
    /// # Arguments
    /// - `self` `[in]`: request borrowed for compact JSON serialization.
    ///
    /// # Returns
    /// `Ok(frame)` with a byte-counted header; `Err` if serialization fails.
    ///
    /// # Errors
    /// Propagates JSON serialization failures.
    ///
    /// # Examples
    /// ```
    /// use lspally::rpc::{Request, RequestId};
    /// let request = Request::new::<lsp_types::request::Shutdown>(RequestId::Number(1), ()).unwrap();
    /// assert!(request.display().unwrap().contains("\r\n\r\n"));
    /// ```
    pub fn display(&self) -> serde_json::Result<String> {
        to_lsp_string(self)
    }
}
impl Notification {
    /// Frame this one-way call through [`to_lsp_string`]; no stream I/O or mutation.
    ///
    /// # Arguments
    /// - `self` `[in]`: notification borrowed for compact JSON serialization.
    ///
    /// # Returns
    /// `Ok(frame)` with a byte-counted header; `Err` if serialization fails.
    ///
    /// # Errors
    /// Propagates JSON serialization failures.
    ///
    /// # Examples
    /// ```
    /// use lspally::rpc::Notification;
    /// let note = Notification::new::<lsp_types::notification::Exit>(()).unwrap();
    /// assert!(note.display().unwrap().ends_with('}'));
    /// ```
    pub fn display(&self) -> serde_json::Result<String> {
        to_lsp_string(self)
    }
}
impl Response {
    /// Frame this reply through [`to_lsp_string`]; no stream I/O or mutation.
    ///
    /// # Arguments
    /// - `self` `[in]`: reply borrowed for validation and compact JSON serialization.
    ///
    /// # Returns
    /// `Ok(frame)` with a byte-counted header; `Err` for invalid or unserializable content.
    ///
    /// # Errors
    /// Rejects success with a null ID and propagates other JSON serialization failures.
    ///
    /// # Examples
    /// ```
    /// use lspally::rpc::{Response, ResponseError};
    /// let reply = Response::error(None, ResponseError::new(-32700, "Invalid JSON"));
    /// assert!(reply.display().unwrap().contains("\"error\""));
    /// ```
    pub fn display(&self) -> serde_json::Result<String> {
        to_lsp_string(self)
    }
}
impl Message {
    /// Frame the contained envelope through [`to_lsp_string`]; no stream I/O or mutation.
    ///
    /// # Arguments
    /// - `self` `[in]`: tagged Rust value serialized without a JSON variant tag.
    ///
    /// # Returns
    /// `Ok(frame)` with a byte-counted header; `Err` if envelope serialization fails.
    ///
    /// # Errors
    /// Propagates nested serialization errors, including success replies with null IDs.
    ///
    /// # Examples
    /// ```
    /// use lspally::rpc::{Message, Notification};
    /// let message = Message::Notification(Notification::new::<lsp_types::notification::Exit>(()).unwrap());
    /// assert!(message.display().unwrap().starts_with("Content-Length: "));
    /// ```
    pub fn display(&self) -> serde_json::Result<String> {
        to_lsp_string(self)
    }
}
