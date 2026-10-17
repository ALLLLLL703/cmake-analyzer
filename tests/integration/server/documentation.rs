// AI-generated builtin/source-documentation hover regressions.
//! Native CMake help and source-comment hover documentation scenarios.
//! Native help is optional; source bindings, comment edits, Markdown and byte limits
//! are checked through [`super::hovers::hover`]. Tests remove their temporary projects.
use super::{
    hovers::hover,
    session::{HEADER, Project, Session, uri},
};
use cmake_analyzer::server::ServerOptions;
use serde_json::json;
/// Run PATH cmake --help-command for a native-help comparison, without configuring a project.
/// Captures stdout and stderr; unlike server inspection this helper has no timeout.
///
/// # Arguments
/// - `command` `[in]`: command-name argument supplied after --help-command, not shell text.
///
/// # Returns
/// Some(stdout) for a successful process; None if spawning/waiting fails or exit is
/// unsuccessful.
///
/// # Panics
/// Panics if successful stdout is not UTF-8. Process errors are returned as None, not Result.
///
/// # Examples
/// ```text
/// let expected = local_help("set").await;
/// ```
async fn local_help(command: &str) -> Option<String> {
    let output = tokio::process::Command::new("cmake")
        .args(["--help-command", command])
        .output()
        .await
        .ok()?;
    output
        .status
        .success()
        .then(|| String::from_utf8(output.stdout).unwrap())
}
#[tokio::test]
/// Check builtin hover uses PATH CMake help independently of disabled project execution.
/// Creates temporary files and an analyzer session; successful completion stops the session and
/// removes fixtures.
///
/// # Arguments
/// None.
///
/// # Returns
/// `()` if every assertion passes; no output parameter.
///
/// # Panics
/// Panics on any failed assertion or unwrapped fixture/transport/process failure.
/// Cleanup is explicit on the success path; a panic may leave a fixture tree.
///
/// # Algorithm
/// Disable runtime and set an invalid project executable, then hover set. Compare exactly with
/// local cmake --help-command set when available; otherwise require a null hover.
///
/// # Flow
/// ```text
/// [Temporary sources + settings]      [Expected: native help or null]
///              |                              |
///              v                              |
/// [LSP scenario messages]                 |
///              |                              |
///              +-------------+----------------+
///                            v
///                  [Compare assertions]
///                            |
///                            v
///             [Finish session + remove tree]
/// ```
///
/// # Examples
/// ```text
/// command_hover_uses_native_help_or_returns_none_without_it(); // #[tokio::test] supplies the runtime
/// ```
async fn command_hover_uses_native_help_or_returns_none_without_it() {
    let p = Project::new().await;
    tokio::fs::write(
        p.0.join(".cmake-analyzer.toml"),
        "[runtime]\nenable=false\nexecutable=\"not-a-cmake-executable\"\n",
    )
    .await
    .unwrap();
    let text = "set(V value)\n";
    let root = p.write("CMakeLists.txt", text).await;
    let mut client = Session::start(&p.0, vec![], true).await;
    client.open(&root, text).await;
    client.diagnostics(&root, 1).await;
    let response = hover(&mut client, &root, 1, 0, 10).await;
    if let Some(expected) = local_help("set").await {
        assert_eq!(
            response["result"]["contents"]["value"], expected,
            "{response}"
        );
    } else {
        assert!(response["result"].is_null(), "{response}");
    }
    client.finish().await;
    p.remove().await;
}
#[tokio::test]
/// Check native command and exact/template variable help uses plaintext without generator
/// metadata.
/// Creates temporary files and an analyzer session; successful completion stops the session and
/// removes fixtures.
///
/// # Arguments
/// None.
///
/// # Returns
/// `()` if every assertion passes; no output parameter.
///
/// # Panics
/// Panics on any failed assertion or unwrapped fixture/transport/process failure.
/// Cleanup is explicit on the success path; a panic may leave a fixture tree.
///
/// # Algorithm
/// If native set help is unavailable, clean up and return. Otherwise hover project/set, a
/// language template, a builtin directory, CMAKE_ARGV7, function ARGV0 and a project SOURCE_DIR
/// variable. Require the expected help headings and no generator path/header leakage.
///
/// # Flow
/// ```text
/// [Temporary sources + settings]      [Expected: native headings + plaintext]
///              |                              |
///              v                              |
/// [LSP scenario messages]                 |
///              |                              |
///              +-------------+----------------+
///                            v
///                  [Compare assertions]
///                            |
///                            v
///             [Finish session + remove tree]
/// ```
///
/// # Examples
/// ```text
/// builtin_commands_and_exact_multi_template_and_argument_variables_have_docs(); // #[tokio::test] supplies the runtime
/// ```
async fn builtin_commands_and_exact_multi_template_and_argument_variables_have_docs() {
    let p = Project::new().await;
    let text = "project(MyProject LANGUAGES CXX)\nset(CMAKE_CXX_FLAGS_DEBUG flags)\nmessage(\"${CMAKE_CURRENT_SOURCE_DIR}\" \"${CMAKE_CXX_FLAGS_DEBUG}\" \"${MyProject_SOURCE_DIR}\")\nmessage(\"${CMAKE_ARGV7}\")\nfunction(f P)\nmessage(\"${ARGV0}\")\nendfunction()\n";
    let root = p.write("CMakeLists.txt", text).await;
    let mut client = Session::start(&p.0, vec![], true).await;
    client.open(&root, text).await;
    client.diagnostics(&root, 1).await;
    if local_help("set").await.is_none() {
        client.finish().await;
        p.remove().await;
        return;
    }
    for (id, line, character, expected) in [
        (10, 1, 0, "project\n"),
        (11, 2, 0, "set\n"),
        (12, 2, 4, "CMAKE_<LANG>_FLAGS_DEBUG\n"),
        (13, 3, 12, "CMAKE_CURRENT_SOURCE_DIR\n"),
        (14, 4, 12, "CMAKE_ARGV0\n"),
        (15, 6, 12, "function\n"),
    ] {
        let result = hover(&mut client, &root, line, character, id).await;
        let content = result["result"]["contents"]["value"].as_str().unwrap();
        assert!(content.contains(expected), "{result}");
        assert!(!content.contains("Script-generated"), "{result}");
        assert!(!content.contains("scripts/gen/"), "{result}");
        assert_eq!(result["result"]["contents"]["kind"], "plaintext");
    }
    let line = text.lines().nth(2).unwrap();
    let character = line.find("MyProject_SOURCE_DIR").unwrap();
    let result = hover(&mut client, &root, 3, character, 16).await;
    assert!(
        result["result"]["contents"]["value"]
            .as_str()
            .unwrap()
            .contains("SOURCE_DIR"),
        "{result}"
    );
    client.finish().await;
    p.remove().await;
}
#[tokio::test]
/// Check imported function, macro and variable comments follow bindings and refresh after
/// comment edits.
/// Creates temporary files and an analyzer session; successful completion stops the session and
/// removes fixtures.
///
/// # Arguments
/// None.
///
/// # Returns
/// `()` if every assertion passes; no output parameter.
///
/// # Panics
/// Panics on any failed assertion or unwrapped fixture/transport/process failure.
/// Cleanup is explicit on the success path; a panic may leave a fixture tree.
///
/// # Algorithm
/// Include a module that supersedes outdated local variable documentation. Hover each imported
/// declaration use, exclude the outdated marker, edit only the function comment (including an
/// emoji), and require the updated text with the old comment absent.
///
/// # Flow
/// ```text
/// [Temporary sources + settings]      [Expected: bound and refreshed comments]
///              |                              |
///              v                              |
/// [LSP scenario messages]                 |
///              |                              |
///              +-------------+----------------+
///                            v
///                  [Compare assertions]
///                            |
///                            v
///             [Finish session + remove tree]
/// ```
///
/// # Examples
/// ```text
/// imported_function_macro_and_set_docs_follow_bindings_and_comment_only_edits(); // #[tokio::test] supplies the runtime
/// ```
async fn imported_function_macro_and_set_docs_follow_bindings_and_comment_only_edits() {
    let p = Project::new().await;
    let shared_text = "# External function documentation.\nfunction(external_fn)\nendfunction()\n#[=[\nExternal variable documentation.\n]=]\nset(EXTERNAL one)\n# External macro documentation.\nmacro(external_macro)\nendmacro()\n";
    let shared = p.write("shared.cmake", shared_text).await;
    let text = "# Outdated variable documentation.\nset(EXTERNAL old)\ninclude(shared.cmake)\nexternal_fn()\nmessage(\"${EXTERNAL}\")\nexternal_macro()\n";
    let root = p.write("CMakeLists.txt", text).await;
    let mut client = Session::start(&p.0, vec![], true).await;
    client.open(&root, text).await;
    client.diagnostics(&root, 1).await;
    client.open(&shared, shared_text).await;
    client.diagnostics(&shared, 1).await;
    for (id, line, character, expected) in [
        (10, 4, 0, "External function documentation."),
        (11, 5, 12, "External variable documentation."),
        (12, 6, 0, "External macro documentation."),
    ] {
        let result = hover(&mut client, &root, line, character, id).await;
        assert!(
            result["result"]["contents"]["value"]
                .as_str()
                .unwrap()
                .contains(expected),
            "{result}"
        );
        assert!(
            !result["result"]["contents"]["value"]
                .as_str()
                .unwrap()
                .contains("Outdated variable documentation"),
            "{result}"
        );
    }
    let edited = shared_text.replace(
        "External function documentation.",
        "Updated function documentation 🌟.",
    );
    client.send(json!({"jsonrpc":"2.0","method":"textDocument/didChange","params":{"textDocument":{"uri":uri(&shared),"version":2},"contentChanges":[{"text":format!("{HEADER}{edited}")}]}})).await;
    let result = hover(&mut client, &root, 4, 0, 13).await;
    let content = result["result"]["contents"]["value"].as_str().unwrap();
    assert!(
        content.contains("Updated function documentation 🌟.")
            && !content.contains("External function documentation."),
        "{result}"
    );
    client.finish().await;
    p.remove().await;
}
#[tokio::test]
/// Check detached/trailing comments stay unattached and Markdown hover respects a UTF-8 byte
/// budget.
/// Creates temporary files and an analyzer session; successful completion stops the session and
/// removes fixtures.
///
/// # Arguments
/// None.
///
/// # Returns
/// `()` if every assertion passes; no output parameter.
///
/// # Panics
/// Panics on any failed assertion or unwrapped fixture/transport/process failure.
/// Cleanup is explicit on the success path; a panic may leave a fixture tree.
///
/// # Algorithm
/// Configure max_view_bytes=128 and Markdown preference. Hover functions near blank/trailing
/// comments and one with a long emoji comment; exclude detached markers, require Markdown and
/// at most 128 bytes, and retain an emoji for the documented function.
///
/// # Flow
/// ```text
/// [Temporary sources + settings]      [Expected: Markdown <=128 bytes]
///              |                              |
///              v                              |
/// [LSP scenario messages]                 |
///              |                              |
///              +-------------+----------------+
///                            v
///                  [Compare assertions]
///                            |
///                            v
///             [Finish session + remove tree]
/// ```
///
/// # Examples
/// ```text
/// blank_lines_and_trailing_comments_do_not_attach_and_markdown_respects_byte_limit(); // #[tokio::test] supplies the runtime
/// ```
async fn blank_lines_and_trailing_comments_do_not_attach_and_markdown_respects_byte_limit() {
    let p = Project::new().await;
    tokio::fs::write(
        p.0.join(".cmake-analyzer.toml"),
        "[runtime]\nmax_view_bytes=128\n",
    )
    .await
    .unwrap();
    let text = format!(
        "# Detached documentation marker.\n\nfunction(no_docs)\nendfunction()\nno_docs() # trailing marker\nfunction(next_fn)\nendfunction()\nnext_fn()\n# {}\nfunction(documented)\nendfunction()\ndocumented()\n",
        "🌟".repeat(100)
    );
    let root = p.write("CMakeLists.txt", &text).await;
    let mut options = ServerOptions::new(p.0.clone());
    options.config.user_file = None;
    let mut client = Session::start_hover_format(options, vec![], true, true).await;
    client.open(&root, &text).await;
    client.diagnostics(&root, 1).await;
    for (id, line) in [(10, 5), (11, 8), (12, 12)] {
        let result = hover(&mut client, &root, line, 0, id).await;
        let content = result["result"]["contents"]["value"].as_str().unwrap();
        assert!(
            !content.contains("Detached documentation marker")
                && !content.contains("trailing marker"),
            "{result}"
        );
        assert!(content.len() <= 128, "{result}");
        assert_eq!(result["result"]["contents"]["kind"], "markdown");
        if line == 12 {
            assert!(content.contains('🌟'), "{result}");
        }
    }
    client.finish().await;
    p.remove().await;
}
