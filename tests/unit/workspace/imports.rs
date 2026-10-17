// AI-generated tests.

//! Check static include/package lookup and independent external indexing policies.
//! Uses temporary version-pinned source files through [`super::Project`]; never runs project commands.

use super::Project;
use cmake_analyzer::{
    parser::build_struct::model::DependencyKind,
    workspace::{Workspace, model::ExternalIndex},
};
/// Import External and its nested module from an app under Full, Functions, and None policies.
/// Writes/reads/removes temporary app/module files; no project execution.
///
/// # Arguments
/// None; the test constructs its own fixtures.
///
/// # Returns
/// `()` after all checks pass.
///
/// # Panics
/// If fixture setup or any documented assertion fails.
///
/// # Algorithm
/// Require resolved include edges without diagnostics; functions survive Full/Functions, while variables, writes, and targets survive Full only.
///
/// # Flow
/// ```text
/// [app] --> [External module] --> [Nested module]
///    |
///    v
/// < indexing mode >
///    |         |          |
///  Full     Functions    None
///    |         |          |
///    v         v          v
/// [all facts] [functions] [edge only]
///    |         |          |
///    +---------+----------+
///              v
///  [require expected visible facts]
/// ```
///
/// # Examples
/// Local test-module context, outside any running Tokio runtime:
/// the macro-generated test wrapper runs its own runtime and returns `()`.
/// Prefer the test harness; ignored measurements require explicit selection.
/// ```text
/// external_modes_control_functions_variables_targets_and_nested_imports();
/// ```
#[tokio::test]
async fn external_modes_control_functions_variables_targets_and_nested_imports() {
    let p = Project::new().await;
    p.write(
        "app/CMakeLists.txt",
        "include(External)\next_call()\nnested_call()\nmessage(\"${EXTERNAL_VALUE}\")\n",
    )
    .await;
    p.write("modules/External.cmake","set(EXTERNAL_VALUE yes)\nadd_library(ExternalTarget INTERFACE)\ninclude(${CMAKE_CURRENT_LIST_DIR}/Nested.cmake)\nfunction(ext_call)\nendfunction()\n").await;
    p.write(
        "modules/Nested.cmake",
        "set(NESTED_VALUE yes)\nfunction(nested_call)\nendfunction()\n",
    )
    .await;
    for mode in [
        ExternalIndex::Full,
        ExternalIndex::Functions,
        ExternalIndex::None,
    ] {
        let mut w = Workspace::new(p.0.join("app/CMakeLists.txt"), None).unwrap();
        w.set_module_paths(vec![p.0.join("modules")]);
        w.set_external_indexing(mode, ExternalIndex::Full, vec![]);
        w.flush().await.unwrap();
        let g = w.root_graph().unwrap();
        assert!(g.diagnostics.is_empty(), "{:?}", g.diagnostics);
        assert_eq!(g.edges[0].kind, DependencyKind::Include);
        assert!(g.edges[0].resolved.is_some());
        for name in ["ext_call", "nested_call"] {
            assert_eq!(
                g.references
                    .functions
                    .iter()
                    .any(|function| function.name == name),
                mode != ExternalIndex::None,
                "{mode:?}: {name}"
            );
        }
        assert_eq!(
            g.targets
                .iter()
                .any(|target| target.name.text == "ExternalTarget"),
            mode == ExternalIndex::Full
        );
        assert_eq!(
            g.references
                .variables
                .iter()
                .any(|variable| variable.name == "EXTERNAL_VALUE" && variable.definition.is_some()),
            mode == ExternalIndex::Full
        );
        assert_eq!(
            g.scopes[0].possible_writes.contains("EXTERNAL_VALUE"),
            mode == ExternalIndex::Full
        );
    }
}
/// Create FindDemo, DemoConfig, Special, and RealConfig fixtures and exercise nine package lookup forms.
/// Writes/reads/removes temporary package layouts and performs static path lookup; no project execution.
///
/// # Arguments
/// None; the test constructs its own fixtures.
///
/// # Returns
/// `()` after all checks pass.
///
/// # Panics
/// If fixture setup or any documented assertion fails.
///
/// # Algorithm
/// Require expected package filenames and visits without diagnostics; separately require functions-only versus disabled package indexing to omit targets and control function visibility.
///
/// # Flow
/// ```text
/// [FindDemo / DemoConfig / custom configs]
///                     |
///                     v
///           [nine static lookup forms] <--+
///                     |                  | next form
///                     v                  |
///           [expected file / visit] -----+
///                     |
///                     v
///          < package indexing policy >
///              /                  \
///         Functions               None
///             |                     |
///             v                     v
///  [functions, no targets]    [no functions/targets]
/// ```
///
/// # Examples
/// Local test-module context, outside any running Tokio runtime:
/// the macro-generated test wrapper runs its own runtime and returns `()`.
/// Prefer the test harness; ignored measurements require explicit selection.
/// ```text
/// find_package_respects_modes_prefixes_hints_names_and_independent_external_policy();
/// ```
#[tokio::test]
async fn find_package_respects_modes_prefixes_hints_names_and_independent_external_policy() {
    let p = Project::new().await;
    p.write(
        "modules/FindDemo.cmake",
        "function(from_module)\nendfunction()\n",
    )
    .await;
    p.write(
        "prefix/lib/cmake/Demo/DemoConfig.cmake",
        "function(from_config)\nendfunction()\nadd_library(Demo::Lib INTERFACE IMPORTED)\n",
    )
    .await;
    p.write(
        "custom/Special.cmake",
        "function(special_call)\nendfunction()\n",
    )
    .await;
    p.write(
        "custom/RealConfig.cmake",
        "function(from_hint)\nendfunction()\n",
    )
    .await;
    for (index, (body, expected)) in [
        (
            "find_package(Demo REQUIRED)\nfrom_module()",
            "FindDemo.cmake",
        ),
        ("find_package(Demo MODULE REQUIRED)", "FindDemo.cmake"),
        (
            "find_package(Demo CONFIG REQUIRED)\nfrom_config()",
            "DemoConfig.cmake",
        ),
        (
            "set(CMAKE_FIND_PACKAGE_PREFER_CONFIG ON)\nfind_package(Demo)",
            "DemoConfig.cmake",
        ),
        (
            "set(CMAKE_PREFIX_PATH ../prefix)\nfind_package(Demo NO_MODULE)",
            "DemoConfig.cmake",
        ),
        (
            "set(Demo_DIR ../prefix/lib/cmake/Demo)\nfind_package(Demo CONFIG)",
            "DemoConfig.cmake",
        ),
        (
            "find_package(Alias CONFIG NAMES Real PATHS ../custom NO_DEFAULT_PATH)",
            "RealConfig.cmake",
        ),
        ("find_package(Alias CONFIG NAMES Real PATHS \"${CMAKE_CURRENT_SOURCE_DIR}/../custom\" NO_DEFAULT_PATH)","RealConfig.cmake"),
        ("find_package(Demo CONFIG CONFIGS Special.cmake PATHS ../custom NO_DEFAULT_PATH)","Special.cmake"),
    ]
    .into_iter()
    .enumerate()
    {
        p.write("app/CMakeLists.txt", body).await;
        let mut w = Workspace::new(p.0.join("app/CMakeLists.txt"), None).unwrap();
        w.set_module_paths(vec![p.0.join("modules")]);
        w.set_external_indexing(
            ExternalIndex::None,
            ExternalIndex::Full,
            if matches!(index,4|5) {
                vec![]
            } else {
                vec![p.0.join("prefix")]
            },
        );
        w.flush().await.unwrap();
        let g = w.root_graph().unwrap();
        assert!(g.diagnostics.is_empty(), "{body}: {:?}", g.diagnostics);
        let edge = &g.edges[0];
        assert_eq!(edge.kind, DependencyKind::Package);
        assert_eq!(
            edge.resolved.as_ref().unwrap().file_name().unwrap(),
            expected,
            "{body}"
        );
        assert!(edge.invocation.is_some());
    }
    p.write(
        "app/CMakeLists.txt",
        "find_package(Demo CONFIG REQUIRED)\nfrom_config()\n",
    )
    .await;
    for mode in [ExternalIndex::Functions, ExternalIndex::None] {
        let mut w = Workspace::new(p.0.join("app/CMakeLists.txt"), None).unwrap();
        w.set_external_indexing(ExternalIndex::Full, mode, vec![p.0.join("prefix")]);
        w.flush().await.unwrap();
        let g = w.root_graph().unwrap();
        assert!(g.edges[0].resolved.is_some());
        assert!(g.targets.is_empty());
        assert_eq!(
            g.references
                .functions
                .iter()
                .any(|function| function.name == "from_config"),
            mode == ExternalIndex::Functions
        );
    }
}
/// Disable module lookup for a named include, literal relative include, and CONFIG package lookup.
/// Writes/reads/removes temporary files and changes lookup settings; no project execution.
///
/// # Arguments
/// None; the test constructs its own fixtures.
///
/// # Returns
/// `()` after all checks pass.
///
/// # Panics
/// If fixture setup or any documented assertion fails.
///
/// # Algorithm
/// Require only the literal file include to resolve despite configured module/prefix paths.
///
/// # Flow
/// ```text
/// [disable module lookup]
///            |
///       +----+---------------+----------------+
///       v                    v                v
/// [named include]    [literal .cmake]    [CONFIG package]
///       |                    |                |
///       v                    v                v
///  [unresolved]          [resolved]       [unresolved]
/// ```
///
/// # Examples
/// Local test-module context, outside any running Tokio runtime:
/// the macro-generated test wrapper runs its own runtime and returns `()`.
/// Prefer the test harness; ignored measurements require explicit selection.
/// ```text
/// disabled_module_lookup_keeps_literal_file_dependencies();
/// ```
#[tokio::test]
async fn disabled_module_lookup_keeps_literal_file_dependencies() {
    let p = Project::new().await;
    p.write("app/CMakeLists.txt","set(CMAKE_MODULE_PATH ../modules)\ninclude(External OPTIONAL)\ninclude(../modules/External.cmake)\nfind_package(Demo CONFIG)\n").await;
    p.write("modules/External.cmake", "set(EXTERNAL yes)\n")
        .await;
    p.write("prefix/DemoConfig.cmake", "").await;
    let mut w = Workspace::new(p.0.join("app/CMakeLists.txt"), None).unwrap();
    w.set_module_paths(vec![p.0.join("modules")]);
    w.set_external_indexing(
        ExternalIndex::Full,
        ExternalIndex::Full,
        vec![p.0.join("prefix")],
    );
    w.set_module_lookup_enabled(false);
    w.flush().await.unwrap();
    let g = w.root_graph().unwrap();
    assert!(g.edges[0].resolved.is_none());
    assert!(g.edges[1].resolved.is_some());
    assert!(g.edges[2].resolved.is_none());
}
/// Analyze absent optional CONFIG and REQUIRED MODULE package requests.
/// Writes/reads/removes a temporary root; static lookup only, no project execution.
///
/// # Arguments
/// None; the test constructs its own fixtures.
///
/// # Returns
/// `()` after all checks pass.
///
/// # Panics
/// If fixture setup or any documented assertion fails.
///
/// # Algorithm
/// Require two edges but exactly one diagnostic naming AlsoAbsent.
///
/// # Flow
/// ```text
/// [two absent package requests]
///             |
///        +----+-------------------+
///        v                        v
/// [optional CONFIG]       [REQUIRED MODULE]
///        |                        |
///        v                        v
/// [edge, no diagnostic]   [edge + AlsoAbsent diagnostic]
///        |                        |
///        +------------+-----------+
///                     v
///         [2 edges / 1 diagnostic]
/// ```
///
/// # Examples
/// Local test-module context, outside any running Tokio runtime:
/// the macro-generated test wrapper runs its own runtime and returns `()`.
/// Prefer the test harness; ignored measurements require explicit selection.
/// ```text
/// optional_missing_packages_and_required_missing_packages_are_distinct();
/// ```
#[tokio::test]
async fn optional_missing_packages_and_required_missing_packages_are_distinct() {
    let p = Project::new().await;
    p.write(
        "CMakeLists.txt",
        "find_package(DefinitelyAbsent CONFIG QUIET)\nfind_package(AlsoAbsent MODULE REQUIRED)\n",
    )
    .await;
    let mut w = p.workspace();
    w.flush().await.unwrap();
    let g = w.root_graph().unwrap();
    assert_eq!(g.edges.len(), 2);
    assert_eq!(g.diagnostics.len(), 1);
    assert!(g.diagnostics[0].message.contains("AlsoAbsent"));
}
