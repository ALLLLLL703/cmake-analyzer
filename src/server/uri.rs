//! Translate normalized absolute local paths and file URIs without disk access.
//! Percent encoding preserves Unix filename bytes. Decoding accepts only the local
//! file scheme with no query/fragment, rejects NUL, and normalizes lexical path parts;
//! it never canonicalizes symlinks.
// AI-authored local file URI conversion; no filesystem canonicalization.
use lspally::server::model::{ServerError, ServerResult, Uri};
use std::path::{Path, PathBuf};

/// Percent-encode local filename bytes into a file URI without inspecting disk.
/// Assumes a normalized absolute path from the caller; does not normalize or canonicalize it.
///
/// # Arguments
/// - `path` `[in]`: Borrowed normalized absolute path. Unix encoded filename bytes are preserved.
///
/// # Returns
/// A file URI with uppercase percent escapes, path separators preserved, and Windows backslashes converted to slashes.
///
/// # Examples
/// ```text
/// let uri = from_path(std::path::Path::new("/tmp/hello world.cmake"));
/// assert_eq!(uri.0, "file:///tmp/hello%20world.cmake");
/// ```
pub(super) fn from_path(path: &Path) -> Uri {
    const HEX: &[u8; 16] = b"0123456789ABCDEF";
    let bytes = path.as_os_str().as_encoded_bytes();
    let mut uri = String::from("file://");
    if !bytes.starts_with(b"/") {
        uri.push('/');
    }
    for &byte in bytes {
        if cfg!(windows) && byte == b'\\' {
            uri.push('/');
        } else if byte.is_ascii_alphanumeric() || matches!(byte, b'/' | b'-' | b'.' | b'_' | b'~') {
            uri.push(char::from(byte));
        } else {
            uri.push('%');
            uri.push(char::from(HEX[(byte >> 4) as usize]));
            uri.push(char::from(HEX[(byte & 15) as usize]));
        }
    }
    Uri(uri)
}

/// Decode a strictly local file URI into a normalized absolute path without filesystem canonicalization.
/// No target-file access or path existence check occurs.
///
/// # Arguments
/// - `uri` `[in]`: Borrowed URI; only an empty or localhost authority is accepted.
///
/// # Returns
/// `Ok(PathBuf)` with decoded normalized absolute path; `Err` for unsupported/malformed URI or invalid path bytes.
///
/// # Errors
/// URI parse errors, wrong scheme/remote authority/query/fragment, decoded NUL, nonabsolute path, non-UTF-8 bytes on non-Unix systems, or normalization errors.
///
/// # Algorithm
/// Parse components, enforce local-file restrictions, percent-decode bytes, reject NUL, construct a platform path and require it to be absolute before lexical normalization. Unix preserves raw bytes; the non-Unix branch requires UTF-8 and removes leading slashes before path construction.
///
/// # Flow
/// ```text
///  [URI components]
///          |
///  <Local file + no query/fragment?> -- no --> [Error]
///          | yes
///  [Decode bytes + reject NUL]
///          |
///       <Platform?>
///        /       \
///      Unix     other
///       |         |
///  [Raw bytes] [UTF-8 + trim leading /]
///       +---------+
///            v
///  [Require absolute + normalize]
/// ```
///
/// # Examples
/// ```text
/// let path = to_path(&Uri("file:///tmp/hello%20world.cmake".into()))?;
/// assert_eq!(path, std::path::PathBuf::from("/tmp/hello world.cmake"));
/// ```
pub(super) fn to_path(uri: &Uri) -> ServerResult<PathBuf> {
    let parsed =
        fluent_uri::Uri::parse(uri.0.as_str()).map_err(|e| ServerError::invalid(e.to_string()))?;
    if !parsed
        .scheme()
        .is_some_and(|s| s.as_str().eq_ignore_ascii_case("file"))
        || parsed.query().is_some()
        || parsed.fragment().is_some()
        || parsed.authority().is_some_and(|a| {
            !a.as_str().is_empty() && !a.as_str().eq_ignore_ascii_case("localhost")
        })
    {
        return Err(ServerError::invalid("Only local file URIs are supported"));
    }
    let bytes = parsed.path().as_estr().decode().into_bytes();
    if bytes.contains(&0) {
        return Err(ServerError::invalid("File URI contains NUL"));
    }
    #[cfg(unix)]
    let path = {
        use std::os::unix::ffi::OsStringExt;
        PathBuf::from(std::ffi::OsString::from_vec(bytes.into_owned()))
    };
    #[cfg(not(unix))]
    let path = {
        let value = std::str::from_utf8(&bytes).map_err(|e| ServerError::invalid(e.to_string()))?;
        PathBuf::from(value.trim_start_matches('/'))
    };
    if !path.is_absolute() {
        return Err(ServerError::invalid(
            "File URI must contain an absolute path",
        ));
    }
    super::roots::normalize(&path).map_err(|e| ServerError::invalid(e.to_string()))
}
