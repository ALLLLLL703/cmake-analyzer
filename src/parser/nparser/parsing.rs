use crate::{
    model::{Spanned, TextSpan},
    parser::{
        lexer::model::{LexContent, LexError},
        nparser::model::{NParsedArgument, NParsedBuffer, NParsedCommand, NParsedError, NParser},
    },
};

pub type NParseResult<T> = Result<T, NParsedError>;
type FunctionName = Spanned<String>;
type FunctionArgs = Vec<Spanned<NParsedArgument>>;
type FunctionArg = Spanned<NParsedArgument>;

impl NParser {
    /// Consumes token strings into commands, retaining their original spans.
    /// Incomplete commands are diagnosed but are not added to the result.
    pub fn parse(&mut self) -> NParsedBuffer {
        let mut result = NParsedBuffer::default();
        for error in std::mem::take(&mut self.buffer.errors) {
            let span = match &error {
                LexError::UnclosedParentheses(span)
                | LexError::UnclosedStringLiteral(span)
                | LexError::UnclosedBracketArgument(span) => *span,
                LexError::ReachTheEof => self.error_span(&NParsedError::ReachTheEof),
            };
            result.add_error(NParsedError::LexError(error), span);
        }

        while self.peek().is_ok() {
            let name = match self.advance_a_function_name() {
                Ok(name) => name,
                Err(error) => {
                    let span = self.error_span(&error);
                    result.add_error(error, span);
                    // Failed name scans leave the cursor unchanged. Skip the bad token.
                    self.cursor.offset += 1;
                    continue;
                }
            };
            match self.advance_a_function_args() {
                Ok(args) => result.add_command(NParsedCommand { name, args }),
                Err(error) => {
                    let span = self.error_span(&error);
                    result.add_error(error, span);
                    // The name was consumed, so a missing '(' cannot stall this loop.
                    // Leave the next token available as a possible command name.
                }
            }
        }
        result
    }

    pub fn peek(&self) -> NParseResult<&Spanned<LexContent>> {
        if self.cursor.offset >= self.cursor.max_length {
            return Err(NParsedError::ReachTheEof);
        }
        self.buffer
            .lex
            .get(self.cursor.offset)
            .ok_or(NParsedError::ReachTheEof)
    }

    /// Consumes an ASCII command identifier. Failure does not advance the cursor.
    pub fn advance_a_function_name(&mut self) -> NParseResult<FunctionName> {
        self.peek()?;
        let token = &mut self.buffer.lex[self.cursor.offset];
        let LexContent::Identifier(name) = &mut token.content else {
            return Err(NParsedError::ExpectedCommandName);
        };
        let mut bytes = name.bytes();
        if !bytes
            .next()
            .is_some_and(|byte| byte.is_ascii_alphabetic() || byte == b'_')
            || !bytes.all(|byte| byte.is_ascii_alphanumeric() || byte == b'_')
        {
            return Err(NParsedError::InvalidCommandName);
        }
        let name = Spanned {
            content: std::mem::take(name),
            span: token.span,
        };
        self.cursor.offset += 1;
        Ok(name)
    }

    /// Called at '('. Consumes both outer parentheses, preserving internal ones.
    pub fn advance_a_function_args(&mut self) -> NParseResult<FunctionArgs> {
        let opening = self.peek()?;
        if !matches!(&opening.content, LexContent::LeftParentheses) {
            return Err(NParsedError::ExpectedLeftParenthesis);
        }
        let opening_span = opening.span;
        self.cursor.offset += 1;
        let mut args = Vec::new();
        let mut depth = 0;
        loop {
            let token = match self.peek() {
                Ok(token) => token,
                Err(NParsedError::ReachTheEof) => {
                    return Err(NParsedError::UnclosedArguments(opening_span));
                }
                Err(error) => return Err(error),
            };
            match &token.content {
                LexContent::LeftParentheses => depth += 1,
                LexContent::RightParentheses if depth == 0 => {
                    self.cursor.offset += 1;
                    return Ok(args);
                }
                LexContent::RightParentheses => depth -= 1,
                _ => {}
            }
            args.push(self.advance_a_arg()?);
        }
    }

    /// Consumes one token as an argument without interpreting or expanding it.
    pub fn advance_a_arg(&mut self) -> NParseResult<FunctionArg> {
        self.peek()?;
        let token = &mut self.buffer.lex[self.cursor.offset];
        let content = match &mut token.content {
            LexContent::Identifier(text) => NParsedArgument::Unquoted(std::mem::take(text)),
            LexContent::StringLiteral(text) => NParsedArgument::Quoted(std::mem::take(text)),
            LexContent::BracketArgument(text) => NParsedArgument::Bracked(std::mem::take(text)),
            LexContent::LeftParentheses => NParsedArgument::LeftParen,
            LexContent::RightParentheses => NParsedArgument::RightParen,
        };
        let argument = Spanned {
            content,
            span: token.span,
        };
        self.cursor.offset += 1;
        Ok(argument)
    }

    fn error_span(&self, error: &NParsedError) -> TextSpan {
        if let NParsedError::UnclosedArguments(span) = error {
            return *span;
        }
        self.peek().map(|token| token.span).unwrap_or_else(|_| {
            self.buffer
                .lex
                .last()
                .map_or(TextSpan::default(), |token| token.span)
        })
    }
}
