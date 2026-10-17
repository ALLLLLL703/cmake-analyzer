//! Leading line/block comment groups bind to declarations and individual set writes, not
//! blank-separated or trailing comments. Rendering also respects a byte cap.

// Contains AI-generated documentation edits.
// AI-generated declaration-documentation binding tests.
use super::analyze;
/// Analyze adjacent line/block documentation before a function, macro and set writes, plus
/// blank-separated and trailing-comment counterexamples.
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
/// If a stated invariant fails or an unwrapped fixture/setup operation fails.
///
/// # Algorithm
/// Check rendered groups bind only to the intended declarations/write, combine adjacent block/line
/// comments and respect a three-byte render cap.
///
/// # Flow
/// ```text
///   [adjacent/separated comments]
///    |
///   [bind declaration docs]----------------> [function + individual writes]
///                                           |
///                                          [compare rendering + byte cap]
///                                           |
///                                          < checks hold? >
///                                           /         \
///                                         [pass]     [panic]
/// ```
///
/// # Examples
/// Direct local call outside any Tokio runtime (the test attribute creates its runtime):
/// ```text
/// leading_comment_groups_bind_functions_macros_and_set_writes_only();
/// ```
#[tokio::test]
async fn leading_comment_groups_bind_functions_macros_and_set_writes_only() {
    let source = "cmake_minimum_required(VERSION 3.31)\n# First line.\n# Second line.\nfunction(f)\nendfunction()\n#[=[\nMacro documentation.\n]=]\nmacro(m)\nendmacro()\n# Variable documentation.\nset(A first)\nset(A second)\n# Separated.\n\nset(B value)\nset(C value) # trailing, not documentation for D\nset(D value)\n#[[Block line.]]\n# Another line.\nset(E value)\n";
    let buffer = analyze(source).await;
    for (name, expected) in [
        ("f", "First line.\nSecond line."),
        ("m", "Macro documentation."),
    ] {
        let function = buffer
            .functions
            .iter()
            .find(|function| function.name == name)
            .unwrap();
        assert_eq!(
            function
                .documentation
                .as_ref()
                .unwrap()
                .render(source, 1024),
            expected
        );
    }
    let a = buffer
        .variables
        .iter()
        .find(|variable| variable.name == "A")
        .unwrap();
    assert_eq!(
        a.changing[0]
            .documentation
            .as_ref()
            .unwrap()
            .render(source, 1024),
        "Variable documentation."
    );
    assert!(a.changing[1].documentation.is_none());
    for name in ["B", "C", "D"] {
        let variable = buffer
            .variables
            .iter()
            .find(|variable| variable.name == name)
            .unwrap();
        assert!(
            variable
                .changing
                .iter()
                .all(|write| write.documentation.is_none()),
            "{name}"
        );
    }
    let e = buffer
        .variables
        .iter()
        .find(|variable| variable.name == "E")
        .unwrap();
    assert_eq!(
        e.changing[0]
            .documentation
            .as_ref()
            .unwrap()
            .render(source, 1024),
        "Block line.\nAnother line."
    );
    assert!(
        e.changing[0]
            .documentation
            .as_ref()
            .unwrap()
            .render(source, 3)
            .len()
            <= 3
    );
}
