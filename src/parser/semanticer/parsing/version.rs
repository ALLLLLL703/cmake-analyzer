// AI-generated implementation.
use crate::parser::{
    nparser::model::NParsedNode,
    semanticer::model::{CMakeVersion, Semanticer},
};

impl CMakeVersion {
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
