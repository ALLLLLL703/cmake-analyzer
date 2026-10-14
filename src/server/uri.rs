// AI-authored local file URI conversion; no filesystem canonicalization.
use lspally::server::model::{ServerError, ServerResult, Uri};
use std::path::PathBuf;

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
