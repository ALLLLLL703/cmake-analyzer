//! Load UTF-8 source for the token inspector. Inline text is memory-only; file
//! and stdin branches perform asynchronous reads, never writes or project execution.
//!
//! Contains AI-generated documentation additions.
use std::io;

use tokio::io::AsyncReadExt;

use crate::args::{self, Args};

/// Choose inline text, a file, or stdin and read it completely as UTF-8.
/// Does not modify files or launch processes. Unlike the other inspectors, `-` is opened as an ordinary file.
///
/// # Arguments
/// - `args` `[in]`: consumed options; color/display fields do not affect reading.
///
/// # Returns
/// `Ok(source)` contains the supplied/read text; `Err` is a read or UTF-8 decoding failure.
///
/// # Errors
/// File/stdin I/O or invalid UTF-8; inline text cannot fail.
///
/// # Examples
/// ```text
/// let source = read_source(Args::parse_from(["inspector", "--text", "set(A 1)"])).await?;
/// ```
pub async fn read_source(args: Args) -> io::Result<String> {
    if let Some(text) = args.text {
        return Ok(text);
    }

    if let Some(path) = args.file {
        return tokio::fs::read_to_string(path).await;
    }

    let mut source = String::new();

    tokio::io::stdin().read_to_string(&mut source).await?;
    Ok(source)
}
