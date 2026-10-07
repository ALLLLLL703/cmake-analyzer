use crate::{
    model::{Spanned, TextSpan},
    parser::{
        lexer::model::{LexContent, LexError},
        nparser::model::{
            NParsedArgument, NParsedBuffer, NParsedCommand, NParsedError, NParsedNode, NParser,
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

                    let mut command = NParsedCommand::new(lex);

                    command.append_arg(&mut args);
                    command.closed = closed;
                    result.add_node(NParsedNode::Command(command), span);
                }
            } else {
                // let span = lex.span;
                // no command_name error handle(maybe too long and complex,,,)
                match &lex.content {
                    LexContent::LeftParentheses => {
                        let opening_span = lex.span;
                        result.add_node(
                            NParsedNode::Error(NParsedError::ShouldBeCommand),
                            opening_span,
                        );
                        let args = self.consume_function_args();
                        match args {
                            Ok(ok) => {
                                for arg in ok.0 {
                                    result.add_node(
                                        NParsedNode::Error(NParsedError::ShouldBeCommand),
                                        arg.span,
                                    );
                                }
                            }
                            Err(NParsedError::ReachTheEof) => {
                                result.add_node(
                                    NParsedNode::Error(NParsedError::UnclosedLeftParentheses),
                                    opening_span,
                                );
                            }
                            _ => {}
                        };
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
    pub fn consume_function_args(&mut self) -> NParseResult<(FunctionArgs, bool)> {
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
                    Ok((result, true))
                } else {
                    Ok((result, false))
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
