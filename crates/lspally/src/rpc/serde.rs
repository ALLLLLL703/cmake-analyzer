// AI-generated implementation.
use super::model::{
    Message, Notification, NotificationFields, RequestId, Response, ResponseFields, ResponsePayload,
};
use serde::{
    Deserialize, Deserializer, Serialize, Serializer,
    de::Error as _,
    ser::{Error as _, SerializeStruct},
};
use serde_json::Value;

pub(super) fn present<'de, D, T>(deserializer: D) -> Result<Option<T>, D::Error>
where
    D: Deserializer<'de>,
    T: Deserialize<'de>,
{
    T::deserialize(deserializer).map(Some)
}
pub(super) fn response_id<'de, D: Deserializer<'de>>(
    deserializer: D,
) -> Result<Option<RequestId>, D::Error> {
    Option::<RequestId>::deserialize(deserializer)
}
impl<'de> Deserialize<'de> for Notification {
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
