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
                                effect: fact.effect.clone(),
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
        self.work = IncrementalWork {
            recomputed: changed.len(),
            reused: reused.len(),
        };
        self.units = units(semantic, &buffer);
        self.buffer = Arc::new(buffer);
    }
}
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
