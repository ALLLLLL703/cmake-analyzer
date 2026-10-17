// AI-generated tests.
use cmake_analyzer::{
    format::{self, Error, Options},
    workspace::{Workspace, model::DocumentSnapshot},
};
use std::sync::Arc;
const HEADER: &str = "cmake_minimum_required(VERSION 3.31)\n";
async fn snapshot(source: &str) -> Arc<DocumentSnapshot> {
    let root = std::env::temp_dir().join(format!("cmake-format-{}.cmake", std::process::id()));
    let mut workspace = Workspace::new(&root, None).unwrap();
    workspace.set_overlay(&root, source).unwrap();
    workspace.flush_documents().await.unwrap();
    workspace.snapshot(&root).unwrap().unwrap()
}
async fn formatted(source: &str, options: Options) -> String {
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
#[tokio::test]
async fn commands_blocks_branches_and_comments_keep_all_literal_bytes() {
    let input = format!(
        "{HEADER}if( TRUE )\nmessage( STATUS \"a  b\" ) # trailing\nelseif( FALSE )\n# body\nset( X [=[\nraw  value\n]=] )\nelse( )\nunknown( abc\\ def )\nendif( )\n"
    );
    let output = formatted(&input, Options::default()).await;
    assert_eq!(
        output,
        format!(
            "{HEADER}if(TRUE)\n  message(STATUS \"a  b\") # trailing\nelseif(FALSE)\n  # body\n  set(\n    X\n    [=[\nraw  value\n]=]\n  )\nelse()\n  unknown(abc\\ def)\nendif()\n"
        )
    );
    assert_eq!(formatted(&output, Options::default()).await, output);
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
#[tokio::test]
async fn long_calls_wrap_with_comments_and_nested_parentheses_without_reflowing_strings() {
    let input = format!(
        "{HEADER}if((TRUE) AND (FALSE))\nset( LONG_NAME \"a long literal whose contents and whitespace must remain completely untouched\" )\nset(A # value\n B #[=[ raw\ncomment ]=]\n C)\nendif()\n"
    );
    let options = Options {
        line_width: 60,
        ..Options::default()
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
#[tokio::test]
async fn crlf_tabs_empty_files_and_escaped_newlines_are_preserved_safely() {
    let input = "cmake_minimum_required(VERSION 3.31)\r\nif(TRUE)\r\nset(X foo\\\r\nbar) # comment\r\nendif()\r\n";
    let options = Options {
        insert_spaces: false,
        indent_width: 4,
        ..Options::default()
    };
    let output = formatted(input, options).await;
    assert!(
        output.contains("\r\n\tset(\r\n\t\tX\r\n\t\tfoo\\\r\nbar\r\n\t) # comment\r\n"),
        "{output:?}"
    );
    assert!(!output.contains("\r\r\n"));
    assert_eq!(formatted(&output, options).await, output);
    assert_eq!(formatted("", Options::default()).await, "");
}
#[tokio::test]
async fn comments_before_parentheses_and_after_empty_calls_keep_token_order() {
    for body in [
        "message #[[name comment]] ( STATUS x )\n",
        "unknown(#[[empty]]) # trailing\n",
    ] {
        let input = format!("{HEADER}{body}");
        let output = formatted(&input, Options::default()).await;
        assert_eq!(formatted(&output, Options::default()).await, output);
        if body.starts_with("message") {
            assert!(output.contains("message #[[name comment]] ("), "{output}");
        }
    }
}
#[tokio::test]
async fn invalid_syntax_invalid_settings_and_cancellation_never_return_rewrites() {
    let source = format!("{HEADER}set(X \"unfinished");
    let snapshot = snapshot(&source).await;
    assert_eq!(
        format::render(
            &source,
            &snapshot.lexed,
            &snapshot.semantics,
            Options::default(),
            || false
        ),
        Err(Error::InvalidSyntax)
    );
    assert_eq!(
        format::render(
            &source,
            &snapshot.lexed,
            &snapshot.semantics,
            Options::default(),
            || true
        ),
        Err(Error::Cancelled)
    );
    assert_eq!(
        format::render(
            &source,
            &snapshot.lexed,
            &snapshot.semantics,
            Options {
                indent_width: 0,
                ..Options::default()
            },
            || false
        ),
        Err(Error::InvalidOptions)
    );
}
