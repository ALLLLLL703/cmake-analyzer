//! Install an independent immutable layer from an already executed CMake trace.
//! Loads optional source mappings and indexes observations on a bounded worker. Source analysis
//! may inspect `cmake --version`; it never configures or executes project code or rewrites static
//! snapshots. Cancellation preserves the previously installed layer.
// Contains AI-generated documentation additions.
// AI-authored execution-derived overlay. Static graphs/snapshots are not rewritten.
mod model;
use super::model::Workspace;
use crate::{parser::semanticer::model::*, runtime_cmake::model::Trace};
pub use model::*;
use std::{
    collections::{BTreeMap, BTreeSet, HashMap},
    io,
    sync::Arc,
};
impl Workspace {
    /// Borrow the independent installed runtime layer. No reads, execution or mutation occur.
    ///
    /// # Arguments
    /// - `self` `[in]`: Workspace whose current overlay is queried.
    ///
    /// # Returns
    /// Some shared layer after successful installation, or None after creation/invalidation/clear.
    ///
    /// # Examples
    /// ```
    /// use cmake_analyzer::workspace::Workspace;
    /// fn observed(workspace: &Workspace) -> bool { workspace.runtime().is_some() }
    /// ```
    pub fn runtime(&self) -> Option<&Arc<RuntimeLayer>> {
        self.runtime.as_ref()
    }
    /// Retire the installed execution overlay from workspace and saved graph. Previously cloned
    /// Arcs remain valid; no I/O or execution occurs.
    ///
    /// # Arguments
    /// - `self` `[in,out]`: Overwrites workspace.runtime and graph.runtime with None when present.
    ///
    /// # Returns
    /// () after retiring live references; static snapshots/indexes are unchanged.
    ///
    /// # Examples
    /// ```
    /// use cmake_analyzer::workspace::Workspace;
    /// fn retire(workspace: &mut Workspace) { workspace.clear_runtime(); }
    /// ```
    pub fn clear_runtime(&mut self) {
        self.runtime = None;
        if let Some(graph) = &mut self.graph {
            graph.runtime = None;
        }
    }
    /// Install an already executed trace using a noncancelled token. May load source files, inspect
    /// `cmake --version` and build observations on a bounded worker; never configures or executes
    /// project code or mutates static snapshots.
    ///
    /// # Arguments
    /// - `self` `[in,out]` : Workspace caches may load sources; runtime/dependency state changes on
    ///   successful installation.
    /// - `trace` `[in]`: Owned event/dependency trace, consumed by the new layer.
    ///
    /// # Returns
    /// Ok(()) after layer publication; Err on file-count/read/worker failures.
    ///
    /// # Errors
    /// Propagates source-count limits, nonmissing source read/analysis failures and bounded-worker
    /// errors. The previous installed layer is preserved on failure, though source caches may
    /// change.
    ///
    /// # Examples
    /// Requires a Tokio runtime. Source files must be readable unless supplied as overlays;
    /// runtime installation consumes an existing trace and does not execute a project.
    /// ```no_run
    /// use cmake_analyzer::workspace::Workspace;
    /// use cmake_analyzer::runtime_cmake::model::{RunId, Trace};
    /// # async fn example(workspace: &mut Workspace) -> std::io::Result<()> {
    /// let trace = Trace { run: RunId::new(1), build_directory: None, events: vec![], dependencies: vec![] };
    /// workspace.install_runtime(trace).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn install_runtime(&mut self, trace: Trace) -> io::Result<()> {
        self.install_runtime_cancellable(trace, lspally::server::model::Cancellation::default())
            .await
    }
    /// Install atomically; cancellation leaves the previous runtime layer intact.
    /// Load/index an existing trace and publish its layer only after successful completion.
    /// Cancellation/errors preserve the previous installed runtime, but source caches may already
    /// have been updated. Source analysis may inspect `cmake --version`; it never configures or
    /// executes project code.
    ///
    /// # Arguments
    /// - `self` `[in,out]` : Loads source caches, then extends dependency tracking and overwrites
    ///   graph/workspace runtime only on success.
    /// - `trace` `[in]`: Owned executed trace, including resolved dependency sources.
    /// - `cancel` `[in]` : Cooperative cancellation token checked before/between loads and during
    ///   worker indexing.
    ///
    /// # Returns
    /// Ok(()) after publishing a shared RuntimeLayer; Err for cancellation, source limits, reads or
    /// worker failures.
    ///
    /// # Errors
    /// Interrupted cancellation; too many distinct trace sources; non-NotFound read/analysis
    /// errors; closed worker semaphore or worker join failure.
    ///
    /// # Algorithm
    /// Deduplicate event/resolved-dependency files and enforce the workspace distinct-file budget.
    /// Load optional source snapshots serially, skipping missing files. Hold a worker permit while
    /// building event/source/value/function indexes, then share the finished layer with workspace
    /// and graph. Cancellation is cooperative at explicit checks, not a transaction over
    /// source-cache loads.
    ///
    /// # Flow
    /// ```text
    /// [trace files] ---> <file budget?> -- exceeded --> [error]
    ///       | allowed
    ///  [load sources] ---- missing ---> [skip source]
    ///       |
    ///  [bounded build_layer worker]
    ///       |
    ///  <success?>
    ///    /           \
    ///  error       [extend dependencies]
    ///    |                  |
    /// [old layer kept] [publish graph/workspace Arc]
    /// ```
    ///
    /// # Examples
    /// Requires a Tokio runtime. Source files must be readable unless supplied as overlays;
    /// runtime installation consumes an existing trace and does not execute a project.
    /// ```no_run
    /// use cmake_analyzer::workspace::Workspace;
    /// use cmake_analyzer::runtime_cmake::model::{RunId, Trace};
    /// use lspally::server::model::Cancellation;
    /// # async fn example(workspace: &mut Workspace) -> std::io::Result<()> {
    /// let trace = Trace { run: RunId::new(1), build_directory: None, events: vec![], dependencies: vec![] };
    /// workspace.install_runtime_cancellable(trace, Cancellation::default()).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn install_runtime_cancellable(
        &mut self,
        trace: Trace,
        cancel: lspally::server::model::Cancellation,
    ) -> io::Result<()> {
        if cancel.is_cancelled() {
            return Err(io::Error::new(
                io::ErrorKind::Interrupted,
                "Runtime loading cancelled",
            ));
        }
        let files: BTreeSet<_> = trace
            .events
            .iter()
            .map(|event| event.file.clone())
            .chain(
                trace
                    .dependencies
                    .iter()
                    .filter_map(|dependency| dependency.resolved.clone()),
            )
            .collect();
        if files.len() > self.limits.files {
            return Err(io::Error::other(
                "Runtime source count exceeds Workspace file limit",
            ));
        }
        let mut documents = BTreeMap::new();
        for file in files {
            if cancel.is_cancelled() {
                return Err(io::Error::new(
                    io::ErrorKind::Interrupted,
                    "Runtime loading cancelled",
                ));
            }
            match self.load_snapshot(&file).await {
                Ok(snapshot) => {
                    documents.insert(file, snapshot);
                }
                Err(error) if error.kind() == io::ErrorKind::NotFound => {}
                Err(error) => return Err(error),
            }
        }
        let permit = self
            .workers
            .clone()
            .acquire_owned()
            .await
            .map_err(io::Error::other)?;
        let layer = Arc::new(
            tokio::task::spawn_blocking(move || {
                let _permit = permit;
                build_layer(trace, documents, cancel)
            })
            .await
            .map_err(io::Error::other)??,
        );
        self.dependencies.extend(layer.documents.keys().cloned());
        if let Some(graph) = &mut self.graph {
            graph.runtime = Some(layer.clone());
        }
        self.runtime = Some(layer);
        Ok(())
    }
}
/// Map executed events to optional source commands and build sparse observed value/function
/// indexes. No I/O or static snapshot mutation; cancellation stops assembly.
///
/// # Arguments
/// - `trace` `[in]`: Owned expanded trace; event IDs are used as indices by consumers.
/// - `documents` `[in]`: Owned map of shared source snapshots loaded by the caller.
/// - `cancel` `[in]`: Cooperative cancellation token checked during source/event loops.
///
/// # Returns
/// Ok independent RuntimeLayer; Err(Interrupted) when cancellation is observed.
///
/// # Errors
/// Returns Interrupted when cancellation is observed; no partial RuntimeLayer is returned.
///
/// # Algorithm
/// Index source commands by file, one-based line and lowercase name; map only unique matches.
/// Replay declaration/call names in event order. Recognize set/unset namespaces conservatively,
/// confirming ordinary writes and forced cache values only. Retain every mapped variable read;
/// attach observed values only for stable whole quoted `${NAME}` operands. Other reads keep unknown
/// values. Preserve frame/event identity and per-event value ranges; unmapped locations stay absent.
///
/// # Flow
/// ```text
/// [source commands]       [expanded events]
///          |                       |
///  [file/line/name map] --> <unique source?>
///                              /        \
///                          mapped      absent
///                              \        /
///                       [ordered event replay]
///                         /              \
///               [functions/calls] [safe reads/writes]
///                         \              /
///                        [immutable layer]
/// ```
///
/// # Examples
/// ```text
/// let layer = build_layer(trace, documents, cancel)?;
/// ```
fn build_layer(
    trace: Trace,
    documents: BTreeMap<std::path::PathBuf, Arc<super::model::DocumentSnapshot>>,
    cancel: lspally::server::model::Cancellation,
) -> io::Result<RuntimeLayer> {
    let mut commands: HashMap<_, HashMap<_, HashMap<String, Vec<CommandId>>>> = HashMap::new();
    for (file, document) in &documents {
        let lines = commands.entry(file.clone()).or_default();
        for command in &document.semantics.commands {
            if cancel.is_cancelled() {
                return Err(io::Error::new(
                    io::ErrorKind::Interrupted,
                    "Runtime indexing cancelled",
                ));
            }
            lines
                .entry(command.span.row + 1)
                .or_default()
                .entry(command.name.to_ascii_lowercase())
                .or_default()
                .push(command.id);
        }
    }
    let mut values = Vec::new();
    let mut event_values = Vec::with_capacity(trace.events.len());
    let mut source_commands = Vec::with_capacity(trace.events.len());
    let mut resolved_calls: BTreeMap<_, Vec<_>> = BTreeMap::new();
    let mut functions: Vec<RuntimeFunction> = Vec::new();
    let mut active = HashMap::new();
    for event in &trace.events {
        if cancel.is_cancelled() {
            return Err(io::Error::new(
                io::ErrorKind::Interrupted,
                "Runtime indexing cancelled",
            ));
        }
        let value_start = values.len();
        let document = documents.get(&event.file);
        let command = document.and_then(|document| {
            commands
                .get(&event.file)
                .and_then(|lines| lines.get(&u64::from(event.line)))
                .and_then(|names| names.get(&event.command))
                .filter(|ids| ids.len() == 1)
                .map(|ids| &document.semantics.commands[ids[0].id as usize])
        });
        source_commands.push(command.map(|command| command.id));
        let location = command
            .zip(document)
            .map(|(command, document)| ReferenceLocation {
                file: document.semantics.id,
                command: command.id,
                span: command.span,
                kind: ReferenceKind::Write,
                possible_value: None,
            });
        if !active.contains_key(&event.command)
            && matches!(event.command.as_str(), "function" | "macro")
        {
            if let Some(name) = event.arguments.first() {
                active.insert(
                    name.to_ascii_lowercase(),
                    model::RuntimeFunctionId::new(functions.len()),
                );
                functions.push(RuntimeFunction {
                    name: name.clone(),
                    kind: if event.command == "function" {
                        FunctionKind::Function
                    } else {
                        FunctionKind::Macro
                    },
                    declaration: event.id,
                    calls: Vec::new(),
                });
            }
        } else if let Some(&index) = active.get(&event.command) {
            functions[index.index()].calls.push(event.id);
            if let Some(command) = command {
                resolved_calls
                    .entry(event.file.clone())
                    .or_default()
                    .push(command.id);
            }
        }
        if matches!(event.command.as_str(), "set" | "unset") && !active.contains_key(&event.command)
        {
            let source_args = command
                .filter(|command| command.args.len() == event.arguments.len())
                .map(|command| command.args.as_slice());
            // Trace expansion precedes unquoted list splitting and empty-argument removal.
            let intact = |index: usize, text: &str| {
                source_args
                    .and_then(|args| args.get(index))
                    .is_some_and(|arg| arg.form != SemanticArgForm::Unquoted)
                    || !text.is_empty() && !text.contains([';', '\\'])
            };
            let arguments_intact = event
                .arguments
                .iter()
                .enumerate()
                .all(|(index, text)| intact(index, text));
            if let Some(name) = event.arguments.first().filter(|name| {
                intact(0, name) && !name.starts_with("ENV{") && !name.starts_with("CACHE{")
            }) {
                let cache = event
                    .arguments
                    .iter()
                    .enumerate()
                    .skip(1)
                    .find(|(index, arg)| {
                        arg.as_str() == "CACHE"
                            && (event.command == "unset" || event.arguments.len() >= index + 3)
                    })
                    .map(|(index, _)| index);
                let namespace = if !arguments_intact {
                    ValueNamespace::Unknown
                } else if cache.is_some() {
                    ValueNamespace::Cache
                } else if event
                    .arguments
                    .last()
                    .is_some_and(|arg| arg == "PARENT_SCOPE")
                {
                    ValueNamespace::ParentScope
                } else {
                    ValueNamespace::Ordinary
                };
                let end = cache.unwrap_or_else(|| {
                    event.arguments.len() - usize::from(namespace == ValueNamespace::ParentScope)
                });
                let confirmed = namespace == ValueNamespace::Ordinary
                    || namespace == ValueNamespace::Cache
                        && event
                            .arguments
                            .last()
                            .is_some_and(|argument| argument == "FORCE");
                let possible_value = (confirmed && event.command == "set" && end > 1)
                    .then(|| PossibleValue::Observed(event.arguments[1..end].join(";").into()));
                values.push(RuntimeValueAt {
                    event: event.id,
                    frame: event.frame,
                    name: name.clone(),
                    location: location.clone(),
                    namespace,
                    possible_value,
                });
            }
        }
        if let Some(command) = command {
            for reference in &command.variable_references {
                let argument = &command.args[reference.argument];
                // An earlier unquoted expansion can shift the trace's argument positions.
                let stable_prefix = command.args[..reference.argument].iter().all(|arg| {
                    arg.form != SemanticArgForm::Unquoted || !arg.content.contains(['$', ';', '\\'])
                });
                let possible_value = (stable_prefix
                    && argument.form == SemanticArgForm::Quoted
                    && argument
                        .content
                        .strip_prefix("${")
                        .and_then(|text| text.strip_suffix('}'))
                        == Some(reference.name.as_str()))
                .then(|| event.arguments.get(reference.argument))
                .flatten()
                .map(|text| PossibleValue::Observed(text.as_str().into()));
                values.push(RuntimeValueAt {
                    event: event.id,
                    frame: event.frame,
                    name: reference.name.clone(),
                    location: Some(reference.location.clone()),
                    namespace: ValueNamespace::Ordinary,
                    possible_value,
                });
            }
        }
        event_values.push(value_start..values.len());
    }
    for value in &mut values {
        if let Some(location) = &mut value.location {
            location.possible_value = value.possible_value.clone();
        }
    }
    Ok(RuntimeLayer {
        source_commands,
        event_values,
        resolved_calls,
        trace,
        documents,
        values,
        functions,
    })
}
