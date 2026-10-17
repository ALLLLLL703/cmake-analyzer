//! Borrowed command/error inspection rendering with optional ANSI colors.
//! Uses full raw argument spans, unlike payload text views; performs no direct I/O.
//! Contains AI-generated documentation additions.

use std::fmt::Display;

use owo_colors::OwoColorize;
use owo_colors::Style;

use crate::parser::lexer::model::LexError;
use crate::parser::nparser::model::NParsedError;
use crate::parser::nparser::model::{NParsedArgument, NParsedBuffer, NParsedNode};

/// Borrowed command/error display adapter; construction has no side effects.
/// Inspection callers obtain it through [`NParsedBuffer::display`] and [`Display`]
/// writes full raw argument text plus closure state, or error location/source.
/// Comments are omitted. Text is unescaped; malformed spans receive fallback labels.
///
/// # Examples
/// ```
/// use cmake_analyzer::parser::{lexer::model::{Lexer, RawBuffer}, nparser::model::NParser};
/// let source = "f(hi)";
/// let buffer = NParser::new(Lexer::new(RawBuffer::new(source)).parse()).parse(source);
/// let view = buffer.display(false, source);
/// assert!(view.to_string().contains("state: Closed"));
/// ```
pub struct NParsedDisplay<'a> {
    /// Parsed nodes borrowed unchanged. Example inside this module: `view.buffer.nodes.len()`.
    buffer: &'a NParsedBuffer,
    /// Matching source for full raw span text. Example inside this module: `assert_eq!(view.source, "f(hi)")`.
    source: &'a str,
    /// Whether argument/error/name styles are retained. Example inside this module: `view.colored = false`.
    colored: bool,
}

impl NParsedBuffer {
    /// Borrow parsed output and source for later inspection rendering; no side effects.
    ///
    /// # Arguments
    /// - `self` `[in]`: buffer borrowed by the adapter, not cloned.
    /// - `colored` `[in]`: true for ANSI styles, false for plain text.
    /// - `source` `[in]`: matching source snapshot borrowed by the adapter.
    ///
    /// # Returns
    /// Lazy display adapter; no output occurs until formatting.
    ///
    /// # Examples
    /// ```
    /// use cmake_analyzer::parser::{lexer::model::{Lexer, RawBuffer}, nparser::model::NParser};
    /// let source = "f(hi)";
    /// let buffer = NParser::new(Lexer::new(RawBuffer::new(source)).parse()).parse(source);
    /// let view = buffer.display(false, source);
    /// assert!(view.to_string().contains("hi"));
    /// ```
    pub fn display<'a>(&'a self, colored: bool, source: &'a str) -> NParsedDisplay<'a> {
        NParsedDisplay {
            buffer: self,
            source,
            colored,
        }
    }
}

impl NParsedDisplay<'_> {
    /// Select the requested style or a plain style without side effects.
    ///
    /// # Arguments
    /// - `self` `[in]`: adapter supplying the color flag; unchanged.
    /// - `style` `[in]`: style to retain when colors are enabled.
    ///
    /// # Returns
    /// Supplied style if colored, otherwise a new empty style.
    ///
    /// # Examples
    /// ```text
    /// let view = buffer.display(false, source);
    /// let plain = view.style(Style::new().red());
    /// ```
    fn style(&self, style: Style) -> Style {
        if self.colored { style } else { Style::new() }
    }
}

