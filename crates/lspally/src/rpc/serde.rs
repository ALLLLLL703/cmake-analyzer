// AI-generated implementation.
//! Wire adapters preserving field presence and enforcing envelope shapes; no stream I/O.
//!
//! A missing field differs from an explicit null result or data value. Message
//! classification uses keys, never a fallback that could turn a bad request into a notification.
use super::model::{
    Message, Notification, NotificationFields, RequestId, Response, ResponseFields, ResponsePayload,
};
use serde::{
    Deserialize, Deserializer, Serialize, Serializer,
    de::Error as _,
    ser::{Error as _, SerializeStruct},
};
use serde_json::Value;

/// Decode a present field into `Some`, preserving explicit null when `T` accepts it; no I/O.
/// Serde's field default, not this function, represents absence as `None`.
///
/// # Arguments
/// - `deserializer` `[in]`: field decoder consumed to obtain `T`.
///
/// # Returns
/// `Ok(Some(value))` on success, never `Ok(None)`; `Err` if `T` rejects the field.
///
/// # Errors
/// Propagates `T`'s deserialization error (for example null for a non-nullable error record).
///
/// # Examples
/// ```text
/// let mut decoder = serde_json::Deserializer::from_str("null");
/// assert_eq!(present::<_, serde_json::Value>(&mut decoder)?, Some(serde_json::Value::Null));
/// ```
pub(super) fn present<'de, D, T>(deserializer: D) -> Result<Option<T>, D::Error>
where
    D: Deserializer<'de>,
    T: Deserialize<'de>,
{
    T::deserialize(deserializer).map(Some)
}
/// Decode a required reply ID, allowing null; no external side effects.
/// The field has no serde default, so omission remains an error in `ResponseFields`.
///
/// # Arguments
/// - `deserializer` `[in]`: decoder consumed for a number, string or null ID.
///
/// # Returns
/// `Ok(Some(id))` for a numeric/string key, `Ok(None)` for null; `Err` for another shape.
///
/// # Errors
/// Propagates ID decoding failures, including numbers outside the underlying integer range.
///
/// # Examples
/// ```text
/// let mut decoder = serde_json::Deserializer::from_str("1");
/// assert_eq!(response_id(&mut decoder)?, Some(RequestId::Number(1)));
/// ```
pub(super) fn response_id<'de, D: Deserializer<'de>>(
    deserializer: D,
) -> Result<Option<RequestId>, D::Error> {
    Option::<RequestId>::deserialize(deserializer)
}
impl<'de> Deserialize<'de> for Notification {
    /// Decode a one-way envelope and reject any present ID; no I/O or shared-state effects.
    ///
    /// # Arguments
    /// - `deserializer` `[in]`: consumed wire decoder; unknown fields follow the record's serde behavior.
    ///
    /// # Returns
    /// `Ok(notification)` with owned method/parameters; `Err` for malformed fields or a present ID.
    ///
    /// # Errors
    /// Rejects missing/invalid required fields, non-object/array parameters and IDs (even null).
    ///
    /// # Examples
    /// ```
    /// use lspally::rpc::Notification;
    /// use serde::Deserialize;
    /// let mut decoder = serde_json::Deserializer::from_str(r#"{"jsonrpc":"2.0","method":"exit"}"#);
    /// let note = Notification::deserialize(&mut decoder).unwrap();
    /// assert_eq!(note.method, "exit");
    /// ```
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let fields = NotificationFields::deserialize(deserializer)?;
        if fields.id.is_some() {
            return Err(D::Error::custom("A notification must not contain id"));
        }
        Ok(Self {
            jsonrpc: fields.jsonrpc,
            method: fields.method,
            params: fields.params,
        })
    }
}
impl Serialize for Response {
    /// Validate and serialize exactly three reply fields; no stream I/O.
    ///
    /// # Arguments
    /// - `self` `[in]`: borrowed reply; never mutated.
    /// - `serializer` `[in]`: consumed serialization destination; writes version, ID and one payload.
    ///
    /// # Returns
    /// `Ok(S::Ok)` after the serializer ends its record; `Err` on validation or destination failure.
    ///
    /// # Errors
    /// Rejects successful replies with null IDs before starting a record. Other serializer errors
    /// can occur after partial destination output; no rollback is attempted.
    ///
    /// # Algorithm
    /// Validate the success ID, start a three-field record, then select result or error by payload.
    ///
    /// # Flow
    /// ```text
    ///              <success with null ID?>
    ///                 /            \
    ///               yes            no
    ///                |              |
    ///             [error]     [write version + ID]
    ///                               |
    ///                           <payload?>
    ///                          /          \
    ///                   [write result] [write error]
    ///                          \          /
    ///                            [end record]
    /// ```
    /// # Examples
    /// ```
    /// use lspally::rpc::{Response, ResponseError};
    /// use serde::Serialize;
    /// let reply = Response::error(None, ResponseError::new(-32700, "Invalid JSON"));
    /// let mut bytes = Vec::new();
    /// reply.serialize(&mut serde_json::Serializer::new(&mut bytes)).unwrap();
    /// assert!(String::from_utf8(bytes).unwrap().contains("\"error\""));
    /// ```
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        if self.id.is_none() && matches!(self.payload, ResponsePayload::Result(_)) {
            return Err(S::Error::custom(
                "A success response must have a request ID",
            ));
        }
        let mut fields = serializer.serialize_struct("Response", 3)?;
        fields.serialize_field("jsonrpc", &self.jsonrpc)?;
        fields.serialize_field("id", &self.id)?;
        match &self.payload {
            ResponsePayload::Result(value) => fields.serialize_field("result", value)?,
            ResponsePayload::Error(error) => fields.serialize_field("error", error)?,
        }
        fields.end()
    }
}
impl<'de> Deserialize<'de> for Response {
    /// Decode a reply with one exclusive payload and a valid ID; no I/O or shared mutation.
    /// Explicit null is a present successful result, not a missing field.
    ///
    /// # Arguments
    /// - `deserializer` `[in]`: consumed reply decoder.
    ///
    /// # Returns
    /// `Ok(response)` with an owned result or error; `Err` for invalid fields or payload combinations.
    ///
    /// # Errors
    /// Rejects both/neither result and error, malformed error records, missing IDs and null success IDs.
    ///
    /// # Algorithm
    /// Decode presence-preserving temporary fields, then select one payload; success requires a key.
    ///
    /// # Flow
    /// ```text
    ///           [result / error presence]
    ///                     |
    ///              <exactly one?>
    ///               /          \
    ///             no           yes
    ///             |             |
    ///          [error]     <success + null ID?>
    ///                        /          \
    ///                      yes          no
    ///                       |            |
    ///                    [error]     [owned reply]
    /// ```
    /// # Examples
    /// ```
    /// use lspally::rpc::{Response, ResponsePayload};
    /// use serde::Deserialize;
    /// let mut decoder = serde_json::Deserializer::from_str(r#"{"jsonrpc":"2.0","id":1,"result":null}"#);
    /// let reply = Response::deserialize(&mut decoder).unwrap();
    /// assert!(matches!(reply.payload, ResponsePayload::Result(serde_json::Value::Null)));
    /// ```
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let fields = ResponseFields::deserialize(deserializer)?;
        let payload = match (fields.result, fields.error) {
            (Some(result), None) if fields.id.is_some() => ResponsePayload::Result(result),
            (None, Some(error)) => ResponsePayload::Error(error),
            (Some(_), None) => {
                return Err(D::Error::custom(
                    "A success response must have a request ID",
                ));
            }
            _ => {
                return Err(D::Error::custom(
                    "A response needs exactly one of result or error",
                ));
            }
        };
        Ok(Self {
            jsonrpc: fields.jsonrpc,
            id: fields.id,
            payload,
        })
    }
}
impl<'de> Deserialize<'de> for Message {
    /// Classify by field presence, then decode the chosen envelope; no I/O or shared mutation.
    /// Malformed requests are never retried as notifications; JSON batches are unsupported.
    ///
    /// # Arguments
    /// - `deserializer` `[in]`: consumed decoder for one complete JSON object.
    ///
    /// # Returns
    /// `Ok(Message)` owning the classified envelope; `Err` if shape or envelope validation fails.
    ///
    /// # Errors
    /// Rejects non-objects, method messages containing result/error, responses containing params,
    /// and nested envelope failures (including null request IDs).
    ///
    /// # Algorithm
    /// Read a JSON value, inspect keys, then move that value into exactly one envelope decoder.
    /// See [`Message`]'s classification diagram; forbidden mixed keys fail before conversion.
    ///
    /// # Flow
    /// ```text
    ///               [JSON value]
    ///                     |
    ///                 <object?>
    ///                 /       \
    ///               no        yes
    ///               |          |
    ///            [error]   <method key?>
    ///                      /          \
    ///                    yes          no
    ///                     |            |
    ///               <id key?>     [Response decode]
    ///                /     \
    ///         [Request] [Notification]
    ///                \     /
    ///             [owned Message or error]
    /// ```
    /// # Examples
    /// ```
    /// use lspally::rpc::Message;
    /// use serde::Deserialize;
    /// let mut decoder = serde_json::Deserializer::from_str(r#"{"jsonrpc":"2.0","method":"exit"}"#);
    /// assert!(matches!(Message::deserialize(&mut decoder).unwrap(), Message::Notification(_)));
    /// ```
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = Value::deserialize(deserializer)?;
        let object = value
            .as_object()
            .ok_or_else(|| D::Error::custom("An RPC message must be an object"))?;
        // Classify by field presence: an invalid request must not fall back to a notification.
        if object.contains_key("method") {
            if object.contains_key("result") || object.contains_key("error") {
                return Err(D::Error::custom(
                    "Requests and notifications must not contain result or error",
                ));
            }
            if object.contains_key("id") {
                serde_json::from_value(value)
                    .map(Self::Request)
                    .map_err(D::Error::custom)
            } else {
                serde_json::from_value(value)
                    .map(Self::Notification)
                    .map_err(D::Error::custom)
            }
        } else {
            if object.contains_key("params") {
                return Err(D::Error::custom("A response must not contain params"));
            }
            serde_json::from_value(value)
                .map(Self::Response)
                .map_err(D::Error::custom)
        }
    }
}
