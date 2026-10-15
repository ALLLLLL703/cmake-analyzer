use super::include;

pub(super) fn body(document: &'static str) -> &'static str {
    document
        .rsplit_once("\n\n<!-- Script-generated content.")
        .map_or(document, |(body, _)| body)
}
pub(super) fn command(name: &str) -> Option<&'static str> {
    let name = name.to_ascii_lowercase();
    include::COMMANDS
        .binary_search_by_key(&name.as_str(), |(name, _)| *name)
        .ok()
        .map(|index| body(include::COMMANDS[index].1))
}
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
/// Exact names win; overlapping template pages use the most literal characters.
pub(super) fn variable(name: &str) -> Option<&'static str> {
    if let Ok(index) = include::VARIABLES.binary_search_by_key(&name, |(name, _)| *name) {
        return Some(body(include::VARIABLES[index].1));
    }
    if ["ARGC", "ARGV", "ARGN"].contains(&name)
        || name
            .strip_prefix("ARGV")
            .is_some_and(|tail| !tail.is_empty() && tail.bytes().all(|byte| byte.is_ascii_digit()))
    {
        return command("function");
    }
    if name
        .strip_prefix("CMAKE_ARGV")
        .is_some_and(|tail| !tail.is_empty() && tail.bytes().all(|byte| byte.is_ascii_digit()))
    {
        return variable("CMAKE_ARGV0");
    }
    include::VARIABLES
        .iter()
        .filter(|(pattern, _)| pattern.contains('<') && template(pattern, name))
        .max_by_key(|(pattern, _)| {
            pattern
                .split('<')
                .map(|part| {
                    part.split_once('>')
                        .map_or(part, |(_, literal)| literal)
                        .len()
                })
                .sum::<usize>()
        })
        .map(|(_, doc)| body(doc))
}
