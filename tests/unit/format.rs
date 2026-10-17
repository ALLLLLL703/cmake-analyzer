//! Formatter regressions using overlay-only document snapshots. Tests compare rendered bytes,
//! token/comment preservation, repeat formatting and refusal paths; they do not write formatted
//! files.

// Contains AI-generated documentation edits.
// AI-generated tests.
use cmake_analyzer::{
    format::{self, FormatError, FormatOptions},
    workspace::{Workspace, model::DocumentSnapshot},
};
use std::sync::Arc;
/// Required-version prefix shared by formatter fixtures; keeps semantic analysis on CMake 3.31.
/// Example: `format!("{HEADER}set(X x)\n")` adds a valid fixture body.
const HEADER: &str = "cmake_minimum_required(VERSION 3.31)\n";
/// Analyze an overlay under a process-specific temporary filename for formatter tests. Mutates a
/// new local workspace and may read supporting paths, but does not write the source or execute
/// CMake.
///
/// # Arguments
/// - `source` `[in]`: CMake text installed as the entire file overlay, replacing any disk contents
///   for analysis.
///
/// # Returns
/// A shared immutable document snapshot containing text, tokens and semantics; remains usable after
/// the local workspace is dropped.
///
/// # Panics
/// If workspace creation, overlay installation, document flushing or snapshot lookup fails or
/// returns no snapshot.
///
/// # Algorithm
/// Create a workspace at a process-specific path, replace its source with an overlay, flush
/// document analysis only and retain the resulting immutable snapshot.
///
/// # Flow
/// ```text
/// [source overlay]
///         |
/// [new local workspace] ----> [flush_documents]
///                                  |
///                         [immutable snapshot]
///                                  |
///                           [return Arc]
/// ```
///
/// # Examples
/// Local test-module context (await inside an async test):
/// ```text
/// let document = snapshot("cmake_minimum_required(VERSION 3.31)\nset(X x)\n").await;
/// assert_eq!(document.semantics.commands.len(), 2);
/// ```
async fn snapshot(source: &str) -> Arc<DocumentSnapshot> {
    let root = std::env::temp_dir().join(format!("cmake-format-{}.cmake", std::process::id()));
    let mut workspace = Workspace::new(&root, None).unwrap();
    workspace.set_overlay(&root, source).unwrap();
    workspace.flush_documents().await.unwrap();
    workspace.snapshot(&root).unwrap().unwrap()
}
/// Render an overlay snapshot with the supplied formatter options and cancellation disabled.
/// Creates local analysis state through [`snapshot`]; does not write formatted text to disk or
/// execute CMake.
///
/// # Arguments
/// - `source` `[in]`: Complete CMake input to analyze and format.
/// - `options` `[in]`: Indentation and line-width policy passed unchanged to the renderer.
///
/// # Returns
/// The rendered replacement string, including the renderer's newline policy; input is not mutated.
///
/// # Panics
/// If snapshot setup fails or rendering returns an error, including invalid source or options.
///
/// # Examples
/// Local test-module context (await inside an async test):
/// ```text
/// let output = formatted("cmake_minimum_required(VERSION 3.31)\nset( X x )\n", FormatOptions::default()).await;
/// assert!(output.ends_with("set(X x)\n"));
/// ```
async fn formatted(source: &str, options: FormatOptions) -> String {
    let snapshot = snapshot(source).await;
    format::render(
        &snapshot.text,
        &snapshot.lexed,
        &snapshot.semantics,
        options,
        || false,
    )
    .unwrap()
}
/// Format an if/elseif/else fixture containing trailing/body comments, a bracket literal and an
/// escaped space.
///
/// Mutates only local workspace overlays/snapshots and may read supporting paths; no source-file
/// writes or CMake execution.
///
/// # Arguments
/// None. The test builds its own fixture and expected results.
///
/// # Returns
/// `()` after all checks pass; no data or output parameter is returned.
///
/// # Panics
/// If a stated invariant fails or an unwrapped fixture/setup operation fails.
///
/// # Algorithm
/// Require exact indentation, idempotence and identical token/comment bytes between old and new
/// snapshots.
///
/// # Flow
/// ```text
///   [blocks/comments/literals]
///    |
///   [format + reformat]-----------------> [old/new snapshots]
///                                        |
///                                       [compare token + comment bytes]
///                                        |
///                                       < checks hold? >
///                                        /         \
///                                      [pass]     [panic]
/// ```
///
/// # Examples
/// Direct local call outside any Tokio runtime (the test attribute creates its runtime):
/// ```text
/// commands_blocks_branches_and_comments_keep_all_literal_bytes();
/// ```
#[tokio::test]
async fn commands_blocks_branches_and_comments_keep_all_literal_bytes() {
    let input = format!(
        "{HEADER}if( TRUE )\nmessage( STATUS \"a  b\" ) # trailing\nelseif( FALSE )\n# body\nset( X [=[\nraw  value\n]=] )\nelse( )\nunknown( abc\\ def )\nendif( )\n"
    );
    let output = formatted(&input, FormatOptions::default()).await;
    assert_eq!(
        output,
        format!(
            "{HEADER}if(TRUE)\n  message(STATUS \"a  b\") # trailing\nelseif(FALSE)\n  # body\n  set(\n    X\n    [=[\nraw  value\n]=]\n  )\nelse()\n  unknown(abc\\ def)\nendif()\n"
        )
    );
    assert_eq!(formatted(&output, FormatOptions::default()).await, output);
    let old = snapshot(&input).await;
    let new = snapshot(&output).await;
    let tokens = |snapshot: &DocumentSnapshot| {
        snapshot
            .lexed
            .lex
            .iter()
            .map(|token| {
                (
                    token.content,
                    token.span.text(&snapshot.text).unwrap().to_owned(),
                )
            })
            .collect::<Vec<_>>()
    };
    assert_eq!(tokens(&old), tokens(&new));
    let comments = |snapshot: &DocumentSnapshot| {
        snapshot
            .lexed
            .comments
            .iter()
            .map(|span| span.text(&snapshot.text).unwrap().to_owned())
            .collect::<Vec<_>>()
    };
    assert_eq!(comments(&old), comments(&new));
}
/// Set a 60-column limit for nested conditions, an oversized quoted literal and interspersed
/// line/block comments.
///
/// Mutates only local workspace overlays/snapshots and may read supporting paths; no source-file
/// writes or CMake execution.
///
/// # Arguments
/// None. The test builds its own fixture and expected results.
///
/// # Returns
/// `()` after all checks pass; no data or output parameter is returned.
///
/// # Panics
/// If a stated invariant fails or an unwrapped fixture/setup operation fails.
///
/// # Algorithm
/// Require vertical wrapping without string reflow, repeat-format stability and identical token
/// bytes.
///
/// # Flow
/// ```text
///   [60-column options]
///    |
///   [wrap calls, preserve literals]----------> [old/new snapshots]
///                                             |
///                                            [compare tokens + stability]
///                                             |
///                                            < checks hold? >
///                                             /         \
///                                           [pass]     [panic]
/// ```
///
/// # Examples
/// Direct local call outside any Tokio runtime (the test attribute creates its runtime):
/// ```text
/// long_calls_wrap_with_comments_and_nested_parentheses_without_reflowing_strings();
/// ```
#[tokio::test]
async fn long_calls_wrap_with_comments_and_nested_parentheses_without_reflowing_strings() {
    let input = format!(
        "{HEADER}if((TRUE) AND (FALSE))\nset( LONG_NAME \"a long literal whose contents and whitespace must remain completely untouched\" )\nset(A # value\n B #[=[ raw\ncomment ]=]\n C)\nendif()\n"
    );
    let options = FormatOptions {
        line_width: 60,
        ..FormatOptions::default()
    };
    let output = formatted(&input, options).await;
    assert!(output.contains("set(\n    LONG_NAME\n    \"a long literal whose contents and whitespace must remain completely untouched\"\n  )"),"{output}");
    assert!(
        output.contains("A # value\n    B #[=[ raw\ncomment ]=]\n    C"),
        "{output}"
    );
    assert_eq!(formatted(&output, options).await, output);
    let old = snapshot(&input).await;
    let new = snapshot(&output).await;
    let tokens = |snapshot: &DocumentSnapshot| {
        snapshot
            .lexed
            .lex
            .iter()
            .map(|token| {
                (
                    token.content,
                    token.span.text(&snapshot.text).unwrap().to_owned(),
                )
            })
            .collect::<Vec<_>>()
    };
    assert_eq!(tokens(&old), tokens(&new));
}
/// Render a CRLF fixture with tab indentation and an escaped CRLF inside an unquoted argument.
///
/// Mutates only local workspace overlays/snapshots and may read supporting paths; no source-file
/// writes or CMake execution.
///
/// # Arguments
/// None. The test builds its own fixture and expected results.
///
/// # Returns
/// `()` after all checks pass; no data or output parameter is returned.
///
/// # Panics
/// If a stated invariant fails.
///
/// # Algorithm
/// Require intact continuation bytes, no doubled carriage returns, idempotence and unchanged empty
/// input.
///
/// # Flow
/// ```text
///   [CRLF + escaped newline]
///    |
///   [render with tabs]----------------> [second render / empty source]
///                                      |
///                                     [check stability + exact bytes]
///                                      |
///                                     < checks hold? >
///                                      /         \
///                                    [pass]     [panic]
/// ```
///
/// # Examples
/// Direct local call outside any Tokio runtime (the test attribute creates its runtime):
/// ```text
/// crlf_tabs_empty_files_and_escaped_newlines_are_preserved_safely();
/// ```
#[tokio::test]
async fn crlf_tabs_empty_files_and_escaped_newlines_are_preserved_safely() {
    let input = "cmake_minimum_required(VERSION 3.31)\r\nif(TRUE)\r\nset(X foo\\\r\nbar) # comment\r\nendif()\r\n";
    let options = FormatOptions {
        insert_spaces: false,
        indent_width: 4,
        ..FormatOptions::default()
    };
    let output = formatted(input, options).await;
    assert!(
        output.contains("\r\n\tset(\r\n\t\tX\r\n\t\tfoo\\\r\nbar\r\n\t) # comment\r\n"),
        "{output:?}"
    );
    assert!(!output.contains("\r\r\n"));
    assert_eq!(formatted(&output, options).await, output);
    assert_eq!(formatted("", FormatOptions::default()).await, "");
}
/// Format a comment between command name and opening parenthesis and a comment inside an empty
/// call. Require stable second formatting and preserve the name/comment/parenthesis ordering.
///
/// Mutates only local workspace overlays/snapshots and may read supporting paths; no source-file
/// writes or CMake execution.
///
/// # Arguments
/// None. The test builds its own fixture and expected results.
///
/// # Returns
/// `()` after all checks pass; no data or output parameter is returned.
///
/// # Panics
/// If a stated invariant fails.
///
/// # Examples
/// Direct local call outside any Tokio runtime (the test attribute creates its runtime):
/// ```text
/// comments_before_parentheses_and_after_empty_calls_keep_token_order();
/// ```
#[tokio::test]
async fn comments_before_parentheses_and_after_empty_calls_keep_token_order() {
    for body in [
        "message #[[name comment]] ( STATUS x )\n",
        "unknown(#[[empty]]) # trailing\n",
    ] {
        let input = format!("{HEADER}{body}");
        let output = formatted(&input, FormatOptions::default()).await;
        assert_eq!(formatted(&output, FormatOptions::default()).await, output);
        if body.starts_with("message") {
            assert!(output.contains("message #[[name comment]] ("), "{output}");
        }
    }
}
/// Use an unfinished quoted set call to require invalid-syntax refusal, cancellation refusal and
/// invalid-options refusal when indent width is zero.
///
/// Mutates only local workspace overlays/snapshots and may read supporting paths; no source-file
/// writes or CMake execution.
///
/// # Arguments
/// None. The test builds its own fixture and expected results.
///
/// # Returns
/// `()` after all checks pass; no data or output parameter is returned.
///
/// # Panics
/// If a stated invariant fails.
///
/// # Algorithm
/// These paths return errors instead of replacement text.
///
/// # Flow
/// ```text
///   [unfinished set source]
///    |
///   [InvalidSyntax]------------------> [cancel / zero indent]
///                                     |
///                                    [Cancelled / InvalidOptions]
///                                     |
///                                    < checks hold? >
///                                     /         \
///                                   [pass]     [panic]
/// ```
///
/// # Examples
/// Direct local call outside any Tokio runtime (the test attribute creates its runtime):
/// ```text
/// invalid_syntax_invalid_settings_and_cancellation_never_return_rewrites();
/// ```
#[tokio::test]
async fn invalid_syntax_invalid_settings_and_cancellation_never_return_rewrites() {
    let source = format!("{HEADER}set(X \"unfinished");
    let snapshot = snapshot(&source).await;
    assert_eq!(
        format::render(
            &source,
            &snapshot.lexed,
            &snapshot.semantics,
            FormatOptions::default(),
            || false
        ),
        Err(FormatError::InvalidSyntax)
    );
    assert_eq!(
        format::render(
            &source,
            &snapshot.lexed,
            &snapshot.semantics,
            FormatOptions::default(),
            || true
        ),
        Err(FormatError::Cancelled)
    );
    assert_eq!(
        format::render(
            &source,
            &snapshot.lexed,
            &snapshot.semantics,
            FormatOptions {
                indent_width: 0,
                ..FormatOptions::default()
            },
            || false
        ),
        Err(FormatError::InvalidOptions)
    );
}
