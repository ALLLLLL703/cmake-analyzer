//! Pure command/block layout using immutable semantic and lexical ranges. Builds a
//! new string, retaining literal/comment bytes; no files, processes or snapshot mutation.
//!
//! Contains AI-generated documentation additions.
use super::{
    model::{FormatError, FormatOptions, FormatWriter},
    trivia,
};
use crate::parser::{
    lexer::model::{LexContent, LexedBuffer},
    semanticer::model::{
        BuiltinCommand, CommandKind, SemanticArgForm, SemanticCommandRepresent, SemanticedBuffer,
    },
};

/// Render validated commands with branch/block indentation, preserving raw lexemes.
/// Allocates a new FormatWriter; no I/O or mutation of inputs.
///
/// # Arguments
/// - `source` `[in]`: original source with matching byte ranges.
/// - `lexed` `[in]`: matching lexical tokens/comments.
/// - `buffer` `[in]`: validated semantic commands in source order.
/// - `options` `[in]`: already validated positive widths.
/// - `stop` `[in]`: caller cancellation predicate.
///
/// # Returns
/// `Ok(String)` is the completed output with normalized final newline; `Err` discards partial in-memory output.
///
/// # Errors
/// Cancellation or invalid command/trivia ranges encountered by child renderers.
///
/// # Panics
/// Inconsistent source byte ranges may panic when sliced; widths must already be validated.
///
/// # Algorithm
/// Select CRLF only when the first LF is preceded by CR. Stream commands in order, render
/// preceding trivia, dedent closing/branch keywords, render calls and increase indentation for openers. Finish trailing trivia.
///
/// # Flow
/// ```text
///  [source newline style] ---> [writer]
///                                |
///                          [next command] <--------+
///                                |                |
///                     [trivia + dedent if needed] |
///                                |                |
///                          [render call]          |
///                                |                |
///                     [indent opener + cursor] ---+
///                                | EOF
///                        [trailing trivia + finish]
/// ```
///
/// # Examples
/// ```text
/// let output = render(source, &lexed, &semantics, FormatOptions::default(), &|| false)?;
/// ```
pub(super) fn render(
    source: &str,
    lexed: &LexedBuffer,
    buffer: &SemanticedBuffer,
    options: FormatOptions,
    stop: &impl Fn() -> bool,
) -> Result<String, FormatError> {
    let newline = if source
        .find('\n')
        .is_some_and(|offset| offset > 0 && source.as_bytes()[offset - 1] == b'\r')
    {
        "\r\n"
    } else {
        "\n"
    };
    let mut out = FormatWriter {
        text: String::with_capacity(source.len()),
        column: 0,
        trailing_breaks: 0,
        newline,
        options,
    };
    let mut cursor = 0;
    let mut level: usize = 0;
    for command in &buffer.commands {
        if stop() {
            return Err(FormatError::Cancelled);
        }
        trivia::render(
            source,
            lexed,
            cursor..command.full_span.start_byte,
            level,
            &mut out,
            stop,
        )?;
        let keyword = matches!(command.kind, CommandKind::Builtin(BuiltinCommand::KeyWord));
        let name = command.name.to_ascii_lowercase();
        let closing = keyword
            && matches!(
                name.as_str(),
                "endif" | "endforeach" | "endwhile" | "endfunction" | "endmacro" | "endblock"
            );
        let branch = keyword && matches!(name.as_str(), "else" | "elseif");
        if closing || branch {
            level = level.saturating_sub(1);
        }
        out.line();
        out.indent(level);
        render_command(source, lexed, command, level, &mut out, stop)?;
        if branch
            || keyword
                && matches!(
                    name.as_str(),
                    "if" | "foreach" | "while" | "function" | "macro" | "block"
                )
        {
            level += 1;
        }
        cursor = command.full_span.end_byte;
    }
    trivia::render(source, lexed, cursor..source.len(), level, &mut out, stop)?;
    Ok(out.finish())
}
/// Decide whether adjacent argument forms need one separating space. Pure comparison; no side effects.
///
/// # Arguments
/// - `previous` `[in]`: previous argument form, or None before the first.
/// - `current` `[in]`: next argument form.
///
/// # Returns
/// True except before the first argument, immediately after left parentheses or immediately before right parentheses.
///
/// # Examples
/// ```text
/// assert!(separated(Some(SemanticArgForm::Unquoted), SemanticArgForm::Unquoted));
/// ```
fn separated(previous: Option<SemanticArgForm>, current: SemanticArgForm) -> bool {
    previous.is_some_and(|previous| {
        previous != SemanticArgForm::LeftParen && current != SemanticArgForm::RightParen
    })
}
/// Append one command using compact or vertical argument layout without altering raw argument text.
/// Only the in-memory writer changes; no I/O or process execution.
///
/// # Arguments
/// - `source` `[in]`: matching original UTF-8 text.
/// - `lexed` `[in]`: tokens and comment ranges used to locate delimiters/attachments.
/// - `command` `[in]`: one closed semantic command with source-qualified argument ranges.
/// - `level` `[in]`: current block indentation depth.
/// - `out` `[in,out]`: receives appended name/delimiters/arguments/trivia; columns and breaks update, never cleared.
/// - `stop` `[in]`: cancellation predicate checked between arguments.
///
/// # Returns
/// `Ok(())` after the closing parenthesis; `Err` may leave partial text appended to out, which the outer renderer discards.
///
/// # Errors
/// InvalidSyntax for missing/invalid spans or delimiters/trivia; Cancelled on a requested stop.
///
/// # Panics
/// Inconsistent UTF-8 comment/command boundaries may panic on slicing; out must use validated widths.
///
/// # Algorithm
/// Find the lexical opening delimiter and comments. Measure scalar/tab columns without
/// splitting lexemes. Compact only when there are no internal comments/multiline arguments and
/// width fits; otherwise emit each argument on a continuation line and track nested parentheses.
///
/// # Flow
/// ```text
///  [name + args + comments]
///              |
///     [locate delimiters + measure]
///              |
///        <fits, plain?>
///         /          \
///       yes          no
///        |            |
///  [compact args] [trivia + vertical args]
///        |            |
///        |       [nested depth updates]
///        \            /
///         [closing delimiter appended]
/// ```
///
/// # Examples
/// ```text
/// render_command(source, &lexed, &command, 0, &mut out, &|| false)?;
/// ```
fn render_command(
    source: &str,
    lexed: &LexedBuffer,
    command: &SemanticCommandRepresent,
    level: usize,
    out: &mut FormatWriter<'_>,
    stop: &impl Fn() -> bool,
) -> Result<(), FormatError> {
    let name = command
        .span
        .text(source)
        .ok_or(FormatError::InvalidSyntax)?;
    let first = lexed
        .lex
        .partition_point(|token| token.span.start_byte < command.span.end_byte);
    let open = lexed
        .lex
        .get(first)
        .filter(|token| token.content == LexContent::LeftParentheses)
        .ok_or(FormatError::InvalidSyntax)?
        .span;
    let end = command.full_span.end_byte;
    if source.as_bytes().get(end.saturating_sub(1)) != Some(&b')') {
        return Err(FormatError::InvalidSyntax);
    }
    let comments = lexed
        .comments
        .partition_point(|comment| comment.end_byte <= command.span.end_byte);
    let has_comments = lexed
        .comments
        .get(comments)
        .is_some_and(|comment| comment.start_byte < end);
    let mut width = out.column + name.chars().count() + 1;
    let mut multiline = false;
    let mut previous = None;
    for arg in &command.args {
        if stop() {
            return Err(FormatError::Cancelled);
        }
        width += usize::from(separated(previous, arg.form));
        for ch in arg
            .span
            .text(source)
            .ok_or(FormatError::InvalidSyntax)?
            .chars()
        {
            match ch {
                '\n' => {
                    multiline = true;
                    width = 0;
                }
                '\r' => {}
                '\t' => width += out.options.indent_width - width % out.options.indent_width,
                _ => width += 1,
            }
        }
        previous = Some(arg.form);
    }
    width += 1;
    out.push(name);
    if lexed
        .comments
        .get(comments)
        .is_some_and(|comment| comment.start_byte < open.start_byte)
    {
        // A comment between the name and '(' must not acquire a new line boundary.
        out.push(&source[command.span.end_byte..open.start_byte]);
    }
    out.push("(");
    if !has_comments && !multiline && width <= out.options.line_width {
        let mut previous = None;
        for arg in &command.args {
            if separated(previous, arg.form) {
                out.push(" ");
            }
            out.push(arg.span.text(source).ok_or(FormatError::InvalidSyntax)?);
            previous = Some(arg.form);
        }
        out.push(")");
        return Ok(());
    }
    let mut cursor = open.end_byte;
    let mut nested: usize = 0;
    for arg in &command.args {
        if stop() {
            return Err(FormatError::Cancelled);
        }
        trivia::render(
            source,
            lexed,
            cursor..arg.span.start_byte,
            level + 1 + nested,
            out,
            stop,
        )?;
        if arg.form == SemanticArgForm::RightParen {
            nested = nested.saturating_sub(1);
        }
        out.line();
        out.indent(level + 1 + nested);
        out.push(arg.span.text(source).ok_or(FormatError::InvalidSyntax)?);
        if arg.form == SemanticArgForm::LeftParen {
            nested += 1;
        }
        cursor = arg.span.end_byte;
    }
    trivia::render(
        source,
        lexed,
        cursor..end - 1,
        level + 1 + nested,
        out,
        stop,
    )?;
    out.line();
    out.indent(level);
    out.push(")");
    Ok(())
}
