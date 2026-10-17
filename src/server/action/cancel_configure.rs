//! Cancel a matching queued or running configure/script profile without starting another child.

// AI-authored explicit cancellation action entrypoint.
use super::{Service, model::ActionRequest};
use lspally::server::model::*;
/// Remove matching queued saves and cancel the matching tracked configure/script job; never start a
/// child.
/// Cancelling signals the independent job token; background process cleanup kills and waits for the
/// direct child. This call does not join cleanup or erase an already installed layer.
///
/// # Arguments
/// - `service` `[in,out]`: saved_runs and running entries may be removed; a removed running token is
///   cancelled.
/// - `request` `[in]`: owner/root/profile selector; its own cancellation token is not inspected.
///
/// # Returns
/// Ok(Some(Data::Bool(true))) if a queue entry or running job was removed; false if neither matched.
/// Never None.
///
/// # Errors
/// Invalid URI or no compatible active project owner.
///
/// # Examples
/// Local server context (requires the initialized service and its client):
/// ```text
/// let cancelled = cancel_configure::run(service, request).await?;
/// ```
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
