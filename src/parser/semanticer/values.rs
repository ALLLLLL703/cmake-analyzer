//! Track a limited set of literal values without interpreting CMake.
//!
//! Epochs invalidate old facts cheaply. Unsupported expansion, unknown calls and conditional
//! writes lose certainty. Workspace replay may supply directories and dependency order;
//! the local pass has no directory context and skips deferred bodies. No I/O.
// Contains AI-generated documentation additions.
// AI-authored limited literal expansion; unknown values are never guessed.
mod arguments;
use super::{model::*, value_model::ValueBinding};
pub(crate) use arguments::argument_value;
use std::{collections::HashMap, sync::Arc};
impl ValueBindings {
    /// Overwrite one name with a text fact or explicit unknown in the current epoch. Copy-on-write isolates shared clones; no I/O.
    ///
    /// # Arguments
    /// - `self` `[in,out]`: inserts/replaces one map entry, cloning shared storage only when necessary.
    /// - `name` `[in]`: case-sensitive variable name copied into the map.
    /// - `value` `[in]`: consumed fact, or None to mask any directory fallback as unknown.
    ///
    /// # Returns
    /// () after storing the value at the current epoch.
    ///
    /// # Examples
    /// ```
    /// use cmake_analyzer::parser::semanticer::model::*;
    /// let mut values = ValueBindings::default();
    /// values.assign("x", Some(PossibleValue::Static("yes".into())));
    /// assert_eq!(values.get("x", None).unwrap().text(), "yes");
    /// ```
    pub fn assign(&mut self, name: &str, value: Option<PossibleValue>) {
        Arc::make_mut(&mut self.entries).insert(
            name.to_owned(),
            ValueBinding {
                value,
                epoch: self.epoch,
            },
        );
    }
    /// Advance the epoch so all existing entries become unknown without visiting them. Does not remove entries or disable fallback for names never assigned; no I/O.
    ///
    /// # Arguments
    /// - `self` `[in,out]`: increments epoch; stored map entries remain unchanged.
    ///
    /// # Returns
    /// () after lazy invalidation.
    ///
    /// # Panics
    /// Epoch overflow can panic when integer overflow checking is enabled.
    ///
    /// # Examples
    /// ```
    /// use cmake_analyzer::parser::semanticer::model::*;
    /// let mut values = ValueBindings::default();
    /// values.assign("x", Some(PossibleValue::Static("yes".into())));
    /// values.invalidate();
    /// assert!(values.get("x", None).is_none());
    /// ```
    pub fn invalidate(&mut self) {
        self.epoch += 1;
    }
    /// Look up a current-epoch text fact, falling back to selected builtin directory names only when no map entry exists. No filesystem access or mutation.
    ///
    /// # Arguments
    /// - `self` `[in]`: value map and epoch.
    /// - `name` `[in]`: case-sensitive variable spelling.
    /// - `directories` `[in]`: optional path context for source/binary/list builtin names.
    ///
    /// # Returns
    /// Some cloned fact for a valid entry, or Static UTF-8 directory text; None for stale/explicit unknown entries, missing context, unsupported names or non-UTF-8 paths.
    ///
    /// # Examples
    /// ```
    /// use cmake_analyzer::parser::semanticer::model::*;
    /// let mut values = ValueBindings::default();
    /// values.assign("x", Some(PossibleValue::Observed("a".into())));
    /// assert_eq!(values.get("x", None).unwrap().text(), "a");
    /// ```
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
    /// Infer argument text using supported simple substitutions. No input mutation or I/O. Output is tagged Static even when substituted bindings were Observed.
    ///
    /// # Arguments
    /// - `self` `[in]`: known variable text bindings.
    /// - `arg` `[in]`: payload and lexical form to inspect.
    /// - `dirs` `[in]`: optional builtin-directory fallback context.
    ///
    /// # Returns
    /// Some Static text for bracket arguments or supported simple ${name} substitutions; None for unknown/malformed names, escapes, semicolons, environment/cache expansions or generator expressions.
    ///
    /// # Algorithm
    /// Return bracket payloads directly. Reject unsupported syntax before scanning simple ${name} segments; append literal segments and known binding text, abandoning the whole result when any substitution is unknown.
    ///
    /// # Flow
    /// ```text
    ///  [argument form]       [binding map / directories]
    ///         |                         |
    ///      <bracket?>                    |
    ///       /      \                    |
    ///  [literal] <supported syntax?>      |
    ///               /        \          |
    ///           [unknown]  [substitute] <-+
    ///                          |
    ///                   <all names known?>
    ///                       /       \
    ///                  [unknown] [Static text]
    /// ```
    ///
    /// # Examples
    /// ```
    /// use cmake_analyzer::parser::semanticer::model::*;
    /// use cmake_analyzer::model::TextSpan;
    /// let mut values = ValueBindings::default();
    /// values.assign("x", Some(PossibleValue::Static("yes".into())));
    /// let arg = SemanticArg { kind: vec![], content: "${x}!".into(), span: TextSpan::default(), form: SemanticArgForm::Quoted };
    /// assert_eq!(values.expand(&arg, None).unwrap().text(), "yes!");
    /// ```
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
    /// Update conservative local text facts for one source command without executing it. Unknown/dynamic-effect commands invalidate facts; supported literal set/project commands may establish them. No I/O.
    ///
    /// # Arguments
    /// - `self` `[in,out]`: overwrites selected bindings or advances the invalidation epoch.
    /// - `command` `[in]`: source command with schema and Out flags already assigned.
    /// - `conditional` `[in]`: whether writes may be conditional; such writes do not establish text facts.
    /// - `dirs` `[in]`: optional source/binary/list context for builtin values and project-derived paths.
    ///
    /// # Returns
    /// () after applying supported facts. Unclosed commands are ignored; nonlocal/cache/environment writes do not establish ordinary local values.
    ///
    /// # Panics
    /// Invalidation can overflow the epoch when integer overflow checking is enabled.
    ///
    /// # Algorithm
    /// Ignore unclosed commands. Dynamic assignment names, unknown schemas, find_package and cmake_language invalidate all prior entries. Project may establish directory-derived facts. For literal local Out slots, unconditional set joins supported argument values with semicolons; other writes become unknown. In-place list/string mutations also lose certainty.
    ///
    /// # Flow
    /// ```text
    ///  [command + Out flags]      [conditional + directories]
    ///            |                           |
    ///            +-------------+-------------+
    ///                          v
    ///                  <unknown/dynamic effect?>
    ///                     /                \
    ///              [invalidate epoch]  <supported local write?>
    ///                                       /          \
    ///                                   [skip]     <literal set/project?>
    ///                                                 /        \
    ///                                            [known text] [unknown]
    ///                                                 |        |
    ///                                                 +---+----+
    ///                                                     v
    ///                                              [overwrite binding]
    /// ```
    ///
    /// # Examples
    /// ```
    /// use cmake_analyzer::parser::semanticer::model::*;
    /// use cmake_analyzer::model::TextSpan;
    /// let command = SemanticCommandRepresent {
    ///     documentation: None, id: CommandId { id: 0 }, kind: CommandKind::Other("custom".into()),
    ///     span: TextSpan::default(), args: vec![], variable_references: vec![], paths: vec![],
    ///     name: "custom".into(), closed: true, schema: None, full_span: TextSpan::default(), block: None, scope: None,
    /// };
    /// let mut values = ValueBindings::default();
    /// values.assign("x", Some(PossibleValue::Static("yes".into())));
    /// values.apply(&command, false, None);
    /// assert!(values.get("x", None).is_none());
    /// ```
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
/// Inspect enclosing blocks for conditional and deferred source contexts. No side effects.
///
/// # Arguments
/// - `buffer` `[in]`: owning block arena.
/// - `command` `[in]`: command whose ancestry is inspected.
///
/// # Returns
/// Tuple (conditional, deferred): any if/while/foreach ancestor, and any function/macro ancestor respectively.
///
/// # Panics
/// Panics if a command or parent block ID is outside buffer.blocks; ancestry must be acyclic.
///
/// # Examples
/// ```text
/// let (conditional, deferred) = flags(&buffer, &buffer.commands[0]);
/// ```
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
    /// Replace local sparse observations and attach matching facts to read/reference locations. Skips deferred bodies for writes and gives their reads no known text; no execution or I/O.
    ///
    /// # Arguments
    /// - `self` `[in]`: analyzer, unchanged.
    /// - `buffer` `[in,out]`: clears/replaces values and overwrites possible_value on command, unresolved and variable references.
    ///
    /// # Returns
    /// () after a source-order pass using independent bindings per optional local scope; no directory context is inferred locally.
    ///
    /// # Panics
    /// Requires valid acyclic block ancestry; binding invalidation can overflow its epoch.
    ///
    /// # Algorithm
    /// For each command, observe reads before applying effects, skip deferred writes and invalidate after includes. Record Out writes after applying effects. Build a command/name/read-or-write lookup and copy facts back to all reference collections.
    ///
    /// # Flow
    /// ```text
    ///  [source-ordered commands]       [scope binding maps]
    ///               |                           |
    ///               +-------------+-------------+
    ///                             v
    ///                    [observe reads first]
    ///                             |
    ///                        <deferred?>
    ///                         /        \
    ///                      [skip]    [apply writes]
    ///                         |           |
    ///                         |     [observe outputs]
    ///                         +-----+-----+
    ///                               v
    ///                     [replace sparse values]
    ///                               |
    ///                               v
    ///                  [annotate reference collections]
    /// ```
    ///
    /// # Examples
    /// ```text
    /// analyzer.collect_values(&mut buffer);
    /// ```
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
