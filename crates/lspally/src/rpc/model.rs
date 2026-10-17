// AI-generated implementation.
//! Owned JSON-RPC envelopes and wire-validation records; constructing them performs no I/O.
//!
//! [`Request::take_params`] moves parameters out while retaining the ID and method for
//! [`Request::decode_response`]. Callers, not these records, track outstanding requests.
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// The supported JSON-RPC version, used by all envelopes. No side effects.
/// See [`Request`] for a message carrying this marker.
///
/// # Examples
/// ```
/// use lspally::rpc::Version;
/// assert_eq!(serde_json::to_string(&Version::V2).unwrap(), "\"2.0\"");
/// ```
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum Version {
    /// JSON-RPC 2.0; example: `let version = lspally::rpc::Version::V2;`.
    #[default]
    #[serde(rename = "2.0")]
    V2,
}

/// Numeric or string correlation key; example: `let id = lspally::rpc::RequestId::Number(1);`.
/// [`Request`] owns a key and [`Response`] echoes it; allocation belongs to the caller.
pub type RequestId = lsp_types::NumberOrString;

/// JSON-RPC parameters are objects or arrays, not arbitrary scalar values.
/// [`Request::new`] and [`Notification::new`] normalize null to absence. No side effects.
/// Example: `let params = lspally::rpc::Params::Right(vec![]);`.
pub type Params = lsp_types::OneOf<lsp_types::LSPObject, lsp_types::LSPArray>;

/// A call expecting one [`Response`]; used by dispatchers and outgoing-call trackers.
/// Owns its JSON parameters; [`Self::take_params`] consumes only that tree. No I/O.
/// Unknown method strings remain available for extension dispatch.
///
/// # Ownership
/// ```text
///        [Request owned by dispatcher]
///             |                 |
///       take_params        retain envelope
///             v                 v
///     [typed parameters]   [ID + method]
///                               |
///                        decode_response
///                               v
///                        [typed result]
/// ```
/// # Examples
/// ```
/// use lspally::rpc::{Request, RequestId};
/// let request = Request::new::<lsp_types::request::Shutdown>(RequestId::Number(1), ()).unwrap();
/// assert_eq!(request.method, "shutdown");
/// ```
#[derive(Debug, Serialize, Deserialize)]
pub struct Request {
    /// Wire version; example: `assert_eq!(request.jsonrpc, lspally::rpc::Version::V2);`.
    pub jsonrpc: Version,
    /// Caller-assigned correlation key; example: `request.id = lspally::rpc::RequestId::Number(2);`.
    pub id: RequestId,
    /// Exact method name; example: `assert_eq!(request.method, "shutdown");`.
    pub method: String,
    /// Owned object/array, omitted when absent; example: `request.params = None;`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub params: Option<Params>,
}

/// A one-way call used by notification dispatchers; no ID or response is expected.
/// [`Self::take_params`] consumes its parameters; constructing it has no I/O effects.
/// Deserialization rejects an `id` field, even when null; see [`Message`] for classification.
///
/// # Examples
/// ```
/// use lspally::rpc::Notification;
/// let notification = Notification::new::<lsp_types::notification::Exit>(()).unwrap();
/// assert_eq!(notification.method, "exit");
/// ```
#[derive(Debug, Serialize)]
pub struct Notification {
    /// Wire version; example: `notification.jsonrpc = lspally::rpc::Version::V2;`.
    pub jsonrpc: Version,
    /// Exact one-way method name; example: `assert_eq!(notification.method, "exit");`.
    pub method: String,
    /// Owned object/array or omission; example: `assert!(notification.params.is_none());`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub params: Option<Params>,
}

