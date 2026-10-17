//! Convert semantic/build diagnostics into protocol ranges and messages.
//! Contextual callers provide cross-file variable warnings and resolved calls to
//! avoid duplicate or false local warnings. Conversion has no I/O or state mutation.
// AI-authored snapshot diagnostic conversion.
use super::text::TextIndex;
use crate::{
    model::TextSpan, parser::semanticer::model::SemanticError, workspace::model::DocumentSnapshot,
};
use lspally::server::model::*;

/// Convert local semantic/build findings while suppressing context-resolved command errors and duplicate local variable warnings.
/// No I/O or snapshot mutation occurs.
///
/// # Arguments
/// - `snapshot` `[in]`: Immutable analyzed text and semantic/build diagnostics.
/// - `encoding` `[in]`: Protocol coordinate units.
/// - `contextual` `[in]`: True when graph-level variable warnings replace file-local warnings.
/// - `resolved_calls` `[in]`: Snapshot-local command IDs resolved by owner/context/runtime observations; invalid IDs are ignored.
/// - `rules` `[in]`: Unknown-function/variable diagnostic levels; syntax/build errors remain errors.
///
/// # Returns
/// `Ok(diagnostics)` in semantic, optional local-variable, then build order; `Err` on range conversion.
///
/// # Errors
/// Invalid diagnostic byte ranges or coordinate overflow from TextIndex.
///
/// # Algorithm
/// Build a set of resolved command spans. Skip unknown-command errors at those spans, apply function severity rules, include local variable warnings only without contextual replacements, then append build errors.
///
/// # Flow
/// ```text
///  [Snapshot findings]       [Resolved command IDs]
///           |                         |
///           |                 [Known span set]
///           +-------------+-----------+
///                         v
///               [Filter command errors]
///                         |
///                  <Contextual?>
///                   /         \
///                 yes         no
///                  |           |
///                [Skip] [Local variable warnings]
///                  +-----------+
///                        v
///                [Append build errors]
/// ```
///
/// # Examples
/// ```text
/// let diagnostics = snapshot_diagnostics(&snapshot, encoding, false, &[], rules)?;
/// ```
pub(super) fn snapshot_diagnostics(
    snapshot: &DocumentSnapshot,
    encoding: CoordinateEncoding,
    contextual: bool,
    resolved_calls: &[crate::parser::semanticer::model::CommandId],
    rules: crate::config::DiagnosticRules,
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
        let level = if matches!(error.content, SemanticError::CommandNotFound(_)) {
            rules.unknown_function
        } else {
            crate::config::DiagnosticLevel::Error
        };
        if let Some(severity) = severity(level) {
            let mut item = diagnostic(&index, error.span, message(&error.content))?;
            item.severity = Some(DiagnosticSeverity(i32::from(severity)));
            diagnostics.push(item);
        }
    }
    if !contextual {
        for warning in &snapshot.semantics.warnings {
            let Some(severity) = variable_severity(warning.content.kind, rules) else {
                continue;
            };
            let mut item = diagnostic(&index, warning.span, warning.content.message())?;
            item.severity = Some(DiagnosticSeverity(i32::from(severity)));
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
/// Map configured diagnostic levels to LSP severity numbers; no side effects.
///
/// # Arguments
/// - `level` `[in]`: Configured error/warn/info/hint/none policy.
///
/// # Returns
/// `Some(1/2/3/4)` for error/warning/information/hint respectively; `None` suppresses the diagnostic.
///
/// # Examples
/// ```text
/// assert_eq!(severity(crate::config::DiagnosticLevel::Warn), Some(2));
/// ```
pub(super) fn severity(level: crate::config::DiagnosticLevel) -> Option<u8> {
    use crate::config::DiagnosticLevel::*;
    match level {
        Error => Some(1),
        Warn => Some(2),
        Info => Some(3),
        Hint => Some(4),
        None => Option::None,
    }
}
/// Keep empty-variable findings as warnings and apply configured levels to other variable warnings. No side effects.
///
/// # Arguments
/// - `kind` `[in]`: Semantic variable-warning category.
/// - `rules` `[in]`: Policy whose unknown_variable level governs nonempty-warning categories.
///
/// # Returns
/// `Some(2)` for Empty regardless of rules; otherwise severity(unknown_variable), including None for suppression.
///
/// # Examples
/// ```text
/// let level = variable_severity(SemanticWarnKind::Empty, rules);
/// assert_eq!(level, Some(2));
/// ```
pub(super) fn variable_severity(
    kind: crate::parser::semanticer::model::SemanticWarnKind,
    rules: crate::config::DiagnosticRules,
) -> Option<u8> {
    if kind == crate::parser::semanticer::model::SemanticWarnKind::Empty {
        Some(2)
    } else {
        severity(rules.unknown_variable)
    }
}
/// Create a source-tagged error diagnostic from one byte span and message. No I/O or input mutation.
///
/// # Arguments
/// - `index` `[in]`: Coordinate index for the exact text containing the span.
/// - `span` `[in]`: Half-open byte span converted endpoint by endpoint.
/// - `message` `[in]`: Consumed user-facing diagnostic text.
///
/// # Returns
/// `Ok(Diagnostic)` with severity error, source cmake-analyzer and converted range; `Err` on invalid endpoints.
///
/// # Errors
/// Invalid UTF-8 byte boundaries, out-of-document endpoints or coordinate overflow.
///
/// # Examples
/// ```text
/// let item = diagnostic(&index, span, "Missing command".into())?;
/// ```
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
/// Render a semantic failure as user-facing text, delegating parser failures to parser_message. No side effects.
///
/// # Arguments
/// - `error` `[in]`: Borrowed semantic failure, including any command/version/block details.
///
/// # Returns
/// Owned message for the exact error category; version messages are preserved verbatim.
///
/// # Examples
/// ```text
/// let text = message(&SemanticError::CommandNotFound("mystery".into()));
/// assert_eq!(text, "Unknown CMake command: mystery");
/// ```
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
/// Translate a lexical/parser failure into stable plain-English diagnostic text. No side effects.
///
/// # Arguments
/// - `error` `[in]`: Borrowed parser failure; matching lexical and parser delimiters share wording.
///
/// # Returns
/// Static message describing the missing delimiter, unexpected token/end, or argument parse failure.
///
/// # Examples
/// ```text
/// let text = parser_message(&crate::parser::nparser::model::NParsedError::ShouldBeCommand);
/// assert_eq!(text, "Expected a command name");
/// ```
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
