use super::model::{TargetCollector, TargetDirectoryId, TargetEntryId};
use crate::parser::{
    build_struct::model::{TargetDeclaration, TargetKind},
    semanticer::schema::NameRole,
};

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
