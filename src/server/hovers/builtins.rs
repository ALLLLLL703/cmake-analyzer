pub(super) mod model;
mod names;
mod process;
use lspally::server::model::{Cancellation, ServerResult};
pub(super) use model::Kind;
use model::LocalHelp;
use std::sync::Arc;

impl LocalHelp {
    async fn read(&self, args: &[&str], cancel: &Cancellation) -> ServerResult<Option<String>> {
        let executable = self
            .executable
            .get_or_init(|| crate::config::cmake_executable(&self.cwd))
            .await;
        let Some(executable) = executable else {
            return Ok(None);
        };
        process::read(executable, &self.cwd, args, &self.limits, cancel).await
    }
    async fn catalog(
        &mut self,
        kind: Kind,
        cancel: &Cancellation,
    ) -> ServerResult<Option<Arc<Vec<String>>>> {
        if let Some(catalog) = self.catalogs.get(&kind) {
            return Ok(catalog.clone());
        }
        let text = self.read(&[kind.list_flag()], cancel).await?;
        let catalog = text.map(|text| {
            let mut names: Vec<_> = text
                .lines()
                .map(str::trim)
                .filter(|name| !name.is_empty() && !name.chars().any(char::is_whitespace))
                .map(str::to_owned)
                .collect();
            names.sort_unstable();
            names.dedup();
            Arc::new(names)
        });
        self.catalogs.insert(kind, catalog.clone());
        Ok(catalog)
    }
    /// Cache canonical local names, not arbitrary source spellings or unknown identifiers.
    pub(super) async fn documentation(
        &mut self,
        mut kind: Kind,
        name: &str,
        cancel: &Cancellation,
    ) -> ServerResult<Option<Arc<str>>> {
        let name = if kind == Kind::Command {
            name.to_ascii_lowercase()
        } else if ["ARGC", "ARGV", "ARGN"].contains(&name) || names::numeric_tail(name, "ARGV") {
            kind = Kind::Command;
            "function".into()
        } else if names::numeric_tail(name, "CMAKE_ARGV") {
            "CMAKE_ARGV0".into()
        } else {
            name.to_owned()
        };
        let Some(catalog) = self.catalog(kind, cancel).await? else {
            return Ok(None);
        };
        let canonical = match kind {
            Kind::Command => catalog
                .binary_search(&name)
                .ok()
                .map(|index| catalog[index].as_str()),
            Kind::Variable => names::variable(&catalog, &name),
        };
        let Some(canonical) = canonical else {
            return Ok(None);
        };
        let key = (kind, canonical.to_owned());
        if let Some(document) = self.documents.get(&key) {
            return Ok(document.clone());
        }
        let document = self
            .read(&[kind.help_flag(), canonical], cancel)
            .await?
            .map(Arc::<str>::from);
        let bytes = document.as_ref().map_or(0, |document| document.len());
        if bytes
            <= self
                .limits
                .max_output_bytes
                .saturating_sub(self.cached_bytes)
        {
            self.cached_bytes += bytes;
            self.documents.insert(key, document.clone());
        }
        Ok(document)
    }
}

#[cfg(all(test, unix))]
#[path = "../../../tests/unit/server/local_help.rs"]
mod tests;
