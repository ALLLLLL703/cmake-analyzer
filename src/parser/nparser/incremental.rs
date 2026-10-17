//! Reuse unchanged closed command units and rebuild diagnostics/comments; no I/O.
//! This still clones tokens and builds a position index for each update.
//! Contains AI-generated documentation additions.

// AI-authored command-level incremental parser.
use super::model::*;
use crate::{
    model::incremental::{IncrementalWork, SourceChange, SourcePositions},
    parser::lexer::model::LexedBuffer,
};
use std::collections::HashMap;

impl IncrementalNParser {
    /// Parse lexical output and seed reusable closed-command units; no I/O.
    ///
    /// # Arguments
    /// - `source` `[in]`: source matching the lexical spans; not retained.
    /// - `lexed` `[in]`: current lexical output, borrowed unchanged.
    ///
    /// # Returns
    /// Cache with parsed commands/errors/comments, reusable closed units and initial work counts.
    ///
    /// # Examples
    /// ```
    /// use cmake_analyzer::parser::{lexer::model::IncrementalLexer, nparser::model::IncrementalNParser};
    /// let lexer = IncrementalLexer::new("f()");
    /// let parser = IncrementalNParser::new(&lexer.source, &lexer.buffer);
    /// assert_eq!(parser.work.recomputed, 1);
    /// ```
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
    /// Replace parsed output and cache units, reusing mapped closed commands; no I/O.
    /// Lexical errors/comments are always inherited from current input, not cached units.
    ///
    /// # Arguments
    /// - `self` `[in,out]`: parser cache whose buffer, units and work are overwritten.
    /// - `source` `[in]`: new source snapshot used for scalar positions; not retained.
    /// - `lexed` `[in]`: lexical output for that source; unchanged.
    /// - `change` `[in]`: envelope from the cached source to the current source; must match both.
    ///
    /// # Returns
    /// `()`; current output/cache/work replaced. Recomputed counts parse_next attempts; reused counts closed commands.
    ///
    /// # Algorithm
    /// Map old closed-unit ranges that avoid the replacement. Walk current tokens:
    /// reuse a unit only when its start, final byte and consumed token count agree; clone
    /// and remap its node. Otherwise call the ordinary one-unit parser and cache only
    /// a single closed-command result. Partial/error units are parsed again. Token
    /// vectors are cloned and source positions rebuilt even when every command is reused.
    ///
    /// # Flow
    /// ```text
    ///  [old closed units]     [current lexer]
    ///           |                    |
    ///   [map ranges]        [copy comments/errors]
    ///           +---------+----------+
    ///                     |
    ///              [next token start] <-----------+
    ///                     |                      |
    ///       <mapped end + token count agree?>     |
    ///              /                \
    ///            yes                no
    ///             |                  |
    ///    [clone/remap node]      [parse_next]
    ///             |                  |
    ///             |         [cache only single closed node]
    ///             +--------+---------+
    ///                      | more tokens
    ///                      +---------------------+
    ///                      | EOF
    ///             [overwrite buffer/units/work]
    /// ```
    ///
    /// # Panics
    /// Inconsistent manually constructed cache units, source changes or lexical input
    /// can invalidate token arithmetic or position mapping. Use the matching lexer
    /// change and source; do not mutate cache fields independently.
    ///
    /// # Examples
    /// ```
    /// use cmake_analyzer::parser::{lexer::model::IncrementalLexer, nparser::model::IncrementalNParser};
    /// let mut lexer = IncrementalLexer::new("f(x) g(y)");
    /// let mut parser = IncrementalNParser::new(&lexer.source, &lexer.buffer);
    /// let change = lexer.update("f(z) g(y)");
    /// parser.update(&lexer.source, &lexer.buffer, &change);
    /// assert_eq!(parser.work.reused, 1);
    /// ```
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
/// Relocate a cached command node and its name/argument spans; no I/O.
///
/// # Arguments
/// - `node` `[in,out]`: node spans overwritten only when each mapping succeeds; payload kinds unchanged.
/// - `change` `[in]`: old-to-new replacement envelope.
/// - `positions` `[in]`: position index for the new source.
///
/// # Returns
/// `()`; failed individual mappings leave their old spans untouched. Does not remap error payloads or Unclosed opening spans; caller only reuses closed commands.
///
/// # Examples
/// ```text
/// let change = SourceChange::between("f()", " f()");
/// let positions = SourcePositions::new(" f()");
/// remap_node(&mut cached_node, &change, &positions);
/// ```
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
