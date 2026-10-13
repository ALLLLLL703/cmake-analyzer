// AI-generated implementation.
use super::args::Args;
use cmake_analyzer::workspace::Workspace;
use std::{
    io,
    path::{Path, PathBuf},
};
use tokio::io::AsyncReadExt;

pub(crate) async fn load_workspace(args: Args) -> io::Result<Workspace> {
    let file = args.file.as_deref().filter(|p| *p != Path::new("-"));
    // Stdin/text needs a real path base for relative includes and subdirectories.
    let root = file.unwrap_or(Path::new("CMakeLists.txt"));
    let source = if let Some(text) = args.text {
        text
    } else if let Some(file) = file {
        tokio::fs::read_to_string(file).await?
    } else {
        let mut text = String::new();
        tokio::io::stdin().read_to_string(&mut text).await?;
        text
    };
    let mut workspace = Workspace::new(root, args.binary_directory)?;
    workspace.set_overlay(root, source)?;
    let cwd = std::env::current_dir()?;
    let paths: Vec<PathBuf> = args
        .module_path
        .into_iter()
        .map(|path| cwd.join(path))
        .collect();
    workspace.set_module_paths(paths);
    workspace.set_builtin_module_directory(args.builtin_module_directory)?;
    workspace.flush().await?;
    Ok(workspace)
}
