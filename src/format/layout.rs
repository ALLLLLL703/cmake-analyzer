use super::{
    model::{Error, Options, Writer},
    trivia,
};
use crate::parser::{
    lexer::model::{LexContent, LexedBuffer},
    semanticer::model::{
        BuiltinCommand, CommandKind, SemanticArgForm, SemanticCommandRepresent, SemanticedBuffer,
    },
};

pub(super) fn render(
    source: &str,
    lexed: &LexedBuffer,
    buffer: &SemanticedBuffer,
    options: Options,
    stop: &impl Fn() -> bool,
) -> Result<String, Error> {
    let newline = if source
        .find('\n')
        .is_some_and(|offset| offset > 0 && source.as_bytes()[offset - 1] == b'\r')
    {
        "\r\n"
    } else {
        "\n"
    };
    let mut out = Writer {
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
            return Err(Error::Cancelled);
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
fn separated(previous: Option<SemanticArgForm>, current: SemanticArgForm) -> bool {
    previous.is_some_and(|previous| {
        previous != SemanticArgForm::LeftParen && current != SemanticArgForm::RightParen
    })
}
fn render_command(
    source: &str,
    lexed: &LexedBuffer,
    command: &SemanticCommandRepresent,
    level: usize,
    out: &mut Writer<'_>,
    stop: &impl Fn() -> bool,
) -> Result<(), Error> {
    let name = command.span.text(source).ok_or(Error::InvalidSyntax)?;
    let first = lexed
        .lex
        .partition_point(|token| token.span.start_byte < command.span.end_byte);
    let open = lexed
        .lex
        .get(first)
        .filter(|token| token.content == LexContent::LeftParentheses)
        .ok_or(Error::InvalidSyntax)?
        .span;
    let end = command.full_span.end_byte;
    if source.as_bytes().get(end.saturating_sub(1)) != Some(&b')') {
        return Err(Error::InvalidSyntax);
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
            return Err(Error::Cancelled);
        }
        width += usize::from(separated(previous, arg.form));
        for ch in arg.span.text(source).ok_or(Error::InvalidSyntax)?.chars() {
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
            out.push(arg.span.text(source).ok_or(Error::InvalidSyntax)?);
            previous = Some(arg.form);
        }
        out.push(")");
        return Ok(());
    }
    let mut cursor = open.end_byte;
    let mut nested: usize = 0;
    for arg in &command.args {
        if stop() {
            return Err(Error::Cancelled);
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
        out.push(arg.span.text(source).ok_or(Error::InvalidSyntax)?);
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
