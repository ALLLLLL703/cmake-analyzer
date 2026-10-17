//! Group flat tokens into complete or partial commands; no source evaluation or I/O.
//! Outer closing parentheses are consumed but not retained as arguments. Lexer
//! errors precede parser output; partial commands can coexist with diagnostics.
//! Contains AI-generated documentation additions.

use crate::{
    model::{Spanned, TextSpan},
    parser::{
        lexer::model::{LexContent, LexError},
        nparser::model::{
            CommandState, NParsedArgument, NParsedBuffer, NParsedCommand, NParsedError,
            NParsedNode, NParser,
        },
    },
};

/// Internal argument-reader result; errors are unlocated until attached to nodes.
/// Example: `let result: NParseResult<()> = Ok(())`.
type NParseResult<T> = Result<T, NParsedError>;
/// Located lexical name-token alias, currently unused by active parsing.
/// Example: `let name: FunctionName = Spanned::new(LexContent::Identifier, TextSpan::default())`.
type FunctionName = Spanned<LexContent>;
/// Located raw argument-form alias used by the argument reader.
/// Example: `let arg: FunctionArg = Spanned::new(NParsedArgument::Unquoted, TextSpan::default())`.
type FunctionArg = Spanned<NParsedArgument>;
/// Ordered owned argument list, including nested parentheses but not the outer pair.
/// Example: `let args: FunctionArgs = Vec::new()`.
type FunctionArgs = Vec<FunctionArg>;

