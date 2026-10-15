// AI-authored incremental semantic analysis.
use super::model::*;
use crate::{
    model::{
        Spanned,
        incremental::{IncrementalWork, SourceChange, SourcePositions},
    },
    parser::nparser::model::{NParsedBuffer, NParsedNode},
};
use std::{
    collections::{HashMap, HashSet},
    sync::Arc,
};

impl IncrementalSemanticer {
    pub async fn new(source: impl Into<Arc<str>>, parsed: NParsedBuffer, file: FileId) -> Self {
        let source = source.into();
        let mut analyzer = Semanticer::new(parsed, HashMap::new());
        let buffer = analyzer.parse(&source, file).await;
        let variable_seed = seed(&analyzer, &buffer);
        let work = IncrementalWork {
            recomputed: buffer.commands.len(),
            reused: 0,
        };
        Self {
            source,
            analyzer,
            buffer: Arc::new(buffer),
            variable_seed,
            work,
            structure_rebuilt: true,
            templates_rebuilt: true,
        }
    }
    pub async fn update(
        &mut self,
        source: impl Into<Arc<str>>,
        parsed: NParsedBuffer,
        change: &SourceChange,
    ) {
        let source = source.into();
        // Catalogue changes invalidate every schema-dependent command. A detected host version stays fixed until a new cache is created.
        if version_input(&self.source, &self.analyzer.buffer) != version_input(&source, &parsed) {
            self.analyzer.buffer = parsed;
            self.buffer = Arc::new(self.analyzer.parse(&source, self.buffer.id.clone()).await);
            self.variable_seed = seed(&self.analyzer, &self.buffer);
            self.work = IncrementalWork {
                recomputed: self.buffer.commands.len(),
                reused: 0,
            };
            self.structure_rebuilt = true;
            self.templates_rebuilt = true;
            self.source = source;
            return;
        }
        let positions = SourcePositions::new(&source);
        let old: HashMap<_, _> = self
            .buffer
            .commands
            .iter()
            .filter(|command| command.closed)
            .filter_map(|command| {
                change
                    .range(command.full_span.start_byte..command.full_span.end_byte)
                    .map(|range| (range.start, (range.end, command)))
            })
            .collect();
        let mut result = SemanticedBuffer::new_with_file_id(self.buffer.id.clone());
        result.version = self.buffer.version;
        result.diagnostics.extend(
            self.buffer
                .diagnostics
                .iter()
                .filter(|d| {
                    matches!(
                        d.content,
                        SemanticError::Version(_) | SemanticError::UnsupportedVersion(_)
                    )
                })
                .cloned(),
        );
        let mut reused = HashSet::new();
        let mut work = IncrementalWork::default();
        for node in &parsed.nodes {
            if let NParsedNode::Command(_) = &node.content {
                if let Some(&(_, command)) = old
                    .get(&node.span.start_byte)
                    .filter(|(end, _)| *end == node.span.end_byte)
                {
                    let mut command = command.clone();
                    command.id = CommandId {
                        id: result.commands.len() as u32,
                    };
                    command.span = positions.remap(change, command.span).unwrap_or(node.span);
                    command.full_span = node.span;
                    for arg in &mut command.args {
                        if let Some(span) = positions.remap(change, arg.span) {
                            arg.span = span;
                        }
                    }
                    for reference in &mut command.variable_references {
                        reference.location.command = command.id;
                        if let Some(span) = positions.remap(change, reference.location.span) {
                            reference.location.span = span;
                        }
                    }
                    if command.schema.is_none() {
                        result.diagnostics.push(Spanned::new(
                            SemanticError::CommandNotFound(command.name.clone()),
                            command.span,
                        ));
                    }
                    reused.insert(command.id);
                    result.commands.push(command);
                    work.reused += 1;
                    continue;
                }
                work.recomputed += 1;
            }
            self.analyzer
                .scan_nodes(&source, std::iter::once(node), &mut result);
        }
        self.structure_rebuilt = structure(&self.buffer) != structure(&result);
        if self.structure_rebuilt {
            self.analyzer.scan_blocks(&mut result);
            self.analyzer.scan_scopes(&mut result);
        } else {
            result.blocks = self.buffer.blocks.clone();
            result.scopes = self.buffer.scopes.clone();
            for block in &mut result.blocks {
                let start = result.commands[block.opening.id as usize].full_span;
                let end = block
                    .closing
                    .and_then(|id| result.commands.get(id.id as usize))
                    .or_else(|| result.commands.last())
                    .map_or(start, |c| c.full_span);
                block.span = crate::model::TextSpan::combine_with_middle(start, end);
            }
            for (command, previous) in result.commands.iter_mut().zip(&self.buffer.commands) {
                command.block = previous.block;
                command.scope = previous.scope;
            }
            result.diagnostics.extend(
                self.buffer
                    .diagnostics
                    .iter()
                    .filter(|d| {
                        matches!(
                            d.content,
                            SemanticError::UnclosedBlock(_)
                                | SemanticError::UnexpectedBlockEnd(_)
                                | SemanticError::UnexpectedBranch(_)
                                | SemanticError::InvalidIfBranch(_)
                        )
                    })
                    .filter_map(|d| {
                        positions
                            .remap(change, d.span)
                            .map(|span| Spanned::new(d.content.clone(), span))
                    }),
            );
        }
        self.analyzer
            .bind_documentation(&source, &parsed.comments, &mut result);
        self.analyzer.scan_functions(&source, &mut result);
        self.templates_rebuilt = template_inputs(&self.buffer) != template_inputs(&result);
        if self.templates_rebuilt {
            self.variable_seed = seed(&self.analyzer, &result);
        }
        result.variables = self.variable_seed.clone();
        self.analyzer
            .track_variable_changes_cached(&mut result, &reused);
        self.analyzer.collect_references(&mut result, &source);
        self.analyzer.collect_values(&mut result);
        self.analyzer.collect_symbols(&mut result);
        self.analyzer.buffer = parsed;
        self.source = source;
        self.buffer = Arc::new(result);
        self.work = work;
    }
}
fn version_input(source: &str, parsed: &NParsedBuffer) -> Vec<String> {
    parsed
        .nodes
        .iter()
        .filter_map(|node| match &node.content {
            NParsedNode::Command(command)
                if command
                    .name
                    .span
                    .text(source)
                    .is_some_and(|s| s.eq_ignore_ascii_case("cmake_minimum_required")) =>
            {
                node.span.text(source).map(str::to_owned)
            }
            _ => None,
        })
        .collect()
}
// Dense IDs require relinking when commands are inserted/deleted or structural keywords move.
fn structure(buffer: &SemanticedBuffer) -> (usize, Vec<(usize, String, bool, Vec<String>)>) {
    (
        buffer.commands.len(),
        buffer
            .commands
            .iter()
            .enumerate()
            .filter(|(_, c)| matches!(c.kind, CommandKind::Builtin(BuiltinCommand::KeyWord)))
            .map(|(i, c)| {
                (
                    i,
                    c.name.to_ascii_lowercase(),
                    c.closed,
                    c.args.iter().map(|a| a.content.clone()).collect(),
                )
            })
            .collect(),
    )
}
fn template_inputs(buffer: &SemanticedBuffer) -> Vec<String> {
    let definitions: HashSet<_> = buffer
        .commands
        .iter()
        .filter(|c| {
            c.closed && matches!(c.name.to_ascii_lowercase().as_str(), "function" | "macro")
        })
        .filter_map(|c| {
            c.args
                .first()
                .and_then(|a| a.literal_text())
                .map(str::to_ascii_lowercase)
        })
        .collect();
    buffer
        .commands
        .iter()
        .filter_map(|command| {
            let name = command.name.to_ascii_lowercase();
            let special = matches!(
                name.as_str(),
                "project"
                    | "enable_language"
                    | "find_package"
                    | "function"
                    | "macro"
                    | "cmake_policy"
                    | "string"
                    | "if"
                    | "elseif"
                    | "target_link_libraries"
            ) || definitions.contains(&name)
                || command.args.iter().any(|a| {
                    a.content == "LINKER_TYPE" || a.content.starts_with("XCODE_ATTRIBUTE_")
                });
            if special {
                Some(format!(
                    "{name}:{}:{:?}",
                    command.closed,
                    command
                        .args
                        .iter()
                        .map(|a| (a.form, &a.content))
                        .collect::<Vec<_>>()
                ))
            } else if matches!(name.as_str(), "set" | "option" | "unset") {
                let first = command.args.first()?;
                if matches!(
                    first.content.as_str(),
                    "CMAKE_BUILD_TYPE" | "CMAKE_CONFIGURATION_TYPES" | "CMAKE_LINKER_TYPE"
                ) {
                    Some(format!(
                        "{name}:{}:{:?}",
                        command.closed,
                        command
                            .args
                            .iter()
                            .map(|a| (a.form, &a.content))
                            .collect::<Vec<_>>()
                    ))
                } else {
                    Some(format!(
                        "{name}:{}:{:?}:{}",
                        command.closed, first.form, first.content
                    ))
                }
            } else {
                None
            }
        })
        .collect()
}
fn seed(analyzer: &Semanticer, buffer: &SemanticedBuffer) -> Vec<SemanticVariable> {
    let mut variables = buffer
        .version
        .and_then(|v| super::variables::builtin::variables_for_version(v.major, v.minor))
        .unwrap_or_else(|| super::variables::builtin::BASE_VARIABLES.clone());
    variables.retain(|v| !v.name.contains(['<', '>']));
    let mut names: HashSet<_> = variables.iter().map(|v| v.name.clone()).collect();
    variables.extend(
        analyzer
            .detect_specialized_variables(buffer)
            .into_iter()
            .filter(|v| names.insert(v.name.clone())),
    );
    variables
}
