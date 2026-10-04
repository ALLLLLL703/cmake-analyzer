use cmake_analyzer::parser::lexer::model::{Lexer, RawBuffer};

#[test]
fn advance_ident_test() {
    let buffer = RawBuffer::new("message(\"114514\")");

    Lexer::new(buffer);
}
