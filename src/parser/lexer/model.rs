//! Lexer storage, token kinds, errors and whole-unit checkpoints; no I/O.
//! Tokens retain full raw byte spans; payload access removes wrappers on demand.
//! Contains AI-generated documentation additions.

use crate::model::{self, Spanned, TextSpan, trait1::ICursor};

/// Borrowed UTF-8 input and a movable byte cursor; no I/O.
/// [`Lexer::new`] consumes this wrapper, not the source string.
/// [`RawBufferCursor`] locates the next byte; [`Self::span_to_text`] reads raw spans.
///
/// # Examples
/// ```
/// use cmake_analyzer::parser::lexer::model::RawBuffer;
/// let buffer = RawBuffer::new("set(A x)");
/// assert_eq!(buffer.cursor.offset, 0);
/// ```
#[derive(Debug)]
pub struct RawBuffer<'a> {
    /// Source borrowed for this scan; never edited. Example: `assert_eq!(buffer.text, "set(A x)")`.
    pub text: &'a str,
    /// Next unread byte and recorded total length. Example: `buffer.cursor.offset = 4` moves to `A` in `set(A x)`.
    pub cursor: RawBufferCursor,
}

/// Byte cursor used by [`RawBuffer`] and [`Lexer`]; no side effects on construction.
/// [`ICursor::to_span`] rescans a source prefix for scalar coordinates.
/// Offset and length are public and not automatically validated. Scanners use source
/// length directly; the recorded length does not bound their reads.
///
/// # Examples
/// ```
/// use cmake_analyzer::parser::lexer::model::RawBufferCursor;
/// let mut cursor = RawBufferCursor::new_with_len(3);
/// cursor.offset = 3;
/// ```
#[derive(Default, Debug)]
pub struct RawBufferCursor {
    /// Next unread UTF-8 byte offset; use scalar boundaries. Example: `cursor.offset = 3` skips `中`.
    pub offset: usize,
    /// Recorded total source byte length, not remaining length. Example: `assert_eq!(RawBufferCursor::new_with_len(3).length, 3)`.
    pub length: usize,
}

/// Mutable scanner over a borrowed [`RawBuffer`]; scanning advances its cursor only.
/// Inspection CLIs call [`Self::parse`]; [`IncrementalLexer`] resumes the shared scanner
/// at checkpoints. [`LexedBuffer`] owns spans/kinds but not source text.
///
/// # Flow
/// ```text
///  [borrowed source]       [byte cursor]
///         +--------------------+
///                     |
///                  [scan]
///              /      |       \
///         [tokens] [comments] [errors]
///              \      |       /
///                [LexedBuffer]
/// ```
///
/// # Examples
/// ```
/// use cmake_analyzer::parser::lexer::model::{Lexer, RawBuffer};
/// let result = Lexer::new(RawBuffer::new("set(A x)")).parse();
/// assert_eq!(result.lex.len(), 5);
/// ```
#[derive(Debug)]
pub struct Lexer<'a> {
    /// Source and advancing cursor owned by this scanner. Example: `lexer.buffer.cursor.offset = 0` restarts byte reading.
    pub buffer: RawBuffer<'a>,
}

/// Owned lexical output with source-backed ranges; no source mutation or I/O.
/// [`crate::parser::nparser::model::NParser`] consumes the token vector and inherits errors.
/// Comments remain separate so formatting and documentation can preserve them.
/// [`Self::parenthesis_depth`] queries checkpoint state for completion.
///
/// # Examples
/// ```
/// use cmake_analyzer::parser::lexer::model::{Lexer, RawBuffer};
/// let result = Lexer::new(RawBuffer::new("# note\nset(A x)")).parse();
/// assert_eq!(result.comments.len(), 1);
/// ```
#[derive(Debug, Default, Clone)]
pub struct LexedBuffer {
    /// Full line/bracket comment ranges, including `#` and bracket delimiters; line
    /// comments exclude LF. Kept outside tokens. Example: `assert_eq!(result.comments.len(), 1)` for `# note`.
    pub comments: Vec<TextSpan>,
    /// Source-order tokens with full raw ranges, including quotes/brackets.
    /// Example: `result.lex[0].text("set(A x)")` borrows the command name.
    pub lex: Vec<Spanned<LexContent>>,
    /// Lexical diagnostics in emission order; unterminated units may stop scanning.
    /// Example: `result.errors.iter().any(|e| matches!(e.content, LexError::UnclosedParentheses))`.
    pub errors: Vec<Spanned<LexError>>,
    /// Whole-unit boundaries before whitespace, with cumulative vector lengths.
    /// Example inside this module: `result.checkpoints[0].tokens == 0`.
    pub(super) checkpoints: Vec<LexCheckpoint>,
}

