// AI-generated implementation.

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
pub(super) fn codec_error(e: impl std::fmt::Display) -> ServerError {
    ServerError::new(-32603, e.to_string())
}
pub(super) fn decode_uri(v: lsp_types::Uri) -> ServerResult<d::Uri> {
    Ok(d::Uri(v.as_str().into()))
}
pub(super) fn encode_uri(v: d::Uri) -> ServerResult<lsp_types::Uri> {
    v.0.parse()
        .map_err(|e| ServerError::new(-32603, format!("Invalid output URI: {e}")))
}
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
