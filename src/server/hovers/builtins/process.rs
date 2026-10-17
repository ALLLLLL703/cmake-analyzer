//! Bounded native-help child inspection, not project execution.
//! Only internal callers supply help flags. Timeout, overflow, read failure and cancellation kill and
//! wait for the direct child; kill_on_drop is a fallback, not recursive process-tree cleanup.

use crate::config::Runtime;
use lspally::server::model::{Cancellation, ServerError, ServerResult};
use std::{path::Path, process::Stdio, time::Duration};
use tokio::io::AsyncReadExt;

/// Help flags only: no project path, script, build directory, or configured interpreter.
/// Inspect one native help child and always attempt direct-child cleanup on unsuccessful work.
/// stdin/stderr are null and stdout is piped. The timeout covers output reading and waiting, not spawn
/// or subsequent cleanup. This helper trusts internal callers to pass only help flags; it does not
/// enforce a sandbox or kill descendants.
///
/// # Arguments
/// - `executable` `[in]`: startup-PATH CMake executable, never the configured project interpreter.
/// - `cwd` `[in]`: startup directory for the child.
/// - `args` `[in]`: help-only arguments supplied by LocalHelp; not user script/project arguments.
/// - `limits` `[in]`: timeout_seconds and max_output_bytes; reads at most cap+1 bytes to detect overflow.
/// - `cancel` `[in]`: token checked before spawn and raced against the timed work.
///
/// # Returns
/// Ok(Some(nonblank UTF-8 text)) after successful exit; Ok(None) on spawn/read/wait failure, nonzero
/// exit, invalid/blank UTF-8, timeout or overflow.
///
/// # Errors
/// Err(cancelled) on request cancellation. Cleanup kill/wait errors are deliberately ignored.
///
/// # Algorithm
/// Reject pre-cancelled requests; spawn with kill_on_drop, capture bounded stdout, wait for success and
/// decode UTF-8. Race work with timeout and cancellation. Any non-success result triggers kill then
/// wait, even if the child has already exited.
///
/// # Flow
/// ```text
///  [Pre-cancelled?] --yes--> [Cancellation error]
///          | no
///  [Spawn help child] --fail--> [None]
///          |
///   +------+------------------+
///   v                         v
///  [Read <= cap+1; wait]   [Timeout / cancellation]
///   | success                 |
///  [UTF-8 + nonblank?] --no--> [Kill + wait]
///   | yes                     |
///  [Some(text)]          [None / cancellation error]
/// ```
///
/// # Examples
/// Local server context:
/// ```text
/// let text = process::read(&cmake, &cwd, &["--help-command", "set"], &limits, &cancel).await?;
/// ```
pub(super) async fn read(
    executable: &Path,
    cwd: &Path,
    args: &[&str],
    limits: &Runtime,
    cancel: &Cancellation,
) -> ServerResult<Option<String>> {
    if cancel.is_cancelled() {
        return Err(ServerError::cancelled());
    }
    let Ok(mut child) = tokio::process::Command::new(executable)
        .args(args)
        .current_dir(cwd)
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .kill_on_drop(true)
        .spawn()
    else {
        return Ok(None);
    };
    let work = async {
        let mut output = Vec::new();
        child
            .stdout
            .take()?
            .take((limits.max_output_bytes as u64).saturating_add(1))
            .read_to_end(&mut output)
            .await
            .ok()?;
        if output.len() > limits.max_output_bytes {
            return None;
        }
        let status = child.wait().await.ok()?;
        if !status.success() {
            return None;
        }
        String::from_utf8(output)
            .ok()
            .filter(|text| !text.trim().is_empty())
    };
    let result = tokio::select! {
        _=cancel.cancelled()=>Err(ServerError::cancelled()),
        result=tokio::time::timeout(Duration::from_secs(limits.timeout_seconds),work)=>Ok(result.ok().flatten()),
    };
    // Timeout, overflow, cancellation, and read failures still kill and reap the child.
    if !matches!(&result, Ok(Some(_))) {
        let _ = child.kill().await;
        let _ = child.wait().await;
    }
    result
}
