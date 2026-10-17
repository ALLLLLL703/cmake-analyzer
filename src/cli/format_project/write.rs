//! Filesystem execution boundary for staged formatting edits. Reads the original,
//! writes a sibling temporary file, preserves permissions and renames over the original.
//! Replacement is per-file atomic, not a batch transaction or durability guarantee.
//!
//! Contains AI-generated documentation additions.
use super::model::Edit;
use std::{
    io,
    sync::atomic::{AtomicU64, Ordering},
};
static NEXT_TEMP: AtomicU64 = AtomicU64::new(0);

/// Replace one unchanged saved input via a uniquely created sibling temporary file.
/// Reads text/permissions, writes and flushes the temporary, sets permissions, then renames it.
/// Best-effort temporary cleanup follows failure after creation; no fsync is performed.
///
/// # Arguments
/// - `edit` `[in]`: destination, expected source and complete new source; not mutated.
///
/// # Returns
/// `Ok(())` after rename; `Err` leaves the original unless replacement already completed.
///
/// # Errors
/// Read/metadata/create/write/flush/permissions/rename errors, or text changed before writing.
/// A later concurrent change after the comparison is not locked out. Cleanup errors are discarded.
///
/// # Algorithm
/// Reject stale source, create a non-overwriting sibling temp using PID and a counter, write the full output, copy permissions, then rename.
///
/// # Flow
/// ```text
///  [compare original] --changed--> [error]
///          |
///  [create sibling temp]
///          |
///  [write + permissions] --error--> [try remove temp]
///          |                              |
///  [rename over original]             [error]
///          |
///        [()]
/// ```
///
/// # Examples
/// ```text
/// apply(&Edit { path, before: old_text.into(), after: formatted }).await?;
/// ```
pub(super) async fn apply(edit: &Edit) -> io::Result<()> {
    if tokio::fs::read_to_string(&edit.path).await?.as_str() != edit.before.as_ref() {
        return Err(io::Error::other(format!(
            "File changed during formatting: {}",
            edit.path.display()
        )));
    }
    let permissions = tokio::fs::metadata(&edit.path).await?.permissions();
    let name = format!(
        ".cmake-analyzer-format-{}-{}.tmp",
        std::process::id(),
        NEXT_TEMP.fetch_add(1, Ordering::Relaxed)
    );
    let path = edit.path.with_file_name(name);
    let mut file = tokio::fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&path)
        .await?;
    let result = async {
        use tokio::io::AsyncWriteExt;
        file.write_all(edit.after.as_bytes()).await?;
        file.flush().await?;
        drop(file);
        tokio::fs::set_permissions(&path, permissions).await?;
        tokio::fs::rename(&path, &edit.path).await
    }
    .await;
    if result.is_err() {
        let _ = tokio::fs::remove_file(&path).await;
    }
    result
}
