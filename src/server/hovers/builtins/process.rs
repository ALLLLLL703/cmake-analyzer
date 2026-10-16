use crate::config::Runtime;
use lspally::server::model::{Cancellation, ServerError, ServerResult};
use std::{path::Path, process::Stdio, time::Duration};
use tokio::io::AsyncReadExt;

/// Help flags only: no project path, script, build directory, or configured interpreter.
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
