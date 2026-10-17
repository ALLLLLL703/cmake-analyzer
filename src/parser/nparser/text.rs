//! Borrow raw argument payloads through lexical wrapper rules; no expansion or I/O.
//! Contains AI-generated documentation additions.

use super::model::NParsedArgument;
use crate::{model::Spanned, parser::lexer::model::LexContent};

impl Spanned<NParsedArgument> {
    /// Borrow an argument payload through the corresponding lexical kind; no side effects.
    /// Keeps escapes and variable references unchanged; quotes/bracket wrappers are omitted.
    ///
    /// # Arguments
    /// - `self` `[in]`: argument form and full raw span; unchanged.
    /// - `source` `[in]`: source snapshot owning the argument range.
    ///
    /// # Returns
    /// `Some` unwrapped payload (or parenthesis text); `None` for invalid byte bounds or wrappers.
    ///
    /// # Examples
    /// ```
    /// use cmake_analyzer::parser::{lexer::model::{Lexer, RawBuffer}, nparser::model::{NParser, NParsedNode}};
    /// let source = "f([=[\nraw]=])";
    /// let result = NParser::new(Lexer::new(RawBuffer::new(source)).parse()).parse(source);
    /// if let NParsedNode::Command(command) = &result.nodes[0].content {
    ///     assert_eq!(command.args[0].text(source), Some("raw"));
    /// }
    /// ```
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
