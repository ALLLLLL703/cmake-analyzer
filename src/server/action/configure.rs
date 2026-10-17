//! Explicit project configuration entrypoint; delegates saved-input checks and tracked child execution
//! to the service.

// AI-authored explicit configure action entrypoint.
use super::{Service, model::ActionRequest};
use lspally::server::model::*;
/// Start tracked CMake configuration for the validated -S/-B project; executes arbitrary project code
/// and may write build files.
/// The service refreshes changed closed inputs and requires related editor text to match disk.
/// Configure rejects a build directory equal to or canonically resolving to the source directory,
/// and a build path lexically equal to another live job's build directory.
/// The process-wide max_workspaces job cap and project timeout_seconds/max_output_bytes budgets
/// apply; max_view_bytes bounds failure details. The runtime process kills and waits for the direct
/// child on cancellation, timeout or output/read failure; descendants are not recursively managed.
/// A start reply precedes completion. Installation later requires the current run ID/profile and saved
/// inputs; invalidation discards stale completions. Request cancellation after admission is not a
/// live-job stop.
///
/// # Arguments
/// - `service` `[in,out]`: service; refreshes analysis, increments the run counter, appends a tracked job
///   and inserts its cancellation handle, then notifies the client.
/// - `request` `[in]`: owner/root/profile selector and pre-admission cancellation token.
///
/// # Returns
/// Ok(Some(object)) with started=true and the unsigned run ID; not the eventual trace. Never Ok(None).
///
/// # Errors
/// Ownership/mode/enable failure, unsaved or unreadable inputs, exhausted job/run-ID budget, invalid
/// build directory, cancellation before admission or notification failure. Notification failure can
/// occur after the job has started.
///
/// # Algorithm
/// Delegate to Service::start_runtime for admission and background execution; later completion handling
/// installs observations separately.
///
/// # Flow
/// ```text
///  [Validate + refresh inputs]
///              |
///        [Saved + budget?] --no--> [Reject]
///              | yes
///       [Track independent job] ---> [Start reply]
///              |
///       [Child trace completes]
///              |
///        [Still current?] --no--> [Discard]
///              | yes
///       [Install runtime layer]
/// ```
///
/// # Examples
/// Local server context (requires the initialized service and its client):
/// ```text
/// let started = configure::run(service, request).await?;
/// ```
pub(super) async fn run(
    service: &mut Service,
    request: ActionRequest,
) -> ServerResult<Option<Data>> {
    service.start_runtime(request).await
}
