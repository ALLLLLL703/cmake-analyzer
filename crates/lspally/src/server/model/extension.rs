// AI-generated implementation.
/// Extensions alone may expose raw JSON. The Wrapper retains the RPC envelope.
#[derive(Debug)]
pub struct Extension {
    pub method: String,
    pub params: Option<serde_json::Value>,
}
