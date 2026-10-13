// AI-generated implementation.
use super::model::{Argument, ProjectDeclaration};
const KEYS: &[&str] = &[
    "VERSION",
    "DESCRIPTION",
    "HOMEPAGE_URL",
    "LANGUAGES",
    "COMPAT_VERSION",
    "SPDX_LICENSE",
];
/// return the ProjectDeclaration
/// parse the project() command
///
/// if **no** language set, default is C & CXX
/// 676767676767
pub(super) fn project(name: Argument, arguments: Vec<Argument>) -> ProjectDeclaration {
    let mut result = ProjectDeclaration {
        name,
        version: None,
        description: None,
        homepage: None,
        languages: Vec::new(),
        arguments,
    };
    let mut index = 0;
    // Named options and legacy positional languages use different layouts.
    let modern = result
        .arguments
        .iter()
        .any(|arg| arg.literal && KEYS.contains(&arg.text.as_str()));
    let mut explicit_languages = false;
    while index < result.arguments.len() {
        let arg = &result.arguments[index];
        if modern && arg.literal && arg.text == "LANGUAGES" {
            explicit_languages = true;
            index += 1;
            while let Some(arg) = result
                .arguments
                .get(index)
                .filter(|arg| !arg.literal || !KEYS.contains(&arg.text.as_str()))
            {
                if !arg.literal || arg.text != "NONE" {
                    result.languages.push(arg.clone());
                }
                index += 1;
            }
        } else if modern && arg.literal && KEYS.contains(&arg.text.as_str()) {
            if let Some(value) = result.arguments.get(index + 1) {
                match arg.text.as_str() {
                    "VERSION" => result.version = Some(value.clone()),
                    "DESCRIPTION" => result.description = Some(value.clone()),
                    "HOMEPAGE_URL" => result.homepage = Some(value.clone()),
                    _ => {}
                }
            }
            // Consume the option value even when it spells another keyword.
            index += 2;
        } else {
            if !modern {
                explicit_languages = true;
                if !arg.literal || arg.text != "NONE" {
                    result.languages.push(arg.clone());
                }
            }
            index += 1;
        }
    }
    // Explicit NONE stays empty; only an omitted language list gets defaults.
    if !explicit_languages {
        result.languages = ["C", "CXX"]
            .into_iter()
            .map(|text| Argument {
                text: text.into(),
                literal: true,
            })
            .collect();
    }
    result
}
