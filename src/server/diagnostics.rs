// AI-authored snapshot diagnostic conversion.
use super::text::TextIndex;
use crate::{
    model::TextSpan, parser::semanticer::model::SemanticError, workspace::model::DocumentSnapshot,
};
use lspally::server::model::*;

pub(super) fn snapshot_diagnostics(
    snapshot: &DocumentSnapshot,
    encoding: CoordinateEncoding,
    contextual: bool,
    resolved_calls: &[crate::parser::semanticer::model::CommandId],
) -> ServerResult<Vec<Diagnostic>> {
    let index = TextIndex::new(&snapshot.text, encoding);
    let mut diagnostics =
        Vec::with_capacity(snapshot.semantics.diagnostics.len() + snapshot.build.diagnostics.len());
    let resolved: std::collections::HashSet<_> = resolved_calls
        .iter()
        .filter_map(|id| snapshot.semantics.commands.get(id.id as usize))
        .map(|command| (command.span.start_byte, command.span.end_byte))
        .collect();
    for error in &snapshot.semantics.diagnostics {
        if matches!(error.content, SemanticError::CommandNotFound(_))
            && resolved.contains(&(error.span.start_byte, error.span.end_byte))
        {
            continue;
        }
        diagnostics.push(diagnostic(&index, error.span, message(&error.content))?);
    }
    if !contextual {
        for warning in &snapshot.semantics.warnings {
            let mut item = diagnostic(&index, warning.span, warning.content.message())?;
            item.severity = Some(DiagnosticSeverity(2));
            diagnostics.push(item);
        }
    }
    for error in &snapshot.build.diagnostics {
        diagnostics.push(diagnostic(
            &index,
            error.source.span,
            error.message.clone(),
        )?);
    }
    Ok(diagnostics)
}
pub(super) fn diagnostic(
    index: &TextIndex<'_>,
    span: TextSpan,
    message: String,
) -> ServerResult<Diagnostic> {
    Ok(Diagnostic {
        range: index.range(span.start_byte..span.end_byte)?,
        severity: Some(DiagnosticSeverity(1)),
        code: None,
        code_description: None,
        source: Some("cmake-analyzer".into()),
        message,
        related_information: None,
        tags: None,
        data: None,
    })
}
fn message(error: &SemanticError) -> String {
    match error {
        SemanticError::CommandNotFound(name) => format!("Unknown CMake command: {name}"),
        SemanticError::Version(message) => message.clone(),
        SemanticError::UnsupportedVersion(version) => format!(
            "Unsupported CMake version: {}.{}",
            version.major, version.minor
        ),
        SemanticError::UnclosedBlock(kind) => format!("Unclosed {kind:?} block"),
        SemanticError::UnexpectedBlockEnd(name) => format!("Unexpected block end: {name}"),
        SemanticError::UnexpectedBranch(name) => format!("Unexpected branch: {name}"),
        SemanticError::InvalidIfBranch(name) => format!("Invalid if branch: {name}"),
        SemanticError::NParsedError(error) => parser_message(error).into(),
    }
}
fn parser_message(error: &crate::parser::nparser::model::NParsedError) -> &'static str {
    use crate::parser::{lexer::model::LexError as L, nparser::model::NParsedError as N};
    match error {
        N::LexError(L::UnclosedParentheses) | N::UnclosedLeftParentheses => "Unclosed parenthesis",
        N::LexError(L::UnclosedStringLiteral) => "Unclosed quoted argument",
        N::LexError(L::UnclosedBracketArgument) => "Unclosed bracket argument",
        N::LexError(L::UnclosedBracketComment) => "Unclosed bracket comment",
        N::LexError(L::ReachTheEof) | N::ReachTheEof => "Unexpected end of file",
        N::ShouldBeCommand => "Expected a command name",
        N::ShouldHaveLeftParentheses => "Expected '(' after the command name",
        N::UnArgStringLiteral | N::UnArgBracket => "Expected a command, not an argument",
        N::CalledInnerError => "Cannot parse command arguments",
    }
}
