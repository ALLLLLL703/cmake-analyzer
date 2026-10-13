// AI-generated implementation.
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum Version {
    #[default]
    #[serde(rename = "2.0")]
    V2,
}

pub type RequestId = lsp_types::NumberOrString;

/// JSON-RPC parameters are objects or arrays, not arbitrary scalar values.
pub type Params = lsp_types::OneOf<lsp_types::LSPObject, lsp_types::LSPArray>;

#[derive(Debug, Serialize, Deserialize)]
pub struct Request {
    pub jsonrpc: Version,
    pub id: RequestId,
    pub method: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub params: Option<Params>,
}

#[derive(Debug, Serialize)]
pub struct Notification {
    pub jsonrpc: Version,
    pub method: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub params: Option<Params>,
}

#[derive(Debug)]
pub struct Response {
    pub jsonrpc: Version,
    /// Null only when an error cannot be linked to a request.
    pub id: Option<RequestId>,
    pub payload: ResponsePayload,
}
#[derive(Debug)]
pub enum ResponsePayload {
    Result(Value),
    Error(ResponseError),
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseError {
    pub code: i32,
    pub message: String,
    #[serde(
        default,
        deserialize_with = "super::serde::present",
        skip_serializing_if = "Option::is_none"
    )]
    pub data: Option<Value>,
}
impl Response {
    pub fn error(id: Option<RequestId>, error: ResponseError) -> Self {
        Self {
            jsonrpc: Version::V2,
            id,
            payload: ResponsePayload::Error(error),
        }
    }
}
impl ResponseError {
    pub fn new(code: i32, message: impl Into<String>) -> Self {
        Self {
            code,
            message: message.into(),
            data: None,
        }
    }
}

#[derive(Debug, Serialize)]
#[serde(untagged)]
pub enum Message {
    Request(Request),
    Notification(Notification),
    Response(Response),
}

#[derive(Debug)]
pub enum DecodeError {
    MethodMismatch,
    IdMismatch,
    Remote(ResponseError),
    InvalidParams(serde_json::Error),
    InvalidResult(serde_json::Error),
}

// Private wire fields preserve the difference between absent and null.
#[derive(Deserialize)]
pub(super) struct NotificationFields {
    pub jsonrpc: Version,
    #[serde(default, deserialize_with = "super::serde::present")]
    pub id: Option<Value>,
    pub method: String,
    pub params: Option<Params>,
}
#[derive(Deserialize)]
pub(super) struct ResponseFields {
    pub jsonrpc: Version,
    #[serde(deserialize_with = "super::serde::response_id")]
    pub id: Option<RequestId>,
    #[serde(default, deserialize_with = "super::serde::present")]
    pub result: Option<Value>,
    #[serde(default, deserialize_with = "super::serde::present")]
    pub error: Option<ResponseError>,
}
