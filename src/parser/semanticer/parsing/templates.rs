// AI-generated implementation.
use crate::parser::semanticer::{
    model::{SemanticVariable, SemanticedBuffer, Semanticer},
    schema::CommandSchemaFrom,
};
use std::collections::{BTreeMap, BTreeSet};

impl Semanticer {
    /// Return fully specialized builtin names; values/scopes remain unbound.
    /// Unknown substitutions are omitted; no CMake code is executed.(static analyze)
    pub fn detect_specialized_variables(&self, buffer: &SemanticedBuffer) -> Vec<SemanticVariable> {
        let version = buffer
            .version
            .map(|v| (v.major, v.minor))
            .filter(|(major, minor)| matches!((*major, *minor), (3, 0..=31) | (4, 0..=4)))
            .unwrap_or((3, 0));
        let templates = crate::parser::semanticer::variables::builtin::template_names_for_version(
            version.0, version.1,
        );
        let mut domains: BTreeMap<&str, BTreeSet<String>> = BTreeMap::new();
        let mut explicit = BTreeSet::new();
        let mut definitions = BTreeSet::new();
        for command in &buffer.commands {
            if command.closed
                && command.schema.as_ref().is_some_and(|s| {
                    matches!(
                        s.from,
                        CommandSchemaFrom::KeyWord | CommandSchemaFrom::Builtin
                    )
                })
                && matches!(
                    command.name.to_ascii_lowercase().as_str(),
                    "function" | "macro"
                )
            {
                if let Some(name) = command.args.first().and_then(|a| a.literal_text()) {
                    definitions.insert(name.to_ascii_lowercase());
                }
            }
        }
        for command in &buffer.commands {
            if !command.closed {
                continue;
            }
            let name = command.name.to_ascii_lowercase();
            let args: Vec<_> = command.args.iter().map(|a| a.literal_text()).collect();
            if definitions.contains(&name) && args.iter().all(Option::is_some) {
                for index in 0..args.len() {
                    explicit.insert(format!("ARGV{index}"));
                }
            }
            if !command.schema.as_ref().is_some_and(|s| {
                matches!(
                    s.from,
                    CommandSchemaFrom::Builtin | CommandSchemaFrom::KeyWord
                )
            }) {
                continue;
            }
            let first = args.first().copied().flatten();
            match name.as_str() {
                "project" => {
                    if args.is_empty() {
                        continue;
                    }
                    if let Some(project) = first {
                        domains
                            .entry("<PROJECT-NAME>")
                            .or_default()
                            .insert(project.to_owned());
                    }
                    let languages = project_languages(&args);
                    domains.entry("<LANG>").or_default().extend(
                        languages
                            .into_iter()
                            .flat_map(|language| language.split(';'))
                            .filter(|language| !language.is_empty())
                            .map(str::to_owned),
                    );
                }
                "enable_language" => {
                    domains.entry("<LANG>").or_default().extend(
                        args.iter()
                            .copied()
                            .flatten()
                            .filter(|s| *s != "OPTIONAL" && *s != "NONE")
                            .flat_map(|language| language.split(';'))
                            .filter(|language| !language.is_empty())
                            .map(str::to_owned),
                    );
                }
                "find_package" => {
                    if let Some(package) = first {
                        domains
                            .entry("<PackageName>")
                            .or_default()
                            .insert(package.to_owned());
                    }
                }
                "set" | "option" | "unset" => {
                    if let Some(variable) = first {
                        explicit.insert(variable.to_owned());
                        let key = match variable {
                            "CMAKE_BUILD_TYPE" | "CMAKE_CONFIGURATION_TYPES" => Some("<CONFIG>"),
                            "CMAKE_LINKER_TYPE" => Some("<TYPE>"),
                            _ => None,
                        };
                        if let Some(key) = key.filter(|_| name == "set") {
                            domains.entry(key).or_default().extend(
                                args.iter()
                                    .skip(1)
                                    .copied()
                                    .flatten()
                                    .take_while(|s| !matches!(*s, "CACHE" | "PARENT_SCOPE"))
                                    .flat_map(|s| s.split(';'))
                                    .filter(|s| !s.is_empty())
                                    .map(|s| {
                                        if key == "<CONFIG>" {
                                            s.to_ascii_uppercase()
                                        } else {
                                            s.to_owned()
                                        }
                                    }),
                            );
                        }
                    }
                }
                "cmake_policy" => {
                    for policy in args
                        .iter()
                        .copied()
                        .flatten()
                        .filter_map(|s| s.strip_prefix("CMP"))
                    {
                        if policy.len() == 4 && policy.bytes().all(|b| b.is_ascii_digit()) {
                            domains
                                .entry("<NNNN>")
                                .or_default()
                                .insert(policy.to_owned());
                        }
                    }
                }
                "string" | "if" | "elseif" => {
                    if args.iter().any(|a| matches!(a, Some("REGEX" | "MATCHES"))) {
                        for n in 0..=9 {
                            explicit.insert(format!("CMAKE_MATCH_{n}"));
                        }
                    }
                }
                _ => {}
            }
            for pair in args.windows(2) {
                if pair[0] == Some("LINKER_TYPE") {
                    if let Some(kind) = pair[1] {
                        domains.entry("<TYPE>").or_default().insert(kind.to_owned());
                    }
                }
            }
            for arg in args.iter().copied().flatten() {
                if let Some(attribute) = arg.strip_prefix("XCODE_ATTRIBUTE_") {
                    explicit.insert(format!("CMAKE_XCODE_ATTRIBUTE_{attribute}"));
                }
            }
            // Generator expressions are not literal values, but a literal feature name is extractable.
            if name == "target_link_libraries" {
                for arg in &command.args {
                    for prefix in ["$<LINK_LIBRARY:", "$<LINK_GROUP:"] {
                        if let Some(rest) = arg.content.strip_prefix(prefix) {
                            if let Some((feature, _)) = rest.split_once(',') {
                                if !feature.is_empty() && !feature.contains(['$', '<', '>', ';']) {
                                    domains
                                        .entry("<FEATURE>")
                                        .or_default()
                                        .insert(feature.to_owned());
                                }
                            }
                        }
                    }
                }
            }
        }
        // Explicit writes can name a template instance without declaring its domain.
        let mut names = BTreeSet::new();
        for template in templates {
            for candidate in &explicit {
                if matches_template(template, candidate) {
                    names.insert(candidate.clone());
                }
            }
            let mut variants = vec![template.to_owned()];
            // Expand one placeholder per round; a missing domain drops that branch.
            while variants.iter().any(|s| s.contains('<')) {
                let mut next = Vec::new();
                for variant in variants {
                    let Some(start) = variant.find('<') else {
                        next.push(variant);
                        continue;
                    };
                    let Some(end) = variant[start..].find('>').map(|n| n + start + 1) else {
                        continue;
                    };
                    if let Some(values) = domains.get(&variant[start..end]) {
                        for value in values {
                            // A placeholder-valued replacement could repeat forever.
                            if value.is_empty() || value.contains(['<', '>']) {
                                continue;
                            }
                            next.push(format!("{}{value}{}", &variant[..start], &variant[end..]));
                        }
                    }
                }
                variants = next;
            }
            names.extend(variants.into_iter().filter(|s| !s.contains(['<', '>'])));
        }
        names
            .into_iter()
            .map(|name| SemanticVariable {
                name,
                builtin: true,
                first_write: None,
                scope: None,
                changing: Vec::new(),
                references: Vec::new(),
            })
            .collect()
    }
}

