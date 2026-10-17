//! Parse declared CMake versions or query the installed executable as a fallback.
//!
//! Only `cmake --version` may run; project configure and scripts are never invoked here.
// Contains AI-generated documentation additions.
// AI-generated implementation.
use crate::parser::{
    nparser::model::NParsedNode,
    semanticer::model::{CMakeVersion, Semanticer},
};

impl CMakeVersion {
    /// Parse two or three unsigned numeric version components. No side effects.
    ///
    /// # Arguments
    /// - `text` `[in]`: dotted major.minor or major.minor.patch spelling.
    ///
    /// # Returns
    /// Ok(version) with patch zero when omitted; Err(message) for invalid syntax.
    ///
    /// # Errors
    /// Missing, nonnumeric, overflowing or extra components return an invalid-version message.
    ///
    /// # Examples
    /// ```
    /// use cmake_analyzer::parser::semanticer::model::*;
    /// let version = CMakeVersion::parse("3.31").unwrap();
    /// assert_eq!((version.major, version.minor, version.patch), (3, 31, 0));
    /// assert!(CMakeVersion::parse("3").is_err());
    /// ```
    pub fn parse(text: &str) -> Result<Self, String> {
        let mut parts = text.split('.');
        let major = parts
            .next()
            .and_then(|s| s.parse().ok())
            .ok_or_else(|| format!("Invalid CMake version: {text}"))?;
        let minor = parts
            .next()
            .and_then(|s| s.parse().ok())
            .ok_or_else(|| format!("Invalid CMake version: {text}"))?;
        let patch = match parts.next() {
            Some(s) => s
                .parse()
                .map_err(|_| format!("Invalid CMake version: {text}"))?,
            None => 0,
        };
        if parts.next().is_some() {
            return Err(format!("Invalid CMake version: {text}"));
        }
        Ok(Self {
            major,
            minor,
            patch,
        })
    }
}

impl Semanticer {
    /// Read the first declared VERSION pair, or asynchronously query `cmake --version` on PATH. Never runs configure, project commands or scripts.
    ///
    /// # Arguments
    /// - `self` `[in]`: analyzer whose parser nodes are scanned without mutation.
    /// - `source` `[in]`: source matching parser-node spans.
    ///
    /// # Returns
    /// Ok(minimum version) for a valid declaration/range, otherwise the detected host version; Err(message) on invalid declaration or host-query failure.
    ///
    /// # Errors
    /// Invalid/reversed declared ranges, mismatched version spans, process launch/nonzero status, non-UTF-8 output or an absent/unparseable version line return messages.
    ///
    /// # Algorithm
    /// Scan command nodes in order for cmake_minimum_required and its VERSION pair. Validate both ends of a range, returning the minimum. Only if no pair is found, run a kill-on-drop version query and parse its stdout.
    ///
    /// # Flow
    /// ```text
    ///  [parser nodes]           [source spans]
    ///        |                       |
    ///        +----------+------------+
    ///                   v
    ///           <VERSION pair?>
    ///              /        \
    ///            yes        no
    ///             |          |
    ///       [parse/range] [cmake --version]
    ///             |          |
    ///             +----+-----+
    ///                  v
    ///         [version or error]
    /// ```
    ///
    /// # Examples
    /// ```no_run
    /// use cmake_analyzer::parser::semanticer::model::*;
    /// use cmake_analyzer::parser::nparser::model::NParsedBuffer;
    /// # async fn example() -> Result<(), String> {
    /// let analyzer = Semanticer::new(NParsedBuffer::default(), Default::default());
    /// let version = analyzer.scan_version("").await?; // Requires cmake on PATH.
    /// # Ok(())
    /// # }
    /// ```
    pub async fn scan_version(&self, source: &str) -> Result<CMakeVersion, String> {
        for node in &self.buffer.nodes {
            let NParsedNode::Command(command) = &node.content else {
                continue;
            };
            if !command
                .name
                .span
                .text(source)
                .is_some_and(|name| name.eq_ignore_ascii_case("cmake_minimum_required"))
            {
                continue;
            }
            for pair in command.args.windows(2) {
                if pair[0].text(source) != Some("VERSION") {
                    continue;
                }
                let text = pair[1]
                    .text(source)
                    .ok_or("CMake version span does not match source")?;
                if let Some((minimum, maximum)) = text.split_once("...") {
                    let minimum = CMakeVersion::parse(minimum)?;
                    let maximum = CMakeVersion::parse(maximum)?;
                    if (maximum.major, maximum.minor, maximum.patch)
                        < (minimum.major, minimum.minor, minimum.patch)
                    {
                        return Err(format!("Reversed CMake version range: {text}"));
                    }
                    return Ok(minimum);
                }
                return CMakeVersion::parse(text);
            }
        }
        let output = tokio::process::Command::new("cmake")
            .arg("--version")
            .kill_on_drop(true)
            .output()
            .await
            .map_err(|e| format!("Cannot detect CMake version: {e}"))?;
        if !output.status.success() {
            return Err(format!(
                "cmake --version failed: {}",
                String::from_utf8_lossy(&output.stderr).trim()
            ));
        }
        let text = std::str::from_utf8(&output.stdout)
            .map_err(|e| format!("Invalid cmake --version output: {e}"))?;
        let version = text
            .lines()
            .find_map(|line| line.strip_prefix("cmake version "))
            .and_then(|line| line.split_whitespace().next())
            .ok_or("Missing version in cmake --version output")?;
        CMakeVersion::parse(version.split('-').next().unwrap_or(version))
    }
}
