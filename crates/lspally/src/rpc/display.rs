// AI-generated implementation.
use super::model::{Message, Notification, Request, Response};
use serde::Serialize;

/// One UTF-8 LSP frame. Content-Length counts bytes, not characters.
pub fn to_lsp_string(message: &impl Serialize) -> serde_json::Result<String> {
    let body = serde_json::to_string(message)?;
    Ok(format!("Content-Length: {}\r\n\r\n{body}", body.len()))
}
impl Request {
    pub fn display(&self) -> serde_json::Result<String> {
        to_lsp_string(self)
    }
}
impl Notification {
    pub fn display(&self) -> serde_json::Result<String> {
        to_lsp_string(self)
    }
}
impl Response {
    pub fn display(&self) -> serde_json::Result<String> {
        to_lsp_string(self)
    }
}
impl Message {
    pub fn display(&self) -> serde_json::Result<String> {
        to_lsp_string(self)
    }
}
