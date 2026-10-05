mod display;

use cmake_analyzer::parser::{
    lexer::model::{Lexer, RawBuffer},
    nparser::model::NParser,
    semanticer::{
        SemanticBuffer, Semanticer,
        model::{BlockKind, DiagnosticKind, Namespace, ScopeKind, Severity, SymbolKind},
    },
};

fn references<'a>(
    buffer: &'a SemanticBuffer,
    name: &str,
    namespace: Namespace,
) -> Vec<&'a cmake_analyzer::parser::semanticer::model::Reference> {
    buffer
        .references
        .iter()
        .filter(|reference| {
            reference.namespace == namespace && buffer.name(&reference.name) == Some(name)
        })
        .collect()
}

#[test]
fn builds_nested_blocks_branches_and_body_scopes() {
    let source = "function(F p)\nif(p)\nforeach(item IN ITEMS a)\nmessage(\"${p}\")\nendforeach()\nelseif(ON)\nelse()\nendif()\nendfunction(f)";
    let buffer = Semanticer::analyze(source);
    assert!(!buffer.has_errors());
    assert_eq!(buffer.blocks.len(), 3);
    assert_eq!(buffer.blocks[0].kind, BlockKind::Function);
    assert_eq!(buffer.blocks[0].closing, Some(8));
    assert_eq!(buffer.blocks[1].parent, Some(0));
    assert_eq!(buffer.blocks[1].branches, [5, 6]);
    assert_eq!(buffer.blocks[2].parent, Some(1));
    assert_eq!(buffer.command_blocks[3], Some(2));
    assert_eq!(buffer.command_scopes[0], 0);
    assert_eq!(buffer.command_scopes[3], 1);
    assert_eq!(buffer.scopes[1].kind, ScopeKind::Function);
    for reference in references(&buffer, "p", Namespace::Variable) {
        assert_eq!(
            buffer.symbols[reference.resolved[0]].kind,
            SymbolKind::Parameter
        );
    }
}

#[test]
fn recovers_mismatched_ends_and_diagnoses_branch_order() {
    let buffer = Semanticer::analyze("if(A)\nforeach(i RANGE 1)\nendif()\nendwhile()");
    assert!(buffer.has_errors());
    assert_eq!(buffer.blocks[0].closing, Some(2));
    assert!(buffer.blocks[1].closing.is_none());
    assert!(buffer.diagnostics.iter().any(|diag| matches!(
        diag.content.kind,
        DiagnosticKind::UnclosedBlock(BlockKind::Foreach)
    )));
    assert!(buffer.diagnostics.iter().any(|diag| matches!(
        diag.content.kind,
        DiagnosticKind::UnmatchedBlockEnd(BlockKind::While)
    )));
    let buffer = Semanticer::analyze("else()\nif(A)\nelse()\nelse()\nelseif(B)");
    assert!(
        buffer
            .diagnostics
            .iter()
            .any(|diag| matches!(diag.content.kind, DiagnosticKind::UnexpectedBranch))
    );
    assert!(
        buffer
            .diagnostics
            .iter()
            .any(|diag| matches!(diag.content.kind, DiagnosticKind::DuplicateElse))
    );
    assert!(
        buffer
            .diagnostics
            .iter()
            .any(|diag| matches!(diag.content.kind, DiagnosticKind::ElseIfAfterElse))
    );
    assert!(buffer.diagnostics.iter().any(|diag| matches!(
        diag.content.kind,
        DiagnosticKind::UnclosedBlock(BlockKind::If)
    )));
}

#[test]
fn blocks_only_create_variable_scopes_when_requested() {
    let buffer = Semanticer::analyze(
        "block(SCOPE_FOR POLICIES)\nset(x a)\nendblock()\nblock(SCOPE_FOR VARIABLES)\nset(y b)\nendblock()",
    );
    assert!(!buffer.has_errors());
    assert_eq!(buffer.scopes.len(), 2);
    assert_eq!(buffer.blocks[0].scope, 0);
    assert_eq!(buffer.blocks[1].scope, 1);
    assert_eq!(buffer.scopes[1].kind, ScopeKind::Variables);
    let buffer = Semanticer::analyze("block()\nendblock()");
    assert_eq!(buffer.scopes.len(), 2);
}

#[test]
fn variables_bind_prior_definitions_and_unset_exposes_cache() {
    let buffer = Semanticer::analyze(
        "set(X cached CACHE STRING docs)\nset(X normal)\nunset(X)\nset(ENV{HOME} local)\nmessage(\"${X}\" \"$CACHE{X}\" \"$ENV{HOME}\" \"${x}\")",
    );
    assert!(!buffer.has_errors());
    let normal = references(&buffer, "X", Namespace::Variable)[0];
    assert_eq!(buffer.symbols[normal.resolved[0]].kind, SymbolKind::Cache);
    let cache = references(&buffer, "X", Namespace::Cache)[0];
    assert_eq!(cache.resolved, normal.resolved);
    let env = references(&buffer, "HOME", Namespace::Environment)[0];
    assert_eq!(
        buffer.symbols[env.resolved[0]].kind,
        SymbolKind::Environment
    );
    assert!(
        references(&buffer, "x", Namespace::Variable)[0]
            .resolved
            .is_empty()
    );
    let buffer = Semanticer::analyze("message(\"${v}\")\nset(v \"${v}\")\nmessage(\"${v}\")");
    let reads = references(&buffer, "v", Namespace::Variable);
    assert!(reads[0].resolved.is_empty());
    assert!(reads[1].resolved.is_empty());
    assert_eq!(reads[2].resolved.len(), 1);
}

