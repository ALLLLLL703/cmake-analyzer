use super::model::Edit;
use std::{
    io,
    sync::atomic::{AtomicU64, Ordering},
};
static NEXT_TEMP: AtomicU64 = AtomicU64::new(0);

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
