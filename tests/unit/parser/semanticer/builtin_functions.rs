// AI-generated used-builtin SemanticFunction tests.
use super::analyze;
use cmake_analyzer::parser::semanticer::model::{FunctionKind, ReferenceKind};
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
