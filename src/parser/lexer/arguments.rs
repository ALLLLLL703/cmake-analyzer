/// Recognize `[=*[` and return the content start and delimiter length.
pub(super) fn bracket_open(bytes: &[u8], start: usize) -> Option<(usize, usize)> {
    if bytes.get(start) != Some(&b'[') {
        return None;
    }
    let mut end = start + 1;
    while bytes.get(end) == Some(&b'=') {
        end += 1;
    }
    (bytes.get(end) == Some(&b'[')).then_some((end + 1, end - start - 1))
}

/// Each candidate run of `=` is consumed once, even for very long delimiters.
pub(super) fn bracket_end(bytes: &[u8], mut offset: usize, equals: usize) -> Option<usize> {
    while offset < bytes.len() {
        if bytes[offset] != b']' {
            offset += 1;
            continue;
        }
        let start = offset;
        offset += 1;
        while bytes.get(offset) == Some(&b'=') {
            offset += 1;
        }
        if offset - start - 1 == equals && bytes.get(offset) == Some(&b']') {
            return Some(offset + 1);
        }
        // The next `]` may itself start a closing delimiter.
    }
    None
}

pub(super) fn quoted_end(bytes: &[u8], start: usize) -> Option<usize> {
    let mut offset = start + 1;
    while offset < bytes.len() {
        match bytes[offset] {
            b'"' => return Some(offset + 1),
            b'\\' => {
                offset += 1;
                if bytes.get(offset) == Some(&b'\r') && bytes.get(offset + 1) == Some(&b'\n') {
                    offset += 2;
                } else if offset < bytes.len() {
                    offset += 1;
                }
            }
            _ => offset += 1,
        }
    }
    None
}

fn make_variable_end(bytes: &[u8], start: usize) -> Option<usize> {
    if bytes.get(start..start + 2) != Some(b"$(") {
        return None;
    }
    let mut offset = start + 2;
    while bytes
        .get(offset)
        .is_some_and(|byte| byte.is_ascii_alphanumeric() || *byte == b'_')
    {
        offset += 1;
    }
    (bytes.get(offset) == Some(&b')')).then_some(offset + 1)
}

fn escaped_end(text: &str, offset: usize) -> Option<usize> {
    let next = offset + 1;
    let ch = text.get(next..)?.chars().next()?;
    // Unquoted arguments do not support backslash-newline continuation.
    if ch == '\0' || ch == '\n' || (ch == '\r' && text.as_bytes().get(next + 1) == Some(&b'\n')) {
        None
    } else {
        Some(next + ch.len_utf8())
    }
}

/// Embedded legacy quotes are literal, balanced and restricted to one line.
fn legacy_quote_end(text: &str, start: usize) -> Option<usize> {
    let bytes = text.as_bytes();
    let mut offset = start + 1;
    while offset < bytes.len() {
        match bytes[offset] {
            b'"' => return Some(offset + 1),
            b'\0' | b'\r' | b'\n' | b'#' | b'(' | b')' => return None,
            b'\\' => offset = escaped_end(text, offset)?,
            b'$' => offset = make_variable_end(bytes, offset).unwrap_or(offset + 1),
            _ => offset += 1,
        }
    }
    None
}

pub(super) fn unquoted_end(text: &str, start: usize) -> usize {
    let bytes = text.as_bytes();
    let mut offset = start;
    while offset < bytes.len() {
        match bytes[offset] {
            b'\0' | b' ' | b'\t' | b'\r' | b'\n' | b'(' | b')' | b'#' => break,
            b'"' => match legacy_quote_end(text, offset) {
                Some(end) if offset > start => offset = end,
                _ => break,
            },
            b'\\' => match escaped_end(text, offset) {
                Some(end) => offset = end,
                None => break,
            },
            b'$' => offset = make_variable_end(bytes, offset).unwrap_or(offset + 1),
            _ => offset += 1,
        }
    }
    offset
}
