use super::model::NParsedArgument;
use crate::{model::Spanned, parser::lexer::model::LexContent};

impl Spanned<NParsedArgument> {
    pub fn text<'a>(&self, source: &'a str) -> Option<&'a str> {
        let content = match self.content {
            NParsedArgument::Unquoted => LexContent::Identifier,
            NParsedArgument::Quoted => LexContent::StringLiteral,
            NParsedArgument::Bracked => LexContent::BracketArgument,
            NParsedArgument::LeftParen => LexContent::LeftParentheses,
            NParsedArgument::RightParen => LexContent::RightParentheses,
        };
        Spanned {
            content,
            span: self.span,
        }
        .text(source)
    }
}