/// A reply correlated with a [`Request`], containing exactly one [`ResponsePayload`].
/// Used by dispatchers and pending-call delivery. Construction does not send anything.
/// Serialization/deserialization reject successful replies with null IDs.
///
/// # Examples
/// ```
/// use lspally::rpc::{Response, ResponseError};
/// let reply = Response::error(None, ResponseError::new(-32700, "Invalid JSON"));
/// assert!(reply.id.is_none());
/// ```
#[derive(Debug)]
pub struct Response {
    /// Wire version; example: `response.jsonrpc = lspally::rpc::Version::V2;`.
    pub jsonrpc: Version,
    /// Null only when an error cannot be linked to a request.
    /// Example: `response.id = Some(lspally::rpc::RequestId::Number(1));`.
    pub id: Option<RequestId>,
    /// Exclusive success/error body; example: `matches!(response.payload, lspally::rpc::ResponsePayload::Error(_));`.
    pub payload: ResponsePayload,
}
/// Exclusive reply content used by [`Response`] and [`Request::decode_response`].
/// Moving this value transfers the result/error without I/O.
///
/// # Examples
/// ```
/// use lspally::rpc::ResponsePayload;
/// let payload = ResponsePayload::Result(serde_json::Value::Null);
/// assert!(matches!(payload, ResponsePayload::Result(_)));
/// ```
#[derive(Debug)]
pub enum ResponsePayload {
    /// Successful JSON value, including null; example: `ResponsePayload::Result(serde_json::Value::Null)`.
    Result(
        /// Owned result tree; example: `if let ResponsePayload::Result(value) = payload { assert!(value.is_null()); }`.
        Value,
    ),
    /// Remote failure; example: `ResponsePayload::Error(lspally::rpc::ResponseError::new(-32601, "Unknown method"))`.
    Error(
        /// Owned error details; example: `if let ResponsePayload::Error(error) = payload { let code = error.code; }`.
        ResponseError,
    ),
}
/// Wire failure code, message and optional data, used by [`Response::error`].
/// Implements [`std::error::Error`] for callers; construction has no side effects.
/// Explicit JSON null data stays `Some(Value::Null)` when decoded.
///
/// # Examples
/// ```
/// use lspally::rpc::ResponseError;
/// let error = ResponseError::new(-32602, "Invalid parameters");
/// assert!(error.data.is_none());
/// ```
#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseError {
    /// JSON-RPC/LSP error code; example: `assert_eq!(error.code, -32602);`.
    pub code: i32,
    /// Human-readable reason; example: `error.message = "Missing URI".into();`.
    pub message: String,
    /// Optional extra JSON, preserving explicit null; example: `error.data = Some(serde_json::Value::Null);`.
    #[serde(
        default,
        deserialize_with = "super::serde::present",
        skip_serializing_if = "Option::is_none"
    )]
    pub data: Option<Value>,
}
impl Response {
    /// Build an error reply without sending it or changing other state.
    ///
    /// # Arguments
    /// - `id` `[in]`: request key, or `None` when the request cannot be identified.
    /// - `error` `[in]`: failure details moved into the reply.
    ///
    /// # Returns
    /// A version-2.0 response with an error payload.
    ///
    /// # Examples
    /// ```
    /// use lspally::rpc::{Response, ResponseError};
    /// let reply = Response::error(None, ResponseError::new(-32700, "Invalid JSON"));
    /// assert!(reply.id.is_none());
    /// ```
    pub fn error(id: Option<RequestId>, error: ResponseError) -> Self {
        Self {
            jsonrpc: Version::V2,
            id,
            payload: ResponsePayload::Error(error),
        }
    }
}
impl ResponseError {
    /// Own a failure message with no attached data; no I/O or shared-state effects.
    ///
    /// # Arguments
    /// - `code` `[in]`: protocol error code retained unchanged.
    /// - `message` `[in]`: reason converted into an owned string.
    ///
    /// # Returns
    /// Error details with `data` set to `None`.
    ///
    /// # Examples
    /// ```
    /// use lspally::rpc::ResponseError;
    /// assert_eq!(ResponseError::new(-32601, "Unknown method").code, -32601);
    /// ```
    pub fn new(code: i32, message: impl Into<String>) -> Self {
        Self {
            code,
            message: message.into(),
            data: None,
        }
    }
}

/// An owned request, notification or response for stream routing.
/// Used by the reader and writer; [`Self::display`] frames it without I/O.
/// Decoding classifies by field presence, so malformed requests never fall back
/// to notifications. Batches (JSON arrays) are not supported.
///
/// # Classification
/// ```text
///                [JSON object]
///                      |
///                 <method?>
///                 /       \
///               yes        no
///                |          |
///              <id?>   [Response]
///             /     \
///     [Request]   [Notification]
/// ```
/// # Examples
/// ```
/// use lspally::rpc::{Message, Notification};
/// let message = Message::Notification(Notification::new::<lsp_types::notification::Exit>(()).unwrap());
/// assert!(message.display().unwrap().starts_with("Content-Length:"));
/// ```
#[derive(Debug, Serialize)]
#[serde(untagged)]
pub enum Message {
    /// Call with an ID; example: `Message::Request(lspally::rpc::Request::new::<lsp_types::request::Shutdown>(lspally::rpc::RequestId::Number(1), ()).unwrap())`.
    Request(
        /// Owned call; example: `if let Message::Request(request) = message { let id = request.id; }`.
        Request,
    ),
    /// One-way call; example: `Message::Notification(lspally::rpc::Notification::new::<lsp_types::notification::Exit>(()).unwrap())`.
    Notification(
        /// Owned notification; example: `if let Message::Notification(note) = message { let method = note.method; }`.
        Notification,
    ),
    /// Reply to a call; example: `Message::Response(lspally::rpc::Response::error(None, lspally::rpc::ResponseError::new(-32700, "Invalid JSON")))`.
    Response(
        /// Owned reply; example: `if let Message::Response(reply) = message { let id = reply.id; }`.
        Response,
    ),
}