#[test]
fn callable_bodies_do_not_invent_executed_writes_in_the_directory() {
    let buffer = Semanticer::analyze(
        "set(x root)\nfunction(F)\nset(x local)\nset(x new PARENT_SCOPE)\nunset(x PARENT_SCOPE)\nmessage(\"${x}\")\nset(D cache CACHE STRING docs)\nset(ENV{D} value)\nadd_library(deferred STATIC a.cpp)\nendfunction()\nmessage(\"$CACHE{D}\" \"$ENV{D}\" \"${x}\")\ntarget_link_libraries(deferred PRIVATE missing)",
    );
    assert!(!buffer.has_errors());
    let reads = references(&buffer, "x", Namespace::Variable);
    assert_eq!(buffer.symbols[reads[0].resolved[0]].scope, 1);
    assert_eq!(buffer.symbols[reads[1].resolved[0]].scope, 0);
    assert!(
        references(&buffer, "D", Namespace::Cache)[0]
            .resolved
            .is_empty()
    );
    assert!(
        references(&buffer, "D", Namespace::Environment)[0]
            .resolved
            .is_empty()
    );
    assert!(
        references(&buffer, "deferred", Namespace::Target)[0]
            .resolved
            .is_empty()
    );
    assert_eq!(
        buffer
            .symbols
            .iter()
            .filter(|symbol| symbol.kind == SymbolKind::ParentWrite)
            .count(),
        2
    );
}

#[test]
fn function_macro_names_are_case_insensitive_and_parameters_are_local() {
    let buffer = Semanticer::analyze(
        "function(F p)\nmessage(\"${p}\")\nendfunction(F)\nmacro(M q)\nmessage(\"${q}\")\nendmacro(m)\nf(a)\nM(b)\nmessage(\"${p}\")",
    );
    assert!(!buffer.has_errors());
    assert_eq!(buffer.scopes[2].kind, ScopeKind::Macro);
    assert_eq!(
        references(&buffer, "f", Namespace::Command)[0]
            .resolved
            .len(),
        1
    );
    assert_eq!(
        references(&buffer, "M", Namespace::Command)[0]
            .resolved
            .len(),
        1
    );
    assert!(
        references(&buffer, "p", Namespace::Variable)
            .last()
            .unwrap()
            .resolved
            .is_empty()
    );
    let buffer =
        Semanticer::analyze("function(F a b)\nendfunction(wrong)\nF(a)\nF(${args})\nF(a b c)");
    assert!(
        buffer
            .diagnostics
            .iter()
            .any(|diag| matches!(diag.content.kind, DiagnosticKind::EndNameMismatch))
    );
    assert_eq!(
        buffer
            .diagnostics
            .iter()
            .filter(|diag| matches!(diag.content.kind, DiagnosticKind::InvalidArity { .. }))
            .count(),
        1
    );
    let dynamic = Semanticer::analyze("function(F ${parameters})\nendfunction()\nF()");
    assert!(!dynamic.has_errors());
    let buffer = Semanticer::analyze("function(message x)\nendfunction()\nmessage(hi)");
    assert_eq!(
        references(&buffer, "message", Namespace::Command)[0]
            .resolved
            .len(),
        1
    );
}

#[test]
fn target_aliases_forward_references_and_duplicates_are_candidates() {
    let buffer = Semanticer::analyze(
        "add_library(alias ALIAS later)\ntarget_link_libraries(app PRIVATE alias systemlib)\nadd_executable(app app.cpp)\nadd_library(later STATIC later.cpp)\nif(ON)\nadd_library(same)\nelse()\nadd_library(same)\nendif()\nadd_dependencies(app same)",
    );
    assert!(!buffer.has_errors());
    assert_eq!(
        references(&buffer, "later", Namespace::Target)[0]
            .resolved
            .len(),
        1
    );
    assert_eq!(
        references(&buffer, "app", Namespace::Target)[0]
            .resolved
            .len(),
        1
    );
    assert_eq!(
        buffer.symbols[references(&buffer, "alias", Namespace::Target)[0].resolved[0]].kind,
        SymbolKind::Alias
    );
    assert!(
        references(&buffer, "systemlib", Namespace::Target)[0]
            .resolved
            .is_empty()
    );
    assert_eq!(
        references(&buffer, "same", Namespace::Target)[0]
            .resolved
            .len(),
        2
    );
    let warning = buffer
        .diagnostics
        .iter()
        .find(|diag| matches!(diag.content.kind, DiagnosticKind::DuplicateTarget))
        .unwrap();
    assert_eq!(warning.content.severity, Severity::Warning);
}

