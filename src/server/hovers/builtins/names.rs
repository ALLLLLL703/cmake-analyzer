//! Match exact or placeholder catalog names in memory; no filesystem access or process execution.

/// Match a catalog placeholder pattern against a complete name without side effects.
/// Each <...> segment consumes a nonempty substring; literal portions retain case and must match.
///
/// # Arguments
/// - `pattern` `[in]`: catalog name, possibly containing one or more angle-bracket placeholders.
/// - `name` `[in]`: candidate source spelling.
///
/// # Returns
/// True for an exact full match, false for missing delimiters, empty placeholder substitutions or
/// incompatible literals.
///
/// # Algorithm
/// Split the first placeholder, require its literal prefix, then recurse at matching literal anchors (or
/// successive character boundaries for adjacent placeholders). A final literal suffix requires at least
/// one substituted byte.
///
/// # Flow
/// ```text
///  [Pattern has placeholder?] --no--> [Exact comparison]
///              | yes
///        [Prefix matches?] --no--> [false]
///              | yes
///        [Remaining tail]
///           +--------+
///  [Final suffix]  [Next placeholder]
///        |              |
///  [Check nonempty] [Try anchors recursively]
///        +--------------+
///               v
///         [Match result]
/// ```
///
/// # Examples
/// Local server context:
/// ```text
/// assert!(template("CMAKE_<LANG>_FLAGS", "CMAKE_C_FLAGS"));
/// ```
fn template(pattern: &str, name: &str) -> bool {
    let Some((prefix, rest)) = pattern.split_once('<') else {
        return pattern == name;
    };
    let Some((_, tail)) = rest.split_once('>') else {
        return false;
    };
    let Some(name) = name.strip_prefix(prefix) else {
        return false;
    };
    if tail.is_empty() {
        return !name.is_empty();
    }
    if !tail.contains('<') {
        return name.len() > tail.len() && name.ends_with(tail);
    }
    let anchor = tail.split('<').next().unwrap_or_default();
    if anchor.is_empty() {
        return name
            .char_indices()
            .skip(1)
            .any(|(offset, _)| template(tail, &name[offset..]));
    }
    name.match_indices(anchor)
        .any(|(offset, _)| offset > 0 && template(tail, &name[offset..]))
}
/// Recognize an ASCII-digit suffix after a prefix without side effects.
///
/// # Arguments
/// - `name` `[in]`: full identifier.
/// - `prefix` `[in]`: required literal prefix.
///
/// # Returns
/// True only for a nonempty suffix made entirely of ASCII digits.
///
/// # Examples
/// Local server context:
/// ```text
/// assert!(numeric_tail("ARGV12", "ARGV"));
/// assert!(!numeric_tail("ARGV", "ARGV"));
/// ```
pub(super) fn numeric_tail(name: &str, prefix: &str) -> bool {
    name.strip_prefix(prefix)
        .is_some_and(|tail| !tail.is_empty() && tail.bytes().all(|byte| byte.is_ascii_digit()))
}
/// Exact local names win; overlapping templates use the most literal characters.
/// Choose a canonical variable catalog spelling without side effects.
/// Exact names win over templates; matching templates compete by total literal-byte count.
///
/// # Arguments
/// - `catalog` `[in]`: sorted local catalog used for binary-search exact matches and template scanning.
/// - `name` `[in]`: case-sensitive variable spelling.
///
/// # Returns
/// Some borrowed catalog spelling for an exact or selected template match; None if no match. Equal
/// specificity follows Iterator::max_by_key (the last equally ranked match).
///
/// # Algorithm
/// Binary search the sorted names first. If absent, test placeholder patterns and choose the one with
/// the most literal bytes.
///
/// # Flow
/// ```text
///  [Source name]
///       |
///  [Exact catalog hit?] --yes--> [Return exact name]
///       | no
///  [Scan matching templates]
///       |
///  [Most literal bytes] ---> [Canonical template / None]
/// ```
///
/// # Examples
/// Local server context:
/// ```text
/// let catalog = vec!["CMAKE_<LANG>_FLAGS".to_owned()];
/// assert_eq!(variable(&catalog, "CMAKE_C_FLAGS"), Some("CMAKE_<LANG>_FLAGS"));
/// ```
pub(super) fn variable<'a>(catalog: &'a [String], name: &str) -> Option<&'a str> {
    if let Ok(index) = catalog.binary_search_by(|candidate| candidate.as_str().cmp(name)) {
        return Some(&catalog[index]);
    }
    catalog
        .iter()
        .filter(|pattern| pattern.contains('<') && template(pattern, name))
        .max_by_key(|pattern| {
            pattern
                .split('<')
                .map(|part| {
                    part.split_once('>')
                        .map_or(part, |(_, literal)| literal)
                        .len()
                })
                .sum::<usize>()
        })
        .map(String::as_str)
}
