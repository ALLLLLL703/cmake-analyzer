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
                let command_name = lex.clone();
            } else {
                // let span = lex.span;
                // no command_name error handle(maybe too long,,,)
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
                                for arg in ok {
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
                    }

                    _ => continue,
                }
            }
        }

        result
    }

    pub fn peek(&self) -> Option<&Spanned<LexContent>> {
        self.buffer.lex.get(self.cursor.offset)
    }

    /// should be called when cursor is at Identifier
    pub fn consume_funtion_name(&mut self) -> FunctionName {
        todo!()
    }

    /// should be called when cursor is at '('
    /// end with the offset at ')'
    pub fn consume_function_args(&mut self) -> NParseResult<FunctionArgs> {
        todo!()
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
