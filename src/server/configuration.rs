//! Decode the `cmake-analyzer/reloadConfig` extension and await its ordered reply.
//! An omitted path reuses existing sources; a supplied path replaces the explicit
//! configuration file on successful reload. Standard configuration notifications are
//! not handled here. This adapter itself does not mutate project state.
// AI-authored custom configuration RPC.
use super::model::*;
use lspally::server::model::*;
use serde::Deserialize;
use std::path::PathBuf;
use tokio::sync::oneshot;

/// Strict extension payload consumed by [`CmakeAnalyzer::custom_server_request`].
/// Omitting all fields uses current sources; unknown fields are rejected by serde.
/// Decoding this record alone performs no I/O or reload.
///
/// # Examples
/// ```text
/// let params = ReloadParams { path: None };
/// ```
#[derive(Default, Deserialize)]
#[serde(default, deny_unknown_fields)]
struct ReloadParams {
    /// Replacement explicit configuration source, or None to retain the current one.
    /// Example: `ReloadParams { path: Some("/work/settings.toml".into()) }`.
    path: Option<PathBuf>,
}

impl CmakeAnalyzer {
    /// Decode the custom reload request, enqueue a configuration barrier and serialize its reply.
    /// This adapter does not directly replace live configuration.
    ///
    /// # Arguments
    /// - `self` `[in]`: Adapter whose lifecycle handle addresses the shared service; not mutated directly.
    /// - `context` `[in]`: Request context consumed for cancellation and/or client session data.
    /// - `params` `[in]`: Consumed extension method and optional JSON object; only cmake-analyzer/reloadConfig is accepted.
    ///
    /// # Returns
    /// `Ok` with {"reloaded": true, "workspaces": count} after a successful actor reload; otherwise `Err`.
    ///
    /// # Errors
    /// Unsupported method, invalid/unknown JSON fields, cancellation, unavailable/stopped service, or errors from reload_config.
    ///
    /// # Algorithm
    /// Validate the extension name and strict payload before creating a reply channel; race the ordered service reply against request cancellation.
    ///
    /// # Flow
    /// ```text
    ///  [Extension + JSON]
    ///           |
    ///      <Valid reload?>
    ///       /          \
    ///      no          yes
    ///      |            |
    ///   [Error]     [Queue barrier]
    ///                   |
    ///            [Reply or cancellation]
    /// ```
    ///
    /// # Examples
    /// ```text
    /// let value = server.custom_server_request(context, Extension {
    ///     method: "cmake-analyzer/reloadConfig".into(), params: None,
    /// }).await?;
    /// ```
    pub(super) async fn custom_server_request(
        &self,
        context: Context,
        params: Extension,
    ) -> ServerResult<serde_json::Value> {
        if params.method != "cmake-analyzer/reloadConfig" {
            return Err(ServerError::unsupported());
        }
        let params: ReloadParams = match params.params {
            None => ReloadParams::default(),
            Some(value) => serde_json::from_value(value)
                .map_err(|error| ServerError::invalid(error.to_string()))?,
        };
        let (reply, response) = oneshot::channel();
        tokio::select! {
            _=context.cancellation.cancelled()=>Err(ServerError::cancelled()),
            result=async {
                self.send(ServiceCommand::ReloadConfig {path:params.path,reply}).await?;
                let workspaces=response.await.map_err(|_|ServerError::new(-32603,"Workspace service stopped"))??;
                Ok(serde_json::json!({"reloaded":true,"workspaces":workspaces}))
            }=>result,
        }
    }
}
