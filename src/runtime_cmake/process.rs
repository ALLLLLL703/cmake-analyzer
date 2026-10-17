//! Runtime execution boundary: own one CMake child, capture bounded stderr, wait or
//! kill/reap on cancellation/timeout/read/overflow errors, then decode on a worker.
//! Project code is not sandboxed; only the direct child is explicitly controlled.
//!
//! Contains AI-generated documentation additions.
// AI-authored cancellable CMake process with bounded captured trace output.
use super::{model::*, trace};
use lspally::server::model::Cancellation;
use std::{io, process::Stdio};
use tokio::io::AsyncReadExt;

/// Run configure or a script with expanded JSON trace, owning one direct CMake child.
/// Child cwd is source_directory; stdin/stdout are null and stderr is captured. Project/script
/// code can write arbitrary files, access networks and spawn descendants; this is not a sandbox.
/// On cancellation, timeout or capture failure, kill and reap the direct child best-effort.
/// On successful capture/exit, decode on a joined blocking worker. No build-target invocation.
///
/// # Arguments
/// - `run` `[in]`: caller-assigned identity attached to the resulting trace.
/// - `options` `[in]`: consumed executable, arguments, mode, paths and time/output limits; caller is responsible for authorization and validation.
/// - `cancel` `[in]`: shared cancellation token consumed here; callers may retain clones to request stopping.
///
/// # Returns
/// `Ok(trace)` only for successful child exit and valid nonempty command trace; `Err` otherwise.
/// File/process effects from the child are not rolled back on any outcome.
///
/// # Errors
/// Interrupted for pre-launch/running/decoder cancellation; TimedOut for the capture/wait deadline;
/// InvalidInput for an unrepresentable limit+1. Spawn/read/wait/worker errors, output overflow,
/// nonzero child exit with bounded-detail suffix, malformed events or empty trace also fail.
/// The timeout does not bound blocking decoding; it checks cancellation per input line.
/// Dropping the future uses kill-on-drop but does not promise joined reaping or descendant termination.
///
/// # Algorithm
/// Precheck cancellation and overflow-safe read bound. Append owned trace and -S/-B or -P
/// flags, spawn the child and race cancellation/deadline against capped stderr read plus wait.
/// Kill/reap on capture errors; otherwise reject unsuccessful status or decode captured bytes.
///
/// # Flow
/// ```text
///  [options + cancellation]
///             |
///  [precheck + spawn child] ---> [project side effects]
///             |
///       [race capture/wait]
///        /      |       \
///   cancelled timeout  complete
///        \     /         |
///       [kill + wait]  <exit success?>
///            |          /      \
///          [Err]     [Err]  [blocking decode]
///                                 |
///                             [Trace / Err]
/// ```
///
/// # Examples
/// ```no_run
/// # use cmake_analyzer::runtime_cmake::{execute, model::{ConfigureOptions, ExecutionMode, RunId}};
/// # use lspally::server::model::Cancellation;
/// # use std::time::Duration;
/// # async fn example() -> std::io::Result<()> {
/// // Requires installed CMake and a trusted saved project; executes its code.
/// let options = ConfigureOptions { mode: ExecutionMode::Configure, executable: "cmake".into(),
///     source_directory: "/work/project".into(), build_directory: "/work/analysis-build".into(),
///     arguments: vec![], timeout: Duration::from_secs(30), max_output_bytes: 1_048_576, max_error_bytes: 4096 };
/// let trace = execute(RunId::new(1), options, Cancellation::default()).await?;
/// # Ok(())
/// # }
/// ```
pub async fn execute(
    run: RunId,
    options: ConfigureOptions,
    cancel: Cancellation,
) -> io::Result<Trace> {
    if cancel.is_cancelled() {
        return Err(io::Error::new(
            io::ErrorKind::Interrupted,
            "CMake execution cancelled",
        ));
    }
    let limit = u64::try_from(options.max_output_bytes)
        .ok()
        .and_then(|value| value.checked_add(1))
        .ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::InvalidInput,
                "Runtime output limit is too large",
            )
        })?;
    let mut command = tokio::process::Command::new(&options.executable);
    command
        .args(&options.arguments)
        .args(["--trace-expand", "--trace-format=json-v1"]);
    let build = match &options.mode {
        ExecutionMode::Configure => {
            command
                .arg("-S")
                .arg(&options.source_directory)
                .arg("-B")
                .arg(&options.build_directory);
            Some(options.build_directory.clone())
        }
        ExecutionMode::Script { file } => {
            command.arg("-P").arg(file);
            None
        }
    };
    let mut child = command
        .current_dir(&options.source_directory)
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::piped())
        .kill_on_drop(true)
        .spawn()?;
    let stderr = child
        .stderr
        .take()
        .ok_or_else(|| io::Error::other("CMake stderr was not captured"))?;
    let result = tokio::select! {
        _=cancel.cancelled()=>Err(io::Error::new(io::ErrorKind::Interrupted,"CMake execution cancelled")),
        _=tokio::time::sleep(options.timeout)=>Err(io::Error::new(io::ErrorKind::TimedOut,"CMake execution timed out")),
        result=async {
            let mut bytes=Vec::new();stderr.take(limit).read_to_end(&mut bytes).await?;
            if bytes.len()>options.max_output_bytes {return Err(io::Error::other("CMake trace output limit exceeded"));}
            let status=child.wait().await?;
            Ok((bytes,status))
        }=>result,
    };
    match result {
        Ok((bytes, status)) => tokio::task::spawn_blocking(move || {
            if !status.success() {
                return Err(io::Error::other(format!(
                    "CMake execution failed ({status}): {}",
                    error_detail(&bytes, options.max_error_bytes)
                )));
            }
            trace::parse(
                run,
                &bytes,
                &options.source_directory,
                build.as_deref(),
                &cancel,
            )
        })
        .await
        .map_err(io::Error::other)?,
        Err(error) => {
            let _ = child.kill().await;
            let _ = child.wait().await;
            Err(error)
        }
    }
}
/// Collect the tail of nonempty, non-JSON-looking stderr lines for a failure message.
/// In-memory only; no I/O or mutation of input.
///
/// # Arguments
/// - `bytes` `[in]`: complete captured stderr; lines beginning with { are skipped.
/// - `limit` `[in]`: source-byte budget for suffix extraction, not a strict rendered UTF-8 bound.
///
/// # Returns
/// Lossy UTF-8 text in original line order, joined by LF. Replacement characters can exceed the input-byte budget.
///
/// # Algorithm
/// Scan lines backwards, skipping empty/JSON-looking lines. Keep each eligible suffix
/// within the remaining input-byte budget, then reverse retained lines for display.
/// A zero budget can retain an empty suffix; decoding uses UTF-8 replacement characters.
///
/// # Flow
/// ```text
///  [captured stderr] ---> [last line] <---------------+
///                             |                      |
///                     <empty or starts {?> --yes-----+
///                             | no                   |
///                     [retain budgeted suffix]       |
///                             |                      |
///                        <budget left?> --yes---------+
///                             | no / exhausted input
///                     [reverse retained lines]
///                             |
///                     [join + lossy UTF-8 text]
/// ```
///
/// # Examples
/// ```text
/// assert_eq!(error_detail(b"{trace}\nerror message\n", 7), "message");
/// ```
fn error_detail(bytes: &[u8], limit: usize) -> String {
    let mut lines = Vec::new();
    let mut remaining = limit;
    for line in bytes.rsplit(|byte| *byte == b'\n') {
        if line.first() == Some(&b'{') || line.is_empty() {
            continue;
        }
        let start = line.len().saturating_sub(remaining);
        lines.push(String::from_utf8_lossy(&line[start..]).into_owned());
        remaining = remaining.saturating_sub(line.len() + 1);
        if remaining == 0 {
            break;
        }
    }
    lines.reverse();
    lines.join("\n")
}
