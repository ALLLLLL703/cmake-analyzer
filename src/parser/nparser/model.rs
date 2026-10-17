//! Command syntax storage and command-level cache records; no I/O.
//! Raw spans retain wrappers; argument text views remove them without evaluation.
//! Contains AI-generated documentation additions.

use crate::{
    model::{Spanned, TextSpan},
    parser::lexer::model::{LexContent, LexError, LexedBuffer},
};

// AI-authored incremental cache. Units contain only parser results, not inherited lexer errors.
/// Command-level cache rebuilt from current lexical output; no I/O.
/// Workspace supplies the [`crate::model::incremental::SourceChange`] from the lexer.
/// [`Self::update`] reuses only complete closed-command units and remaps their spans;
/// lexer errors/comments are copied afresh. [`NParser`] supplies the shared parser.
/// Token cloning and source indexing can remain linear despite command reuse.
///
/// # Flow
/// ```text
///  [old closed units]          [current lexer]
///           |                        |
///      [map unchanged]         [copy errors/comments]
///           |                        |
///           +----> [token walk] <-----+
///                   /       \
///              [reuse]    [parse_next]
///                   \       /
///                  [replace buffer/cache]
/// ```
///
/// # Examples
/// ```
/// use cmake_analyzer::parser::{lexer::model::IncrementalLexer, nparser::model::IncrementalNParser};
/// let lexer = IncrementalLexer::new("set(A x)");
/// let parser = IncrementalNParser::new(&lexer.source, &lexer.buffer);
/// assert_eq!(parser.work.recomputed, 1);
/// ```
pub struct IncrementalNParser {
    /// Current parsed output; overwritten by update. Example: `let nodes = parser.buffer.nodes.len()`.
    pub buffer: NParsedBuffer,
    /// Latest parse-attempt and reused-command counts, not token counts.
    /// Example: `let reused_commands = parser.work.reused`.
    pub work: crate::model::incremental::IncrementalWork,
    /// Reusable closed commands only; excludes inherited errors and partial commands.
    /// Example inside this module: `let reusable = parser.units.len()`.
    pub(super) units: Vec<ParsedUnit>,
}
/// Cached closed command and its token extent; no side effects.
/// [`IncrementalNParser`] compares mapped byte ends and token counts before reuse.
/// [`NParsedNode`] supplies the remappable command payload.
///
/// # Examples
/// ```text
/// let unit = ParsedUnit { span: node.span, tokens: 3, node };
/// ```
pub(super) struct ParsedUnit {
    /// Complete command byte envelope including its consumed outer close.
    /// Example: `let bytes = unit.span.range()`.
    pub span: TextSpan,
    /// Number of lexer tokens consumed by the command. Example: `unit.tokens == 3` for `f()`.
    pub tokens: usize,
    /// Located closed command retained for cloning/remapping. Example: `let cached = unit.node.clone()`.
    pub node: Spanned<NParsedNode>,
}

/// Mutable token cursor and owned lexical input; parsing advances only the cursor.
/// CLIs and [`IncrementalNParser`] use this to group commands, without evaluating CMake.
/// [`Self::parse`] returns [`NParsedBuffer`] and forwards lexical comments/errors.
///
/// # Examples
/// ```
/// use cmake_analyzer::parser::{lexer::model::{Lexer, RawBuffer}, nparser::model::NParser};
/// let lexed = Lexer::new(RawBuffer::new("f()")).parse();
/// let parsed = NParser::new(lexed).parse("f()");
/// assert_eq!(parsed.nodes.len(), 1);
/// ```
pub struct NParser {
    /// Owned lexical input; source strings are not retained.
    /// Example: `let token_count = parser.buffer.lex.len()`.
    pub buffer: LexedBuffer,
    /// Index of the next token to inspect. Example: `parser.cursor.offset = 0` restarts parsing.
    pub cursor: NParseCursor,
}

/// Token-index cursor for [`NParser`]; no side effects on construction.
/// Unlike byte spans, offsets index [`LexedBuffer::lex`]. [`NParser::new`] records
/// the initial token count; `peek` reads the vector rather than this maximum.
///
/// # Examples
/// ```
/// use cmake_analyzer::parser::nparser::model::NParseCursor;
/// let cursor = NParseCursor { offset: 0, max_length: 3 };
/// assert_eq!(cursor.offset, 0);
/// ```
#[derive(Default, Debug)]
pub struct NParseCursor {
    /// Next unread token index, not byte offset. Example: `cursor.offset = 1` skips a command name.
    pub offset: usize,
    /// Initial token count; parse short-circuits when zero. Example: `cursor.max_length = 3` records `f()` tokens.
    pub max_length: usize,
}

