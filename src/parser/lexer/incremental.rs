//! Reuse lexical prefixes/suffixes only at whole-unit boundaries with matching state.
//! Updates replace shared buffers, preserving retained snapshots; no I/O.
//! Contains AI-generated documentation additions.

// AI-authored incremental lexer.
use super::model::*;
use crate::model::incremental::{IncrementalWork, SourceChange, SourcePositions};
use std::{collections::HashMap, sync::Arc};

impl IncrementalLexer {
    /// Fully lex an owned/shared source snapshot and initialize work counts; no I/O.
    ///
    /// # Arguments
    /// - `source` `[in]`: source converted into `Arc<str>`; conversion may allocate for borrowed input.
    ///
    /// # Returns
    /// Cache with immutable lexical output, recomputed equal to emitted token count and reused zero.
    ///
    /// # Examples
    /// ```
    /// use cmake_analyzer::parser::lexer::model::IncrementalLexer;
    /// let lexer = IncrementalLexer::new("f()");
    /// assert_eq!(lexer.work.recomputed, 3);
    /// assert_eq!(lexer.work.reused, 0);
    /// ```
    pub fn new(source: impl Into<Arc<str>>) -> Self {
        let source = source.into();
        let buffer = Lexer::new(RawBuffer::new(&source)).parse();
        let work = IncrementalWork {
            recomputed: buffer.lex.len(),
            reused: 0,
        };
        Self {
            source,
            buffer: Arc::new(buffer),
            work,
        }
    }
    /// Refresh lexical output by rescanning a changed region and remapping reusable ranges; no I/O.
    /// Replaces source/buffer/work on a changed snapshot. Equal text only updates work,
    /// keeping the existing source/buffer Arcs; retained old buffers are never mutated.
    ///
    /// # Arguments
    /// - `self` `[in,out]`: cache whose current state is replaced and work counts overwritten.
    /// - `source` `[in]`: new complete snapshot converted into `Arc<str>`, not a text-edit operation.
    ///
    /// # Returns
    /// UTF-8-safe change envelope connecting old and new text for downstream caches. Equal text yields empty EOF ranges.
    ///
    /// # Algorithm
    /// Find the byte replacement envelope. Resume from the last whole-unit checkpoint
    /// strictly before it, or default state at zero. Scan with the full lexer until an
    /// unchanged suffix checkpoint has the same depth and, when depth is nonzero, the
    /// same remapped most-recent opener. Without alignment, scan through EOF.
    /// Splice prefix/chunk/suffix tokens, comments, errors and checkpoint counts; rebuild
    /// suffix coordinates from the new source. Work counts tokens, not total CPU cost.
    ///
    /// # Flow
    /// ```text
    ///                 [compare source]
    ///                        |
    ///                   <identical?>
    ///                   /         \
    ///                 yes         no
    ///                  |           |
    ///         [work: all reused] [envelope + position index]
    ///                  |           |
    ///               [return] [checkpoint before edit]
    ///                              |
    ///                        [scan whole units] <------+
    ///                              |                  |
    ///                        <suffix/state match?> ---+ no, more input
    ///                          /            \
    ///                        yes          [EOF]
    ///                         |              |
    ///                [remap suffix]    [no suffix reuse]
    ///                         +------+-------+
    ///                                |
    ///                      [replace Arc output/work]
    /// ```
    ///
    /// # Panics
    /// The public cache fields must remain mutually consistent. Manually replacing
    /// source or buffer independently can invalidate checkpoint slices and cursor bounds.
    ///
    /// # Examples
    /// ```
    /// use cmake_analyzer::parser::lexer::model::IncrementalLexer;
    /// let mut lexer = IncrementalLexer::new("f(x) g(y)");
    /// let old = lexer.buffer.clone();
    /// let change = lexer.update("f(z) g(y)");
    /// assert_eq!(change.old, 2..3);
    /// assert_eq!(old.lex[2].text("f(x) g(y)"), Some("x"));
    /// ```
    pub fn update(&mut self, source: impl Into<Arc<str>>) -> SourceChange {
        let source = source.into();
        let change = SourceChange::between(&self.source, &source);
        if self.source == source {
            self.work = IncrementalWork {
                recomputed: 0,
                reused: self.buffer.lex.len(),
            };
            return change;
        }
        let positions = SourcePositions::new(&source);
        let start = self
            .buffer
            .checkpoints
            .iter()
            .rev()
            .find(|c| c.state.position.start_byte < change.old.start)
            .copied()
            .unwrap_or(LexCheckpoint {
                state: LexState::default(),
                tokens: 0,
                errors: 0,
                comments: 0,
            });
        let suffix: HashMap<_, _> = self
            .buffer
            .checkpoints
            .iter()
            .enumerate()
            .filter_map(|(i, c)| {
                (c.state.position.start_byte >= change.old.end)
                    .then(|| {
                        change
                            .offset(c.state.position.start_byte)
                            .map(|offset| (offset, i))
                    })
                    .flatten()
            })
            .collect();
        let mut aligned = None;
        let mut raw = RawBuffer::new(&source);
        raw.cursor.offset = start.state.position.start_byte;
        let chunk = Lexer::new(raw).parse_until(start.state, |state| {
            if state.position.start_byte < change.new.end {
                return false;
            }
            let Some(&index) = suffix.get(&state.position.start_byte) else {
                return false;
            };
            let old = self.buffer.checkpoints[index].state;
            let same = state.depth == old.depth
                && (state.depth == 0
                    || positions.remap(&change, old.last_opening) == Some(state.last_opening));
            if same {
                aligned = Some(index);
            }
            same
        });
        let work = IncrementalWork {
            recomputed: chunk.lex.len(),
            reused: start.tokens
                + aligned.map_or(0, |i| {
                    self.buffer.lex.len() - self.buffer.checkpoints[i].tokens
                }),
        };
        let mut comments = self.buffer.comments[..start.comments].to_vec();
        comments.extend(chunk.comments);
        let mut lex = self.buffer.lex[..start.tokens].to_vec();
        lex.extend(chunk.lex);
        let mut errors = self.buffer.errors[..start.errors].to_vec();
        errors.extend(chunk.errors);
        let mut checkpoints: Vec<_> = self
            .buffer
            .checkpoints
            .iter()
            .copied()
            .take_while(|c| c.state.position.start_byte < start.state.position.start_byte)
            .collect();
        checkpoints.extend(chunk.checkpoints.into_iter().map(|mut c| {
            c.tokens += start.tokens;
            c.errors += start.errors;
            c.comments += start.comments;
            c
        }));
        if let Some(index) = aligned {
            let old = self.buffer.checkpoints[index];
            let token_base = lex.len();
            let error_base = errors.len();
            let comment_base = comments.len();
            comments.extend(
                self.buffer.comments[old.comments..]
                    .iter()
                    .filter_map(|span| positions.remap(&change, *span)),
            );
            lex.extend(self.buffer.lex[old.tokens..].iter().filter_map(|token| {
                positions
                    .remap(&change, token.span)
                    .map(|span| crate::model::Spanned {
                        content: token.content,
                        span,
                    })
            }));
            errors.extend(self.buffer.errors[old.errors..].iter().filter_map(|error| {
                positions
                    .remap(&change, error.span)
                    .map(|span| crate::model::Spanned {
                        content: error.content.clone(),
                        span,
                    })
            }));
            for c in &self.buffer.checkpoints[index + 1..] {
                let Some(position) = change
                    .offset(c.state.position.start_byte)
                    .map(|b| positions.span(b..b))
                else {
                    continue;
                };
                let last_opening = positions
                    .remap(&change, c.state.last_opening)
                    .unwrap_or_default();
                checkpoints.push(LexCheckpoint {
                    state: LexState {
                        position,
                        last_opening,
                        depth: c.state.depth,
                    },
                    tokens: token_base + c.tokens - old.tokens,
                    errors: error_base + c.errors - old.errors,
                    comments: comment_base + c.comments - old.comments,
                });
            }
        }
        self.source = source;
        self.buffer = Arc::new(LexedBuffer {
            comments,
            lex,
            errors,
            checkpoints,
        });
        self.work = work;
        change
    }
}
