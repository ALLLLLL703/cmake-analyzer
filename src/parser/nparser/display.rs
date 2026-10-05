use std::fmt::{self, Display};

use owo_colors::{OwoColorize, Style};

use crate::parser::lexer::model::LexError;

use super::model::{NParsedArgument, NParsedBuffer, NParsedError};

pub struct NParseDisplay<'a> {
    buffer: &'a NParsedBuffer,
    colored: bool,
}

impl NParsedBuffer {
    pub fn display(&self, colored: bool) -> NParseDisplay<'_> {
        NParseDisplay {
            buffer: self,
            colored,
        }
    }
}

impl Display for NParsedBuffer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.display(false))
    }
}

impl Display for NParseDisplay<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for command in &self.buffer.commands {
            line(
                f,
                format_args!("Command: {}", command.name.content),
                Style::new().cyan(),
                self.colored,
            )?;
            for argument in &command.args {
                let (label, text, style) = match &argument.content {
                    NParsedArgument::Unquoted(text) => {
                        ("Unquoted", Some(text), Style::new().blue())
                    }
                    NParsedArgument::Quoted(text) => ("Quoted", Some(text), Style::new().green()),
                    NParsedArgument::Bracked(text) => ("Bracked", Some(text), Style::new().green()),
                    NParsedArgument::LeftParen => ("(", None, Style::new().yellow()),
                    NParsedArgument::RightParen => (")", None, Style::new().yellow()),
                };
                if let Some(text) = text {
                    // Escaping keeps multiline/control-character contents on one safe line.
                    line(f, format_args!("  {label}: {text:?}"), style, self.colored)?;
                } else {
                    line(f, format_args!("  {label}"), style, self.colored)?;
                }
            }
        }
        for error in &self.buffer.error {
            line(
                f,
                format_args!("{} at {:?}", error_label(&error.content), error.span),
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

fn error_label(error: &NParsedError) -> &'static str {
    match error {
        NParsedError::LexError(error) => match error {
            LexError::ReachTheEof => "LexError(ReachTheEof)",
            LexError::UnclosedParentheses(_) => "LexError(UnclosedParentheses)",
            LexError::UnclosedStringLiteral(_) => "LexError(UnclosedStringLiteral)",
            LexError::UnclosedBracketArgument(_) => "LexError(UnclosedBracketArgument)",
        },
        NParsedError::ReachTheEof => "ReachTheEof",
        NParsedError::ExpectedCommandName => "ExpectedCommandName",
        NParsedError::InvalidCommandName => "InvalidCommandName",
        NParsedError::ExpectedLeftParenthesis => "ExpectedLeftParenthesis",
        NParsedError::UnclosedArguments(_) => "UnclosedArguments",
    }
}
