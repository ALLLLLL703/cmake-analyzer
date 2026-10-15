// AI-authored incremental lexer.
use super::model::*;
use crate::model::incremental::{IncrementalWork, SourceChange, SourcePositions};
use std::{collections::HashMap, sync::Arc};

impl IncrementalLexer {
    pub fn new(source: impl Into<Arc<str>>) -> Self {
        let source = source.into();
        let buffer = Lexer::new(RawBuffer::new(&source)).parse();
        let work = IncrementalWork {
            recomputed: buffer.lex.len(),
            reused: 0,
        };
        Self {
            source,
            buffer,
            work,
        }
    }
    /// Relex from the previous whole unit until the unchanged suffix has the same scan state.
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
        self.buffer = LexedBuffer {
            comments,
            lex,
            errors,
            checkpoints,
        };
        self.work = work;
        change
    }
}
