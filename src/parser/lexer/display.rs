use std::fmt::Display;

use owo_colors::OwoColorize;
use owo_colors::Style;

use crate::parser::lexer::model::LexError;
use crate::parser::lexer::model::{LexContent, LexedBuffer};

pub struct LexDisplay<'a> {
    buffer: &'a LexedBuffer,
    colored: bool,
}

impl LexedBuffer {
    pub fn display(&self, colored: bool) -> LexDisplay<'_> {
        LexDisplay {
            buffer: self,
            colored,
        }
    }
}

impl Display for LexDisplay<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for token in &self.buffer.lex {
            let sytle = match &token.content {
                LexContent::Identifier(_) => Style::new().cyan(),
                LexContent::StringLiteral(_) | LexContent::BracketArgument(_) => {
                    Style::new().green()
                }
                LexContent::LeftParentheses | LexContent::RightParentheses => Style::new().yellow(),
            };

            let text = match &token.content {
                LexContent::LeftParentheses => String::from("("),
                LexContent::RightParentheses => String::from(")"),
                LexContent::Identifier(s) => format!("{}: {}", "Identifier", s),
                LexContent::StringLiteral(s) => format!("StringLiteral: {}", s),
                LexContent::BracketArgument(s) => format!("BracketArgument: {}", s),
            };

            if self.colored {
                let _ = writeln!(f, "{}", format_args!("{text}").style(sytle));
            } else {
                let _ = writeln!(f, "{text}");
            }
        }

        for error in &self.buffer.errors {
            let style = Style::new().red();
            let option_text = match error {
                LexError::UnclosedStringLiteral(s) => {
                    Some(format!("UnclosedStringLiteral at {s:?}"))
                }
                LexError::UnclosedParentheses(s) => Some(format!("UnclosedParentheses at {s:?}")),
                LexError::UnclosedBracketArgument(s) => {
                    Some(format!("UnclosedBracketArgument at {s:?}"))
                }
                _ => None,
            };
            if let Some(text) = option_text {
                if self.colored {
                    let _ = writeln!(f, "{}", format_args!("{text}").style(style));
                } else {
                    let _ = writeln!(f, "{}", format_args!("{text}"));
                }
            }
        }
        Ok(())
    }
}
