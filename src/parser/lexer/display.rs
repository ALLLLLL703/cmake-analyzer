//! Borrowed inspection rendering of tokens and diagnostics, with optional ANSI colors.
//! Writes only to the formatter selected by the caller; does not print or perform I/O.
//! Contains AI-generated documentation additions.

use super::model::{LexContent, LexError, LexedBuffer};
use owo_colors::{OwoColorize, Style};
use std::fmt::{self, Display};

/// Borrowed token/error display adapter; constructing it has no side effects.
/// Inspection callers obtain it through [`LexedBuffer::display`] and use [`Display`]
/// to write payload text and diagnostics. Comments/checkpoints are not rendered.
/// Source text is not escaped, so embedded newlines remain literal output.
///
/// # Examples
/// ```
/// use cmake_analyzer::parser::lexer::model::{Lexer, RawBuffer};
/// let source = "f(hi)";
/// let buffer = Lexer::new(RawBuffer::new(source)).parse();
/// let view = buffer.display(false, source);
/// assert!(view.to_string().contains("Identifier: f"));
/// ```
pub struct LexDisplay<'a> {
    /// Lexical output borrowed unchanged. Example inside this module: `view.buffer.lex.len()`.
    buffer: &'a LexedBuffer,
    /// Matching UTF-8 snapshot used for payload views. Example inside this module: `assert_eq!(view.source, "f(hi)")`.
    source: &'a str,
    /// Whether rendering emits ANSI styling. Example inside this module: `view.colored = false`.
    colored: bool,
}

impl LexedBuffer {
    /// Borrow lexical output and source for later rendering; no side effects.
    ///
    /// # Arguments
    /// - `self` `[in]`: buffer borrowed by the adapter, not cloned.
    /// - `colored` `[in]`: true to emit ANSI styles, false for plain text.
    /// - `source` `[in]`: source snapshot matching every token span, borrowed for the adapter lifetime.
    ///
    /// # Returns
    /// Lazy display adapter; formatting writes to the caller-provided destination.
    ///
    /// # Examples
    /// ```
    /// use cmake_analyzer::parser::lexer::model::{Lexer, RawBuffer};
    /// let source = "f(hi)";
    /// let buffer = Lexer::new(RawBuffer::new(source)).parse();
    /// let output = buffer.display(false, source).to_string();
    /// assert!(output.contains("Identifier: hi"));
    /// ```
    pub fn display<'a>(&'a self, colored: bool, source: &'a str) -> LexDisplay<'a> {
        LexDisplay {
            buffer: self,
            source,
            colored,
        }
    }
}

impl Display for LexDisplay<'_> {
    /// Write token payloads followed by diagnostic labels to the caller formatter; no direct I/O.
    /// Each token/error gets a newline; comments and checkpoints are omitted.
    ///
    /// # Arguments
    /// - `self` `[in]`: borrowed buffer/source/color policy; unchanged.
    /// - `f` `[in,out]`: formatter appended to; existing output is not cleared.
    ///
    /// # Returns
    /// `Ok(())` after writing all entries; `Err(fmt::Error)` for invalid token text or destination failure, possibly after partial output.
    ///
    /// # Errors
    /// Returns a formatting error when a token payload cannot be read from source or
    /// the destination rejects a write. Diagnostics do not validate source slices.
    ///
    /// # Algorithm
    /// Choose a label/style for each kind, borrow its payload, and write one line.
    /// Then render each error kind with its debug span in red when colors are enabled.
    ///
    /// # Flow
    /// ```text
    ///  [token kind]        [source span]
    ///       |                   |
    ///  [label/style]       [payload view]
    ///       +--------+----------+
    ///                |
    ///         [write token line] <--- [next token]
    ///                | tokens done
    ///         [write error lines] <-- [next error]
    ///                |
    ///         [Ok or write error]
    /// ```
    ///
    /// # Examples
    /// ```
    /// use cmake_analyzer::parser::lexer::model::{Lexer, RawBuffer};
    /// let source = "f(hi)";
    /// let buffer = Lexer::new(RawBuffer::new(source)).parse();
    /// // format! invokes Display::fmt for this adapter.
    /// let output = format!("{}", buffer.display(false, source));
    /// assert!(output.starts_with("Identifier: f\n"));
    /// ```
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
                // AI-generated diagnostic.
                LexError::UnclosedBracketComment => "UnclosedBracketComment",
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

/// Append one optionally styled line to a formatter; no direct I/O.
///
/// # Arguments
/// - `f` `[in,out]`: destination appended to, including a final newline.
/// - `text` `[in]`: already constructed formatting arguments to write.
/// - `style` `[in]`: ANSI style used only if colored is true.
/// - `colored` `[in]`: whether to apply style or write plain arguments.
///
/// # Returns
/// `Ok(())` on success; `Err(fmt::Error)` if writing fails after possible partial output.
///
/// # Errors
/// Propagates the destination formatter write error without rollback.
///
/// # Examples
/// ```text
/// line(f, format_args!("Identifier: hi"), Style::new().cyan(), false)?;
/// ```
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