/// Owned commands and diagnostics with separately forwarded comments; no I/O.
/// [`NParser::parse`] appends inherited lexer errors before parser results, so nodes
/// are not globally source-sorted. [`crate::parser::semanticer::model::Semanticer`]
/// consumes this structure to interpret command names and arguments.
///
/// # Examples
/// ```
/// use cmake_analyzer::parser::nparser::model::NParsedBuffer;
/// let result = NParsedBuffer::default();
/// assert!(result.nodes.is_empty());
/// ```
#[derive(Default, Debug, Clone)]
pub struct NParsedBuffer {
    /// Original lexical comment ranges, never command arguments.
    /// Example: `let count = parsed.comments.len()` counts forwarded comments.
    pub comments: Vec<TextSpan>,
    /// Inherited errors followed by parser commands/errors; one attempt can emit
    /// multiple nodes or none. Example: `parsed.nodes.iter().filter(|n| matches!(n.content, NParsedNode::Command(_)))`.
    pub nodes: Vec<Spanned<NParsedNode>>,
}

/// A command or source-located syntax diagnostic; no side effects.
/// [`NParsedBuffer`] stores it in [`Spanned`]; semantic analysis visits commands
/// and forwards errors instead of stopping at the first failure.
///
/// # Examples
/// ```
/// use cmake_analyzer::parser::nparser::model::{NParsedNode, NParsedError};
/// let node = NParsedNode::new_error(NParsedError::ShouldBeCommand);
/// assert!(matches!(node, NParsedNode::Error(_)));
/// ```
#[derive(Clone, Debug)]
pub enum NParsedNode {
    /// Parsed complete or partial command. Example: `let node = NParsedNode::Command(command)`.
    Command(
        /// Command payload; the outer node span covers its source envelope.
        /// Example: `if let NParsedNode::Command(command) = node { let _ = command.args.len(); }`.
        NParsedCommand,
    ),
    /// Syntax failure; location lives on the outer node. Example: `let node = NParsedNode::Error(NParsedError::ShouldBeCommand)`.
    Error(
        /// Failure kind, not its source location.
        /// Example: `if let NParsedNode::Error(error) = node { let _ = error; }`.
        NParsedError,
    ),
}

/// Command name, flat raw argument forms and outer closure state; no evaluation or I/O.
/// [`NParser::consume_function_args`] retains inner parentheses as arguments but
/// excludes the outer pair. [`NParsedArgument`] text access strips literal wrappers;
/// [`CommandState`] retains partial calls for editor analysis.
///
/// # Examples
/// ```
/// use cmake_analyzer::{model::{Spanned, TextSpan}, parser::{lexer::model::LexContent, nparser::model::NParsedCommand}};
/// let command = NParsedCommand::new(Spanned::new(LexContent::Identifier, TextSpan::default()));
/// assert!(command.args.is_empty());
/// ```
#[derive(Debug, Clone)]
pub struct NParsedCommand {
    /// Original name token, normally an Identifier; not copied/lowercased here.
    /// Example: `command.name.text(source)` borrows the spelling.
    pub name: Spanned<LexContent>,
    /// Flat arguments in source order, including inner parenthesis tokens.
    /// Example: `command.args[0].text(source)` borrows a payload.
    pub args: Vec<Spanned<NParsedArgument>>,
    /// Whether the outer opening parenthesis was matched.
    /// Example: `command.closed == CommandState::Closed` tests completeness.
    pub closed: CommandState,
}

/// Outer command-parenthesis closure status; no side effects.
/// [`NParsedCommand`] keeps partial calls usable; [`IncrementalNParser`] caches only
/// `Closed` calls. The unclosed location is always the outer opener, not the last inner one.
///
/// # Examples
/// ```
/// use cmake_analyzer::parser::nparser::model::CommandState;
/// let state = CommandState::Closed;
/// assert_eq!(state, CommandState::Closed);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CommandState {
    /// Outer right parenthesis was consumed. Example: `command.closed = CommandState::Closed`.
    Closed,
    /// Input ended before the outer close; arguments collected so far are retained.
    /// Example: `let state = CommandState::Unclosed { opening_span: TextSpan::default() }`.
    Unclosed {
        /// Outer opening `(` range, not a nested argument opener.
        /// Example: `if let CommandState::Unclosed { opening_span } = state { let _ = opening_span.range(); }`.
        opening_span: TextSpan,
    },
}

