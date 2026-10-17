//! Explicit process execution and expanded trace observations, separate from static parsing.
//! [`execute`] (also exported as [`configure`]) runs project or script code with filesystem/
//! subprocess effects; importing the module and constructing [`model`] records do not.
//!
//! Contains AI-generated documentation additions.
// AI-authored explicit CMake execution; never invoked by ordinary parsing.
pub mod model;
mod process;
mod trace;
pub use process::execute;
pub use process::execute as configure;
