// AI-authored validation for opt-in configure arguments.
use std::io;
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
