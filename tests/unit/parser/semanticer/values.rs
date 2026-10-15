// AI-generated sparse possible-value regression tests.
use super::analyze;
use cmake_analyzer::parser::semanticer::model::ReferenceKind;
#[tokio::test]
async fn literals_and_reads_retain_values_at_distinct_commands() {
    let buffer=analyze("cmake_minimum_required(VERSION 3.31)\nset(A one)\nmessage(\"${A}\")\nset(A two)\nset(B \"${A}/suffix\")\nmessage(\"${A}\")\nset(UNKNOWN \"${MISSING}\")\n").await;
    let reads: Vec<_> = buffer
        .values
        .iter()
        .filter(|value| value.name == "A" && value.location.kind == ReferenceKind::Read)
        .map(|value| {
            (
                value.location.command.id,
                value.possible_value.as_ref().map(|value| value.text()),
            )
        })
        .collect();
    assert_eq!(
        reads,
        vec![(2, Some("one")), (4, Some("two")), (5, Some("two"))]
    );
    assert_eq!(
        buffer
            .variables
            .iter()
            .find(|variable| variable.name == "B")
            .unwrap()
            .possible_value
            .as_ref()
            .unwrap()
            .text(),
        "two/suffix"
    );
    assert!(
        buffer
            .variables
            .iter()
            .find(|variable| variable.name == "UNKNOWN")
            .unwrap()
            .possible_value
            .is_none()
    );
}
#[tokio::test]
async fn conditional_nonlocal_and_deferred_writes_do_not_invent_values() {
    let buffer=analyze("cmake_minimum_required(VERSION 3.31)\nset(A root)\nif(FLAG)\nset(A branch)\nendif()\nmessage(\"${A}\")\nfunction(f)\nset(X local)\nendfunction()\nset(C cached CACHE STRING doc)\nset(P value PARENT_SCOPE)\n").await;
    for name in ["A", "X", "C", "P"] {
        assert!(
            buffer
                .variables
                .iter()
                .find(|variable| variable.name == name)
                .unwrap()
                .possible_value
                .is_none(),
            "{name}"
        );
    }
}
