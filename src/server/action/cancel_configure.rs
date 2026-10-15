// AI-authored explicit cancellation action entrypoint.
use super::{Service, model::ActionRequest};
use lspally::server::model::*;
pub(super) async fn run(
    service: &mut Service,
    request: ActionRequest,
) -> ServerResult<Option<Data>> {
    service.action_project(&request)?;
    let cancelled = if let Some(task) = service.running.remove(&request.root) {
        task.cancel.cancel();
        true
    } else {
        false
    };
    Ok(Some(Data::Bool(cancelled)))
}
