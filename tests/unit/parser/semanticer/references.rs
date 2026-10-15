// AI-generated reference extraction, function, and warning tests.
use super::analyze;
use cmake_analyzer::parser::semanticer::model::{
    FunctionKind, ReferenceKind, ReferenceTarget, SemanticWarnKind,
};

#[tokio::test]
async fn references_keep_argument_command_file_and_exact_name_spans() {
    let source = r#"cmake_minimum_required(VERSION 3.31)
set(变量 value)
message("${变量}:${MISSING}" [=[${BRACKET}]=] "\${ESCAPED}" "$ENV{PATH}" "$CACHE{CACHE_NAME}" "${${PREFIX}_DYNAMIC}")
list(LENGTH 变量 COUNT)
"#;
    let buffer = analyze(source).await;
    let command = &buffer.commands[2];
    let names: Vec<_> = command
        .variable_references
        .iter()
        .map(|reference| reference.name.as_str())
        .collect();
    assert_eq!(names, vec!["变量", "MISSING", "PREFIX"]);
    for reference in &command.variable_references {
        assert_eq!(reference.location.file, buffer.id);
        assert_eq!(reference.location.command, command.id);
        assert_eq!(
            reference.location.span.text(source),
            Some(reference.name.as_str())
        );
        assert!(reference.argument < command.args.len());
    }
    let variable = buffer
        .variables
        .iter()
        .find(|variable| variable.name == "变量")
        .unwrap();
    assert_eq!(
        variable
            .references
            .iter()
            .map(|reference| reference.kind)
            .collect::<Vec<_>>(),
        vec![
            ReferenceKind::Definition,
            ReferenceKind::Read,
            ReferenceKind::Read
        ]
    );
    assert_eq!(buffer.commands[3].variable_references[0].name, "变量");
    assert!(
        buffer.commands[3]
            .variable_references
            .iter()
            .all(|reference| reference.name != "COUNT")
    );
    assert!(
        buffer
            .unresolved_references
            .iter()
            .any(|reference| reference.name == "MISSING"
                && reference.target == ReferenceTarget::Variable)
    );
    assert!(buffer.warnings.iter().all(|warning| !matches!(
        warning.content.variable.as_str(),
        "BRACKET" | "ESCAPED" | "PATH" | "CACHE_NAME"
    )));
}

#[test]
fn reference_ids_keep_the_underlying_index_layout() {
    use cmake_analyzer::parser::semanticer::model::{
        ReferenceEnvironmentId, ReferenceInvocationId,
    };
    assert_eq!(
        std::mem::size_of::<ReferenceEnvironmentId>(),
        std::mem::size_of::<usize>()
    );
    assert_eq!(
        std::mem::align_of::<ReferenceEnvironmentId>(),
        std::mem::align_of::<usize>()
    );
    assert_eq!(
        std::mem::size_of::<ReferenceInvocationId>(),
        std::mem::size_of::<usize>()
    );
    assert_eq!(
        std::mem::align_of::<ReferenceInvocationId>(),
        std::mem::align_of::<usize>()
    );
}

#[tokio::test]
async fn complete_expansions_are_indexed_inside_unclosed_commands() {
    let buffer =
        analyze("cmake_minimum_required(VERSION 3.31)\nset(A value)\nmessage(\"${A}\"").await;
    assert!(!buffer.diagnostics.is_empty());
    assert!(
        buffer
            .variables
            .iter()
            .find(|variable| variable.name == "A")
            .unwrap()
            .references
            .iter()
            .any(|reference| reference.kind == ReferenceKind::Read)
    );
}

#[tokio::test]
async fn functions_macros_and_parameters_keep_definitions_and_calls() {
    let source = "cmake_minimum_required(VERSION 3.31)\nfunction(F P)\nmessage(\"${P}\")\nset(LOCAL \"\")\nmessage(\"${LOCAL}\")\nendfunction()\nf(value)\nmacro(M ARG)\nmessage(\"${ARG}\")\nendmacro()\nm(value)\nunknown_fn()\n";
    let buffer = analyze(source).await;
    assert_eq!(buffer.functions.len(), 2);
    for function in &buffer.functions {
        assert_eq!(function.references.len(), 2, "{function:?}");
        assert_eq!(function.references[0].kind, ReferenceKind::Definition);
        assert_eq!(function.references[1].kind, ReferenceKind::Call);
        assert_eq!(
            function.definition.span.text(source),
            Some(function.name.as_str())
        );
        assert_eq!(function.parameters.len(), 1);
    }
    assert_eq!(buffer.functions[0].kind, FunctionKind::Function);
    assert_eq!(buffer.functions[1].kind, FunctionKind::Macro);
    assert!(buffer.variables.iter().any(|variable| {
        variable.name == "P"
            && variable
                .references
                .iter()
                .any(|reference| reference.kind == ReferenceKind::Definition)
    }));
    assert!(
        !buffer
            .warnings
            .iter()
            .any(|warning| matches!(warning.content.variable.as_str(), "P" | "ARG"))
    );
    assert!(
        buffer
            .warnings
            .iter()
            .any(|warning| warning.content.variable == "LOCAL"
                && warning.content.kind == SemanticWarnKind::Empty)
    );
    assert_eq!(buffer.diagnostics.len(), 1, "{:?}", buffer.diagnostics);
    assert!(
        buffer
            .unresolved_references
            .iter()
            .any(|reference| reference.name == "unknown_fn"
                && reference.target == ReferenceTarget::Function)
    );
    assert!(format!("{}", buffer.display(false)).contains("WARNINGS"));
}

