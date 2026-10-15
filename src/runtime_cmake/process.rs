// AI-authored cancellable configure process with bounded captured trace output.
use super::{model::*, trace};
use lspally::server::model::Cancellation;
use std::{io, process::Stdio};
use tokio::io::AsyncReadExt;

pub async fn configure(
    run: RunId,
    options: ConfigureOptions,
    cancel: Cancellation,
) -> io::Result<Trace> {
    if cancel.is_cancelled() {
        return Err(io::Error::new(
            io::ErrorKind::Interrupted,
            "CMake configure cancelled",
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
    let mut child = tokio::process::Command::new(&options.executable)
        .arg("-S")
        .arg(&options.source_directory)
        .arg("-B")
        .arg(&options.build_directory)
        .args(&options.arguments)
        .args(["--trace-expand", "--trace-format=json-v1"])
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
        _=cancel.cancelled()=>Err(io::Error::new(io::ErrorKind::Interrupted,"CMake configure cancelled")),
        _=tokio::time::sleep(options.timeout)=>Err(io::Error::new(io::ErrorKind::TimedOut,"CMake configure timed out")),
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
                    "CMake configure failed ({status}): {}",
                    error_detail(&bytes, options.max_error_bytes)
                )));
            }
            trace::parse(
                run,
                &bytes,
                &options.source_directory,
                &options.build_directory,
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