#[test]
fn expansion_spans_track_unicode_newlines_and_namespaces() {
    let source = "set(变量 value)\nmessage(\"前\n${变量} $ENV{HOME} $CACHE{MODE}\")";
    let buffer = Semanticer::analyze(source);
    assert!(!buffer.has_errors());
    let reference = references(&buffer, "变量", Namespace::Variable)[0];
    assert_eq!(
        source.get(reference.span.start_byte..reference.span.end_byte),
        Some("变量")
    );
    assert_eq!((reference.span.row, reference.span.column), (2, 2));
    assert_eq!(reference.resolved.len(), 1);
    for reference in buffer
        .references
        .iter()
        .filter(|reference| reference.name.argument.is_some())
    {
        assert_eq!(
            source.get(reference.span.start_byte..reference.span.end_byte),
            buffer.name(&reference.name)
        );
    }
}

#[test]
fn bracket_arguments_do_not_expand_and_escaped_dollars_are_skipped() {
    let buffer = Semanticer::analyze(
        r#"set(x value) message("\${x} \\${x}" [[${x}]]) set([[${literal}]] value)"#,
    );
    assert!(!buffer.has_errors());
    assert_eq!(references(&buffer, "x", Namespace::Variable).len(), 1);
    assert!(
        buffer
            .symbols
            .iter()
            .any(|symbol| buffer.name(&symbol.name) == Some("${literal}"))
    );
}

#[test]
fn implicit_condition_loop_list_and_property_facts_are_indexed() {
    let buffer = Semanticer::analyze(
        "set(xs a b)\nlist(APPEND xs c)\nlist(LENGTH xs count)\nstring(CONCAT joined a b)\nforeach(item IN LISTS xs)\nif(DEFINED count AND TARGET app)\nmessage(\"${item}\")\nendif()\nendforeach()\nadd_executable(app a.cpp)\nget_target_property(output app SOURCES)",
    );
    assert!(!buffer.has_errors());
    assert_eq!(references(&buffer, "xs", Namespace::Variable).len(), 3);
    assert!(
        references(&buffer, "count", Namespace::Variable)[0]
            .resolved
            .len()
            == 1
    );
    assert!(
        buffer
            .symbols
            .iter()
            .any(|symbol| buffer.name(&symbol.name) == Some("joined"))
    );
    assert!(
        buffer
            .symbols
            .iter()
            .any(|symbol| buffer.name(&symbol.name) == Some("output"))
    );
    assert!(
        references(&buffer, "app", Namespace::Target)
            .iter()
            .all(|reference| reference.resolved.len() == 1)
    );
    let buffer = Semanticer::analyze(
        "option(X PARENT_SCOPE ON)\nset(Y CACHE STRING PARENT_SCOPE)\nset(Z PARENT_SCOPE value)",
    );
    assert_eq!(
        buffer
            .symbols
            .iter()
            .filter(|symbol| symbol.kind == SymbolKind::ParentWrite)
            .count(),
        0
    );
}

#[test]
fn selective_arity_checks_skip_dynamic_unquoted_expansion() {
    let buffer = Semanticer::analyze(
        "set()\noption(X)\nget_target_property(out app)\nget_target_property(${args})",
    );
    assert_eq!(
        buffer
            .diagnostics
            .iter()
            .filter(|diag| matches!(diag.content.kind, DiagnosticKind::InvalidArity { .. }))
            .count(),
        3
    );
}

#[test]
fn parsed_storage_is_moved_and_prior_errors_are_preserved() {
    let parsed =
        NParser::new(Lexer::new(RawBuffer::new("set(x value)\n[=[unfinished")).parse()).parse();
    let pointer = parsed.commands[0].name.content.as_ptr();
    let mut analyzer = Semanticer::new(parsed);
    let buffer = analyzer.parse();
    assert_eq!(buffer.parsed.commands[0].name.content.as_ptr(), pointer);
    assert!(buffer.has_errors());
    assert_eq!(buffer.symbols.len(), 1);
    assert!(analyzer.buffer.commands.is_empty());
    let empty = analyzer.parse();
    assert!(!empty.has_errors());
    assert_eq!(empty.scopes.len(), 1);
}

#[test]
fn deep_blocks_and_nested_references_use_flat_iterative_storage() {
    let depth = 2048;
    let source = format!("{}{}", "if(ON)\n".repeat(depth), "endif()\n".repeat(depth));
    let buffer = Semanticer::analyze(&source);
    assert!(!buffer.has_errors());
    assert_eq!(buffer.blocks.len(), depth);
    let source = format!(
        "set(x value)\nmessage(\"{}x{}\")",
        "${".repeat(depth),
        "}".repeat(depth)
    );
    let buffer = Semanticer::analyze(&source);
    let references: Vec<_> = buffer
        .references
        .iter()
        .filter(|reference| reference.namespace == Namespace::Variable)
        .collect();
    assert_eq!(references.len(), depth);
    assert_eq!(
        references
            .iter()
            .filter(|reference| reference.dynamic)
            .count(),
        depth - 1
    );
    assert_eq!(references[0].resolved.len(), 1);
}