// Skip option/value pairs: DESCRIPTION LANGUAGES is not a language-list trigger.
fn project_languages<'a>(args: &[Option<&'a str>]) -> Vec<&'a str> {
    let keys = [
        "VERSION",
        "DESCRIPTION",
        "HOMEPAGE_URL",
        "COMPAT_VERSION",
        "SPDX_LICENSE",
        "LANGUAGES",
    ];
    let mut index = 1;
    let mut modern = false;
    let mut languages = None;
    while index < args.len() {
        match args[index] {
            Some("LANGUAGES") => {
                modern = true;
                index += 1;
                let mut values = Vec::new();
                while index < args.len() && !args[index].is_some_and(|value| keys.contains(&value))
                {
                    if let Some(value) = args[index] {
                        values.extend(
                            value
                                .split(';')
                                .filter(|value| !value.is_empty() && *value != "NONE"),
                        );
                    }
                    index += 1;
                }
                languages = Some(values);
            }
            Some(key) if keys.contains(&key) => {
                modern = true;
                index += 2;
            }
            _ => index += 1,
        }
    }
    languages.unwrap_or_else(|| {
        if modern || args.len() == 1 {
            vec!["C", "CXX"]
        } else {
            args.iter()
                .skip(1)
                .copied()
                .flatten()
                .filter(|value| *value != "NONE")
                .collect()
        }
    })
}

// Try UTF-8 boundaries; numeric placeholders must not match arbitrary name fragments.
fn matches_template(template: &str, candidate: &str) -> bool {
    let Some(start) = template.find('<') else {
        return template == candidate;
    };
    let Some(end) = template[start..].find('>').map(|n| n + start + 1) else {
        return false;
    };
    let Some(rest) = candidate.strip_prefix(&template[..start]) else {
        return false;
    };
    for index in rest
        .char_indices()
        .map(|(n, _)| n)
        .skip(1)
        .chain(std::iter::once(rest.len()))
    {
        let value = &rest[..index];
        let numeric = match &template[start..end] {
            "<n>" => !value.is_empty() && value.bytes().all(|byte| byte.is_ascii_digit()),
            "<NNNN>" => value.len() == 4 && value.bytes().all(|byte| byte.is_ascii_digit()),
            _ => !value.is_empty(),
        };
        if numeric
            && matches_template(&template[end..], &rest[index..])
            && !value.contains(['<', '>'])
        {
            return true;
        }
    }
    false
}
