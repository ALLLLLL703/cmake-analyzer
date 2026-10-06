use crate::{
    model::{Spanned, TextSpan}, parser::{
        lexer::model::{LexContent, LexError}, nparser::model::{
            NParsedArgument, NParsedBuffer, NParsedCommand, NParsedError, NParsedNode, NParser,
        },
    },
};

type NParseResult<T> = Result<T, NParsedError>;
type FunctionName = Spanned<String>;
type FunctionArg = Spanned<NParsedArgument>;
type FunctionArgs = Vec<FunctionArg>;

impl NParser {
    pub fn parse(&mut self) -> NParsedBuffer {
        let mut result = NParsedBuffer::default();
        self.inherit_error(&mut result);
        if self.cursor.max_length == 0 {
            return result;
        }


        let mut left_parenths = Vec::<Spanned<LexContent>>::new();
        while let Some(lex) = self.peek() {
            if let LexContent::Identifier(ident) = &lex.content {
            } else {
                let span = lex.span;
                self.cursor.offset += 1;
                if let Some(lex) = self.peek() {
                    match &lex.content {
                        LexContent::Identifier(i) => {
                            continue;
                        }
                        LexContent::LeftParentheses => {
                            let args = self.consume_function_args();
                            for arg in args {
                                result.add_node(
                                    NParsedNode::Error(NParsedError::ShouldBeCommand),
                                    arg.span,
                                );
                            }
                        }
                        LexContent::RightParentheses => {
                            result.add_node(
                                NParsedNode::Error(NParsedError::ShouldHaveLeftParentheses),
                                lex.span,
                            );
                            self.cursor.offset += 1;
                        }
                        LexContent
                        _ => {}
                    }
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
    pub fn consume_function_args(&mut self) -> FunctionArgs {
        todo!()
    }

    pub fn inherit_error(&self,result:&mut NParsedBuffer ) {
        for lex_error in &self.buffer.errors{
            result.add_node(
                NParsedNode::Error(NParsedError::LexError(lex_error.content.clone())),
                lex_error.span,
            );
        }
    }
}
