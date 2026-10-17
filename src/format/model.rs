//! Pure formatter options, errors and private output state. [`super::render`] consumes
//! [`Options`] and returns [`Error`] on rejection; none of these records performs I/O.
//!
//! Contains AI-generated documentation additions.
use std::fmt;

static DEFAULT_INDENT_WIDTH: usize = 2;
static DEFAULT_LINE_WIDTH: usize = 100;
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
/// Pure layout policy for [`super::render`], also used as configuration defaults.
/// Constructing options performs no validation or I/O; render validates positive widths.
///
/// # Examples
/// ```
/// # use cmake_analyzer::format::Options;
/// let options = Options { indent_width: 4, line_width: 80, insert_spaces: false };
/// ```
pub struct Options {
    /// Columns per indentation level and tab stop; render accepts 1..=255, default 2. Example: `options.indent_width = 4`.
    pub indent_width: usize,
    /// Positive Unicode-scalar target width, default 100; atomic literals can exceed it. Example: `options.line_width = 80`.
    pub line_width: usize,
    /// True emits spaces; false emits one tab per indentation level. Example: `options.insert_spaces = false`.
    pub insert_spaces: bool,
}
impl Default for Options {
    /// Return two-space, 100-column layout options in memory; no I/O.
    ///
    /// # Arguments
    /// None.
    ///
    /// # Returns
    /// Options with indent_width 2, line_width 100 and insert_spaces true.
    ///
    /// # Examples
    /// ```
    /// # use cmake_analyzer::format::Options;
    /// assert_eq!(Options::default().indent_width, 2);
    /// ```
    fn default() -> Self {
        Self {
            indent_width: DEFAULT_INDENT_WIDTH,
            line_width: DEFAULT_LINE_WIDTH,
            insert_spaces: true,
        }
    }
}
#[derive(Debug, PartialEq, Eq)]
/// Why [`super::render`] could not produce a safe replacement. Callers can omit
/// editor edits on syntax/cancellation rejection. Construction has no side effects.
///
/// # Examples
/// ```
/// # use cmake_analyzer::format::Error;
/// assert_eq!(Error::Cancelled.to_string(), "Formatting cancelled");
/// ```
pub enum Error {
    /// Malformed/unsupported syntax or inconsistent source ranges; no output is returned. Example: `let error = Error::InvalidSyntax`.
    InvalidSyntax,
    /// Zero widths or indent width above 255. Example: `let error = Error::InvalidOptions`.
    InvalidOptions,
    /// Caller stop callback requested cancellation at a checkpoint. Example: `let error = Error::Cancelled`.
    Cancelled,
}
impl fmt::Display for Error {
    /// Write a human-readable rejection reason to the formatter; no file/process effects of its own.
    ///
    /// # Arguments
    /// - `self` `[in]`: error value being displayed.
    /// - `f` `[in,out]`: destination formatter receives appended message text; existing content is not cleared.
    ///
    /// # Returns
    /// `Ok(())` on successful formatting; `Err(fmt::Error)` if the formatter rejects the write.
    ///
    /// # Errors
    /// Propagates a formatter write error.
    ///
    /// # Examples
    /// ```
    /// # use cmake_analyzer::format::Error;
    /// assert_eq!(format!("{}", Error::Cancelled), "Formatting cancelled");
    /// ```
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::InvalidSyntax => "Cannot format invalid or unsupported CMake syntax",
            Self::InvalidOptions => {
                "Formatting indent_width must be 1..=255 and line_width must be positive"
            }
            Self::Cancelled => "Formatting cancelled",
        })
    }
}
impl std::error::Error for Error {}
/// Owned output and current display position used by layout/trivia rendering.
/// [`Self::push`] updates text and bookkeeping; [`Self::finish`] consumes it. No I/O.
///
/// # Flow
/// ```text
///  [layout]     [trivia]
///      \          /
///        [Writer owns]
///         /        \
///     [text]   [column/breaks]
///         \        /
///         [finish: String]
/// ```
///
/// # Examples
/// ```text
/// let out = Writer { text: String::new(), column: 0, trailing_breaks: 0, newline: "\n", options: Options::default() };
/// ```
pub(super) struct Writer<'a> {
    /// Complete emitted buffer, append-only until finish trims final line breaks. Example: `out.text.as_str()`.
    pub text: String,
    /// Current scalar-column count, tabs advancing to configured stops. Example: `out.column == 0`.
    pub column: usize,
    /// Number of currently tracked terminal LF breaks; ordinary text resets it. Example: `out.trailing_breaks >= 1`.
    pub trailing_breaks: usize,
    /// Selected LF/CRLF sequence for inserted breaks. Example: `out.newline == "\r\n"`.
    pub newline: &'a str,
    /// Validated layout settings controlling indentation and columns. Example: `out.options.insert_spaces`.
    pub options: Options,
}
