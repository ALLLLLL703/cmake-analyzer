//! Reuse closed command payloads and rebuild dependent semantic facts after edits.
//!
//! IDs are dense snapshot-local indices, not persistent identities. Edits remap byte
//! spans and can rebuild structure or template seeds. Reference/value passes still run
//! for the resulting document. Version-input changes use a full parse; no project runs.
// Contains AI-generated documentation additions.
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
    /// Create a per-file semantic cache with one complete static parse. May query host CMake version when no declaration is present; never executes project code.
    ///
    /// # Arguments
    /// - `source` `[in]`: source text converted into shared immutable storage.
    /// - `parsed` `[in]`: parser snapshot consumed by the cache; spans must match source.
    /// - `file` `[in]`: file identity retained in every published snapshot.
    ///
    /// # Returns
    /// Cache with all commands counted as recomputed and both rebuild flags true.
    ///
    /// # Examples
    /// ```
    /// use cmake_analyzer::parser::{
    ///     lexer::model::IncrementalLexer,
    ///     nparser::model::IncrementalNParser,
    ///     semanticer::model::{FileId, IncrementalSemanticer},
    /// };
    /// # tokio::runtime::Runtime::new().unwrap().block_on(async {
    /// let source = "cmake_minimum_required(VERSION 3.31)\nset(x yes)";
    /// let lexer = IncrementalLexer::new(source);
    /// let parser = IncrementalNParser::new(source, &lexer.buffer);
    /// let cache = IncrementalSemanticer::new(source, parser.buffer, FileId { id: 0 }).await;
    /// assert_eq!(cache.buffer.commands.len(), 2);
    /// # });
    /// ```
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
    /// Replace the published snapshot after an edit, reusing closed command payloads where byte ranges survive. Mutates only the cache; version-input changes may trigger a host version query, never project execution.
    ///
    /// # Arguments
    /// - `self` `[in,out]`: overwrites source, parser state, snapshot, work counts and rebuild flags; may refresh variable seed.
    /// - `source` `[in]`: new shared source snapshot.
    /// - `parsed` `[in]`: new parser nodes matching source.
    /// - `change` `[in]`: mapping from old to new source byte ranges.
    ///
    /// # Returns
    /// () after publishing a new Arc snapshot; existing Arc clones remain unchanged.
    ///
    /// # Panics
    /// Requires internally consistent dense block/command IDs and a change mapping for the cached source. Structure reuse indexes those arenas directly.
    ///
    /// # Algorithm
    /// Changed version declarations force a full parse. Otherwise map old closed commands into new ranges, reuse exact surviving node extents and rescan the rest. Compare structure/template fingerprints, rebuild or remap dependent data, then always bind comments, references, values and symbols before replacing the snapshot.
    ///
    /// # Flow
    /// ```text
    ///  [old snapshot]       [new nodes + edit]
    ///        |                       |
    ///        +-----------+-----------+
    ///                    v
    ///          <version input changed?>
    ///              /            \
    ///            yes            no
    ///             |              |
    ///        [full parse] [reuse spans / rescan]
    ///             |              |
    ///             |     <structure/template changed?>
    ///             |        /                 \
    ///             |    [rebuild]         [reuse/remap]
    ///             |        |                 |
    ///             |        +--------+--------+
    ///             |                 v
    ///             |    [comments/references/values/symbols]
    ///             |                 |
    ///             +--------+--------+
    ///                      v
    ///             [publish new Arc snapshot]
    /// ```
    ///
    /// # Examples
    /// ```
    /// use cmake_analyzer::parser::{
    ///     lexer::model::IncrementalLexer,
    ///     nparser::model::IncrementalNParser,
    ///     semanticer::model::{FileId, IncrementalSemanticer},
    /// };
    /// # tokio::runtime::Runtime::new().unwrap().block_on(async {
    /// let source = "cmake_minimum_required(VERSION 3.31)\nset(x yes)";
    /// let lexer = IncrementalLexer::new(source);
    /// let parser = IncrementalNParser::new(source, &lexer.buffer);
    /// use cmake_analyzer::model::incremental::SourceChange;
    /// let mut cache = IncrementalSemanticer::new(source, parser.buffer, FileId { id: 0 }).await;
    /// let changed = "cmake_minimum_required(VERSION 3.31)\nset(x no)";
    /// let lexer = IncrementalLexer::new(changed);
    /// let parser = IncrementalNParser::new(changed, &lexer.buffer);
    /// let change = SourceChange::between(source, changed);
    /// cache.update(changed, parser.buffer, &change).await;
    /// assert_eq!(cache.work.recomputed, 1);
    /// # });
    /// ```
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
/// Collect complete cmake_minimum_required source strings for catalog invalidation. No side effects.
///
/// # Arguments
/// - `source` `[in]`: source matching parsed spans.
/// - `parsed` `[in]`: parser snapshot scanned in source order.
///
/// # Returns
/// Owned command strings; invalid full spans are omitted.
///
/// # Examples
/// ```text
/// let keys = version_input(source, &parsed);
/// ```
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
/// Fingerprint dense command count and structural keyword inputs for relinking. No side effects.
///
/// # Arguments
/// - `buffer` `[in]`: current semantic snapshot.
///
/// # Returns
/// Tuple of total command count and keyword tuples: command position, lowercase name, closed flag and argument payload strings.
///
/// # Examples
/// ```text
/// let (count, keywords) = structure(&buffer);
/// ```
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
/// Fingerprint commands that can change specialized builtin names. No side effects.
///
/// # Arguments
/// - `buffer` `[in]`: semantic command inputs.
///
/// # Returns
/// Ordered keys retaining relevant command names/forms/payloads; ordinary set/option/unset commands retain only their first argument unless they set a template-domain variable.
///
/// # Algorithm
/// First collect literal function/macro names. Then retain domain-producing commands, their calls and feature hints; reduce ordinary assignments to the output-name information needed by specialization.
///
/// # Flow
/// ```text
///  [commands]           [literal definitions]
///      |                         |
///      +------------+------------+
///                   v
///        <domain/feature/call input?>
///           /                 \
///         yes                 no
///          |                   |
///  [full form/payload key] <assignment?>
///          |                /      \
///          |        [first/domain] [omit]
///          |                |       |
///          +----------------+-------+
///                           v
///                     [ordered keys]
/// ```
///
/// # Examples
/// ```text
/// let keys = template_inputs(&buffer);
/// ```
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
/// Build unbound builtin and specialized variable records, deduplicated by name. No I/O or project execution.
///
/// # Arguments
/// - `analyzer` `[in]`: template-specialization engine.
/// - `buffer` `[in]`: version and command hints.
///
/// # Returns
/// Owned variable seed without unresolved placeholder names; unsupported versions fall back to baseline names.
///
/// # Examples
/// ```text
/// let variables = seed(&analyzer, &buffer);
/// ```
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