impl NParser {
    /// Parse remaining tokens, advancing the token cursor; no I/O or source mutation.
    /// Every call copies comments and inherits all lexical errors again, even if the cursor
    /// is already at EOF. The source argument is currently unused: nodes refer to token spans.
    ///
    /// # Arguments
    /// - `self` `[in,out]`: parser whose cursor advances through remaining tokens; input vectors unchanged.
    /// - `source` `[in]`: source snapshot associated with token spans; currently not read or retained.
    ///
    /// # Returns
    /// New buffer containing comments, then inherited lexer-error nodes, then parser-produced command/error nodes. Empty input may still return lexical errors.
    ///
    /// # Algorithm
    /// Copy comments and lexical diagnostics, then repeatedly use the same one-unit
    /// parser as incremental updates. Continue until no token remains.
    ///
    /// # Flow
    /// ```text
    ///    [copy comments]    [inherit lexer errors]
    ///           +--------------------+
    ///                       |
    ///                 <token remains?> <--------+
    ///                  /          \            |
    ///                no           yes           |
    ///                 |            |            |
    ///           [return buffer] [parse_next] -----+
    /// ```
    ///
    /// # Examples
    /// ```
    /// use cmake_analyzer::parser::{lexer::model::{Lexer, RawBuffer}, nparser::model::NParser};
    /// let source = "f(x) g()";
    /// let mut parser = NParser::new(Lexer::new(RawBuffer::new(source)).parse());
    /// let result = parser.parse(source);
    /// assert_eq!(result.nodes.len(), 2);
    /// ```
    pub fn parse<'src>(&mut self, source: &'src str) -> NParsedBuffer {
        let mut result = NParsedBuffer {
            comments: self.buffer.comments.clone(),
            ..Default::default()
        };
        self.inherit_error(&mut result);
        if self.cursor.max_length == 0 {
            return result;
        }

        // let mut left_parenths = Vec::<Spanned<LexContent>>::new();
        while self.peek().is_some() {
            self.parse_next(&mut result);
        }
        result
    }

    // AI-authored extraction: a closed command has no parser dependencies outside its tokens.
    /// Consume one command/recovery unit and append parser output; no I/O.
    /// A unit may append zero, one or several nodes. A final bare name is consumed
    /// without a node; missing `(` is only diagnosed when a following token exists.
    ///
    /// # Arguments
    /// - `self` `[in,out]`: parser cursor advanced past the consumed name, argument group or unexpected token.
    /// - `result` `[in,out]`: output nodes appended to, never cleared; comments unchanged.
    ///
    /// # Returns
    /// `()`; output and cursor carry the result. With no token, neither changes.
    ///
    /// # Algorithm
    /// An identifier begins a command: consume its outer argument group, retain partial
    /// arguments at EOF, and compute the envelope through the consumed outer close (not
    /// the following token). A partial command also appends an unclosed-opener error.
    /// A bare `(` group becomes a command-name error; top-level literals/closes become
    /// errors and advance. Lexer errors are inherited separately, not by this routine.
    ///
    /// # Flow
    /// ```text
    ///                  [peek token]
    ///                        |
    ///                    <kind?>
    ///           /            |              \
    ///       [name]      [bare opener]    [literal/close]
    ///          |             |                |
    ///    [read arguments] [read group]    [append error]
    ///        /      \         |                |
    ///     [Ok]     [Err] [append name error] [advance one]
    ///      |         |
    ///  [command +  [following token?]
    ///  optional         | yes
    ///  error]      [missing-open error]
    /// ```
    ///
    /// # Examples
    /// ```text
    /// let mut parser = NParser::new(Lexer::new(RawBuffer::new("f(x)")).parse());
    /// let mut result = NParsedBuffer::default();
    /// parser.parse_next(&mut result);
    /// assert_eq!(result.nodes.len(), 1);
    /// ```
    pub(super) fn parse_next(&mut self, result: &mut NParsedBuffer) {
        if let Some(lex) = self.peek() {
            if let LexContent::Identifier = lex.content {
                self.cursor.offset += 1;
                let args_result = self.consume_function_args();

                if let Ok(ok) = args_result {
                    let (mut args, closed) = ok;

                    let mut command = NParsedCommand::new(lex.clone());

                    command.closed = closed;
                    if let CommandState::Unclosed { opening_span: span } = closed {
                        result.add_node(
                            NParsedNode::Error(NParsedError::UnclosedLeftParentheses),
                            span,
                        );
                    }
                    let mut temp_lex = lex.clone();
                    for arg in &args {
                        temp_lex.span.self_combine_with_middle(arg.span);
                    }
                    // AI-generated span fix.
                    match closed {
                        CommandState::Closed => {
                            if let Some(closing) = self
                                .cursor
                                .offset
                                .checked_sub(1)
                                .and_then(|index| self.buffer.lex.get(index))
                            {
                                temp_lex.span.self_combine_with_middle(closing.span);
                            }
                        }
                        CommandState::Unclosed { opening_span } => {
                            temp_lex.span.self_combine_with_middle(opening_span)
                        }
                    }
                    command.append_arg(&mut args);
                    result.add_node(NParsedNode::Command(command), temp_lex.span);
                } else if let Err(NParsedError::CalledInnerError) = args_result {
                    if let Some(lex) = self.peek() {
                        result.add_node(
                            NParsedNode::Error(NParsedError::ShouldHaveLeftParentheses),
                            lex.span,
                        );
                    }
                } else {
                    if let Some(lex) = self.peek() {
                        result.add_node(
                            NParsedNode::Error(NParsedError::ShouldHaveLeftParentheses),
                            lex.span,
                        );
                    }
                }
            } else {
                // let span = lex.span;
                // no command_name error handle(maybe too long and complex,,,)
                match &lex.content {
                    LexContent::LeftParentheses => {
                        let opening_span = lex.span;
                        match self.consume_function_args() {
                            Ok((args, state)) => {
                                let mut error_span = opening_span;
                                for arg in args {
                                    error_span.self_combine_with_middle(arg.span);
                                }
                                result.add_node(
                                    NParsedNode::Error(NParsedError::ShouldBeCommand),
                                    error_span,
                                );
                                if let CommandState::Unclosed { opening_span } = state {
                                    result.add_node(
                                        NParsedNode::Error(NParsedError::UnclosedLeftParentheses),
                                        opening_span,
                                    );
                                }
                            }
                            Err(error) => {
                                result.add_node(
                                    NParsedNode::Error(NParsedError::ShouldBeCommand),
                                    opening_span,
                                );
                                let error = match error {
                                    NParsedError::ReachTheEof => {
                                        NParsedError::UnclosedLeftParentheses
                                    }
                                    other => other,
                                };
                                result.add_node(NParsedNode::Error(error), opening_span);
                            }
                        }
                    }
                    LexContent::RightParentheses => {
                        result.add_node(
                            NParsedNode::Error(NParsedError::ShouldHaveLeftParentheses),
                            lex.span,
                        );
                        self.cursor.offset += 1;
                    }
                    LexContent::StringLiteral => {
                        result.add_node(
                            NParsedNode::Error(NParsedError::UnArgStringLiteral),
                            lex.span,
                        );
                        self.cursor.offset += 1;
                    }

                    LexContent::BracketArgument => {
                        result.add_node(NParsedNode::Error(NParsedError::UnArgBracket), lex.span);
                        self.cursor.offset += 1;
                    }
                    _ => {}
                }
            }
        }
    }

    /// Clone the next token without advancing or other side effects.
    ///
    /// # Arguments
    /// - `self` `[in]`: parser whose token cursor is inspected; unchanged.
    ///
    /// # Returns
    /// `Some` cloned token at cursor.offset; `None` when the vector has no such index. max_length is not consulted.
    ///
    /// # Examples
    /// ```
    /// use cmake_analyzer::parser::{lexer::model::{Lexer, RawBuffer}, nparser::model::NParser};
    /// let parser = NParser::new(Lexer::new(RawBuffer::new("f()")).parse());
    /// assert_eq!(parser.peek().unwrap().text("f()"), Some("f"));
    /// assert_eq!(parser.cursor.offset, 0);
    /// ```
    pub fn peek(&self) -> Option<Spanned<LexContent>> {
        self.buffer
            .lex
            .get(self.cursor.offset)
            .map(|s| s.to_owned())
    }

    /* /// should be called when cursor is at Identifier
    pub fn consume_funtion_name(&mut self) -> FunctionName {
        todo!()
    } */

    /// Consume an outer parenthesis group into flat arguments, advancing past its close; no I/O.
    /// At EOF inside a group, retain parsed arguments and return an unclosed state.
    ///
    /// # Arguments
    /// - `self` `[in,out]`: parser expected at `(`; cursor advances through the group, or remains unchanged on error.
    ///
    /// # Returns
    /// `Ok((args, state))`: args excludes the outer pair, includes inner parentheses, and state is Closed after consuming the outer close or Unclosed at EOF with the outer opening span. `Err` indicates invalid entry state.
    ///
    /// # Errors
    /// `ReachTheEof` if no token exists; `CalledInnerError` if the next token is not `(`.
    /// Neither error advances the cursor. Missing closes are an `Ok` partial result,
    /// not an `Err`; the caller decides which diagnostic nodes to append.
    ///
    /// # Algorithm
    /// Keep a stack of opening spans. Convert each token into a raw argument form.
    /// Nested opens/closes update the stack and stay in the argument list; the close
    /// that empties the outer level is consumed and omitted. EOF returns the outer
    /// opening span regardless of which inner open was last encountered.
    ///
    /// # Flow
    /// ```text
    ///           <starts at (?> ---- no ---> [Err; no advance]
    ///                  | yes
    ///         [push outer; advance]
    ///                  |
    ///              <next token?> <----------------+
    ///              /          \                  |
    ///           [EOF]       [token]               |
    ///             |             |                 |
    ///    [Ok args, Unclosed] <outer close?>        |
    ///                         /       \          |
    ///                       yes       no          |
    ///                        |         |          |
    ///                [consume close] [update stack, append arg]
    ///                        |         |          |
    ///                [Ok args, Closed] +----------+
    /// ```
    ///
    /// # Examples
    /// ```
    /// use cmake_analyzer::parser::{lexer::model::{Lexer, RawBuffer}, nparser::model::NParser};
    /// use cmake_analyzer::parser::nparser::model::CommandState;
    /// let mut parser = NParser::new(Lexer::new(RawBuffer::new("(x (y))")).parse());
    /// let (args, state) = parser.consume_function_args().unwrap();
    /// assert_eq!(args.len(), 4);
    /// assert_eq!(state, CommandState::Closed);
    /// assert!(parser.peek().is_none());
    /// ```
    pub fn consume_function_args(&mut self) -> NParseResult<(FunctionArgs, CommandState)> {
        if let Some(lex) = self.peek() {
            let first_lex = lex.clone();

            let mut left_parens = Vec::<TextSpan>::new();
            if let LexContent::LeftParentheses = first_lex.content {
                let mut result = FunctionArgs::new();
                self.cursor.offset += 1;
                left_parens.push(first_lex.span);
                let mut success_exit_loop = false;
                while let Some(lex) = self.peek() {
                    let arg = match lex.content {
                        LexContent::StringLiteral => {
                            Spanned::new(NParsedArgument::Quoted, lex.span)
                        }

                        LexContent::Identifier => Spanned::new(NParsedArgument::Unquoted, lex.span),
                        LexContent::LeftParentheses => {
                            left_parens.push(lex.span);
                            Spanned::new(NParsedArgument::LeftParen, lex.span)
                        }
                        LexContent::RightParentheses => {
                            if left_parens.len() == 1 {
                                left_parens.clear();
                                success_exit_loop = true;
                                self.cursor.offset += 1;
                                break;
                            }

                            left_parens.pop();
                            Spanned::new(NParsedArgument::RightParen, lex.span)
                        }
                        LexContent::BracketArgument => {
                            Spanned::new(NParsedArgument::Bracked, lex.span)
                        }
                    };
                    result.push(arg);
                    self.cursor.offset += 1;
                }
                if success_exit_loop {
                    Ok((result, CommandState::Closed))
                } else {
                    Ok((
                        result,
                        CommandState::Unclosed {
                            opening_span: first_lex.span,
                        },
                    ))
                }
            } else {
                Err(NParsedError::CalledInnerError)
            }
        } else {
            Err(NParsedError::ReachTheEof)
        }
    }

    /// Append all lexical diagnostics as parser error nodes, preserving locations; no I/O.
    ///
    /// # Arguments
    /// - `self` `[in]`: parser containing lexical errors; unchanged.
    /// - `result` `[in,out]`: destination nodes appended to, not overwritten; repeated calls duplicate errors.
    ///
    /// # Returns
    /// `()`; comments and existing nodes remain untouched.
    ///
    /// # Examples
    /// ```
    /// use cmake_analyzer::parser::{lexer::model::{Lexer, RawBuffer}, nparser::model::NParser};
    /// use cmake_analyzer::parser::nparser::model::NParsedBuffer;
    /// let parser = NParser::new(Lexer::new(RawBuffer::new("\"unfinished")).parse());
    /// let mut result = NParsedBuffer::default();
    /// parser.inherit_error(&mut result);
    /// assert_eq!(result.nodes.len(), 1);
    /// ```
    pub fn inherit_error(&self, result: &mut NParsedBuffer) {
        for lex_error in &self.buffer.errors {
            result.add_node(
                NParsedNode::Error(NParsedError::LexError(lex_error.content.clone())),
                lex_error.span,
            );
        }
    }
}