/// Raw argument form, with bytes stored by its surrounding [`Spanned`]; no side effects.
/// [`NParsedCommand::args`] preserves source order. Text access delegates to
/// [`LexContent`] payload rules and does not expand variables or unescape strings.
///
/// # Examples
/// ```
/// use cmake_analyzer::parser::nparser::model::NParsedArgument;
/// let form = NParsedArgument::Quoted;
/// assert!(matches!(form, NParsedArgument::Quoted));
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NParsedArgument {
    /// Unquoted word, possibly containing escapes/embedded quotes. Example: `let form = NParsedArgument::Unquoted`.
    Unquoted,
    /// Double-quoted argument; raw span includes quotes. Example: `let form = NParsedArgument::Quoted`.
    Quoted,
    /// Bracket argument (existing API spelling); raw span includes delimiters.
    /// Example: `let form = NParsedArgument::Bracked` for `[=[raw]=]`.
    Bracked,
    /// Nested opening parenthesis retained as an argument. Example: `let form = NParsedArgument::LeftParen`.
    LeftParen,
    /// Nested closing parenthesis, not the command close. Example: `let form = NParsedArgument::RightParen`.
    RightParen,
}

/// Lexical or command-structure failure kind; no side effects.
/// [`NParsedBuffer::nodes`] stores source locations outside the enum.
/// [`NParser::consume_function_args`] also uses some variants as internal return signals.
///
/// # Examples
/// ```
/// use cmake_analyzer::parser::nparser::model::NParsedError;
/// let error = NParsedError::ShouldBeCommand;
/// assert!(matches!(error, NParsedError::ShouldBeCommand));
/// ```
#[derive(Debug, Clone)]
pub enum NParsedError {
    /// Forwarded lexical failure, preserving the lexer span.
    /// Example: `let error = NParsedError::LexError(LexError::UnclosedParentheses)`.
    LexError(
        /// Original lexer failure kind.
        /// Example: `if let NParsedError::LexError(kind) = error { let _ = kind; }`.
        LexError,
    ),
    /// Bare opening-parenthesis group without a command name. Example: `let error = NParsedError::ShouldBeCommand` for `()`.
    ShouldBeCommand,
    /// Missing opener after a name, or unexpected top-level close. A name at EOF
    /// currently emits no such node. Example: `let error = NParsedError::ShouldHaveLeftParentheses`.
    ShouldHaveLeftParentheses,
    /// Outer argument group reached EOF. Named groups retain a partial command;
    /// unnamed groups also emit `ShouldBeCommand` without a command node.
    /// Example: `let error = NParsedError::UnclosedLeftParentheses` for `f(`.
    UnclosedLeftParentheses,
    /// Argument reader was called with no token; usually an internal signal.
    /// Example: `let error = NParsedError::ReachTheEof`.
    ReachTheEof,
    /// Quoted token outside command arguments. Example: `let error = NParsedError::UnArgStringLiteral` for top-level `"x"`.
    UnArgStringLiteral,
    /// Argument reader was called on a token other than `(`; cursor is unchanged.
    /// Example: `let error = NParsedError::CalledInnerError`.
    CalledInnerError,
    /// Bracket token outside command arguments. Example: `let error = NParsedError::UnArgBracket` for top-level `[[x]]`.
    UnArgBracket,
}

impl NParser {
    /// Own lexical input and initialize a token cursor; no I/O.
    ///
    /// # Arguments
    /// - `buffer` `[in]`: lexical output moved into the parser, including errors/comments.
    ///
    /// # Returns
    /// Parser at token zero with the initial token count recorded.
    ///
    /// # Examples
    /// ```
    /// use cmake_analyzer::parser::{lexer::model::LexedBuffer, nparser::model::NParser};
    /// let parser = NParser::new(LexedBuffer::default());
    /// assert_eq!(parser.cursor.max_length, 0);
    /// ```
    pub fn new(buffer: LexedBuffer) -> Self {
        let cursor = NParseCursor {
            max_length: buffer.lex.len(),
            ..Default::default()
        };
        NParser { buffer, cursor }
    }
}

impl NParsedBuffer {
    /// Append a located command or error; no I/O or source changes.
    ///
    /// # Arguments
    /// - `self` `[in,out]`: output whose nodes vector gains one entry; existing entries remain.
    /// - `node` `[in]`: syntax payload moved into the vector.
    /// - `span` `[in]`: source location copied unchanged.
    ///
    /// # Returns
    /// `()`; appends only to nodes.
    ///
    /// # Examples
    /// ```
    /// use cmake_analyzer::{model::TextSpan, parser::nparser::model::{NParsedBuffer, NParsedNode, NParsedError}};
    /// let mut result = NParsedBuffer::default();
    /// result.add_node(NParsedNode::Error(NParsedError::ShouldBeCommand), TextSpan::default());
    /// assert_eq!(result.nodes.len(), 1);
    /// ```
    pub fn add_node(&mut self, node: NParsedNode, span: TextSpan) {
        self.nodes.push(Spanned {
            content: node,
            span,
        });
    }
}

