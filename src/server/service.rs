// AI-authored bounded Workspace service.
mod configuration;
mod documents;
mod projects;
mod reports;
mod runtime;
use super::model::*;
use lspally::server::model::*;
use tokio::sync::{mpsc, oneshot};

static SERVICE_QUEUE_CAPACITY: usize = 64;
static EDIT_BATCH_SIZE: usize = 64;

impl CmakeAnalyzer {
    pub(super) fn sender(&self) -> ServerResult<mpsc::Sender<ServiceCommand>> {
        self.state
            .lock()
            .map_err(|_| ServerError::new(-32603, "Server state poisoned"))?
            .service
            .as_ref()
            .map(|service| service.sender.clone())
            .ok_or_else(|| ServerError::new(-32002, "Workspace service is not running"))
    }
    pub(super) async fn send(&self, command: ServiceCommand) -> ServerResult<()> {
        self.sender()?
            .send(command)
            .await
            .map_err(|_| ServerError::new(-32603, "Workspace service stopped"))
    }
    pub(super) async fn query_symbols(
        &self,
        context: Context,
        uri: Uri,
    ) -> ServerResult<Option<DocumentSymbolResponse>> {
        let (reply, response) = oneshot::channel();
        tokio::select! {
            _=context.cancellation.cancelled()=>Err(ServerError::cancelled()),
            result=async {self.send(ServiceCommand::Symbols{uri,reply}).await?;response.await.map_err(|_|ServerError::new(-32603,"Workspace service stopped"))?}=>result,
        }
    }
    /// Stop and join the worker on shutdown or after Wrapper::run returns on EOF.
    pub async fn stop(&self) -> ServerResult<()> {
        let service = self
            .state
            .lock()
            .map_err(|_| ServerError::new(-32603, "Server state poisoned"))?
            .service
            .take();
        if let Some(mut service) = service {
            service.stop.cancel();
            (&mut service.task)
                .await
                .map_err(|e| ServerError::new(-32603, e.to_string()))?;
        }
        Ok(())
    }
    pub(super) fn start(&self, context: Context) -> ServerResult<()> {
        let mut state = self
            .state
            .lock()
            .map_err(|_| ServerError::new(-32603, "Server state poisoned"))?;
        if state.service.is_some() {
            return Err(ServerError::invalid("Workspace service already started"));
        }
        let (sender, receiver) = mpsc::channel(SERVICE_QUEUE_CAPACITY);
        let stop = Cancellation::default();
        let roots = std::mem::take(&mut state.roots);
        let hierarchical = state.hierarchical;
        let config = state
            .config
            .take()
            .ok_or_else(|| ServerError::new(-32002, "Server is not initialized"))?;
        let task = tokio::spawn(runtime::run(
            roots,
            config,
            context,
            hierarchical,
            receiver,
            stop.clone(),
        ));
        state.service = Some(ServiceHandle { sender, stop, task });
        Ok(())
    }
}
pub(super) fn internal(error: impl std::fmt::Display) -> ServerError {
    ServerError::new(-32603, error.to_string())
}
