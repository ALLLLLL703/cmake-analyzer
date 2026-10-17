use std::fmt;

static DEFAULT_INDENT_WIDTH: usize = 2;
static DEFAULT_LINE_WIDTH: usize = 100;
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Options {
    pub indent_width: usize,
    pub line_width: usize,
    pub insert_spaces: bool,
}
impl Default for Options {
    fn default() -> Self {
        Self {
            indent_width: DEFAULT_INDENT_WIDTH,
            line_width: DEFAULT_LINE_WIDTH,
            insert_spaces: true,
        }
    }
}
#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    InvalidSyntax,
    InvalidOptions,
    Cancelled,
}
impl fmt::Display for Error {
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
pub(super) struct Writer<'a> {
    pub text: String,
    pub column: usize,
    pub trailing_breaks: usize,
    pub newline: &'a str,
    pub options: Options,
}
