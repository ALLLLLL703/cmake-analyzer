// AI-authored command-level incremental parser.
use super::model::*;
use crate::{
    model::incremental::{IncrementalWork, SourceChange, SourcePositions},
    parser::lexer::model::LexedBuffer,
};
use std::collections::HashMap;

impl IncrementalNParser {
    pub fn new(source: &str, lexed: &LexedBuffer) -> Self {
        let mut cache = Self {
            buffer: NParsedBuffer::default(),
            units: Vec::new(),
            work: Default::default(),
        };
        cache.update(
            source,
            lexed,
            &SourceChange {
                old: 0..0,
                new: 0..source.len(),
            },
        );
        cache
    }
    pub fn update(&mut self, source: &str, lexed: &LexedBuffer, change: &SourceChange) {
        let positions = SourcePositions::new(source);
        let reusable: HashMap<_, _> = self
            .units
            .iter()
            .filter_map(|unit| {
                change
                    .range(unit.span.start_byte..unit.span.end_byte)
                    .map(|range| (range.start, (range.end, unit)))
            })
            .collect();
        let mut tokens = LexedBuffer::default();
        tokens.lex = lexed.lex.clone();
        tokens.errors = lexed.errors.clone();
        let mut parser = NParser::new(tokens);
        let mut result = NParsedBuffer {
            comments: lexed.comments.clone(),
            ..Default::default()
        };
        parser.inherit_error(&mut result);
        let mut units = Vec::new();
        let mut work = IncrementalWork::default();
        while let Some(first) = parser.peek() {
            if let Some(&(end, unit)) = reusable.get(&first.span.start_byte) {
                let next = parser
                    .buffer
                    .lex
                    .partition_point(|token| token.span.start_byte < end);
                if next - parser.cursor.offset == unit.tokens
                    && parser
                        .buffer
                        .lex
                        .get(next.saturating_sub(1))
                        .is_some_and(|t| t.span.end_byte == end)
                {
                    let mut node = unit.node.clone();
                    remap_node(&mut node, change, &positions);
                    result.nodes.push(node.clone());
                    units.push(ParsedUnit {
                        span: node.span,
                        tokens: unit.tokens,
                        node,
                    });
                    parser.cursor.offset = next;
                    work.reused += 1;
                    continue;
                }
            }
            let start = parser.cursor.offset;
            let before = result.nodes.len();
            parser.parse_next(&mut result);
            work.recomputed += 1;
            if result.nodes.len() == before + 1 {
                let node = &result.nodes[before];
                if matches!(&node.content,NParsedNode::Command(command) if command.closed==CommandState::Closed)
                {
                    units.push(ParsedUnit {
                        span: node.span,
                        tokens: parser.cursor.offset - start,
                        node: node.clone(),
                    });
                }
            }
        }
        self.buffer = result;
        self.units = units;
        self.work = work;
    }
}
fn remap_node(
    node: &mut crate::model::Spanned<NParsedNode>,
    change: &SourceChange,
    positions: &SourcePositions,
) {
    if let Some(span) = positions.remap(change, node.span) {
        node.span = span;
    }
    if let NParsedNode::Command(command) = &mut node.content {
        if let Some(span) = positions.remap(change, command.name.span) {
            command.name.span = span;
        }
        for arg in &mut command.args {
            if let Some(span) = positions.remap(change, arg.span) {
                arg.span = span;
            }
        }
    }
}
