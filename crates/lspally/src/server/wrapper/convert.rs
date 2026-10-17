// AI-generated implementation.
//! Adapt business records to typed LSP values and back without file/network I/O.
//! URI syntax and JSON number checks happen here, not in the business models.

use crate::server::model as d;
use d::{ServerError, ServerResult};
mod call_hierarchy;
pub(super) use call_hierarchy::*;
mod code_action;
pub(super) use code_action::*;
mod code_lens;
pub(super) use code_lens::*;
mod color;
pub(super) use color::*;
mod common;
pub(super) use common::*;
mod completion;
pub(super) use completion::*;
mod document_diagnostic;
pub(super) use document_diagnostic::*;
mod document_highlight;
pub(super) use document_highlight::*;
mod document_link;
pub(super) use document_link::*;
mod document_symbols;
pub(super) use document_symbols::*;
mod file_operations;
pub(super) use file_operations::*;
mod folding_range;
pub(super) use folding_range::*;
mod formatting;
pub(super) use formatting::*;
mod hover;
pub(super) use hover::*;
mod inlay_hint;
pub(super) use inlay_hint::*;
mod inline_value;
pub(super) use inline_value::*;
mod linked_editing;
pub(super) use linked_editing::*;
mod moniker;
pub(super) use moniker::*;
mod notebook;
pub(super) use notebook::*;
mod progress;
pub(super) use progress::*;
mod references;
pub(super) use references::*;
mod rename;
pub(super) use rename::*;
mod selection_range;
pub(super) use selection_range::*;
mod semantic_tokens;
pub(super) use semantic_tokens::*;
mod signature_help;
pub(super) use signature_help::*;
mod trace;
pub(super) use trace::*;
mod type_hierarchy;
pub(super) use type_hierarchy::*;
mod window;
pub(super) use window::*;
mod workspace_diagnostic;
pub(super) use workspace_diagnostic::*;
mod workspace_folders;
pub(super) use workspace_folders::*;
mod workspace_symbols;
pub(super) use workspace_symbols::*;
/// Wrap a serialization or response-decoding failure as InternalError; no I/O.
///
/// # Arguments
/// - `e` `[in]`: failure whose display text becomes the message.
/// # Returns
/// Error -32603 with the displayed explanation; no output parameters.
/// # Examples
/// ```text
/// let error = codec_error("malformed response");
/// assert_eq!(error.code, -32603);
/// ```
pub(super) fn codec_error(e: impl std::fmt::Display) -> ServerError {
    ServerError::new(-32603, e.to_string())
}
/// Copy an already validated wire URI into the business representation; no I/O.
///
/// # Arguments
/// - `v` `[in]`: parsed wire URI consumed by the adapter.
/// # Returns
/// Always `Ok(uri)` containing the URI's original spelling; no resource is opened.
/// # Examples
/// ```text
/// let uri = decode_uri("file:///tmp/a".parse().unwrap())?;
/// assert_eq!(uri.0, "file:///tmp/a");
/// ```
pub(super) fn decode_uri(v: lsp_types::Uri) -> ServerResult<d::Uri> {
    Ok(d::Uri(v.as_str().into()))
}
/// Validate business URI text for transmission; no resource or network access.
///
/// # Arguments
/// - `v` `[in]`: owned URI text consumed by parsing.
/// # Returns
/// `Ok(uri)` for syntactically valid URI text; `Err(error)` for invalid text.
/// No output parameters are mutated.
/// # Errors
/// Invalid URI syntax becomes -32603 with an `Invalid output URI` explanation.
/// # Examples
/// ```text
/// let wire = encode_uri(d::Uri("file:///tmp/a".into()))?;
/// assert_eq!(wire.as_str(), "file:///tmp/a");
/// ```
pub(super) fn encode_uri(v: d::Uri) -> ServerResult<lsp_types::Uri> {
    v.0.parse()
        .map_err(|e| ServerError::new(-32603, format!("Invalid output URI: {e}")))
}
/// Consume a JSON tree into business data, preserving nested values; no I/O.
///
/// # Arguments
/// - `v` `[in]`: owned JSON tree; object entries become a sorted map.
/// # Returns
/// `Ok(data)` for representable values, or `Err(error)` if a number cannot be read.
/// No output parameters or shared state are changed.
/// # Errors
/// A number with no signed, unsigned or floating representation yields InvalidParams.
/// Nested conversion failures propagate without returning a partial tree.
/// # Algorithm
/// Recursively consume arrays and objects. Prefer signed integers, then unsigned
/// integers, then floating-point numbers; do not round whole numbers through `f64`.
/// # Flow
/// ```text
///          [JSON value]
///                |
///         <value shape?>
///         /      |       \\
///    scalar    number   container
///      |         |          |
///   [copy]  [i64/u64/f64] [convert children]
///      |         |          |
///      +---------+----------+
///                v
///        [Data or first error]
/// ```
/// # Examples
/// ```text
/// let data = data_from_wire(serde_json::json!([1, true]))?;
/// assert!(matches!(data, d::Data::List(values) if values.len() == 2));
/// ```
pub(super) fn data_from_wire(v: serde_json::Value) -> ServerResult<d::Data> {
    use serde_json::Value;
    Ok(match v {
        Value::Null => d::Data::Null,
        Value::Bool(v) => d::Data::Bool(v),
        Value::String(v) => d::Data::Text(v),
        Value::Number(v) => {
            if let Some(i) = v.as_i64() {
                d::Data::Integer(i)
            } else if let Some(i) = v.as_u64() {
                d::Data::Unsigned(i)
            } else {
                d::Data::Float(
                    v.as_f64()
                        .ok_or_else(|| ServerError::invalid("Invalid number"))?,
                )
            }
        }
        Value::Array(v) => d::Data::List(
            v.into_iter()
                .map(data_from_wire)
                .collect::<ServerResult<_>>()?,
        ),
        Value::Object(v) => d::Data::Object(
            v.into_iter()
                .map(|(k, v)| Ok((k, data_from_wire(v)?)))
                .collect::<ServerResult<_>>()?,
        ),
    })
}
/// Consume business data into a JSON tree, rejecting non-finite floats; no I/O.
///
/// # Arguments
/// - `v` `[in]`: owned payload, including nested lists and objects.
/// # Returns
/// `Ok(value)` containing the complete JSON tree, or `Err(error)` on invalid numbers.
/// No output parameters or shared state are changed.
/// # Errors
/// NaN and infinity yield -32603 (`Non-finite output number`), including inside
/// containers. A failed conversion does not expose a partially converted tree.
/// # Algorithm
/// Match scalars directly and recursively collect lists and objects. Use JSON's
/// finite-number constructor for floating-point values instead of stringifying them.
/// # Flow
/// ```text
///           [Data value]
///                 |
///          <value shape?>
///          /      |       \\
///      scalar   Float    List/Object
///        |        |           |
///      [copy] <finite?>  [convert children]
///        |     /    \\        |
///        |   yes    no        |
///        |    |    [error]    |
///        +----+---------------+
///             v
///       [complete JSON]
/// ```
/// # Examples
/// ```text
/// assert_eq!(data_to_wire(d::Data::Integer(42))?, serde_json::json!(42));
/// assert!(data_to_wire(d::Data::Float(f64::NAN)).is_err());
/// ```
pub(super) fn data_to_wire(v: d::Data) -> ServerResult<serde_json::Value> {
    use serde_json::Value;
    Ok(match v {
        d::Data::Null => Value::Null,
        d::Data::Bool(v) => Value::Bool(v),
        d::Data::Text(v) => Value::String(v),
        d::Data::Integer(v) => Value::Number(v.into()),
        d::Data::Unsigned(v) => Value::Number(v.into()),
        d::Data::Float(v) => Value::Number(
            serde_json::Number::from_f64(v)
                .ok_or_else(|| ServerError::new(-32603, "Non-finite output number"))?,
        ),
        d::Data::List(v) => Value::Array(
            v.into_iter()
                .map(data_to_wire)
                .collect::<ServerResult<_>>()?,
        ),
        d::Data::Object(v) => Value::Object(
            v.into_iter()
                .map(|(k, v)| Ok((k, data_to_wire(v)?)))
                .collect::<ServerResult<_>>()?,
        ),
    })
}
