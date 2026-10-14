// AI-generated variable write-location tests.
use super::analyze;
use cmake_analyzer::workspace::Workspace;

#[tokio::test]
async fn first_write_and_each_argument_location_are_preserved() {
    let source = "cmake_minimum_required(VERSION 3.31)\nset(变量 first)\nset(变量 second)\nset(\"QUOTED\" x)\nset([=[BRACKET]=] x)\nlist(POP_BACK ITEMS DUP DUP)\n";
    let result = analyze(source).await;
    let variable = result.variables.iter().find(|v| v.name == "变量").unwrap();
    assert_eq!(variable.first_write, Some(variable.changing[0].span));
    assert_eq!(variable.changing.len(), 2);
    for change in &variable.changing {
        assert_eq!(change.span.text(source), Some("变量"));
        let command = &result.commands[change.command.id as usize];
        assert!(command.args.iter().any(|arg| arg.span == change.span));
    }
    assert_eq!(variable.first_write.unwrap().row, 1);
    assert_eq!(variable.first_write.unwrap().column, 4);
    for (name, raw) in [("QUOTED", "\"QUOTED\""), ("BRACKET", "[=[BRACKET]=]")] {
        let variable = result.variables.iter().find(|v| v.name == name).unwrap();
        assert_eq!(variable.first_write.unwrap().text(source), Some(raw));
    }
    let duplicate = result.variables.iter().find(|v| v.name == "DUP").unwrap();
    assert_eq!(duplicate.changing.len(), 2);
    assert_eq!(duplicate.changing[0].command, duplicate.changing[1].command);
    assert_ne!(duplicate.changing[0].span, duplicate.changing[1].span);
    let builtin = result
        .variables
        .iter()
        .find(|v| v.name == "CMAKE_CURRENT_SOURCE_DIR" && v.scope.is_none())
        .unwrap();
    assert!(builtin.first_write.is_none());
    assert!(builtin.changing.is_empty());
}

#[tokio::test]
async fn conditional_first_write_is_static_and_scopes_have_separate_locations() {
    let source = "cmake_minimum_required(VERSION 3.31)\nif(FLAG)\nset(V first)\nelse()\nset(V second)\nendif()\nfunction(f)\nset(V local)\nendfunction()\n";
    let result = analyze(source).await;
    let global = result
        .variables
        .iter()
        .find(|v| v.name == "V" && v.scope.is_none())
        .unwrap();
    let local = result
        .variables
        .iter()
        .find(|v| v.name == "V" && v.scope.is_some())
        .unwrap();
    assert_eq!(global.changing.len(), 2);
    assert_eq!(local.changing.len(), 1);
    assert_eq!(global.first_write.unwrap().row, 2);
    assert_eq!(local.first_write.unwrap().row, 7);
}

#[tokio::test]
async fn incremental_locations_follow_reused_commands_and_deleted_first_writes() {
    let file = std::env::temp_dir().join(format!(
        "cmake-variable-locations-{}.cmake",
        std::process::id()
    ));
    let mut workspace = Workspace::new(&file, None).unwrap();
    let original =
        "cmake_minimum_required(VERSION 3.31)\nset(A first)\nset(A second)\nset(KEEP x)\n";
    workspace.set_overlay(&file, original).unwrap();
    workspace.flush().await.unwrap();
    let old = workspace.snapshot(&file).unwrap().unwrap();
    let old_span = old
        .semantics
        .variables
        .iter()
        .find(|v| v.name == "A")
        .unwrap()
        .first_write
        .unwrap();
    let shifted = format!("# 🌟\n{original}");
    workspace.set_overlay(&file, shifted.clone()).unwrap();
    workspace.flush().await.unwrap();
    let snapshot = workspace.snapshot(&file).unwrap().unwrap();
    assert!(snapshot.work.semantics.reused > 0);
    let span = snapshot
        .semantics
        .variables
        .iter()
        .find(|v| v.name == "A")
        .unwrap()
        .first_write
        .unwrap();
    assert_eq!(span.row, old_span.row + 1);
    assert_eq!(span.start_byte, old_span.start_byte + "# 🌟\n".len());
    assert_eq!(
        old.semantics
            .variables
            .iter()
            .find(|v| v.name == "A")
            .unwrap()
            .first_write,
        Some(old_span)
    );
    let edited = shifted.replace("set(A first)\n", "");
    workspace.set_overlay(&file, edited.clone()).unwrap();
    workspace.flush().await.unwrap();
    let snapshot = workspace.snapshot(&file).unwrap().unwrap();
    let full = analyze(&edited).await;
    assert_eq!(
        format!("{:?}", snapshot.semantics.variables),
        format!("{:?}", full.variables)
    );
    let variable = snapshot
        .semantics
        .variables
        .iter()
        .find(|v| v.name == "A")
        .unwrap();
    assert_eq!(variable.changing.len(), 1);
    let command = &snapshot.semantics.commands[variable.changing[0].command.id as usize];
    assert!(command.args.iter().any(|arg| arg.content == "second"));
    assert_eq!(variable.first_write, Some(variable.changing[0].span));
    let edited = edited.replace("set(A second)", "set(NEW second)");
    workspace.set_overlay(&file, edited.clone()).unwrap();
    workspace.flush().await.unwrap();
    let snapshot = workspace.snapshot(&file).unwrap().unwrap();
    assert!(!snapshot.semantics.variables.iter().any(|v| v.name == "A"));
    let new = snapshot
        .semantics
        .variables
        .iter()
        .find(|v| v.name == "NEW")
        .unwrap();
    assert_eq!(new.first_write.unwrap().text(&edited), Some("NEW"));
}
