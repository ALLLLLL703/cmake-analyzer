// AI-generated incremental equivalence and reuse tests.
use super::semanticer::analyze;
use cmake_analyzer::parser::{
    build_struct::model::{AnalysisKey, BuildStructAnalyzer, IncrementalBuildStruct},
    lexer::model::{IncrementalLexer, Lexer, RawBuffer},
    nparser::model::{IncrementalNParser, NParser},
    semanticer::model::{FileId, IncrementalSemanticer},
};
use std::sync::Arc;

async fn compare(
    lexer: &IncrementalLexer,
    parser: &IncrementalNParser,
    semantic: &IncrementalSemanticer,
    build: &IncrementalBuildStruct,
) {
    let full = Lexer::new(RawBuffer::new(&lexer.source)).parse();
    assert_eq!(
        format!("{:?}", lexer.buffer.lex),
        format!("{:?}", full.lex),
        "tokens for {}",
        lexer.source
    );
    assert_eq!(
        format!("{:?}", lexer.buffer.errors),
        format!("{:?}", full.errors),
        "lexer errors for {}",
        lexer.source
    );
    let parsed = NParser::new(full).parse(&lexer.source);
    assert_eq!(
        format!("{:?}", parser.buffer),
        format!("{:?}", parsed),
        "parser for {}",
        lexer.source
    );
    let semantic_full = analyze(&lexer.source).await;
    assert_eq!(
        format!("{:?}", semantic.buffer),
        format!("{:?}", semantic_full),
        "semantics for {}",
        lexer.source
    );
    let build_full = BuildStructAnalyzer::new(&semantic_full, build.buffer.analysis).analyze();
    assert_eq!(
        format!("{:?}", build.buffer),
        format!("{:?}", build_full),
        "facts for {}",
        lexer.source
    );
}

#[tokio::test]
async fn local_argument_edit_reuses_every_layer() {
    let original:Arc<str>="cmake_minimum_required(VERSION 3.31)\nset(A one)\nadd_library(core STATIC a.cpp)\nmessage(你好)\n".into();
    let mut lexer = IncrementalLexer::new(original.clone());
    let mut parser = IncrementalNParser::new(&original, &lexer.buffer);
    let mut semantic =
        IncrementalSemanticer::new(original, parser.buffer.clone(), FileId { id: 7 }).await;
    let mut build = IncrementalBuildStruct::new(
        &semantic.buffer,
        AnalysisKey {
            file: 7,
            revision: 1,
        },
    );
    let edited:Arc<str>="cmake_minimum_required(VERSION 3.31)\nset(A two)\nadd_library(core STATIC a.cpp)\nmessage(你好)\n".into();
    let change = lexer.update(edited.clone());
    parser.update(&edited, &lexer.buffer, &change);
    semantic
        .update(edited, parser.buffer.clone(), &change)
        .await;
    build.update(
        &semantic.buffer,
        AnalysisKey {
            file: 7,
            revision: 2,
        },
        &change,
    );
    assert!(lexer.work.reused > 0);
    assert_eq!(parser.work.recomputed, 1);
    assert_eq!(semantic.work.recomputed, 1);
    assert_eq!(build.work.recomputed, 1);
    assert!(!semantic.structure_rebuilt);
    assert!(!semantic.templates_rebuilt);
    compare(&lexer, &parser, &semantic, &build).await;
}

