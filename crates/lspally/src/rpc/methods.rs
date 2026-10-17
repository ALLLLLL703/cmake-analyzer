// AI-generated implementation.
//! Typed LSP marker conversion without stream I/O or outstanding-request storage.
//!
//! Parameter extraction moves the JSON tree; method mismatches leave it untouched.
//! Reply decoding consumes the response while borrowing the retained request envelope.
use super::model::{
    DecodeError, Notification, Params, Request, RequestId, Response, ResponseError,
    ResponsePayload, Version,
};
use lsp_types::{notification::Notification as LspNotification, request::Request as LspRequest};
use serde::Serialize;
use serde_json::Value;
use std::{error::Error, fmt};

/// Serialize parameters to an owned object/array, or omit null; no external side effects.
///
/// # Arguments
/// - `params` `[in]`: owned serializable method parameters.
///
/// # Returns
/// `Ok(Some(...))` for object/array, `Ok(None)` for null; `Err` on serialization or scalar input.
///
/// # Errors
/// Propagates serializer failures; rejects strings, booleans and numbers as RPC parameters.
///
/// # Examples
/// ```text
/// assert!(encode_params(())?.is_none());
/// assert!(encode_params(serde_json::json!({"uri": "file:///a"}))?.is_some());
/// ```
fn encode_params(params: impl Serialize) -> serde_json::Result<Option<Params>> {
    match serde_json::to_value(params)? {
        Value::Object(value) => Ok(Some(Params::Left(value))),
        Value::Array(value) => Ok(Some(Params::Right(value))),
        Value::Null => Ok(None),
        _ => Err(serde::ser::Error::custom(
            "RPC params must be an object, array, or absent",
        )),
    }
}
/// Move a parameter tree out, replacing its slot with `None`; no I/O.
///
/// # Arguments
/// - `params` `[in,out]`: slot consumed and overwritten with `None`, including on later decode failure.
///
/// # Returns
/// The owned object/array, or JSON null if the slot was already empty.
///
/// # Examples
/// ```text
/// let mut params = Some(Params::Right(vec![]));
/// assert_eq!(take_value(&mut params), serde_json::json!([]));
/// assert!(params.is_none());
/// ```
fn take_value(params: &mut Option<Params>) -> Value {
    match params.take() {
        Some(Params::Left(value)) => Value::Object(value),
        Some(Params::Right(value)) => Value::Array(value),
        None => Value::Null,
    }
}
impl Request {
    /// Build a typed request envelope without sending it or allocating a correlation ID.
    ///
    /// # Arguments
    /// - `id` `[in]`: caller-owned unique key moved into the envelope.
    /// - `params` `[in]`: parameters for marker `R`, serialized and moved into owned JSON.
    ///
    /// # Returns
    /// `Ok(request)` with `R::METHOD`, version 2.0 and normalized parameters; `Err` if encoding fails.
    ///
    /// # Errors
    /// Serialization failures or parameters encoded as a non-null scalar.
    ///
    /// # Examples
    /// ```
    /// use lspally::rpc::{Request, RequestId};
    /// let request = Request::new::<lsp_types::request::Shutdown>(RequestId::Number(1), ()).unwrap();
    /// assert!(request.params.is_none());
    /// ```
    pub fn new<R: LspRequest>(id: RequestId, params: R::Params) -> serde_json::Result<Self> {
        Ok(Self {
            jsonrpc: Version::V2,
            id,
            method: R::METHOD.into(),
            params: encode_params(params)?,
        })
    }
    /// Check the method, then move parameters into the selected LSP type; no I/O.
    /// The ID and method remain available for response correlation.
    ///
    /// # Arguments
    /// - `self` `[in,out]`: on matching method, `params` is overwritten with `None` even if decoding fails;
    ///   on method mismatch the entire envelope is unchanged.
    ///
    /// # Returns
    /// `Ok(parameters)` decoded as `R::Params`, or `Err` describing method/parameter mismatch.
    ///
    /// # Errors
    /// `MethodMismatch` for another method; `InvalidParams` for incompatible JSON (including absent
    /// parameters decoded as null). Retrying after a decode failure sees absent parameters.
    ///
    /// # Examples
    /// ```
    /// use lspally::rpc::{Request, RequestId};
    /// let mut request = Request::new::<lsp_types::request::Shutdown>(RequestId::Number(1), ()).unwrap();
    /// request.take_params::<lsp_types::request::Shutdown>().unwrap();
    /// assert_eq!(request.method, "shutdown");
    /// ```
    pub fn take_params<R: LspRequest>(&mut self) -> Result<R::Params, DecodeError> {
        if self.method != R::METHOD {
            return Err(DecodeError::MethodMismatch);
        }
        serde_json::from_value(take_value(&mut self.params)).map_err(DecodeError::InvalidParams)
    }
    /// Validate correlation and consume a reply into its typed result; no I/O or tracking changes.
    /// The caller must associate replies with outstanding envelopes before this check.
    ///
    /// # Arguments
    /// - `self` `[in]`: retained method and ID; parameters are not examined or changed.
    /// - `response` `[in]`: owned reply consumed on success and on every error.
    ///
    /// # Returns
    /// `Ok(R::Result)` for a matching successful reply, including valid null results;
    /// `Err` for correlation, remote or result-decoding failures.
    ///
    /// # Errors
    /// Checks `MethodMismatch` before `IdMismatch`, then returns `Remote` for error payloads
    /// or `InvalidResult` for incompatible success JSON.
    ///
    /// # Algorithm
    /// Check the selected method, then the response ID, before inspecting its payload.
    /// Only matching success JSON is deserialized; remote errors retain their original details.
    ///
    /// # Flow
    /// ```text
    ///           <method + ID match?>
    ///             /             \
    ///           no              yes
    ///            |               |
    ///       [mismatch]       <payload?>
    ///                       /          \
    ///                  [remote Err]  [decode result]
    ///                                     |
    ///                              [typed Ok / JSON Err]
    /// ```
    /// # Examples
    /// ```
    /// use lspally::rpc::{Request, RequestId, Response};
    /// use lsp_types::request::Shutdown;
    /// let request = Request::new::<Shutdown>(RequestId::Number(1), ()).unwrap();
    /// let reply = Response::success::<Shutdown>(RequestId::Number(1), ()).unwrap();
    /// request.decode_response::<Shutdown>(reply).unwrap();
    /// ```
    pub fn decode_response<R: LspRequest>(
        &self,
        response: Response,
    ) -> Result<R::Result, DecodeError> {
        if self.method != R::METHOD {
            return Err(DecodeError::MethodMismatch);
        }
        if response.id.as_ref() != Some(&self.id) {
            return Err(DecodeError::IdMismatch);
        }
        match response.payload {
            ResponsePayload::Result(value) => {
                serde_json::from_value(value).map_err(DecodeError::InvalidResult)
            }
            ResponsePayload::Error(error) => Err(DecodeError::Remote(error)),
        }
    }
}
impl Notification {
    /// Build a typed one-way envelope without sending it; no external side effects.
    ///
    /// # Arguments
    /// - `params` `[in]`: marker `N` parameters serialized into owned object/array JSON or omission.
    ///
    /// # Returns
    /// `Ok(notification)` with version 2.0 and `N::METHOD`; `Err` on parameter encoding failure.
    ///
    /// # Errors
    /// Serialization failures or parameters encoded as a non-null scalar.
    ///
    /// # Examples
    /// ```
    /// use lspally::rpc::Notification;
    /// let notification = Notification::new::<lsp_types::notification::Exit>(()).unwrap();
    /// assert_eq!(notification.method, "exit");
    /// ```
    pub fn new<N: LspNotification>(params: N::Params) -> serde_json::Result<Self> {
        Ok(Self {
            jsonrpc: Version::V2,
            method: N::METHOD.into(),
            params: encode_params(params)?,
        })
    }
    /// Check a notification method and move its parameters into the selected type; no I/O.
    ///
    /// # Arguments
    /// - `self` `[in,out]`: matching method consumes `params`, overwriting it with `None` even
    ///   on decode failure; a method mismatch leaves the envelope unchanged.
    ///
    /// # Returns
    /// `Ok(N::Params)` decoded from the owned tree (or null for absence); `Err` on mismatch.
    ///
    /// # Errors
    /// `MethodMismatch` for another method; `InvalidParams` when JSON does not fit `N::Params`.
    ///
    /// # Examples
    /// ```
    /// use lspally::rpc::Notification;
    /// let mut note = Notification::new::<lsp_types::notification::Exit>(()).unwrap();
    /// note.take_params::<lsp_types::notification::Exit>().unwrap();
    /// assert!(note.params.is_none());
    /// ```
    pub fn take_params<N: LspNotification>(&mut self) -> Result<N::Params, DecodeError> {
        if self.method != N::METHOD {
            return Err(DecodeError::MethodMismatch);
        }
        serde_json::from_value(take_value(&mut self.params)).map_err(DecodeError::InvalidParams)
    }
}
impl Response {
    /// Encode a typed successful result, retaining null as a real result; no I/O.
    ///
    /// # Arguments
    /// - `id` `[in]`: originating request key moved into a non-null response ID.
    /// - `result` `[in]`: marker `R` result converted into owned JSON.
    ///
    /// # Returns
    /// `Ok(response)` with a result payload, or `Err` if result serialization fails.
    ///
    /// # Errors
    /// Propagates JSON serialization failures from the result type.
    ///
    /// # Examples
    /// ```
    /// use lspally::rpc::{RequestId, Response, ResponsePayload};
    /// let reply = Response::success::<lsp_types::request::Shutdown>(RequestId::Number(1), ()).unwrap();
    /// assert!(matches!(reply.payload, ResponsePayload::Result(serde_json::Value::Null)));
    /// ```
    pub fn success<R: LspRequest>(id: RequestId, result: R::Result) -> serde_json::Result<Self> {
        Ok(Self {
            jsonrpc: Version::V2,
            id: Some(id),
            payload: ResponsePayload::Result(serde_json::to_value(result)?),
        })
    }
}
impl fmt::Display for ResponseError {
    /// Write the RPC code and message, omitting optional data; no I/O beyond the formatter.
    ///
    /// # Arguments
    /// - `self` `[in]`: error details borrowed without mutation.
    /// - `f` `[in,out]`: formatter receiving appended text; existing sink content is not cleared.
    ///
    /// # Returns
    /// `Ok(())` after formatting; `Err(fmt::Error)` if the destination rejects a write.
    ///
    /// # Errors
    /// Propagates destination formatting failures.
    ///
    /// # Examples
    /// ```
    /// use lspally::rpc::ResponseError;
    /// assert_eq!(ResponseError::new(-1, "bad").to_string(), "RPC error -1: bad");
    /// ```
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "RPC error {}: {}", self.code, self.message)
    }
}
impl Error for ResponseError {}
impl fmt::Display for DecodeError {
    /// Write a correlation reason or nested error description; no other side effects.
    ///
    /// # Arguments
    /// - `self` `[in]`: error details borrowed without mutation.
    /// - `f` `[in,out]`: formatter receiving appended text; existing sink content is not cleared.
    ///
    /// # Returns
    /// `Ok(())` after formatting; `Err(fmt::Error)` if the destination rejects a write.
    ///
    /// # Errors
    /// Propagates destination formatting failures.
    ///
    /// # Examples
    /// ```
    /// use lspally::rpc::DecodeError;
    /// assert!(DecodeError::MethodMismatch.to_string().contains("method"));
    /// ```
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MethodMismatch => write!(f, "RPC method does not match the selected LSP type"),
            Self::IdMismatch => write!(f, "Response ID does not match the request"),
            Self::Remote(error) => fmt::Display::fmt(error, f),
            Self::InvalidParams(error) => write!(f, "Invalid RPC parameters: {error}"),
            Self::InvalidResult(error) => write!(f, "Invalid RPC result: {error}"),
        }
    }
}
impl Error for DecodeError {
    /// Borrow the nested remote/JSON cause without mutation or I/O.
    ///
    /// # Arguments
    /// - `self` `[in]`: decoding error whose cause is inspected.
    ///
    /// # Returns
    /// `Some(cause)` for remote, parameter and result failures; `None` for method/ID mismatches.
    ///
    /// # Examples
    /// ```
    /// use lspally::rpc::DecodeError;
    /// use std::error::Error;
    /// assert!(DecodeError::IdMismatch.source().is_none());
    /// ```
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Remote(error) => Some(error),
            Self::InvalidParams(error) | Self::InvalidResult(error) => Some(error),
            Self::MethodMismatch | Self::IdMismatch => None,
        }
    }
}
