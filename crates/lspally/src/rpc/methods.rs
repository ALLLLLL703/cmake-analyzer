// AI-generated implementation.
use super::model::{
    DecodeError, Notification, Params, Request, RequestId, Response, ResponseError,
    ResponsePayload, Version,
};
use lsp_types::{notification::Notification as LspNotification, request::Request as LspRequest};
use serde::Serialize;
use serde_json::Value;
use std::{error::Error, fmt};

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
fn take_value(params: &mut Option<Params>) -> Value {
    match params.take() {
        Some(Params::Left(value)) => Value::Object(value),
        Some(Params::Right(value)) => Value::Array(value),
        None => Value::Null,
    }
}
impl Request {
    pub fn new<R: LspRequest>(id: RequestId, params: R::Params) -> serde_json::Result<Self> {
        Ok(Self {
            jsonrpc: Version::V2,
            id,
            method: R::METHOD.into(),
            params: encode_params(params)?,
        })
    }
    /// Check the method, then consume the parameters without cloning their JSON tree.
    pub fn take_params<R: LspRequest>(&mut self) -> Result<R::Params, DecodeError> {
        if self.method != R::METHOD {
            return Err(DecodeError::MethodMismatch);
        }
        serde_json::from_value(take_value(&mut self.params)).map_err(DecodeError::InvalidParams)
    }
    /// The caller tracks outstanding requests; method and ID must both match.
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
    pub fn new<N: LspNotification>(params: N::Params) -> serde_json::Result<Self> {
        Ok(Self {
            jsonrpc: Version::V2,
            method: N::METHOD.into(),
            params: encode_params(params)?,
        })
    }
    pub fn take_params<N: LspNotification>(&mut self) -> Result<N::Params, DecodeError> {
        if self.method != N::METHOD {
            return Err(DecodeError::MethodMismatch);
        }
        serde_json::from_value(take_value(&mut self.params)).map_err(DecodeError::InvalidParams)
    }
}
impl Response {
    pub fn success<R: LspRequest>(id: RequestId, result: R::Result) -> serde_json::Result<Self> {
        Ok(Self {
            jsonrpc: Version::V2,
            id: Some(id),
            payload: ResponsePayload::Result(serde_json::to_value(result)?),
        })
    }
}
impl fmt::Display for ResponseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "RPC error {}: {}", self.code, self.message)
    }
}
impl Error for ResponseError {}
impl fmt::Display for DecodeError {
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
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Remote(error) => Some(error),
            Self::InvalidParams(error) | Self::InvalidResult(error) => Some(error),
            Self::MethodMismatch | Self::IdMismatch => None,
        }
    }
}