impl Display for NParsedDisplay<'_> {
    /// Append command/error inspection records to the formatter; no direct I/O.
    /// Uses full raw argument spans, including wrappers. Each label already ends in a
    /// newline and writeln adds another, leaving a blank line between records.
    ///
    /// # Arguments
    /// - `self` `[in]`: adapter whose parsed output/source/color policy are unchanged.
    /// - `f` `[in,out]`: formatter appended to; partial writes are not rolled back.
    ///
    /// # Returns
    /// `Ok(())` after all records; `Err(fmt::Error)` on destination failure.
    ///
    /// # Errors
    /// Propagates destination write failures. Invalid source spans render fallback
    /// strings instead of returning an error.
    ///
    /// # Algorithm
    /// For commands, read the name and raw arguments, color by argument form and append
    /// closure state. For errors, choose the diagnostic label and read its raw source
    /// range plus location label. Missing source gets explicit fallback text.
    ///
    /// # Flow
    /// ```text
    ///                  [next node] <----------------+
    ///                       |                      |
    ///                   <variant?>                 |
    ///                   /       \
    ///             [command]     [error]
    ///                 |            |
    ///     [name + raw args]  [kind + location + source]
    ///                 |            |
    ///         [closure state]      |
    ///                 +------+-----+
    ///                        |
    ///             [write record + blank line] -----+
    ///                        | all nodes done
    ///                      [Ok]
    /// ```
    ///
    /// # Examples
    /// ```
    /// use cmake_analyzer::parser::{lexer::model::{Lexer, RawBuffer}, nparser::model::NParser};
    /// let source = "f(hi)";
    /// let buffer = NParser::new(Lexer::new(RawBuffer::new(source)).parse()).parse(source);
    /// // format! invokes Display::fmt for this adapter.
    /// let output = format!("{}", buffer.display(false, source));
    /// assert!(output.contains("state: Closed"));
    /// ```
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for node in &self.buffer.nodes {
            let label = match &node.content {
                NParsedNode::Command(command) => {
                    let mut command_label = match command.name.span.text(self.source) {
                        Some(s) => s,
                        None => "command name lost,,,",
                    };
                    let mut args_label = format!("");
                    for arg in &command.args {
                        let mut style = match arg.content {
                            NParsedArgument::Unquoted => Style::new().blue(),
                            NParsedArgument::Quoted => Style::new().green(),
                            NParsedArgument::Bracked => Style::new().white(),
                            NParsedArgument::LeftParen => Style::new().yellow(),
                            NParsedArgument::RightParen => Style::new().yellow(),
                        };
                        let arg_label = match arg.span.text(self.source) {
                            Some(s) => s.to_owned(),
                            None => {
                                style = Style::new().red();
                                "command arg name lost,,,".to_string()
                            }
                        };
                        args_label = format!(
                            "{}, {}",
                            args_label.clone(),
                            arg_label.style(self.style(style))
                        );
                    }

                    format!(
                        "{}: [ {} ] state: {:?}\n",
                        command_label.style(self.style(Style::new().cyan())),
                        args_label,
                        command.closed
                    )
                }
                NParsedNode::Error(e) => {
                    let span = node.span;
                    let source = match span.text(self.source) {
                        Some(s) => s,
                        None => "error source not found,,,",
                    };

                    let label = match e {
                        NParsedError::LexError(lexe) => Some(match lexe {
                            LexError::ReachTheEof => "ReachTheEof(LexError)",
                            LexError::UnclosedStringLiteral => "UnclosedStringLitera(LexError)",
                            LexError::UnclosedParentheses => "UnclosedParentheses(LexError)",
                            LexError::UnclosedBracketArgument => {
                                "UnclosedBracketArgument(LexError)"
                            }
                            // AI-generated diagnostic.
                            LexError::UnclosedBracketComment => "UnclosedBracketComment(LexError)",
                        }),
                        NParsedError::ShouldBeCommand => Some("ShouldBeCommand"),
                        NParsedError::UnclosedLeftParentheses => Some("UnclosedLeftParentheses"),
                        NParsedError::UnArgStringLiteral => Some("UnArgStringLiteral"),
                        NParsedError::ShouldHaveLeftParentheses => {
                            Some("ShouldHaveLeftParentheses")
                        }
                        NParsedError::UnArgBracket => Some("UnArgBracket"),
                        NParsedError::ReachTheEof => Some("ReachTheEof"),
                        NParsedError::CalledInnerError => Some("CalledInnerError"),
                    };

                    let loc_info = span.loc_info();

                    if let Some(label) = label {
                        format!(
                            "({}: [ loc: {},  source: {} ])\n",
                            label.style(self.style(Style::new().red())),
                            loc_info.style(self.style(Style::new().yellow())),
                            source.style(self.style(Style::new().white()))
                        )
                    } else {
                        format!("")
                    }
                }
            };
            writeln!(f, "{}", label)?;
        }
        Ok(())
    }
}
