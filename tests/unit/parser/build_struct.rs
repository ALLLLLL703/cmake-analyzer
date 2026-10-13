// AI-generated tests.
use super::semanticer::analyze;
use cmake_analyzer::parser::build_struct::model::{
    AnalysisKey, BuildEffect, BuildStructAnalyzer, TargetKind, Visibility,
};

#[tokio::test]
async fn preserves_source_order_and_snapshot_identity() {
    let semantic=analyze("cmake_minimum_required(VERSION 3.31)\nproject(Demo VERSION 1.2 LANGUAGES CXX)\nadd_library(core STATIC a.cpp)\nadd_library(Demo::core ALIAS core)\n").await;
    let origin = AnalysisKey {
        file: 7,
        revision: 42,
    };
    let build = BuildStructAnalyzer::new(&semantic, origin).analyze();
    assert_eq!(build.analysis, origin);
    assert!(build.facts.iter().all(|f| f.source.analysis == origin));
    assert!(
        build
            .facts
            .windows(2)
            .all(|w| w[0].source.command.id <= w[1].source.command.id)
    );
    let targets: Vec<_> = build
        .facts
        .iter()
        .filter_map(|f| match &f.effect {
            BuildEffect::Target(t) => Some(t),
            _ => None,
        })
        .collect();
    assert_eq!(targets.len(), 2);
    assert_eq!(targets[0].kind, TargetKind::StaticLibrary);
    assert_eq!(targets[0].sources[0].text, "a.cpp");
    assert_eq!(targets[1].kind, TargetKind::Alias);
    assert_eq!(targets[1].alias.as_ref().unwrap().text, "core");
}

#[tokio::test]
async fn conditional_and_deferred_facts_do_not_claim_configured_targets() {
    let semantic=analyze("cmake_minimum_required(VERSION 3.31)\nif(OPTION)\nadd_executable(maybe ${SOURCES})\nendif()\nfunction(make)\nadd_library(later INTERFACE)\nendfunction()\n").await;
    let build = BuildStructAnalyzer::new(
        &semantic,
        AnalysisKey {
            file: 7,
            revision: 1,
        },
    )
    .analyze();
    let targets: Vec<_> = build
        .facts
        .iter()
        .filter(|f| matches!(f.effect, BuildEffect::Target(_)))
        .collect();
    assert_eq!(targets.len(), 2);
    assert!(targets[0].source.conditional);
    assert!(targets[1].source.deferred);
    if let BuildEffect::Target(target) = &targets[0].effect {
        assert!(!target.sources[0].literal);
    }
}

#[tokio::test]
async fn sources_keep_visibility_and_file_sets() {
    let semantic=analyze("cmake_minimum_required(VERSION 3.31)\nadd_library(core STATIC)\ntarget_sources(core PRIVATE a.cpp PUBLIC FILE_SET api TYPE HEADERS BASE_DIRS include FILES include/a.h)\n").await;
    let build = BuildStructAnalyzer::new(
        &semantic,
        AnalysisKey {
            file: 7,
            revision: 1,
        },
    )
    .analyze();
    let source = build
        .facts
        .iter()
        .find_map(|f| match &f.effect {
            BuildEffect::Sources(s) => Some(s),
            _ => None,
        })
        .unwrap();
    assert_eq!(source.sections[0].visibility, Some(Visibility::Private));
    assert_eq!(source.sections[0].files[0].text, "a.cpp");
    let public = source
        .sections
        .iter()
        .find(|s| s.visibility == Some(Visibility::Public))
        .unwrap();
    assert_eq!(public.file_sets[0].name.text, "api");
    assert_eq!(public.file_sets[0].files[0].text, "include/a.h");
}