#[tokio::test]
async fn namespaces_are_not_mixed_and_nonlocal_writes_remain_unresolved() {
    let source = "cmake_minimum_required(VERSION 3.31)\nset(WORD CACHE)\nmessage(\"${WORD}\")\nset(CKEY value CACHE STRING doc)\nset(ENV{ENVKEY} value)\nmessage(\"$CACHE{CKEY}\" \"$ENV{ENVKEY}\")\nfunction(F)\nset(PARENT value PARENT_SCOPE)\nendfunction()\n";
    let buffer = analyze(source).await;
    assert!(buffer.warnings.is_empty(), "{:?}", buffer.warnings);
    assert!(
        buffer
            .variables
            .iter()
            .filter(|variable| variable.name == "CKEY" || variable.name == "ENV{ENVKEY}")
            .all(|variable| variable.references.is_empty())
    );
    assert!(
        buffer
            .unresolved_references
            .iter()
            .any(|reference| reference.name == "PARENT"
                && reference.location.kind == ReferenceKind::Write)
    );
}

#[tokio::test]
async fn deferred_calls_link_later_definitions_and_calls_invalidate_value_facts() {
    let source = "cmake_minimum_required(VERSION 3.31)\nfunction(F)\nLater()\nendfunction()\nmacro(M)\nset(V changed)\nendmacro()\nset(V \"\")\nM()\nmessage(\"${V}\")\nfunction(Later)\nendfunction()\nset(L \"\")\nlist(APPEND L x)\nmessage(\"${L}\")\n";
    let buffer = analyze(source).await;
    assert!(buffer.diagnostics.is_empty(), "{:?}", buffer.diagnostics);
    let later = buffer
        .functions
        .iter()
        .find(|function| function.name == "Later")
        .unwrap();
    assert!(
        later
            .references
            .iter()
            .any(|reference| reference.kind == ReferenceKind::Call)
    );
    assert!(
        !buffer
            .warnings
            .iter()
            .any(|warning| warning.span.row == 9 && warning.content.variable == "V")
    );
    assert!(
        !buffer
            .warnings
            .iter()
            .any(|warning| warning.span.row == 14 && warning.content.variable == "L")
    );
}

#[tokio::test]
async fn warnings_distinguish_unset_empty_unknown_and_conditional_values() {
    let source = "cmake_minimum_required(VERSION 3.31)\nmessage(\"${MISSING}\")\nunset(U)\nmessage(\"${U}\")\nset(E \"\")\nmessage(\"${E}\")\nset(D \"${INPUT}\")\nmessage(\"${D}\")\nset(V \"\")\nif(FLAG)\nset(V value)\nelse()\nset(V \"\")\nendif()\nmessage(\"${V}\")\nset(Z)\nmessage(\"${Z}\")\n";
    let buffer = analyze(source).await;
    let warning = |name: &str| {
        buffer
            .warnings
            .iter()
            .find(|warning| warning.content.variable == name)
            .unwrap()
            .content
            .kind
    };
    assert_eq!(warning("MISSING"), SemanticWarnKind::PossiblyUnset);
    assert_eq!(warning("U"), SemanticWarnKind::Unset);
    assert_eq!(warning("E"), SemanticWarnKind::Empty);
    assert_eq!(warning("INPUT"), SemanticWarnKind::PossiblyUnset);
    assert_eq!(warning("Z"), SemanticWarnKind::Unset);
    assert!(
        !buffer
            .warnings
            .iter()
            .any(|warning| matches!(warning.content.variable.as_str(), "D" | "V"))
    );
    assert!(
        buffer
            .variables
            .iter()
            .find(|variable| variable.name == "U")
            .unwrap()
            .references
            .iter()
            .all(|reference| reference.kind != ReferenceKind::Definition)
    );
}
