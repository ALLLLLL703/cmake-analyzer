// AI-authored custom configuration RPC.
use super::model::*;
use lspally::server::model::*;
use serde::Deserialize;
use std::path::PathBuf;
use tokio::sync::oneshot;

#[derive(Default, Deserialize)]
#[serde(default, deny_unknown_fields)]
struct ReloadParams {
    path: Option<PathBuf>,
}

impl CmakeAnalyzer {
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
