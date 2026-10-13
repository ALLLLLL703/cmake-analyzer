// AI-generated tests.
use cmake_analyzer::parser::{
    lexer::model::{Lexer, RawBuffer},
    nparser::model::NParser,
    semanticer::model::{BlockKind, FileId, SemanticError, SemanticedBuffer, Semanticer},
};

pub(super) async fn analyze(source: &str) -> SemanticedBuffer {
    let lexed = Lexer::new(RawBuffer::new(source)).parse();
    let parsed = NParser::new(lexed).parse(source);
    Semanticer::new(parsed, Default::default())
        .parse(source, FileId { id: 7 })
        .await
}

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
