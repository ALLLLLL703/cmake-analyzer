//! Used builtin commands get definitionless function records; user declarations with the same name
//! retain separate definitions and calls. No external side effects.

// Contains AI-generated documentation edits.
// AI-generated used-builtin SemanticFunction tests.
use super::analyze;
use cmake_analyzer::parser::semanticer::model::{FunctionKind, ReferenceKind};
/// Analyze two differently cased native message calls, a shadowing message function, a later user
/// call and an unfinished set.
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
/// Require definitionless builtin call records only for used names, separate user references and no
/// unused add_library record.
///
/// # Flow
/// ```text
///   [native + shadow message calls]
///    |
///   [analyze function records]---------------> [builtin / user references]
///                                             |
///                                            [check separate calls + metadata]
///                                             |
///                                            < checks hold? >
///                                             /         \
///                                           [pass]     [panic]
/// ```
///
/// # Examples
/// Direct local call outside any Tokio runtime (the test attribute creates its runtime):
/// ```text
/// only_used_builtins_are_definitionless_and_user_shadows_keep_their_own_calls();
/// ```
#[tokio::test]
async fn only_used_builtins_are_definitionless_and_user_shadows_keep_their_own_calls() {
    let source = "cmake_minimum_required(VERSION 3.31)\nmessage(before)\nMESSAGE(again)\nfunction(message P)\nendfunction()\nmessage(after)\nset(";
    let buffer = analyze(source).await;
    let native = buffer
        .functions
        .iter()
        .find(|function| function.name == "message" && function.kind == FunctionKind::Builtin)
        .unwrap();
    assert_eq!(native.references.len(), 2);
    for function in buffer
        .functions
        .iter()
        .filter(|function| function.kind == FunctionKind::Builtin)
    {
        assert!(
            function.definition.is_none()
                && function.block.is_none()
                && function.scope.is_none()
                && function.parameters.is_empty()
        );
        assert!(
            function
                .references
                .iter()
                .all(|reference| reference.kind == ReferenceKind::Call)
        );
    }
    for name in ["cmake_minimum_required", "function", "endfunction", "set"] {
        assert!(
            buffer
                .functions
                .iter()
                .any(|function| function.name == name && function.kind == FunctionKind::Builtin),
            "{name}"
        );
    }
    assert!(
        !buffer
            .functions
            .iter()
            .any(|function| function.name == "add_library")
    );
    let user = buffer
        .functions
        .iter()
        .find(|function| function.name == "message" && function.kind == FunctionKind::Function)
        .unwrap();
    assert_eq!(user.references.len(), 2);
    assert_eq!(
        user.definition.as_ref().unwrap().span.text(source),
        Some("message")
    );
}
