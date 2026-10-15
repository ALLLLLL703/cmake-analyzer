// AI-generated reference-qualified possible-value regression tests.
use super::analyze;
use cmake_analyzer::parser::semanticer::model::ReferenceKind;
#[tokio::test]
async fn literals_and_reads_retain_values_at_distinct_commands() {
    let buffer=analyze("cmake_minimum_required(VERSION 3.31)\nset(A one)\nmessage(\"${A}\")\nset(A two)\nset(B \"${A}/suffix\")\nmessage(\"${A}\")\nset(UNKNOWN \"${MISSING}\")\n").await;
    let a = buffer
        .variables
        .iter()
        .find(|variable| variable.name == "A")
        .unwrap();
    let reads: Vec<_> = a
        .references
        .iter()
        .filter(|reference| reference.kind == ReferenceKind::Read)
        .map(|reference| {
            (
                reference.command.id,
                reference.possible_value.as_ref().map(|value| value.text()),
            )
        })
        .collect();
    assert_eq!(
        reads,
        vec![(2, Some("one")), (4, Some("two")), (5, Some("two"))]
    );
    let writes: Vec<_> = a
        .references
        .iter()
        .filter(|reference| reference.kind != ReferenceKind::Read)
        .map(|reference| reference.possible_value.as_ref().map(|value| value.text()))
        .collect();
    assert_eq!(writes, vec![Some("one"), Some("two")]);
    for (name, expected) in [("B", Some("two/suffix")), ("UNKNOWN", None)] {
        let reference = buffer
            .variables
            .iter()
            .find(|variable| variable.name == name)
            .unwrap()
            .references
            .last()
            .unwrap();
        assert_eq!(
            reference.possible_value.as_ref().map(|value| value.text()),
            expected
        );
    }
    for value in &buffer.values {
        assert_eq!(value.location.possible_value, value.possible_value);
    }
}
#[tokio::test]
async fn conditional_nonlocal_and_deferred_writes_do_not_invent_values() {
    let buffer=analyze("cmake_minimum_required(VERSION 3.31)\nset(A root)\nif(FLAG)\nset(A branch)\nendif()\nmessage(\"${A}\")\nfunction(f)\nset(X local)\nendfunction()\nset(C cached CACHE STRING doc)\nset(P value PARENT_SCOPE)\n").await;
    for name in ["A", "X", "C", "P"] {
        let variable = buffer
            .variables
            .iter()
            .find(|variable| variable.name == name)
            .unwrap();
        assert!(
            variable
                .references
                .last()
                .and_then(|reference| reference.possible_value.as_ref())
                .is_none(),
            "{name}"
        );
    }
}