// AI-authored incremental state; checkpoints are outside complete lexical units.
/// Resumable state outside a complete token/comment; no side effects.
/// [`Lexer::parse_until`] records it; [`IncrementalLexer::update`] checks suffix agreement.
/// Depth plus the relevant last opener prevent reuse across changed nesting.
///
/// # Examples
/// ```text
/// let state = LexState { depth: 0, ..Default::default() };
/// ```
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub(super) struct LexState {
    /// Zero-width location of the next byte before whitespace.
    /// Example: `LexState { position: TextSpan::default(), ..Default::default() }`.
    pub position: TextSpan,
    /// Unmatched left-parenthesis count, ignoring payloads/comments.
    /// Example: `LexState { depth: 1, ..Default::default() }` resumes inside parentheses.
    pub depth: usize,
    /// Most recently scanned opening span; not a stack of outstanding openers.
    /// Retained even at depth zero. Example: `state.last_opening = opening_span`.
    pub last_opening: TextSpan,
}
/// Scanner state plus output lengths at one whole-unit boundary; no side effects.
/// [`IncrementalLexer`] splices prefix/chunk/suffix vectors using these counts.
/// [`LexState`] supplies state agreement, not just byte alignment.
///
/// # Examples
/// ```text
/// let checkpoint = LexCheckpoint { state: LexState::default(), tokens: 0, errors: 0, comments: 0 };
/// ```
#[derive(Clone, Copy, Debug)]
pub(super) struct LexCheckpoint {
    /// State before scanning the next unit and its leading whitespace.
    /// Example: `checkpoint.state.depth == 0` tests top-level state.
    pub state: LexState,
    /// Token vector length at this boundary. Example: `let prefix = &result.lex[..checkpoint.tokens]`.
    pub tokens: usize,
    /// Diagnostic vector length at this boundary. Example: `let prefix = &result.errors[..checkpoint.errors]`.
    pub errors: usize,
    /// Comment vector length at this boundary. Example: `let prefix = &result.comments[..checkpoint.comments]`.
    pub comments: usize,
}
/// Per-file lexical cache with immutable shared outputs; updates replace state, no I/O.
/// Workspace passes [`Self::update`] changes to the incremental parser.
/// [`LexedBuffer`] retains checkpoint state; [`crate::model::incremental::SourceChange`]
/// marks changed bytes. Old `Arc` buffers remain valid for retained snapshots.
/// Source comparison, position indexes and output materialization can remain linear.
///
/// # Flow
/// ```text
///  [old source + checkpoints]     [new source]
///              \                     /
///                   [change envelope]
///                           |
///                 [rescan until alignment]
///                     /            \
///              [new chunk]     [reused suffix]
///                     \            /
///                     [replace Arc buffer]
/// ```
///
/// # Examples
/// ```
/// use cmake_analyzer::parser::lexer::model::IncrementalLexer;
/// let mut lexer = IncrementalLexer::new("set(A x)");
/// let old = lexer.buffer.clone();
/// lexer.update("set(A y)");
/// assert_eq!(old.lex[2].text("set(A x)"), Some("A"));
/// ```
pub struct IncrementalLexer {
    /// Current owned source snapshot, shared with clients. Example: `assert_eq!(&*lexer.source, "set(A x)")` before an update.
    pub source: std::sync::Arc<str>,
    /// Current immutable lexical result; replaced, not mutated, by update. Example: `let retained = lexer.buffer.clone()`.
    pub buffer: std::sync::Arc<LexedBuffer>,
    /// Latest token recomputation/reuse counts; excludes comments, errors and copying.
    /// Example: `let scanned = lexer.work.recomputed`.
    pub work: crate::model::incremental::IncrementalWork,
}

