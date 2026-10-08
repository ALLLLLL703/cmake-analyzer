use super::args::Args;
use std::{io, path::Path};
use tokio::io::AsyncReadExt;

pub(crate) async fn read_source(args: Args) -> io::Result<String> {
    if let Some(text) = args.text {
        return Ok(text);
    }
    if let Some(path) = args.file {
        if path != Path::new("-") {
            return tokio::fs::read_to_string(path).await;
        }
    }
    let mut source = String::new();
    tokio::io::stdin().read_to_string(&mut source).await?;
    Ok(source)
}
