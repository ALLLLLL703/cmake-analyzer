use std::path::PathBuf;
pub(super) struct Request {
    pub names: Vec<String>,
    pub hints: Vec<PathBuf>,
    pub suffixes: Vec<PathBuf>,
    pub filenames: Vec<String>,
    pub config_only: bool,
    pub module_only: bool,
    pub defaults: bool,
    pub package_roots: bool,
    pub cmake_paths: bool,
    pub environment: bool,
    pub system_paths: bool,
}
