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
    pub fn runtime(&self) -> Option<&Arc<RuntimeLayer>> {
        self.runtime.as_ref()
    }
    pub fn clear_runtime(&mut self) {
        self.runtime = None;
        if let Some(graph) = &mut self.graph {
            graph.runtime = None;
        }
    }
    pub async fn install_runtime(&mut self, trace: Trace) -> io::Result<()> {
        self.install_runtime_cancellable(trace, lspally::server::model::Cancellation::default())
            .await
    }
    /// Install atomically; cancellation leaves the previous runtime layer intact.
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
        let location = command
            .zip(document)
            .map(|(command, document)| ReferenceLocation {
                file: document.semantics.id,
                command: command.id,
                span: command.span,
                kind: ReferenceKind::Write,
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
    Ok(RuntimeLayer {
        event_values,
        resolved_calls,
        trace,
        documents,
        values,
        functions,
    })
}
