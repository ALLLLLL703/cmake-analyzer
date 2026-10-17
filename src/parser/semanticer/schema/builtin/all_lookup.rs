use super::*;
mod control;
mod find;
mod property;

pub(super) fn schemas(version: (u32, u32)) -> Vec<(&'static str, Vec<ArgSchemaKind>)> {
    let mut result = vec![
        ("project", super::project::args(version)),
        ("find_package", super::find_package::args(version)),
    ];
    result.extend(find::schemas(version));
    result.extend(property::schemas(version));
    result.extend(control::schemas(version));
    result
}
#[cfg(test)]
#[path = "../../../../../tests/unit/parser/semanticer/keyword_lookup.rs"]
mod tests;
