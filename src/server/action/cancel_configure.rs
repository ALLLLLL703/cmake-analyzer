// AI-authored explicit cancellation action entrypoint.
use super::{Service, model::ActionRequest};
use lspally::server::model::*;
pub(super) async fn run(
    service: &mut Service,
    request: ActionRequest,
) -> ServerResult<Option<Data>> {
    let owner = service.action_project(&request)?;
    let queued = service
        .saved_runs
        .get(&request.root)
        .is_some_and(|saved| service.projects[owner].execution.as_ref() == Some(&saved.mode));
    if queued {
        service.saved_runs.remove(&request.root);
    }
    if !service
        .running
        .get(&request.root)
        .is_some_and(|task| service.projects[owner].execution.as_ref() == Some(&task.mode))
    {
        return Ok(Some(Data::Bool(queued)));
    }
    let cancelled = if let Some(task) = service.running.remove(&request.root) {
        task.cancel.cancel();
        true
    } else {
        false
    };
    Ok(Some(Data::Bool(cancelled || queued)))
}
