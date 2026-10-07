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

type NParseResult<T> = Result<T, NParsedError>;
type FunctionName = Spanned<LexContent>;
type FunctionArg = Spanned<NParsedArgument>;
type FunctionArgs = Vec<FunctionArg>;

impl NParser {
    pub fn parse<'src>(&mut self, source: &'src str) -> NParsedBuffer {
        let mut result = NParsedBuffer::default();
        self.inherit_error(&mut result);
        if self.cursor.max_length == 0 {
            return result;
        }

        // let mut left_parenths = Vec::<Spanned<LexContent>>::new();
        while let Some(lex) = self.peek() {
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
                    if let Some(right_parenth_lex) = self.peek() {
                        temp_lex
                            .span
                            .self_combine_with_middle(right_parenth_lex.span);
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
                    _ => continue,
                }
            }
        }

        result
    }

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

    /// should be called when cursor is at '(' or error
    /// when error ,the offset will not advance
    /// end with the offset at ')'
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

    pub fn inherit_error(&self, result: &mut NParsedBuffer) {
        for lex_error in &self.buffer.errors {
            result.add_node(
                NParsedNode::Error(NParsedError::LexError(lex_error.content.clone())),
                lex_error.span,
            );
        }
    }
}
