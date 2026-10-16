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
pub(super) fn numeric_tail(name: &str, prefix: &str) -> bool {
    name.strip_prefix(prefix)
        .is_some_and(|tail| !tail.is_empty() && tail.bytes().all(|byte| byte.is_ascii_digit()))
}
/// Exact local names win; overlapping templates use the most literal characters.
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
