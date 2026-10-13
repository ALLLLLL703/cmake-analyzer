// AI-generated implementation.
use super::model::{Argument, FileSet, SourceDeclaration, SourceSection, Visibility};
///  handle the target_sources() command
pub(super) fn sources(target: Argument, arguments: Vec<Argument>) -> SourceDeclaration {
    let mut sections = Vec::new();
    let mut section = SourceSection {
        visibility: None,
        files: Vec::new(),
        file_sets: Vec::new(),
    };
    let mut index = 0;
    // Each visibility section owns its plain files and FILE_SET declarations.
    while let Some(arg) = arguments.get(index) {
        let visibility = if arg.literal {
            match arg.text.as_str() {
                "PRIVATE" => Some(Visibility::Private),
                "PUBLIC" => Some(Visibility::Public),
                "INTERFACE" => Some(Visibility::Interface),
                _ => None,
            }
        } else {
            None
        };
        if let Some(visibility) = visibility {
            if section.visibility.is_some()
                || !section.files.is_empty()
                || !section.file_sets.is_empty()
            {
                sections.push(section);
            }
            section = SourceSection {
                visibility: Some(visibility),
                files: Vec::new(),
                file_sets: Vec::new(),
            };
            index += 1;
        } else if arg.literal && arg.text == "FILE_SET" {
            index += 1;
            let Some(name) = arguments.get(index) else {
                break;
            };
            // The built-in HEADERS set implies TYPE HEADERS.
            let mut set = FileSet {
                name: name.clone(),
                kind: (name.literal && name.text == "HEADERS").then(|| name.clone()),
                base_dirs: Vec::new(),
                files: Vec::new(),
            };
            index += 1;
            // BASE_DIRS and FILES redirect values until the next set/section.
            let mut bases = false;
            while let Some(item) = arguments.get(index) {
                if item.literal
                    && matches!(
                        item.text.as_str(),
                        "PRIVATE" | "PUBLIC" | "INTERFACE" | "FILE_SET"
                    )
                {
                    break;
                }
                if item.literal && item.text == "TYPE" {
                    set.kind = arguments.get(index + 1).cloned();
                    index += 2;
                } else if item.literal && item.text == "BASE_DIRS" {
                    bases = true;
                    index += 1;
                } else if item.literal && item.text == "FILES" {
                    bases = false;
                    index += 1;
                } else {
                    if bases {
                        set.base_dirs.push(item.clone());
                    } else {
                        set.files.push(item.clone());
                    }
                    index += 1;
                }
            }
            section.file_sets.push(set);
        } else {
            /*
             *i wish all the cmake commands is as simple as this one, but it doesnt,,,, T_T
             */
            section.files.push(arg.clone());
            index += 1;
        }
    }
    if section.visibility.is_some() || !section.files.is_empty() || !section.file_sets.is_empty() {
        sections.push(section);
    }
    SourceDeclaration {
        target,
        sections,
        arguments,
    }
}