#[tokio::test]
async fn syntax_errors_unicode_and_global_triggers_match_full_analysis() {
    let prefix = "cmake_minimum_required(VERSION 3.31)\n";
    let mut lexer = IncrementalLexer::new(prefix);
    let mut parser = IncrementalNParser::new(prefix, &lexer.buffer);
    let mut semantic =
        IncrementalSemanticer::new(prefix, parser.buffer.clone(), FileId { id: 7 }).await;
    let mut build = IncrementalBuildStruct::new(
        &semantic.buffer,
        AnalysisKey {
            file: 7,
            revision: 0,
        },
    );
    let bodies = [
        "set(A one)\nmessage(你好)",
        "set(A two)\nmessage(你好)",
        "set(A two)\nmessage(\"你好)",
        "set(A two)\nmessage(\"你好\")",
        "#[=[ comment\n]=]\nset(A one)",
        "#[=[ comment\n]]\nset(A one)",
        "message([==[\n🌟\n]==])\nset(A one)",
        "message([==[\n🌟\n]=])\nset(A one)",
        "function(f)\nset(A one)\nendfunction()\nset(B two)",
        "function(f)\nset(A three)\nendfunction()\nset(B two)",
        "function(f P)\nmessage(\"${P} ${A}\")\nendfunction()\nf(x)\nset(A \"\")\nmessage(\"${A}\")",
        "# 🌟\nfunction(f P)\nmessage(\"${P} ${A}\")\nendfunction()\nf(x)\nset(A \"\")\nmessage(\"${A}\")",
        "# 🌟\nfunction(g P)\nmessage(\"${P} ${A}\")\nendfunction()\nf(x)\nset(A value)\nmessage(\"${A}\")",
        "project(Demo LANGUAGES CXX)\nset(CMAKE_BUILD_TYPE Debug)",
        "project(Other LANGUAGES C)\nset(CMAKE_BUILD_TYPE Release)",
        "set(A one)\nmessage(x)",
        "set(A one message(x)",
        "set(A one)\nmessage(x)",
        "set(A one)\nmessage(x)\n",
        "set(A one)\nmessage(\"🌟\")\n",
    ];
    for (revision, body) in bodies.iter().enumerate() {
        let text: Arc<str> = format!("{prefix}{body}").into();
        let change = lexer.update(text.clone());
        parser.update(&text, &lexer.buffer, &change);
        semantic.update(text, parser.buffer.clone(), &change).await;
        build.update(
            &semantic.buffer,
            AnalysisKey {
                file: 7,
                revision: revision as u64 + 1,
            },
            &change,
        );
        compare(&lexer, &parser, &semantic, &build).await;
    }
    let text: Arc<str> = "cmake_minimum_required(VERSION 4.4)\nset(A one)\n".into();
    let change = lexer.update(text.clone());
    parser.update(&text, &lexer.buffer, &change);
    semantic.update(text, parser.buffer.clone(), &change).await;
    build.update(
        &semantic.buffer,
        AnalysisKey {
            file: 7,
            revision: 100,
        },
        &change,
    );
    assert_eq!(semantic.work.reused, 0);
    compare(&lexer, &parser, &semantic, &build).await;
}

#[tokio::test]
async fn character_edits_converge_even_inside_comments_and_delimiters() {
    let original =
        "cmake_minimum_required(VERSION 3.31)\nset(A [=[hi🌟]=])\n# comment\nmessage(\"x\\\"y\")\n";
    let mut lexer = IncrementalLexer::new(original);
    let mut parser = IncrementalNParser::new(original, &lexer.buffer);
    let mut semantic =
        IncrementalSemanticer::new(original, parser.buffer.clone(), FileId { id: 7 }).await;
    let mut build = IncrementalBuildStruct::new(
        &semantic.buffer,
        AnalysisKey {
            file: 7,
            revision: 0,
        },
    );
    for (revision, (byte, ch)) in original.char_indices().filter(|(i, _)| *i > 34).enumerate() {
        for replacement in ["", "x", "\n", "\""] {
            let edited: Arc<str> = format!(
                "{}{replacement}{}",
                &original[..byte],
                &original[byte + ch.len_utf8()..]
            )
            .into();
            let change = lexer.update(edited.clone());
            parser.update(&edited, &lexer.buffer, &change);
            semantic
                .update(edited, parser.buffer.clone(), &change)
                .await;
            build.update(
                &semantic.buffer,
                AnalysisKey {
                    file: 7,
                    revision: revision as u64,
                },
                &change,
            );
            compare(&lexer, &parser, &semantic, &build).await;
        }
    }
}
