//! Pure allow-list validation of extra CMake arguments. These helpers do not execute,
//! resolve paths or grant execution authority; the server checks script arguments again
//! at admission, while configuration loading accepts the configure superset.
//!
//! Contains AI-generated documentation additions.
// AI-authored validation for opt-in CMake arguments.
/// Reject unsupported extra arguments before process admission. Pure validation: no I/O,
/// mutation or process launch. Only -D/-U with separate or joined values, joined -W options and --no-warn-unused-cli are allowed.
///
/// # Arguments
/// - `arguments` `[in]`: direct child argument strings, not shell syntax; not modified.
///
/// # Returns
/// `Ok(())` if every option is supported with any required next value; `Err` on the first unsupported/missing option value.
/// Separate values are consumed as supplied without interpreting their contents.
///
/// # Errors
/// InvalidData for an unsupported option or a missing separate value; this does not validate CMake
/// semantics of a value or grant execution authority.
///
/// # Algorithm
/// Walk options once. Consume one following string for separate-value flags; accept
/// known standalone/joined forms. Stop on a missing value or unsupported option.
///
/// # Flow
/// ```text
///  [next argument] <------------------------------+
///         |             none --> [Ok(())]         |
///         | present                               |
///  <needs separate value?>                        |
///      /             \                            |
/// [consume value] [test joined/standalone]         |
///      \             /                            |
///       <valid?> --yes-----------------------------+
///          | no
///        [Err]
/// ```
///
/// # Examples
/// ```text
/// validate_script_arguments(&["-DINPUT=file".to_owned()])?;
/// ```
pub(crate) fn validate_script_arguments(arguments: &[String]) -> std::io::Result<()> {
    let mut arguments = arguments.iter();
    while let Some(argument) = arguments.next() {
        if matches!(argument.as_str(), "-D" | "-U") {
            if arguments.next().is_none() {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    "Missing script CMake option value",
                ));
            }
        } else if argument == "--no-warn-unused-cli"
            || ["-D", "-U", "-W"]
                .iter()
                .any(|prefix| argument.starts_with(prefix) && argument.len() > prefix.len())
        {
        } else {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Runtime script arguments accept only -D, -U, -W and --no-warn-unused-cli",
            ));
        }
    }
    Ok(())
}
use std::io;
/// Reject unsupported extra arguments before process admission. Pure validation: no I/O,
/// mutation or process launch. Allow -D/-U/-G/-A/-T and --toolchain with separate values, their joined forms, joined -W, --fresh and --no-warn-unused-cli.
///
/// # Arguments
/// - `arguments` `[in]`: direct child argument strings, not shell syntax; not modified.
///
/// # Returns
/// `Ok(())` if every option is supported with any required next value; `Err` on the first unsupported/missing option value.
/// Separate values are consumed as supplied without interpreting their contents.
///
/// # Errors
/// InvalidData for an unsupported option or a missing separate value; this does not validate CMake
/// semantics of a value or grant execution authority.
///
/// # Algorithm
/// Walk options once. Consume one following string for separate-value flags; accept
/// known standalone/joined forms. Stop on a missing value or unsupported option.
///
/// # Flow
/// ```text
///  [next argument] <------------------------------+
///         |             none --> [Ok(())]         |
///         | present                               |
///  <needs separate value?>                        |
///      /             \                            |
/// [consume value] [test joined/standalone]         |
///      \             /                            |
///       <valid?> --yes-----------------------------+
///          | no
///        [Err]
/// ```
///
/// # Examples
/// ```text
/// validate_arguments(&["-G".to_owned(), "Ninja".to_owned()])?;
/// ```
pub(super) fn validate_arguments(arguments: &[String]) -> io::Result<()> {
    let mut arguments = arguments.iter();
    while let Some(argument) = arguments.next() {
        if matches!(
            argument.as_str(),
            "-D" | "-U" | "-G" | "-A" | "-T" | "--toolchain"
        ) {
            if arguments.next().is_none() {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "Missing runtime CMake option value",
                ));
            }
        } else if argument == "--fresh"
            || argument == "--no-warn-unused-cli"
            || ["-D", "-U", "-G", "-A", "-T", "-W", "--toolchain="]
                .iter()
                .any(|prefix| argument.starts_with(prefix) && argument.len() > prefix.len())
        {
        } else {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!(
                    "Unsupported runtime CMake argument {argument:?}; source, build, execution mode, and trace flags are controlled by the server"
                ),
            ));
        }
    }
    Ok(())
}