/// Kind of a source-backed token; construction has no side effects.
/// [`LexedBuffer`] stores it in [`Spanned`]; payload access lives on that wrapper
/// and strips only outer delimiters, never escapes or variable expressions.
///
/// # Examples
/// ```
/// use cmake_analyzer::parser::lexer::model::{LexContent, Lexer, RawBuffer};
/// let result = Lexer::new(RawBuffer::new("message(hi)")).parse();
/// assert_eq!(result.lex[0].content, LexContent::Identifier);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LexContent {
    /// Unquoted word, including legacy embedded quotes/escapes, without name validation.
    /// Example: `matches!(LexContent::Identifier, LexContent::Identifier)`.
    Identifier,
    /// Opening `(` outside literals/comments. Example: `let opening = LexContent::LeftParentheses`.
    LeftParentheses,
    /// Closing `)` with a preceding unmatched opener. Example: `let closing = LexContent::RightParentheses`.
    RightParentheses,
    /// Quoted token; raw span includes both quotes, payload access omits them.
    /// Example: `let quoted = LexContent::StringLiteral` classifies `"hi"`.
    StringLiteral,
    /// Bracket token; raw span keeps delimiters and initial newline. Payload access
    /// omits both delimiters and at most one initial LF/CRLF. Example: `let bracket = LexContent::BracketArgument`.
    BracketArgument,
}

/// Lexical failure kind, located by the surrounding [`Spanned`]; no side effects.
/// [`Lexer::parse`] collects failures in [`LexedBuffer::errors`]; low-level scanners
/// return [`LexResult`]. Normal parse EOF is not itself a diagnostic.
///
/// # Examples
/// ```
/// use cmake_analyzer::parser::lexer::model::LexError;
/// let error = LexError::UnclosedStringLiteral;
/// assert!(matches!(error, LexError::UnclosedStringLiteral));
/// ```
#[derive(Debug, Clone)]
pub enum LexError {
    /// Low-level read has no character or cannot slice at the cursor.
    /// Example: `let end = LexError::ReachTheEof`; parse treats ordinary EOF as success.
    ReachTheEof,
    /// Unexpected right parenthesis, or remaining left parentheses at scan end.
    /// The diagnostic points to the unexpected close or last scanned open, respectively.
    /// Example: `let nesting = LexError::UnclosedParentheses`.
    UnclosedParentheses,
    /// Quoted text reaches EOF before its closing quote.
    /// Example: `let quoted = LexError::UnclosedStringLiteral` for `"unfinished`.
    UnclosedStringLiteral,
    /// Bracket argument reaches EOF without an exact closing delimiter.
    /// Example: `let bracket = LexError::UnclosedBracketArgument` for `[=[unfinished`.
    UnclosedBracketArgument,
    // AI-generated diagnostic.
    /// Bracket comment reaches EOF; its full range is also retained as a comment.
    /// Example: `let comment = LexError::UnclosedBracketComment` for `#[=[unfinished`.
    UnclosedBracketComment,
}

/// Low-level scanner result carrying a source-located [`LexError`]; no side effects.
/// Example: `let ok: LexResult<()> = Ok(())` represents a completed cursor operation.
pub type LexResult<T> = Result<T, Spanned<LexError>>;

impl<'a> RawBuffer<'a> {
    /// Rescan the prefix to locate the cursor without advancing it; no side effects.
    ///
    /// # Arguments
    /// - `self` `[in]`: source/cursor wrapper; unchanged.
    ///
    /// # Returns
    /// Zero-width byte span with LF row and scalar column. Uses cursor offset even if manually set outside the source.
    ///
    /// # Examples
    /// ```
    /// use cmake_analyzer::parser::lexer::model::RawBuffer;
    /// let buffer = RawBuffer::new("中x");
    /// assert_eq!(buffer.current_byte_to_span().range(), 0..0);
    /// ```
    pub fn current_byte_to_span(&self) -> model::TextSpan {
        self.cursor.to_span(self.text)
    }

    /// Read total source byte length, not remaining length; no side effects.
    ///
    /// # Arguments
    /// - `self` `[in]`: borrowed source wrapper; unchanged.
    ///
    /// # Returns
    /// Source UTF-8 byte count, independent of recorded cursor length.
    ///
    /// # Examples
    /// ```
    /// use cmake_analyzer::parser::lexer::model::RawBuffer;
    /// let buffer = RawBuffer::new("中x");
    /// assert_eq!(buffer.buffer_advance_length(), 4);
    /// ```
    pub fn buffer_advance_length(&self) -> usize {
        self.text.len()
    }

    /// Borrow the full raw range without moving the cursor; no side effects.
    ///
    /// # Arguments
    /// - `self` `[in]`: wrapper borrowing the source; unchanged.
    /// - `span` `[in]`: half-open byte range to validate.
    ///
    /// # Returns
    /// `Some` source slice for valid ordered UTF-8 boundaries; `None` otherwise.
    ///
    /// # Examples
    /// ```
    /// use cmake_analyzer::parser::lexer::model::RawBuffer;
    /// let buffer = RawBuffer::new("中x");
    /// use cmake_analyzer::model::TextSpan;
    /// assert_eq!(buffer.span_to_text(TextSpan { end_byte: 3, ..Default::default() }), Some("中"));
    /// ```
    pub fn span_to_text(&self, span: TextSpan) -> Option<&'a str> {
        span.text(self.text)
    }
}

