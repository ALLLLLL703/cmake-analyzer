// AI-generated comment-span and incremental side-channel tests.
use cmake_analyzer::parser::{
    lexer::model::{IncrementalLexer, LexError, Lexer, RawBuffer},
    nparser::model::NParser,
};
#[test]
fn comment_ranges_preserve_delimiters_without_turning_payloads_into_comments() {
    let text = "# first 🌟\r\nset(A \"# quoted\" [=[# bracket]=] escaped\\#)\n#[==[\r\nblock\r\n]==]\n# last";
    let lexed = Lexer::new(RawBuffer::new(text)).parse();
    let comments: Vec<_> = lexed
        .comments
        .iter()
        .map(|span| span.text(text).unwrap())
        .collect();
    assert_eq!(
        comments,
        vec!["# first 🌟\r", "#[==[\r\nblock\r\n]==]", "# last"]
    );
    assert!(lexed.errors.is_empty());
    let parsed = NParser::new(lexed.clone()).parse(text);
    assert_eq!(parsed.comments, lexed.comments);
    assert_eq!(parsed.nodes.len(), 1);
}
#[test]
fn unclosed_block_comments_retain_their_range_and_error() {
    let text = "#[=[unclosed 🌟\ncomment";
    let lexed = Lexer::new(RawBuffer::new(text)).parse();
    assert_eq!(lexed.comments.len(), 1);
    assert_eq!(lexed.comments[0].text(text), Some(text));
    assert!(
        lexed
            .errors
            .iter()
            .any(|error| matches!(error.content, LexError::UnclosedBracketComment))
    );
}
#[test]
fn incremental_comments_match_fresh_prefix_suffix_and_error_recovery() {
    let mut lexer = IncrementalLexer::new("# one\nset(A x)\n# last\nmessage(x)\n");
    for text in [
        "# changed 🌟\nset(A x)\n# last\nmessage(x)\n",
        "# changed 🌟\nset(A x)\n#[=[block\ntext]=]\nmessage(x)\n",
        "set(A x)\nmessage(x)\n",
        "#[=[unterminated",
        "# repaired\nmessage(x)\n",
    ] {
        lexer.update(text);
        let fresh = Lexer::new(RawBuffer::new(text)).parse();
        assert_eq!(lexer.buffer.comments, fresh.comments, "{text}");
        assert_eq!(
            format!("{:?}", lexer.buffer.lex),
            format!("{:?}", fresh.lex)
        );
        assert_eq!(
            format!("{:?}", lexer.buffer.errors),
            format!("{:?}", fresh.errors)
        );
    }
}
