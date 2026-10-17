use super::model::{Error, Writer};
use crate::parser::lexer::model::LexedBuffer;
use std::ops::Range;

pub(super) fn render(
    source: &str,
    lexed: &LexedBuffer,
    range: Range<usize>,
    level: usize,
    out: &mut Writer<'_>,
    stop: &impl Fn() -> bool,
) -> Result<(), Error> {
    let mut cursor = range.start;
    let first = lexed
        .comments
        .partition_point(|comment| comment.end_byte <= cursor);
    for comment in lexed.comments[first..]
        .iter()
        .take_while(|comment| comment.start_byte < range.end)
    {
        if stop() {
            return Err(Error::Cancelled);
        }
        if comment.start_byte < cursor || comment.end_byte > range.end {
            return Err(Error::InvalidSyntax);
        }
        whitespace(&source[cursor..comment.start_byte], out)?;
        if out.column == 0 {
            out.indent(level);
        } else {
            out.push(" ");
        }
        let mut raw = &source[comment.start_byte..comment.end_byte];
        // The lexer includes CR in line comments, but not the following LF.
        if source.as_bytes().get(comment.end_byte) == Some(&b'\n') {
            raw = raw.strip_suffix('\r').unwrap_or(raw);
        }
        out.push(raw);
        cursor = comment.end_byte;
    }
    whitespace(&source[cursor..range.end], out)
}
fn whitespace(raw: &str, out: &mut Writer<'_>) -> Result<(), Error> {
    if !raw.chars().all(char::is_whitespace) {
        return Err(Error::InvalidSyntax);
    }
    let count = raw.bytes().filter(|byte| *byte == b'\n').count();
    if count > 0 {
        out.breaks(count);
    }
    Ok(())
}
