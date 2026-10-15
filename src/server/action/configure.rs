// AI-authored explicit configure action entrypoint.
use super::{Service, model::ActionRequest};
use lspally::server::model::*;
pub(super) async fn run(
    service: &mut Service,
    request: ActionRequest,
) -> ServerResult<Option<Data>> {
    service.start_configure(request).await
}
