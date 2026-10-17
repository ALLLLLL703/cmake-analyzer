//! Shared in-memory semantic-analysis fixtures and regressions for scopes, template specialization,
//! diagnostics and analyzer reuse. Helpers are also used by build-fact and incremental tests.

// Contains AI-generated documentation edits.
// AI-generated tests.
mod builtin_functions;
mod documentation;
mod locations;
mod references;
mod values;
use cmake_analyzer::parser::{
    lexer::model::{Lexer, RawBuffer},
    nparser::model::NParser,
    semanticer::model::{BlockKind, FileId, SemanticError, SemanticedBuffer, Semanticer},
};

/// Analyze a CMake source fixture with fixed file ID 7 through [`analyze_file`]. Allocates fresh
/// lexical, parsed and semantic state; performs no file/process I/O.
///
/// # Arguments
/// - `source` `[in]`: Complete source text; diagnostics are retained rather than unwrapped as
///   failures.
///
/// # Returns
/// The semantic buffer for file 7, including any syntax/version/semantic diagnostics.
///
/// # Examples
/// Local test-module context (await inside an async test):
/// ```text
/// let buffer = analyze("cmake_minimum_required(VERSION 3.31)\nset(A x)\n").await;
/// assert_eq!(buffer.id.id, 7);
/// ```
pub(super) async fn analyze(source: &str) -> SemanticedBuffer {
    analyze_file(source, FileId { id: 7 }).await
}
/// Run a fresh lexer, command parser and semantic analyzer under a caller-supplied identity. Used
/// for full-analysis comparisons when workspace file IDs differ from 7; no file/process I/O.
///
/// # Arguments
/// - `source` `[in]`: Source scanned by all three stages; not modified.
/// - `file` `[in]`: File identity assigned to semantic commands and references.
///
/// # Returns
/// A newly owned semantic buffer with the supplied file identity. Invalid input is represented by
/// diagnostics, not a Result.
///
/// # Algorithm
/// Lex the borrowed source, parse commands using the same text, then parse semantics with no
/// injected schemas and the supplied file identity.
///
/// # Flow
/// ```text
/// [source]
///     |
/// [lexer] ------------> [command parser]
///                            |
/// [file identity] ----> [semantic analyzer]
///                            |
///                    [owned semantic buffer]
/// ```
///
/// # Examples
/// Local test-module context (await inside an async test):
/// ```text
/// let buffer = analyze_file("cmake_minimum_required(VERSION 3.31)\n", FileId { id: 2 }).await;
/// assert_eq!(buffer.id.id, 2);
/// ```
pub(super) async fn analyze_file(source: &str, file: FileId) -> SemanticedBuffer {
    let lexed = Lexer::new(RawBuffer::new(source)).parse();
    let parsed = NParser::new(lexed).parse(source);
    Semanticer::new(parsed, Default::default())
        .parse(source, file)
        .await
}

/// Analyze a function with nested condition, a macro and a variable-scope block.
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
/// Require four blocks but two scopes, correct condition parentage and distinct local/global
/// writes; macro writes remain global.
///
/// # Flow
/// ```text
///   [function/if/macro/block]
///    |
///   [analyze blocks + scopes]----------> [local/global variable records]
///                                       |
///                                      [compare parents + memberships]
///                                       |
///                                      < checks hold? >
///                                       /         \
///                                     [pass]     [panic]
/// ```
///
/// # Examples
/// Direct local call outside any Tokio runtime (the test attribute creates its runtime):
/// ```text
/// blocks_and_scopes_are_distinct();
/// ```
#[tokio::test]
async fn blocks_and_scopes_are_distinct() {
    let result=analyze("cmake_minimum_required(VERSION 3.31)\nset(V root)\nfunction(f)\nset(V local)\nif(YES)\nset(I local)\nendif()\nendfunction()\nmacro(m)\nset(M root)\nendmacro()\nblock(SCOPE_FOR VARIABLES)\nset(B local)\nendblock()\n").await;
    assert!(result.diagnostics.is_empty(), "{:?}", result.diagnostics);
    assert_eq!(result.blocks.len(), 4);
    assert_eq!(result.scopes.len(), 2);
    let function = result
        .blocks
        .iter()
        .find(|b| b.kind == BlockKind::Function)
        .unwrap();
    let condition = result
        .blocks
        .iter()
        .find(|b| b.kind == BlockKind::If)
        .unwrap();
    assert_eq!(condition.parent, Some(function.id));
    assert!(!function.contains.contains(&condition.closing.unwrap()));
    let i = result.variables.iter().find(|v| v.name == "I").unwrap();
    let local = result
        .variables
        .iter()
        .find(|v| v.name == "V" && v.scope.is_some())
        .unwrap();
    assert_eq!(i.scope, local.scope);
    assert!(
        result
            .variables
            .iter()
            .any(|v| v.name == "V" && v.scope.is_none())
    );
    assert!(
        result
            .variables
            .iter()
            .any(|v| v.name == "M" && v.scope.is_none())
    );
    assert!(
        result
            .variables
            .iter()
            .any(|v| v.name == "B" && v.scope.is_some())
    );
}

