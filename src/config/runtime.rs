// AI-authored validation for opt-in CMake arguments.
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
