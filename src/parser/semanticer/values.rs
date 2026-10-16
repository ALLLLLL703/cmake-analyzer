// AI-authored limited literal expansion; unknown values are never guessed.
mod arguments;
use super::{model::*, value_model::ValueBinding};
pub(crate) use arguments::argument_value;
use std::{collections::HashMap, sync::Arc};
impl ValueBindings {
    pub fn assign(&mut self, name: &str, value: Option<PossibleValue>) {
        Arc::make_mut(&mut self.entries).insert(
            name.to_owned(),
            ValueBinding {
                value,
                epoch: self.epoch,
            },
        );
    }
    pub fn invalidate(&mut self) {
        self.epoch += 1;
    }
    pub fn get(&self, name: &str, directories: Option<&ValueDirectories>) -> Option<PossibleValue> {
        if let Some(binding) = self.entries.get(name) {
            return (binding.epoch == self.epoch)
                .then(|| binding.value.clone())
                .flatten();
        }
        let dirs = directories?;
        let path = match name {
            "CMAKE_SOURCE_DIR" => Some(dirs.source_root.as_path()),
            "CMAKE_CURRENT_SOURCE_DIR" => Some(dirs.source.as_path()),
            "CMAKE_CURRENT_LIST_DIR" => dirs.file.parent(),
            "CMAKE_CURRENT_LIST_FILE" => Some(dirs.file.as_path()),
            "CMAKE_BINARY_DIR" => dirs.binary_root.as_deref(),
            "CMAKE_CURRENT_BINARY_DIR" => dirs.binary.as_deref(),
            _ => None,
        }?;
        Some(PossibleValue::Static(path.to_str()?.into()))
    }
    pub fn expand(
        &self,
        arg: &SemanticArg,
        dirs: Option<&ValueDirectories>,
    ) -> Option<PossibleValue> {
        if arg.form == SemanticArgForm::Bracket {
            return Some(PossibleValue::Static(arg.content.as_str().into()));
        }
        let text = &arg.content;
        if text.contains(['\\', ';'])
            || text.contains("$ENV{")
            || text.contains("$CACHE{")
            || text.contains("$<")
        {
            return None;
        }
        let mut rest = text.as_str();
        let mut expanded = String::new();
        while let Some(start) = rest.find("${") {
            expanded.push_str(&rest[..start]);
            rest = &rest[start + 2..];
            let end = rest.find('}')?;
            let name = &rest[..end];
            if name.contains(['$', '{']) {
                return None;
            }
            expanded.push_str(self.get(name, dirs)?.text());
            rest = &rest[end + 1..];
        }
        expanded.push_str(rest);
        Some(PossibleValue::Static(expanded.into()))
    }
    pub fn apply(
        &mut self,
        command: &SemanticCommandRepresent,
        conditional: bool,
        dirs: Option<&ValueDirectories>,
    ) {
        if !command.closed {
            return;
        }
        let name = command.name.to_ascii_lowercase();
        if matches!(name.as_str(), "set" | "unset")
            && command
                .args
                .first()
                .is_some_and(|arg| arg.literal_text().is_none())
        {
            self.invalidate();
            return;
        }
        if matches!(name.as_str(), "find_package" | "cmake_language") {
            self.invalidate();
            return;
        }
        if command.schema.is_none() {
            self.invalidate();
            return;
        }
        if name == "project" {
            if conditional {
                self.assign("PROJECT_SOURCE_DIR", None);
                return;
            }
            if let (Some(dirs), Some(project)) = (
                dirs,
                command.args.first().and_then(SemanticArg::literal_text),
            ) {
                let source = dirs
                    .source
                    .to_str()
                    .map(|path| PossibleValue::Static(path.into()));
                self.assign("PROJECT_SOURCE_DIR", source.clone());
                self.assign(&format!("{project}_SOURCE_DIR"), source);
                let binary = dirs
                    .binary
                    .as_ref()
                    .and_then(|path| path.to_str())
                    .map(|path| PossibleValue::Static(path.into()));
                self.assign("PROJECT_BINARY_DIR", binary.clone());
                self.assign(&format!("{project}_BINARY_DIR"), binary);
            }
            return;
        }
        for (index, arg) in command.args.iter().enumerate() {
            if !(arg.kind.iter().any(|kind| matches!(kind, ArgKind::Out))
                || index == 0 && name == "unset")
            {
                continue;
            }
            let Some(variable) = arg.literal_text() else {
                continue;
            };
            if variable.starts_with("ENV{") || variable.starts_with("CACHE{") {
                continue;
            }
            // Nonlocal/cache writes do not establish an ordinary local value.
            if command.args.last().and_then(SemanticArg::literal_text) == Some("PARENT_SCOPE") {
                continue;
            }
            if name == "set"
                && command.args.iter().enumerate().skip(1).any(|(index, arg)| {
                    arg.literal_text() == Some("CACHE") && command.args.len() >= index + 3
                })
            {
                continue;
            }
            let value = if name == "set" && !conditional && command.args.len() > 1 {
                command.args[1..]
                    .iter()
                    .map(|arg| self.expand(arg, dirs).map(|value| value.text().to_owned()))
                    .collect::<Option<Vec<_>>>()
                    .map(|values| PossibleValue::Static(values.join(";").into()))
            } else {
                None
            };
            self.assign(variable, value);
        }
        if matches!(name.as_str(), "list" | "string")
            && matches!(
                command.args.first().and_then(SemanticArg::literal_text),
                Some(
                    "APPEND"
                        | "PREPEND"
                        | "INSERT"
                        | "REMOVE_ITEM"
                        | "REMOVE_AT"
                        | "REMOVE_DUPLICATES"
                        | "FILTER"
                        | "TRANSFORM"
                        | "REVERSE"
                        | "SORT"
                        | "POP_FRONT"
                        | "POP_BACK"
                )
            )
        {
            if let Some(name) = command.args.get(1).and_then(SemanticArg::literal_text) {
                self.assign(name, None);
            }
        }
    }
}
pub(crate) fn flags(buffer: &SemanticedBuffer, command: &SemanticCommandRepresent) -> (bool, bool) {
    let (mut conditional, mut deferred) = (false, false);
    let mut block = command.block;
    while let Some(id) = block {
        let entry = &buffer.blocks[id.id as usize];
        conditional |= matches!(
            entry.kind,
            BlockKind::If | BlockKind::While | BlockKind::Foreach
        );
        deferred |= matches!(entry.kind, BlockKind::Function | BlockKind::Macro);
        block = entry.parent;
    }
    (conditional, deferred)
}
impl Semanticer {
    pub(crate) fn collect_values(&self, buffer: &mut SemanticedBuffer) {
        let mut scopes: HashMap<Option<ScopeId>, ValueBindings> = HashMap::new();
        buffer.values.clear();
        for command in &buffer.commands {
            let (conditional, deferred) = flags(buffer, command);
            let scope = command.scope;
            let values = scopes.entry(scope).or_default();
            for reference in &command.variable_references {
                buffer.values.push(VariableValueAt {
                    name: reference.name.clone(),
                    location: reference.location.clone(),
                    scope,
                    invocation: None,
                    possible_value: if deferred {
                        None
                    } else {
                        values.get(&reference.name, None)
                    },
                });
            }
            if deferred {
                continue;
            }
            values.apply(command, conditional, None);
            if command.name.eq_ignore_ascii_case("include") {
                values.invalidate();
            }
            for arg in command
                .args
                .iter()
                .filter(|arg| arg.kind.iter().any(|kind| matches!(kind, ArgKind::Out)))
            {
                if let Some(name) = arg.literal_text() {
                    buffer.values.push(VariableValueAt {
                        name: name.to_owned(),
                        location: ReferenceLocation {
                            file: buffer.id,
                            command: command.id,
                            span: arg.span,
                            kind: ReferenceKind::Write,
                            possible_value: values.get(name, None),
                        },
                        scope,
                        invocation: None,
                        possible_value: values.get(name, None),
                    });
                }
            }
        }
        for value in &mut buffer.values {
            value.location.possible_value = value.possible_value.clone();
        }
        let known: HashMap<_, _> = buffer
            .values
            .iter()
            .map(|value| {
                (
                    (
                        value.location.command,
                        value.name.as_str(),
                        value.location.kind == ReferenceKind::Read,
                    ),
                    value.possible_value.clone(),
                )
            })
            .collect();
        for command in &mut buffer.commands {
            for reference in &mut command.variable_references {
                reference.location.possible_value = known
                    .get(&(reference.location.command, reference.name.as_str(), true))
                    .cloned()
                    .flatten();
            }
        }
        for reference in &mut buffer.unresolved_references {
            reference.location.possible_value = known
                .get(&(
                    reference.location.command,
                    reference.name.as_str(),
                    reference.location.kind == ReferenceKind::Read,
                ))
                .cloned()
                .flatten();
        }
        for variable in &mut buffer.variables {
            for reference in &mut variable.references {
                reference.possible_value = known
                    .get(&(
                        reference.command,
                        variable.name.as_str(),
                        reference.kind == ReferenceKind::Read,
                    ))
                    .cloned()
                    .flatten();
            }
        }
    }
}
