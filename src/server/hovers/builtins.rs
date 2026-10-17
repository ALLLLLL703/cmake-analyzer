//! Cache local CMake help catalogs and canonical bodies, including unavailable results.
//! Only help flags are used; project executable, arguments, enable and mode settings are ignored.
//! A successful configuration reload replaces this cache; a failed reload preserves it.

pub(super) mod model;
mod names;
mod process;
use lspally::server::model::{Cancellation, ServerResult};
pub(super) use model::Kind;
use model::LocalHelp;
use std::sync::Arc;

impl LocalHelp {
    /// Resolve startup-PATH CMake once and read help-only output under global budgets; may inspect
    /// filesystem metadata and start a child. PATH discovery precedes the child's timeout/cancellation
    /// race; discovery itself does not use that timeout.
    ///
    /// # Arguments
    /// - `self` `[in]`: cache with an interior-mutable executable cell; discovery result is initialized
    ///   once.
    /// - `args` `[in]`: internal fixed help flag and optional canonical catalog name; no project inputs.
    /// - `cancel` `[in]`: request token forwarded to child inspection.
    ///
    /// # Returns
    /// Ok(Some(text)) for successful nonblank UTF-8 help; Ok(None) if discovery or inspection is
    /// unavailable.
    ///
    /// # Errors
    /// Cancellation reported by process inspection. If executable discovery returns None, the method
    /// returns None without polling cancellation itself.
    ///
    /// # Examples
    /// Local server context:
    /// ```text
    /// let text = help.read(&["--help-command-list"], &cancel).await?;
    /// ```
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
    /// Load and remember a local canonical-name catalog, including unavailability; may run one help-list
    /// child.
    ///
    /// # Arguments
    /// - `self` `[in,out]`: inserts the namespace result into catalogs after successful inspection;
    ///   cancellation is not cached.
    /// - `kind` `[in]`: command or variable list namespace.
    /// - `cancel` `[in]`: request cancellation passed to inspection.
    ///
    /// # Returns
    /// Ok(Some(shared sorted names)) or Ok(None) for unavailable list help; cached results are reused.
    ///
    /// # Errors
    /// Cancellation during an uncached inspection.
    ///
    /// # Algorithm
    /// Reuse an existing entry; otherwise trim lines, reject blank/whitespace-containing names, sort,
    /// deduplicate and store the optional list.
    ///
    /// # Flow
    /// ```text
    ///  [Namespace]
    ///       |
    ///  [Cached?] --yes--> [Clone optional list]
    ///       | no
    ///  [Run help-list]
    ///       |
    ///  [Trim + sort + deduplicate]
    ///       |
    ///  [Store result] ---> [Return shared list]
    /// ```
    ///
    /// # Examples
    /// Local server context:
    /// ```text
    /// let commands = help.catalog(Kind::Command, &cancel).await?;
    /// ```
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
    /// Resolve source spelling to canonical local help and cache its body; never execute a project.
    /// Command names are lowercased. ARGC/ARGV/ARGN and numeric ARGV tails use function help; numeric
    /// CMAKE_ARGV tails use CMAKE_ARGV0. Unknown names never become body-cache keys.
    ///
    /// # Arguments
    /// - `self` `[in,out]`: may initialize executable, catalogs and documents and increase cached_bytes;
    ///   no eviction or source mutation.
    /// - `kind` `[in]`: requested namespace; local normalization may redirect it to Command.
    /// - `name` `[in]`: source identifier to normalize and match.
    /// - `cancel` `[in]`: request cancellation for uncached child work; cache hits do not poll it.
    ///
    /// # Returns
    /// Ok(Some(shared body)) if available, including an uncached body when the aggregate cap is full;
    /// Ok(None) for unknown names or unavailable help.
    ///
    /// # Errors
    /// Cancellation during uncached inspection; unavailable help is represented as None, not an error.
    ///
    /// # Algorithm
    /// Normalize special names, resolve an exact or most-literal template catalog key, and reuse cached
    /// bodies. On a miss, request the canonical body; cache it only if cached_bytes plus body length
    /// fits max_output_bytes. None costs zero bytes.
    ///
    /// # Flow
    /// ```text
    ///  [Normalize identifier]
    ///           |
    ///  [Catalog match?] --no--> [None]
    ///           | yes
    ///  [Body cached?] --yes--> [Return cached result]
    ///           | no
    ///  [Read canonical help]
    ///           |
    ///  [Budget fits?] --no--> [Return uncached]
    ///           | yes
    ///  [Insert + count bytes] ---> [Return result]
    /// ```
    ///
    /// # Examples
    /// Local server context:
    /// ```text
    /// let doc = help.documentation(Kind::Command, "SET", &cancel).await?;
    /// ```
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

// Contains AI-generated test-module documentation edits.
#[cfg(all(test, unix))]
#[path = "../../../tests/unit/server/local_help.rs"]
/// Unix private-cache tests loaded from the external test file; compiling this module has no runtime
/// side effects.
mod tests;