/// Typed parameter/result decoding failure returned by [`Request`] and [`Notification`].
/// Matching or displaying it has no side effects; [`std::error::Error::source`]
/// exposes nested JSON or remote errors, but not correlation mismatches.
///
/// # Examples
/// ```
/// use lspally::rpc::DecodeError;
/// assert_eq!(DecodeError::IdMismatch.to_string(), "Response ID does not match the request");
/// ```
#[derive(Debug)]
pub enum DecodeError {
    /// Selected LSP marker names another method; example: `matches!(error, DecodeError::MethodMismatch)`.
    MethodMismatch,
    /// Reply key differs from the retained request; example: `matches!(error, DecodeError::IdMismatch)`.
    IdMismatch,
    /// Client returned a protocol error; example: `DecodeError::Remote(ResponseError::new(-32601, "Unknown method"))`.
    Remote(
        /// Owned remote failure; example: `if let DecodeError::Remote(remote) = error { let code = remote.code; }`.
        ResponseError,
    ),
    /// Parameters did not fit the selected marker; example: `matches!(error, DecodeError::InvalidParams(_))`.
    InvalidParams(
        /// JSON conversion failure; example: `if let DecodeError::InvalidParams(cause) = error { let reason = cause.to_string(); }`.
        serde_json::Error,
    ),
    /// Successful payload did not fit the expected result; example: `matches!(error, DecodeError::InvalidResult(_))`.
    InvalidResult(
        /// JSON conversion failure; example: `if let DecodeError::InvalidResult(cause) = error { let reason = cause.to_string(); }`.
        serde_json::Error,
    ),
}

// Private wire fields preserve the difference between absent and null.
/// Temporary notification fields used by its custom deserializer; no I/O.
/// [`Notification`] rejects any present `id`, including null.
///
/// # Examples
/// ```text
/// let fields: NotificationFields = serde_json::from_str(r#"{"jsonrpc":"2.0","method":"exit"}"#)?;
/// assert!(fields.id.is_none());
/// ```
#[derive(Deserialize)]
pub(super) struct NotificationFields {
    /// Required version; example: `assert_eq!(fields.jsonrpc, Version::V2);`.
    pub jsonrpc: Version,
    /// Presence marker, retaining null; example: `assert!(fields.id.is_some());` for `"id": null`.
    #[serde(default, deserialize_with = "super::serde::present")]
    pub id: Option<Value>,
    /// Method text; example: `assert_eq!(fields.method, "exit");`.
    pub method: String,
    /// Parameters, normalizing absent/null to `None`; example: `assert!(fields.params.is_none());`.
    pub params: Option<Params>,
}
/// Temporary reply fields used to validate [`Response`]; no I/O.
/// [`ResponsePayload`] is chosen only after checking the ID and exclusive payload.
///
/// # Examples
/// ```text
/// let fields: ResponseFields = serde_json::from_str(r#"{"jsonrpc":"2.0","id":1,"result":null}"#)?;
/// assert_eq!(fields.result, Some(serde_json::Value::Null));
/// ```
#[derive(Deserialize)]
pub(super) struct ResponseFields {
    /// Required version; example: `assert_eq!(fields.jsonrpc, Version::V2);`.
    pub jsonrpc: Version,
    /// Required key, possibly null for errors; example: `assert_eq!(fields.id, Some(RequestId::Number(1)));`.
    #[serde(deserialize_with = "super::serde::response_id")]
    pub id: Option<RequestId>,
    /// Present success value, retaining null; example: `assert_eq!(fields.result, Some(Value::Null));`.
    #[serde(default, deserialize_with = "super::serde::present")]
    pub result: Option<Value>,
    /// Present non-null error object; example: `assert!(fields.error.is_none());` for a success reply.
    #[serde(default, deserialize_with = "super::serde::present")]
    pub error: Option<ResponseError>,
}
