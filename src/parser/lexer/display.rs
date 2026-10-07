use super::model::{LexContent, LexError, LexedBuffer};
use owo_colors::{OwoColorize, Style};
use std::fmt::{self, Display};

pub struct LexDisplay<'a> {
    buffer: &'a LexedBuffer,
    source: &'a str,
    colored: bool,
}

impl LexedBuffer {
    pub fn display<'a>(&'a self, colored: bool, source: &'a str) -> LexDisplay<'a> {
        LexDisplay {
            buffer: self,
            source,
            colored,
        }
    }
}

impl Display for LexDisplay<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for token in &self.buffer.lex {
            let (label, style) = match token.content {
                LexContent::Identifier => (Some("Identifier"), Style::new().cyan()),
                LexContent::StringLiteral => (Some("StringLiteral"), Style::new().green()),
                LexContent::BracketArgument => (Some("BracketArgument"), Style::new().green()),
                LexContent::LeftParentheses | LexContent::RightParentheses => {
                    (None, Style::new().yellow())
                }
            };
            let text = token.text(self.source).ok_or(fmt::Error)?;
            if let Some(label) = label {
                line(f, format_args!("{label}: {text}"), style, self.colored)?;
            } else {
                line(f, format_args!("{text}"), style, self.colored)?;
            }
        }
        for error in &self.buffer.errors {
            let label = match error.content {
                LexError::ReachTheEof => "ReachTheEof",
                LexError::UnclosedStringLiteral => "UnclosedStringLiteral",
                LexError::UnclosedParentheses => "UnclosedParentheses",
                LexError::UnclosedBracketArgument => "UnclosedBracketArgument",
            };
            line(
                f,
                format_args!("{label} at {:?}", error.span),
                Style::new().red(),
                self.colored,
            )?;
        }
        Ok(())
    }
}

fn line(
    f: &mut fmt::Formatter<'_>,
    text: fmt::Arguments<'_>,
    style: Style,
    colored: bool,
) -> fmt::Result {
    if colored {
        writeln!(f, "{}", text.style(style))
    } else {
        writeln!(f, "{text}")
    }
}
