//! Private Unix native-help tests attached by `hovers::builtins`. Shell fixtures log help flags and
//! simulate output, failure, sleeping and overflow; temporary files and direct children are cleaned
//! up on success.

// Contains AI-generated documentation edits.
// AI-generated local CMake help discovery, caching, and child-lifetime tests.
use super::{Kind, model::LocalHelp};
use crate::config::Runtime;
use lspally::server::model::Cancellation;
use std::{
    os::unix::fs::PermissionsExt,
    path::PathBuf,
    sync::{
        Arc,
        atomic::{AtomicUsize, Ordering},
    },
    time::Duration,
};
/// Process-local suffix counter for collision-free temporary fixture names.
/// Example: `NEXT.fetch_add(1, Ordering::Relaxed)` reserves the next local suffix.
static NEXT: AtomicUsize = AtomicUsize::new(0);
/// Wait until a newly written fixture can be executed for help-list inspection. Retries only
/// executable-file-busy errors caused by concurrent inherited writers; each successful probe runs
/// the fixture and may append to its calls log.
///
/// # Arguments
/// - `path` `[in]`: Path to a Unix fake CMake executable that accepts --help-command-list.
///
/// # Returns
/// `()` once a probe exits successfully, within five seconds; no document cache is changed.
///
/// # Panics
/// If the five-second guard expires, process launch fails for another reason or the probe exits
/// unsuccessfully.
///
/// # Algorithm
/// Run --help-command-list repeatedly. Sleep five milliseconds only after executable-file-busy;
/// otherwise require success and finish. The enclosing timeout bounds the entire retry loop.
///
/// # Flow
/// ```text
///   [launch help-list probe]
///               |
///      < launch result? >
///       /       |       \
///    busy     success   other error
///     |         |           |
/// [sleep 5ms] [return]    [panic]
///     |
///     +----> [retry, within 5s]
/// ```
///
/// # Examples
/// Local test-module context (await inside an async test):
/// ```text
/// executable_ready(&root.join("cmake")).await; // after writing and chmod of the fixture
/// ```
async fn executable_ready(path: &std::path::Path) {
    // Parallel fork/exec can briefly inherit a fixture's writer before CLOEXEC closes it.
    tokio::time::timeout(Duration::from_secs(5), async {
        loop {
            match tokio::process::Command::new(path)
                .arg("--help-command-list")
                .output()
                .await
            {
                Err(error) if error.kind() == std::io::ErrorKind::ExecutableFileBusy => {
                    tokio::time::sleep(Duration::from_millis(5)).await
                }
                result => {
                    assert!(result.unwrap().status.success());
                    break;
                }
            }
        }
    })
    .await
    .unwrap();
}
/// Create a unique fake CMake help installation and seed LocalHelp with its executable. Writes an
/// executable that logs all arguments, probes readiness, then deletes the probe log so test call
/// counts start empty.
///
/// # Arguments
/// - `body` `[in]`: Fixed shell dispatcher body appended after logging; may simulate help output,
///   misses or sleeping children.
/// - `limits` `[in]`: Runtime inspection timeout/output/cache budgets moved into the LocalHelp
///   instance.
///
/// # Returns
/// A pair: temporary root for explicit cleanup, then a LocalHelp cache with executable already
/// selected and no readiness calls left in its log. The helper does execute the readiness probe,
/// not a project configure.
///
/// # Panics
/// If fixture file/directory/permission operations, readiness probing, log removal or
/// executable-cell initialization fail.
///
/// # Algorithm
/// Write the logger and body, chmod it, await executable_ready and clear calls. Construct LocalHelp
/// with the root as working directory and seed its executable cell to bypass PATH discovery.
///
/// # Flow
/// ```text
///   [body + limits]
///          |
/// [write/chmod executable]
///          |
///  [readiness child] ------> [calls log]
///          |                     |
///          |               [delete probe log]
///          +----------+----------+
///                     v
///          [seed LocalHelp executable]
///                     |
///            [return root + cache]
/// ```
///
/// # Examples
/// Local test-module context (await inside an async test):
/// ```text
/// let (root, mut help) = fixture(CATALOG, Runtime::default()).await;
/// let docs = help.documentation(Kind::Command, "message", &Cancellation::default()).await.unwrap();
/// assert!(docs.is_some());
/// tokio::fs::remove_dir_all(root).await.unwrap();
/// ```
async fn fixture(body: &str, limits: Runtime) -> (PathBuf, LocalHelp) {
    let root = std::env::temp_dir().join(format!(
        "cmake-local-help-{}-{}",
        std::process::id(),
        NEXT.fetch_add(1, Ordering::Relaxed)
    ));
    tokio::fs::create_dir_all(&root).await.unwrap();
    let executable = root.join("cmake");
    let script = format!(
        "#!/bin/sh\n# Script-generated by tests/unit/server/local_help.rs.\nprintf '%s\\n' \"$*\" >> '{}'\n{}",
        root.join("calls").display(),
        body
    );
    tokio::fs::write(&executable, script).await.unwrap();
    tokio::fs::set_permissions(&executable, std::fs::Permissions::from_mode(0o755))
        .await
        .unwrap();
    executable_ready(&executable).await;
    tokio::fs::remove_file(root.join("calls")).await.unwrap();
    let help = LocalHelp::new(root.clone(), limits);
    help.executable.set(Some(executable)).unwrap();
    (root, help)
}
/// Fixed shell dispatcher for command/variable catalogs and help bodies. Lists a templated
/// language/config variable; MISSING exits unsuccessfully to exercise negative caching.
/// Example: `fixture(CATALOG, Runtime::default()).await` creates a logging help installation.
const CATALOG: &str = "case \"$1\" in\n--help-command-list) printf 'function\\nmessage\\n';;\n--help-variable-list) printf 'CMAKE_<LANG>_FLAGS_<CONFIG>\\nKNOWN\\nMISSING\\n';;\n--help-command) printf '%s command docs\\n' \"$2\";;\n--help-variable) [ \"$2\" = MISSING ] && exit 1; printf '%s variable docs\\n' \"$2\";;\nesac\n";
/// Query fake native catalogs for two language/config template names, repeated missing/unknown
/// variables, ARGV7 and uppercase MESSAGE.
///
/// Creates temporary executables/logs and launches fake help children; removes the temporary root
/// on success. No project or network access.
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
/// Require shared canonical bodies, cached misses, one list/missing lookup and four document-cache
/// entries.
///
/// # Flow
/// ```text
///   [fake help catalogs]
///    |
///   [canonicalize names + query]----------> [repeat hits/misses]
///                                          |
///                                         [check shared bodies + call counts]
///                                          |
///                                         < checks hold? >
///                                          /         \
///                                        [pass]     [panic]
/// ```
///
/// # Examples
/// Direct local call outside any Tokio runtime (the test attribute creates its runtime):
/// ```text
/// local_catalogs_resolve_templates_aliases_and_cache_canonical_hits_and_misses();
/// ```
#[tokio::test]
async fn local_catalogs_resolve_templates_aliases_and_cache_canonical_hits_and_misses() {
    let (root, mut help) = fixture(CATALOG, Runtime::default()).await;
    let cancel = Cancellation::default();
    let first = help
        .documentation(Kind::Variable, "CMAKE_CXX_FLAGS_DEBUG", &cancel)
        .await
        .unwrap()
        .unwrap();
    let second = help
        .documentation(Kind::Variable, "CMAKE_C_FLAGS_RELEASE", &cancel)
        .await
        .unwrap()
        .unwrap();
    assert!(Arc::ptr_eq(&first, &second));
    assert!(first.contains("CMAKE_<LANG>_FLAGS_<CONFIG>"));
    assert!(
        help.documentation(Kind::Variable, "MISSING", &cancel)
            .await
            .unwrap()
            .is_none()
    );
    assert!(
        help.documentation(Kind::Variable, "MISSING", &cancel)
            .await
            .unwrap()
            .is_none()
    );
    for name in ["user_name", "other_name"] {
        assert!(
            help.documentation(Kind::Variable, name, &cancel)
                .await
                .unwrap()
                .is_none()
        );
    }
    let args = help
        .documentation(Kind::Variable, "ARGV7", &cancel)
        .await
        .unwrap()
        .unwrap();
    assert!(args.contains("function command docs"));
    assert!(
        help.documentation(Kind::Command, "MESSAGE", &cancel)
            .await
            .unwrap()
            .unwrap()
            .contains("message command docs")
    );
    let calls = tokio::fs::read_to_string(root.join("calls")).await.unwrap();
    assert_eq!(
        calls
            .lines()
            .filter(|line| line.starts_with("--help-variable-list"))
            .count(),
        1
    );
    assert_eq!(
        calls
            .lines()
            .filter(|line| *line == "--help-variable MISSING")
            .count(),
        1
    );
    assert_eq!(help.documents.len(), 4);
    tokio::fs::remove_dir_all(root).await.unwrap();
}
/// Set a 64-byte cache budget and request template-variable, function and message help.
///
/// Creates temporary executables/logs and launches fake help children; removes the temporary root
/// on success. No project or network access.
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
/// Require returned documents, correct cached-byte accounting and the overflowing message body
/// absent from the cache.
///
/// # Flow
/// ```text
///   [64-byte cache budget]
///    |
///   [request three help bodies]----------> [sum cached bodies]
///                                         |
///                                        [overflow body returned, not cached]
///                                         |
///                                        < checks hold? >
///                                         /         \
///                                       [pass]     [panic]
/// ```
///
/// # Examples
/// Direct local call outside any Tokio runtime (the test attribute creates its runtime):
/// ```text
/// cached_help_bodies_share_the_global_output_byte_budget();
/// ```
#[tokio::test]
async fn cached_help_bodies_share_the_global_output_byte_budget() {
    let mut limits = Runtime::default();
    limits.max_output_bytes = 64;
    let (root, mut help) = fixture(CATALOG, limits).await;
    let cancel = Cancellation::default();
    for (kind, name) in [
        (Kind::Variable, "CMAKE_CXX_FLAGS_DEBUG"),
        (Kind::Command, "function"),
        (Kind::Command, "message"),
    ] {
        assert!(
            help.documentation(kind, name, &cancel)
                .await
                .unwrap()
                .is_some()
        );
    }
    assert!(help.cached_bytes <= 64);
    assert_eq!(
        help.cached_bytes,
        help.documents
            .values()
            .flatten()
            .map(|document| document.len())
            .sum::<usize>()
    );
    assert!(
        !help
            .documents
            .contains_key(&(Kind::Command, "message".into()))
    );
    tokio::fs::remove_dir_all(root).await.unwrap();
}
/// Seed an absent executable and require no builtin documentation.
///
/// Creates temporary executables/logs and launches fake help children; removes the temporary root
/// on success. No project or network access.
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
/// Then run a fixture with an endless help body under a 64-byte capture cap; require bounded
/// completion with no document rather than fallback text.
///
/// # Flow
/// ```text
///   [missing executable]
///    |
///   [return no document]----------> [endless help under byte cap]
///                                  |
///                                 [bounded no-document result]
///                                  |
///                                 < checks hold? >
///                                  /         \
///                                [pass]     [panic]
/// ```
///
/// # Examples
/// Direct local call outside any Tokio runtime (the test attribute creates its runtime):
/// ```text
/// missing_native_executable_and_oversized_help_have_no_embedded_fallback();
/// ```
#[tokio::test]
async fn missing_native_executable_and_oversized_help_have_no_embedded_fallback() {
    let mut help = LocalHelp::new(std::env::temp_dir(), Runtime::default());
    help.executable.set(None).unwrap();
    assert!(
        help.documentation(Kind::Command, "message", &Cancellation::default())
            .await
            .unwrap()
            .is_none()
    );
    let mut limits = Runtime::default();
    limits.max_output_bytes = 64;
    let (root,mut help)=fixture("if [ \"$1\" = --help-command-list ]; then printf 'message\\n'; else while :; do printf 'xxxxxxxxxxxxxxxx'; done; fi\n",limits).await;
    assert!(
        tokio::time::timeout(
            Duration::from_secs(5),
            help.documentation(Kind::Command, "message", &Cancellation::default())
        )
        .await
        .unwrap()
        .unwrap()
        .is_none()
    );
    tokio::fs::remove_dir_all(root).await.unwrap();
}
/// Start a fake help child that writes its PID and sleeps, wait for that marker and cancel the
/// request.
///
/// Creates temporary executables/logs and launches fake help children; removes the temporary root
/// on success. No project or network access.
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
/// Require an error, an empty document cache and a failed kill-zero PID probe; rewrite the
/// executable with normal catalog help and require a successful retry.
///
/// # Flow
/// ```text
///   [sleeping help child + PID]
///    |
///   [cancel after startup]---------------> [rewrite executable + retry]
///                                         |
///                                        [check reaped PID + uncached failure]
///                                         |
///                                        < checks hold? >
///                                         /         \
///                                       [pass]     [panic]
/// ```
///
/// # Examples
/// Direct local call outside any Tokio runtime (the test attribute creates its runtime):
/// ```text
/// cancellation_reaps_the_help_child_and_does_not_cache_a_failure();
/// ```
#[tokio::test]
async fn cancellation_reaps_the_help_child_and_does_not_cache_a_failure() {
    let (root,mut help)=fixture("if [ \"$1\" = --help-command-list ]; then printf 'message\\n'; else echo $$ > pid; exec sleep 30; fi\n",Runtime::default()).await;
    let cancel = Cancellation::default();
    let task_cancel = cancel.clone();
    let task = tokio::spawn(async move {
        let result = help
            .documentation(Kind::Command, "message", &task_cancel)
            .await;
        (help, result)
    });
    tokio::time::timeout(Duration::from_secs(5), async {
        while tokio::fs::metadata(root.join("pid")).await.is_err() {
            tokio::time::sleep(Duration::from_millis(5)).await;
        }
    })
    .await
    .unwrap();
    cancel.cancel();
    let (mut help, result) = tokio::time::timeout(Duration::from_secs(5), task)
        .await
        .unwrap()
        .unwrap();
    assert!(result.is_err());
    assert!(help.documents.is_empty());
    let pid = tokio::fs::read_to_string(root.join("pid")).await.unwrap();
    assert!(
        !tokio::process::Command::new("kill")
            .args(["-0", pid.trim()])
            .stderr(std::process::Stdio::null())
            .status()
            .await
            .unwrap()
            .success()
    );
    tokio::fs::write(
        root.join("cmake"),
        format!("#!/bin/sh\n# Script-generated by tests/unit/server/local_help.rs.\n{CATALOG}"),
    )
    .await
    .unwrap();
    executable_ready(&root.join("cmake")).await;
    assert!(
        help.documentation(Kind::Command, "message", &Cancellation::default())
            .await
            .unwrap()
            .is_some()
    );
    tokio::fs::remove_dir_all(root).await.unwrap();
}
/// Run a fake help child that sleeps after catalog discovery with a one-second runtime timeout. An
/// outer five-second guard must finish with no document, then remove the temporary fixture.
///
/// Creates temporary executables/logs and launches fake help children; removes the temporary root
/// on success. No project or network access.
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
/// help_timeout_returns_no_document();
/// ```
#[tokio::test]
async fn help_timeout_returns_no_document() {
    let mut limits = Runtime::default();
    limits.timeout_seconds = 1;
    let (root, mut help) = fixture(
        "if [ \"$1\" = --help-command-list ]; then printf 'message\\n'; else exec sleep 30; fi\n",
        limits,
    )
    .await;
    assert!(
        tokio::time::timeout(
            Duration::from_secs(5),
            help.documentation(Kind::Command, "message", &Cancellation::default())
        )
        .await
        .unwrap()
        .unwrap()
        .is_none()
    );
    tokio::fs::remove_dir_all(root).await.unwrap();
}
