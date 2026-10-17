mod context;
mod discovery;
pub(crate) mod model;
mod targets;
use super::Workspace;
use std::{io, path::Path};

fn check(stop: &impl Fn() -> bool) -> io::Result<()> {
    if stop() {
        Err(io::Error::new(
            io::ErrorKind::Interrupted,
            "Completion cancelled",
        ))
    } else {
        Ok(())
    }
}
impl Workspace {
    pub(crate) async fn completion_catalog(
        &mut self,
        path: &Path,
        request: &model::Request,
        stop: impl Fn() -> bool + Send + Sync + Clone + 'static,
    ) -> io::Result<model::Catalog> {
        check(&stop)?;
        let mut result = model::Catalog::default();
        if (request.modules || request.packages) && self.module_lookup_enabled {
            let contexts = context::capture(self, path, request.command, stop.clone()).await?;
            result = discovery::collect(self, path, &contexts, request, &stop).await?;
        }
        if request.targets {
            result.targets = targets::collect(self, path, &stop).await?;
        }
        Ok(result)
    }
}
