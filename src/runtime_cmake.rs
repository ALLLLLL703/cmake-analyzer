// AI-authored explicit CMake execution; never invoked by ordinary parsing.
pub mod model;
mod process;
mod trace;
pub use process::configure;
