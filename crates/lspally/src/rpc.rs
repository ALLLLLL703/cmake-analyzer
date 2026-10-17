// Contains AI-generated additions.
//! JSON-RPC ownership, typed LSP conversion, validation and frame formatting.
//!
//! [`Request`] retains IDs/methods while moving parameter trees into typed handlers.
//! [`Request::decode_response`] checks correlation but does not track pending calls.
//! [`display::to_lsp_string`] builds bytes-as-text without touching streams.
pub mod display;
mod methods;
pub mod model;
mod serde;

pub use model::{
    DecodeError, Message, Notification, Params, Request, RequestId, Response, ResponseError,
    ResponsePayload, Version,
};