impl<'a> RawBuffer<'a> {
    /// Borrow source and initialize the cursor at zero; no side effects.
    ///
    /// # Arguments
    /// - `content` `[in]`: UTF-8 source retained by reference, not copied.
    ///
    /// # Returns
    /// Wrapper recording the source byte length.
    ///
    /// # Examples
    /// ```
    /// use cmake_analyzer::parser::lexer::model::RawBuffer;
    /// let buffer = RawBuffer::new("中");
    /// assert_eq!(buffer.cursor.length, 3);
    /// ```
    pub fn new(content: &'a str) -> Self {
        RawBuffer {
            text: content,
            cursor: RawBufferCursor::new_with_len(content.len()),
        }
    }
}

impl ICursor for RawBufferCursor {
    /// Rescan source characters before this cursor; no mutation or I/O.
    ///
    /// # Arguments
    /// - `self` `[in]`: cursor whose offset is used; length is ignored.
    /// - `content` `[in]`: source used for LF rows and scalar columns.
    ///
    /// # Returns
    /// Point span at offset. Invalid public offsets are not rejected; coordinates count characters starting before offset.
    ///
    /// # Examples
    /// ```
    /// use cmake_analyzer::{model::trait1::ICursor, parser::lexer::model::RawBufferCursor};
    /// let mut cursor = RawBufferCursor::new_with_len(4);
    /// cursor.offset = 3;
    /// assert_eq!(cursor.to_span("中x").column, 1);
    /// ```
    fn to_span(&self, content: &str) -> model::TextSpan {
        let mut span = TextSpan::default();
        for (offset, ch) in content.char_indices() {
            if offset >= self.offset {
                break;
            }
            if ch == '\n' {
                span.row += 1;
                span.column = 0;
            } else {
                span.column += 1;
            }
        }
        span.start_byte = self.offset;
        span.end_byte = self.offset;

        span
    }
}

impl RawBufferCursor {
    /// Create a byte cursor at the beginning; no side effects.
    ///
    /// # Arguments
    /// - `len` `[in]`: total source byte length to record.
    ///
    /// # Returns
    /// Cursor with offset zero and the supplied length.
    ///
    /// # Examples
    /// ```
    /// use cmake_analyzer::parser::lexer::model::RawBufferCursor;
    /// assert_eq!(RawBufferCursor::new_with_len(3).offset, 0);
    /// ```
    pub fn new_with_len(len: usize) -> Self {
        RawBufferCursor {
            offset: 0,
            length: len,
        }
    }
}

impl LexedBuffer {
    /// Append one located diagnostic without changing tokens or source; no I/O.
    ///
    /// # Arguments
    /// - `self` `[in,out]`: buffer whose errors vector gains one entry; prior entries remain.
    /// - `error` `[in]`: failure kind moved into the entry.
    /// - `span` `[in]`: diagnostic location copied unchanged.
    ///
    /// # Returns
    /// `()`; appends only to errors.
    ///
    /// # Examples
    /// ```
    /// use cmake_analyzer::{model::TextSpan, parser::lexer::model::{LexedBuffer, LexError}};
    /// let mut buffer = LexedBuffer::default();
    /// buffer.add_error(LexError::ReachTheEof, TextSpan::default());
    /// assert_eq!(buffer.errors.len(), 1);
    /// ```
    pub fn add_error(&mut self, error: LexError, span: TextSpan) {
        self.errors.push(Spanned {
            content: error,
            span,
        })
    }

    /// Append one token without changing source or checkpoint state; no I/O.
    ///
    /// # Arguments
    /// - `self` `[in,out]`: buffer whose token vector gains one entry.
    /// - `lex` `[in]`: located token moved into the vector.
    ///
    /// # Returns
    /// `()`; prior tokens remain in order.
    ///
    /// # Examples
    /// ```
    /// use cmake_analyzer::{model::{Spanned, TextSpan}, parser::lexer::model::{LexedBuffer, LexContent}};
    /// let mut buffer = LexedBuffer::default();
    /// buffer.add_lex(Spanned::new(LexContent::Identifier, TextSpan::default()));
    /// assert_eq!(buffer.lex.len(), 1);
    /// ```
    pub fn add_lex(&mut self, lex: Spanned<LexContent>) {
        self.lex.push(lex);
    }
}
