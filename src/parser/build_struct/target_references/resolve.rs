//! Resolve unique visible target names and aliases, then normalize reference lists.
//! Directory-local imports shadow farther declarations; equal-distance ties stay unbound. No I/O.
// Contains AI-generated documentation additions.
use super::model::{TargetCollector, TargetDirectoryId, TargetEntryId};
use crate::parser::{
    build_struct::model::{TargetDeclaration, TargetKind},
    semanticer::schema::NameRole,
};

/// Test whether a declaration class satisfies a schema name role. No side effects.
///
/// # Arguments
/// - `class` `[in]`: Constructor class; aliases have their original constructor category.
/// - `role` `[in]`: Required schema category.
///
/// # Returns
/// True for compatible target roles; variable/language/module/package roles are false.
///
/// # Examples
/// ```text
/// assert!(accepts(TargetKind::Executable, NameRole::Executable));
/// ```
fn accepts(class: TargetKind, role: NameRole) -> bool {
    match role {
        NameRole::Target => true,
        NameRole::Executable => class == TargetKind::Executable,
        NameRole::CustomTarget => class == TargetKind::Custom,
        NameRole::Library => matches!(
            class,
            TargetKind::Library
                | TargetKind::StaticLibrary
                | TargetKind::SharedLibrary
                | TargetKind::ModuleLibrary
                | TargetKind::ObjectLibrary
                | TargetKind::InterfaceLibrary
                | TargetKind::UnknownLibrary
        ),
        NameRole::Variable | NameRole::LanguageType | NameRole::Module | NameRole::Package => false,
    }
}
impl TargetCollector {
    /// Walk use-site parents to a declaration directory. No mutation or I/O.
    ///
    /// # Arguments
    /// - `self` `[in]`: Collector owning both directory IDs.
    /// - `current` `[in]`: Use-site directory; traversed toward its root.
    /// - `wanted` `[in]`: Declaration directory to find.
    ///
    /// # Returns
    /// Some ancestor distance, zero for the same directory; None for a different branch/root.
    ///
    /// # Panics
    /// Panics if a traversed ID is not in this collector; callers must use IDs allocated by
    /// directory().
    ///
    /// # Examples
    /// ```text
    /// let visible = collector.distance(child, root);
    /// ```
    fn distance(&self, mut current: TargetDirectoryId, wanted: TargetDirectoryId) -> Option<usize> {
        let mut distance = 0;
        loop {
            if current == wanted {
                return Some(distance);
            };
            current = self.directories[current.index()]?;
            distance += 1;
        }
    }
    /// Local imported names take the closest visible directory; ties stay unresolved.
    /// Choose a unique nearest visible name and verify its target class. No mutation or I/O.
    ///
    /// # Arguments
    /// - `self` `[in]`: Collected directories, entries and name candidates.
    /// - `name` `[in]`: Case-sensitive literal name.
    /// - `directory` `[in]`: Use-site directory in this collector.
    /// - `role` `[in]`: Required target class.
    /// - `original` `[in]`: True excludes aliases when resolving an alias original.
    ///
    /// # Returns
    /// Some unique compatible entry; None for missing, invisible, tied or class-incompatible names.
    ///
    /// # Algorithm
    /// Ignore aliases when resolving an original. Directory-local candidates must be ancestors of
    /// the use; their distance outranks project-visible entries. Replace the winner on a closer
    /// candidate and mark equal-distance ties ambiguous. Check the required class only after
    /// selecting a unique name; an incompatible nearest declaration does not fall back to another
    /// class.
    ///
    /// # Flow
    /// ```text
    /// [name candidates]
    ///        |
    ///  <allowed/visible?> -- no --> [next candidate]
    ///        | yes
    ///  [ancestor distance or global rank]
    ///        |
    ///  <closer / same distance?>
    ///       /                \
    ///  [new winner]     [mark tie]
    ///       \                /
    ///        [unique final winner?]
    ///                  |
    ///            <class accepted?>
    ///              /          \
    ///            Some         None
    /// ```
    ///
    /// # Examples
    /// ```text
    /// let target = collector.lookup("core", directory, NameRole::Library, false);
    /// ```
    fn lookup(
        &self,
        name: &str,
        directory: TargetDirectoryId,
        role: NameRole,
        original: bool,
    ) -> Option<TargetEntryId> {
        let mut found = None;
        let mut best = usize::MAX;
        let mut ambiguous = false;
        for &id in self.names.get(name)? {
            let entry = &self.entries[id.index()];
            if original && entry.declaration.alias.is_some() {
                continue;
            }
            let distance = if entry.local {
                let Some(distance) = self.distance(directory, entry.directory) else {
                    continue;
                };
                distance
            } else {
                usize::MAX
            };
            if found.is_none() || distance < best {
                found = Some(id);
                best = distance;
                ambiguous = false;
            } else if distance == best {
                ambiguous = true;
            }
        }
        (!ambiguous)
            .then_some(found)
            .flatten()
            .filter(|id| accepts(self.entries[id.index()].class, role))
    }
    /// Consume staged data, bind unique visible reads and return normalized declarations. No I/O;
    /// source snapshots remain unchanged.
    ///
    /// # Arguments
    /// - `self` `[in]`: Owned collector consumed and internally normalized.
    ///
    /// # Returns
    /// Declarations with references sorted by file/span and deduplicated by
    /// revision/command/span/kind; duplicate global visits are omitted.
    ///
    /// # Algorithm
    /// Infer alias visibility from a literal nonalias original. Remove repeated global definitions,
    /// then resolve pending uses by nearest visible directory and class. Sort/deduplicate each
    /// retained reference list; ambiguity does not create references.
    ///
    /// # Flow
    /// ```text
    /// [aliases]          [directory tree]
    ///     |                     |
    ///  [infer locality] <-------+
    ///     |
    ///  [drop global duplicates]
    ///     |
    ///  [pending uses] --> <unique visible match?>
    ///                        /          \
    ///                     append       omit
    ///                        \          /
    ///                   [sort/deduplicate]
    /// ```
    ///
    /// # Examples
    /// ```text
    /// let targets = collector.finish();
    /// ```
    pub(crate) fn finish(mut self) -> Vec<TargetDeclaration> {
        // An alias of a nonglobal import inherits its directory visibility.
        let aliases: Vec<_> = self
            .entries
            .iter()
            .enumerate()
            .filter_map(|(position, entry)| {
                let alias = entry
                    .declaration
                    .alias
                    .as_ref()
                    .filter(|alias| alias.literal)?;
                let original = self.lookup(&alias.text, entry.directory, NameRole::Target, true)?;
                Some((
                    TargetEntryId::new(position),
                    self.entries[original.index()].local,
                ))
            })
            .collect();
        for (id, local) in aliases {
            self.entries[id.index()].local = local;
        }
        // A shared file can declare the same now-global alias in several visit directories.
        let mut definitions = std::collections::HashSet::new();
        let mut duplicates = std::collections::HashSet::new();
        for (position, entry) in self
            .entries
            .iter()
            .enumerate()
            .filter(|(_, entry)| !entry.local)
        {
            if let Some(definition) = entry.declaration.references.first() {
                if !definitions.insert((definition.source.analysis, definition.source.command)) {
                    duplicates.insert(TargetEntryId::new(position));
                }
            }
        }
        for ids in self.names.values_mut() {
            ids.retain(|id| !duplicates.contains(id));
        }
        for usage in std::mem::take(&mut self.uses) {
            if let Some(id) = self.lookup(&usage.name, usage.directory, usage.role, false) {
                self.entries[id.index()]
                    .declaration
                    .references
                    .push(usage.reference);
            }
        }
        self.entries
            .into_iter()
            .enumerate()
            .filter(|(position, _)| !duplicates.contains(&TargetEntryId::new(*position)))
            .map(|(_, mut entry)| {
                entry.declaration.references.sort_by_key(|reference| {
                    (
                        reference.source.analysis.file,
                        reference.span.start_byte,
                        reference.span.end_byte,
                    )
                });
                entry.declaration.references.dedup_by(|a, b| {
                    a.source.analysis == b.source.analysis
                        && a.source.command == b.source.command
                        && a.span == b.span
                        && a.kind == b.kind
                });
                entry.declaration
            })
            .collect()
    }
}