/// Analyze project `Demo` with CXX and an explicit CXX flag write. Require
/// project/language-specific builtin names, no angle-bracket placeholders and no duplicate
/// name/scope pairs.
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
/// Direct local call outside any Tokio runtime (the test attribute creates its runtime):
/// ```text
/// literal_triggers_specialize_without_leaking_placeholders();
/// ```
#[tokio::test]
async fn literal_triggers_specialize_without_leaking_placeholders() {
    let result=analyze("cmake_minimum_required(VERSION 3.31)\nproject(Demo LANGUAGES CXX)\nset(CMAKE_CXX_FLAGS flags)\n").await;
    assert!(result.variables.iter().any(|v| v.name == "Demo_SOURCE_DIR"));
    assert!(result.variables.iter().any(|v| v.name == "CMAKE_CXX_FLAGS"));
    assert!(result.variables.iter().all(|v| !v.name.contains('<')));
    let names: std::collections::HashSet<_> = result
        .variables
        .iter()
        .map(|v| (&v.name, v.scope))
        .collect();
    assert_eq!(names.len(), result.variables.len());
}

/// Analyze an unclosed if containing an unknown command. Require both the unclosed-block and
/// command-not-found diagnostics.
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
/// Direct local call outside any Tokio runtime (the test attribute creates its runtime):
/// ```text
/// unmatched_blocks_and_unknown_commands_are_diagnostics();
/// ```
#[tokio::test]
async fn unmatched_blocks_and_unknown_commands_are_diagnostics() {
    let result =
        analyze("cmake_minimum_required(VERSION 3.31)\nif(YES)\nnot_a_cmake_command()\n").await;
    assert!(
        result
            .diagnostics
            .iter()
            .any(|d| matches!(d.content, SemanticError::UnclosedBlock(BlockKind::If)))
    );
    assert!(
        result
            .diagnostics
            .iter()
            .any(|d| matches!(d.content, SemanticError::CommandNotFound(_)))
    );
}

/// Parse file 1 with CMake 3.31 and OLD, replace the analyzer's parsed buffer, then parse file 2
/// with CMake 4.4 and NEW. Require new identity/version/names with no retained OLD variable.
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
/// # Examples
/// Direct local call outside any Tokio runtime (the test attribute creates its runtime):
/// ```text
/// reusing_analyzer_does_not_retain_previous_file_state();
/// ```
#[tokio::test]
async fn reusing_analyzer_does_not_retain_previous_file_state() {
    let source = "cmake_minimum_required(VERSION 3.31)\nset(OLD value)\n";
    let parsed = NParser::new(Lexer::new(RawBuffer::new(source)).parse()).parse(source);
    let mut analyzer = Semanticer::new(parsed, Default::default());
    analyzer.parse(source, FileId { id: 1 }).await;
    let source = "cmake_minimum_required(VERSION 4.4)\nset(NEW value)\n";
    analyzer.buffer = NParser::new(Lexer::new(RawBuffer::new(source)).parse()).parse(source);
    let next = analyzer.parse(source, FileId { id: 2 }).await;
    assert_eq!(next.id.id, 2);
    assert_eq!(next.version.unwrap().major, 4);
    assert!(next.variables.iter().any(|v| v.name == "NEW"));
    assert!(!next.variables.iter().any(|v| v.name == "OLD"));
}
