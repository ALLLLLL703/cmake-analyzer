//! Reuse unchanged per-command effects while rebinding all source and target locations.
//! Updates replace owned outputs; readers of older shared results remain unchanged. No I/O.
// Contains AI-generated documentation additions.
// AI-authored incremental build fact extraction.
use super::model::*;
use crate::{
    model::incremental::{IncrementalWork, SourceChange},
    parser::semanticer::model::SemanticedBuffer,
};
use std::{
    collections::{HashMap, HashSet},
    sync::Arc,
};

impl IncrementalBuildStruct {
    /// Extract a full build result and initialize per-command reuse ranges. No I/O; semantic input
    /// is unchanged.
    ///
    /// # Arguments
    /// - `semantic` `[in]`: Semantic snapshot to extract.
    /// - `origin` `[in]`: Identity stamped onto facts and diagnostics.
    ///
    /// # Returns
    /// Cache whose work counts all commands as recomputed and none reused.
    ///
    /// # Examples
    /// ```
    /// use cmake_analyzer::parser::build_struct::model::{AnalysisKey, IncrementalBuildStruct};
    /// use cmake_analyzer::parser::semanticer::model::SemanticedBuffer;
    /// let semantic = SemanticedBuffer::default();
    /// let cache = IncrementalBuildStruct::new(&semantic, AnalysisKey { file: 0, revision: 1 });
    /// ```
    pub fn new(semantic: &SemanticedBuffer, origin: AnalysisKey) -> Self {
        let buffer = BuildStructAnalyzer::new(semantic, origin).analyze();
        let units = units(semantic, &buffer);
        Self {
            buffer: Arc::new(buffer),
            units,
            work: IncrementalWork {
                recomputed: semantic.commands.len(),
                reused: 0,
            },
        }
    }
    /// Replace cached build output while reusing unaffected command effects. Rebinds sources and
    /// target reads; old Arc readers stay immutable. No I/O.
    ///
    /// # Arguments
    /// - `self` `[in,out]` : Overwrites buffer, units and work counters; does not append to old
    ///   output.
    /// - `semantic` `[in]`: New semantic snapshot.
    /// - `origin` `[in]`: New file/revision identity.
    /// - `change` `[in]`: Unchanged-range mapping between old and new source.
    ///
    /// # Returns
    /// () after publishing the replacement result.
    ///
    /// # Algorithm
    /// Match mapped full spans and schema Arc identity. Extract changed commands only, merge old
    /// payloads with fresh source references in command order, clear reused target bindings, then
    /// rebind all local targets and rebuild unit ranges.
    ///
    /// # Flow
    /// ```text
    /// [old spans]       [new commands]
    ///       |                 |
    ///       +----> [span/schema match]
    ///                     /       \
    ///                 reuse      extract
    ///                     \       /
    ///                  [ordered merge]
    ///                         |
    ///                  [bind targets]
    ///                         |
    ///                [replace Arc/counters]
    /// ```
    ///
    /// # Examples
    /// ```
    /// use cmake_analyzer::parser::build_struct::model::{AnalysisKey, IncrementalBuildStruct};
    /// use cmake_analyzer::{parser::semanticer::model::SemanticedBuffer, model::incremental::SourceChange};
    /// let semantic = SemanticedBuffer::default();
    /// let mut cache = IncrementalBuildStruct::new(&semantic, AnalysisKey { file: 0, revision: 1 });
    /// let change = SourceChange::between("", "");
    /// cache.update(&semantic, AnalysisKey { file: 0, revision: 2 }, &change);
    /// ```
    pub fn update(
        &mut self,
        semantic: &SemanticedBuffer,
        origin: AnalysisKey,
        change: &SourceChange,
    ) {
        let old: HashMap<_, _> = self
            .units
            .iter()
            .filter_map(|unit| {
                change
                    .range(unit.span.start_byte..unit.span.end_byte)
                    .map(|range| (range.start, (range.end, unit)))
            })
            .collect();
        let mut reused = HashMap::new();
        let mut changed = HashSet::new();
        for command in &semantic.commands {
            let candidate = old
                .get(&command.full_span.start_byte)
                .filter(|(end, unit)| {
                    *end == command.full_span.end_byte
                        && match (&command.schema, &unit.schema) {
                            (Some(a), Some(b)) => Arc::ptr_eq(a, b),
                            (None, None) => true,
                            _ => false,
                        }
                });
            if let Some((_, unit)) = candidate {
                reused.insert(command.id, *unit);
            } else {
                changed.insert(command.id);
            }
        }
        let analyzer = BuildStructAnalyzer::new(semantic, origin);
        let computed = analyzer.analyze_selected(|command| changed.contains(&command.id));
        let mut facts = computed.facts.into_iter().peekable();
        let mut diagnostics = computed.diagnostics.into_iter().peekable();
        let mut buffer = BuildStruct {
            analysis: origin,
            facts: Vec::new(),
            diagnostics: Vec::new(),
        };
        for command in &semantic.commands {
            if let Some(unit) = reused.get(&command.id) {
                let source = analyzer.source_for(command);
                buffer
                    .facts
                    .extend(
                        self.buffer.facts[unit.facts.clone()]
                            .iter()
                            .map(|fact| BuildFact {
                                source: source.clone(),
                                effect: match &fact.effect {
                                    BuildEffect::Target(target) => {
                                        BuildEffect::Target(target.clone_unbound())
                                    }
                                    effect => effect.clone(),
                                },
                            }),
                    );
                buffer.diagnostics.extend(
                    self.buffer.diagnostics[unit.diagnostics.clone()]
                        .iter()
                        .map(|diagnostic| BuildDiagnostic {
                            source: source.clone(),
                            message: diagnostic.message.clone(),
                        }),
                );
            } else {
                while facts.peek().is_some_and(|f| f.source.command == command.id) {
                    if let Some(fact) = facts.next() {
                        buffer.facts.push(fact);
                    }
                }
                while diagnostics
                    .peek()
                    .is_some_and(|d| d.source.command == command.id)
                {
                    if let Some(diagnostic) = diagnostics.next() {
                        buffer.diagnostics.push(diagnostic);
                    }
                }
            }
        }
        super::target_references::bind_local(&mut buffer, semantic);
        self.work = IncrementalWork {
            recomputed: changed.len(),
            reused: reused.len(),
        };
        self.units = units(semantic, &buffer);
        self.buffer = Arc::new(buffer);
    }
}
/// Index each command into contiguous fact and diagnostic ranges. No mutation or I/O.
///
/// # Arguments
/// - `semantic` `[in]`: Commands whose source order defines units.
/// - `buffer` `[in]`: Extraction output ordered by the same commands.
///
/// # Returns
/// One unit per semantic command, including empty-output commands.
///
/// # Algorithm
/// Walk commands and advance independent fact/diagnostic cursors while their origin matches the
/// command. Save each consumed half-open range together with the full span and shared schema
/// identity.
///
/// # Flow
/// ```text
/// [command order]
///        |
///  [save cursor starts]
///       /            \
///  [matching facts] [matching diagnostics]
///       |            |
///       +----[save ranges/schema]
///                       |
///                [next command]
/// ```
///
/// # Examples
/// ```text
/// let cached_units = units(semantic, &build);
/// ```
fn units(semantic: &SemanticedBuffer, buffer: &BuildStruct) -> Vec<BuildUnit> {
    let (mut facts, mut diagnostics) = (0, 0);
    semantic
        .commands
        .iter()
        .map(|command| {
            let first_fact = facts;
            let first_diagnostic = diagnostics;
            while buffer
                .facts
                .get(facts)
                .is_some_and(|f| f.source.command == command.id)
            {
                facts += 1;
            }
            while buffer
                .diagnostics
                .get(diagnostics)
                .is_some_and(|d| d.source.command == command.id)
            {
                diagnostics += 1;
            }
            BuildUnit {
                span: command.full_span,
                schema: command.schema.clone(),
                facts: first_fact..facts,
                diagnostics: first_diagnostic..diagnostics,
            }
        })
        .collect()
}