impl NParsedCommand {
    /// Create an argument-free command marked closed; no side effects.
    ///
    /// # Arguments
    /// - `name` `[in]`: located token moved into the command; kind is not validated.
    ///
    /// # Returns
    /// Command with empty args and provisional `Closed` state.
    ///
    /// # Examples
    /// ```
    /// use cmake_analyzer::{model::{Spanned, TextSpan}, parser::{lexer::model::LexContent, nparser::model::{NParsedCommand, NParsedArgument}}};
    /// let name = Spanned::new(LexContent::Identifier, TextSpan::default());
    /// let command = NParsedCommand::new(name);
    /// assert!(command.args.is_empty());
    /// ```
    pub fn new(name: Spanned<LexContent>) -> Self {
        Self {
            name,
            args: Vec::new(),
            closed: CommandState::Closed,
        }
    }

    /// Append one argument without changing closure or name; no I/O.
    ///
    /// # Arguments
    /// - `self` `[in,out]`: command whose args vector gains one entry.
    /// - `argument` `[in]`: raw argument form.
    /// - `span` `[in]`: full raw byte range, including any wrappers.
    ///
    /// # Returns
    /// `()`; prior arguments remain ordered.
    ///
    /// # Examples
    /// ```
    /// use cmake_analyzer::{model::{Spanned, TextSpan}, parser::{lexer::model::LexContent, nparser::model::{NParsedCommand, NParsedArgument}}};
    /// let name = Spanned::new(LexContent::Identifier, TextSpan::default());
    /// let mut command = NParsedCommand::new(name);
    /// command.add_argument(NParsedArgument::Unquoted, TextSpan::default());
    /// assert_eq!(command.args.len(), 1);
    /// ```
    pub fn add_argument(&mut self, argument: NParsedArgument, span: TextSpan) {
        self.args.push(Spanned {
            content: argument,
            span,
        });
    }

    /// Move all supplied arguments onto this command; no I/O.
    ///
    /// # Arguments
    /// - `self` `[in,out]`: command whose argument vector is appended to, not overwritten.
    /// - `args` `[in,out]`: source vector drained to empty; its entries move in order.
    ///
    /// # Returns
    /// `()`; args is empty and the command retains all appended entries.
    ///
    /// # Examples
    /// ```
    /// use cmake_analyzer::{model::{Spanned, TextSpan}, parser::{lexer::model::LexContent, nparser::model::{NParsedCommand, NParsedArgument}}};
    /// let name = Spanned::new(LexContent::Identifier, TextSpan::default());
    /// let mut command = NParsedCommand::new(name);
    /// let mut args = vec![Spanned::new(NParsedArgument::Unquoted, TextSpan::default())];
    /// command.append_arg(&mut args);
    /// assert!(args.is_empty());
    /// ```
    pub fn append_arg(&mut self, args: &mut Vec<Spanned<NParsedArgument>>) {
        self.args.append(args);
    }
}

impl NParsedNode {
    /// Wrap a syntax failure without adding a source location; no side effects.
    ///
    /// # Arguments
    /// - `error` `[in]`: failure kind moved into the node.
    ///
    /// # Returns
    /// `Error` node; caller attaches its span separately.
    ///
    /// # Examples
    /// ```
    /// use cmake_analyzer::parser::nparser::model::{NParsedNode, NParsedError};
    /// let node = NParsedNode::new_error(NParsedError::UnArgBracket);
    /// assert!(matches!(node, NParsedNode::Error(_)));
    /// ```
    pub fn new_error(error: NParsedError) -> Self {
        NParsedNode::Error(error)
    }

    /// Wrap a command without adding a source location; no side effects.
    ///
    /// # Arguments
    /// - `content` `[in]`: command payload moved into the node.
    ///
    /// # Returns
    /// `Command` node; caller attaches its full span separately.
    ///
    /// # Examples
    /// ```
    /// use cmake_analyzer::{model::{Spanned, TextSpan}, parser::{lexer::model::LexContent, nparser::model::{NParsedCommand, NParsedArgument}}};
    /// let name = Spanned::new(LexContent::Identifier, TextSpan::default());
    /// use cmake_analyzer::parser::nparser::model::NParsedNode;
    /// let node = NParsedNode::new_command(NParsedCommand::new(name));
    /// assert!(matches!(node, NParsedNode::Command(_)));
    /// ```
    pub fn new_command(content: NParsedCommand) -> Self {
        NParsedNode::Command(content)
    }
}
