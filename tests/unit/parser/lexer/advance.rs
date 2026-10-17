//! Construction-only lexer smoke test. Despite its historical name, the test does not call an
//! advance method or inspect tokens.

// Contains AI-generated documentation edits.
use cmake_analyzer::parser::lexer::model::{Lexer, RawBuffer};

/// Construct a raw buffer containing a quoted message and pass it to the lexer constructor. This is
/// only a no-panic construction smoke test: it neither advances nor asserts identifier contents.
///
/// Only local values and analysis buffers are created or changed; no file, network or process side
/// effects.
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
/// Local test-module call:
/// ```text
/// advance_ident_test();
/// ```
#[test]
fn advance_ident_test() {
    let buffer = RawBuffer::new("message(\"114514\")");

    Lexer::new(buffer);
}
