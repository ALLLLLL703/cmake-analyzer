use super::{
    model::{
        BlockKind, Diagnostic, DiagnosticKind, Facts, Name, Namespace, Reference, Severity,
        Structure, Symbol, SymbolKind,
    },
    references,
    text::{argument_text, dynamic_arity, literal},
};
use crate::{
    model::Spanned,
    parser::nparser::model::{NParsedArgument, NParsedBuffer},
};

pub(super) fn extract(parsed: &NParsedBuffer, structure: &Structure) -> Facts {
    let mut facts = Facts::default();
    // Variable-only blocks do not localize cache/environment, targets, or commands.
    // Callable containers still guard definitions whose execution is unknown.
    let mut registrations = Vec::with_capacity(structure.scopes.len());
    for (id, scope) in structure.scopes.iter().enumerate() {
        let registry = if scope.kind == super::model::ScopeKind::Variables {
            scope.parent.map_or(id, |parent| registrations[parent])
        } else { id };
        registrations.push(registry);
    }
    for (index, command) in parsed.commands.iter().enumerate() {
        let scope = structure.command_scopes[index];
        let name = command.name.content.to_ascii_lowercase();
        for argument in 0..command.args.len() {
            references::expansions(parsed, index, argument, scope, &mut facts.references);
        }
        if !structural(&name) {
            facts.references.push(Reference {
                name: Name {
                    command: index,
                    argument: None,
                    bytes: 0..command.name.content.len(),
                },
                namespace: Namespace::Command,
                scope,
                span: command.name.span,
                dynamic: false,
                resolved: Vec::new(),
            });
        }
        let limits = match name.as_str() {
            "set" => Some((1, None)),
            "unset" => Some((1, Some(2))),
            "option" => Some((2, Some(3))),
            "function" | "macro" | "foreach" | "if" | "elseif" | "while" => Some((1, None)),
            "endfunction" | "endmacro" => Some((0, Some(1))),
            "endblock" => Some((0, Some(0))),
            "add_library" | "add_executable" => Some((1, None)),
            "get_target_property" => Some((3, Some(3))),
            "get_filename_component" => Some((3, None)),
            "list" | "string" => Some((1, None)),
            _ => None,
        };
        if let Some((min, max)) = limits {
            let actual = command.args.len();
            if !dynamic_arity(parsed, index)
                && (actual < min || max.is_some_and(|max| actual > max))
            {
                diagnostic(
                    &mut facts,
                    parsed,
                    index,
                    DiagnosticKind::InvalidArity { min, max, actual },
                );
            }
        }
        match name.as_str() {
            "set" | "unset" | "option" => {
                let Some(first) = command.args.first().and_then(|arg| literal(&arg.content)) else {
                    continue;
                };
                let namespace = if first.starts_with("ENV{") && first.ends_with('}') {
                    Namespace::Environment
                } else if name == "option"
                    || command
                        .args
                        .iter()
                        .skip(1)
                        .any(|arg| argument_text(&arg.content) == Some("CACHE"))
                {
                    Namespace::Cache
                } else {
                    Namespace::Variable
                };
                let parent = namespace == Namespace::Variable
                    && match name.as_str() {
                        "set" => {
                            command
                                .args
                                .last()
                                .and_then(|arg| argument_text(&arg.content))
                                == Some("PARENT_SCOPE")
                        }
                        "unset" => {
                            command
                                .args
                                .get(1)
                                .and_then(|arg| argument_text(&arg.content))
                                == Some("PARENT_SCOPE")
                        }
                        _ => false,
                    };
                let kind = if parent {
                    SymbolKind::ParentWrite
                } else if name == "unset" || (name == "set" && command.args.len() == 1) {
                    SymbolKind::Unset
                } else {
                    match namespace {
                        Namespace::Cache => SymbolKind::Cache,
                        Namespace::Environment => SymbolKind::Environment,
                        _ => SymbolKind::Variable,
                    }
                };
                define(&mut facts, parsed, index, 0, scope, namespace, kind);
                if namespace == Namespace::Environment {
                    if let Some(symbol) = facts
                        .symbols
                        .last_mut()
                        .filter(|symbol| symbol.name.command == index)
                    {
                        symbol.name.bytes = 4..first.len() - 1;
                    }
                }
            }
            "function" | "macro" => {
                let kind = if name == "function" {
                    SymbolKind::Function
                } else {
                    SymbolKind::Macro
                };
                define(
                    &mut facts,
                    parsed,
                    index,
                    0,
                    scope,
                    Namespace::Command,
                    kind,
                );
                if let Some(block) = structure.command_blocks[index].map(|id| &structure.blocks[id])
                {
                    for argument in 1..command.args.len() {
                        define(
                            &mut facts,
                            parsed,
                            index,
                            argument,
                            block.scope,
                            Namespace::Variable,
                            SymbolKind::Parameter,
                        );
                    }
                }
            }
            "endfunction" | "endmacro" => {
                if let Some(block) = structure.command_blocks[index]
                    .map(|id| &structure.blocks[id])
                    .filter(|block| {
                        block.closing == Some(index)
                            && matches!(block.kind, BlockKind::Function | BlockKind::Macro)
                    })
                {
                    let opening = parsed.commands[block.opening]
                        .args
                        .first()
                        .and_then(|arg| literal(&arg.content));
                    let ending = command.args.first().and_then(|arg| literal(&arg.content));
                    if let (Some(opening), Some(ending)) = (opening, ending) {
                        if !opening.eq_ignore_ascii_case(ending) {
                            diagnostic(&mut facts, parsed, index, DiagnosticKind::EndNameMismatch);
                        }
                    }
                }
            }
            "foreach" => {
                define(
                    &mut facts,
                    parsed,
                    index,
                    0,
                    scope,
                    Namespace::Variable,
                    SymbolKind::Variable,
                );
                let mut lists = false;
                for (argument, arg) in command.args.iter().enumerate().skip(1) {
                    match argument_text(&arg.content) {
                        Some("LISTS") => lists = true,
                        Some("ITEMS") => lists = false,
                        _ if lists => refer(
                            &mut facts,
                            parsed,
                            index,
                            argument,
                            scope,
                            Namespace::Variable,
                        ),
                        _ => {}
                    }
                }
            }
            "if" | "elseif" | "while" => conditions(&mut facts, parsed, index, scope),
            "add_library" | "add_executable" => {
                let alias = command
                    .args
                    .get(1)
                    .and_then(|arg| argument_text(&arg.content))
                    == Some("ALIAS");
                define(
                    &mut facts,
                    parsed,
                    index,
                    0,
                    scope,
                    Namespace::Target,
                    if alias {
                        SymbolKind::Alias
                    } else {
                        SymbolKind::Target
                    },
                );
                if alias {
                    refer(&mut facts, parsed, index, 2, scope, Namespace::Target);
                    if command.args.len() != 3 && !dynamic_arity(parsed, index) {
                        diagnostic(
                            &mut facts,
                            parsed,
                            index,
                            DiagnosticKind::InvalidArity {
                                min: 3,
                                max: Some(3),
                                actual: command.args.len(),
                            },
                        );
                    }
                }
            }
            "target_link_libraries"
            | "target_sources"
            | "target_include_directories"
            | "target_compile_definitions"
            | "target_compile_options"
            | "target_compile_features"
            | "target_link_options"
            | "target_link_directories"
            | "target_precompile_headers" => {
                refer(&mut facts, parsed, index, 0, scope, Namespace::Target);
                if name == "target_link_libraries" {
                    for (argument, arg) in command.args.iter().enumerate().skip(1) {
                        if !matches!(
                            argument_text(&arg.content),
                            Some(
                                "PUBLIC"
                                    | "PRIVATE"
                                    | "INTERFACE"
                                    | "LINK_PUBLIC"
                                    | "LINK_PRIVATE"
                                    | "LINK_INTERFACE_LIBRARIES"
                                    | "debug"
                                    | "optimized"
                                    | "general"
                            )
                        ) {
                            refer(
                                &mut facts,
                                parsed,
                                index,
                                argument,
                                scope,
                                Namespace::Target,
                            );
                        }
                    }
                }
            }
            "add_dependencies" => {
                for argument in 0..command.args.len() {
                    refer(
                        &mut facts,
                        parsed,
                        index,
                        argument,
                        scope,
                        Namespace::Target,
                    );
                }
            }
            "get_target_property" => {
                define(
                    &mut facts,
                    parsed,
                    index,
                    0,
                    scope,
                    Namespace::Variable,
                    SymbolKind::Variable,
                );
                refer(&mut facts, parsed, index, 1, scope, Namespace::Target);
            }
            "set_target_properties" => {
                for (argument, arg) in command.args.iter().enumerate() {
                    if argument_text(&arg.content) == Some("PROPERTIES") {
                        break;
                    }
                    refer(
                        &mut facts,
                        parsed,
                        index,
                        argument,
                        scope,
                        Namespace::Target,
                    );
                }
            }
            "set_property"
                if command
                    .args
                    .first()
                    .and_then(|arg| argument_text(&arg.content))
                    == Some("TARGET") =>
            {
                for (argument, arg) in command.args.iter().enumerate().skip(1) {
                    if matches!(
                        argument_text(&arg.content),
                        Some("PROPERTY" | "APPEND" | "APPEND_STRING")
                    ) {
                        break;
                    }
                    refer(
                        &mut facts,
                        parsed,
                        index,
                        argument,
                        scope,
                        Namespace::Target,
                    );
                }
            }
            "get_filename_component"
            | "get_property"
            | "get_directory_property"
            | "get_cmake_property" => define(
                &mut facts,
                parsed,
                index,
                0,
                scope,
                Namespace::Variable,
                SymbolKind::Variable,
            ),
            "math"
                if command
                    .args
                    .first()
                    .and_then(|arg| argument_text(&arg.content))
                    == Some("EXPR") =>
            {
                define(
                    &mut facts,
                    parsed,
                    index,
                    1,
                    scope,
                    Namespace::Variable,
                    SymbolKind::Variable,
                )
            }
            "list" => lists(&mut facts, parsed, index, scope),
            "string" => strings(&mut facts, parsed, index, scope),
            _ => {}
        }
    }
    for symbol in &mut facts.symbols {
        if symbol.namespace != Namespace::Variable { symbol.scope = registrations[symbol.scope]; }
    }
    facts
}

fn define(
    facts: &mut Facts,
    parsed: &NParsedBuffer,
    command: usize,
    argument: usize,
    scope: usize,
    namespace: Namespace,
    kind: SymbolKind,
) {
    let Some(arg) = parsed.commands[command].args.get(argument) else {
        return;
    };
    let Some(text) = literal(&arg.content).filter(|text| !text.is_empty()) else {
        return;
    };
    facts.symbols.push(Symbol {
        name: Name {
            command,
            argument: Some(argument),
            bytes: 0..text.len(),
        },
        kind,
        namespace,
        scope,
        span: arg.span,
    });
}

fn refer(
    facts: &mut Facts,
    parsed: &NParsedBuffer,
    command: usize,
    argument: usize,
    scope: usize,
    namespace: Namespace,
) {
    let Some(arg) = parsed.commands[command].args.get(argument) else {
        return;
    };
    let Some(text) = literal(&arg.content).filter(|text| !text.is_empty()) else {
        return;
    };
    facts.references.push(Reference {
        name: Name {
            command,
            argument: Some(argument),
            bytes: 0..text.len(),
        },
        namespace,
        scope,
        span: arg.span,
        dynamic: false,
        resolved: Vec::new(),
    });
}

fn conditions(facts: &mut Facts, parsed: &NParsedBuffer, command: usize, scope: usize) {
    let mut previous = "";
    for (argument, arg) in parsed.commands[command].args.iter().enumerate() {
        let NParsedArgument::Unquoted(text) = &arg.content else {
            previous = "";
            continue;
        };
        let keyword = text.to_ascii_uppercase();
        let constant = matches!(
            keyword.as_str(),
            "AND"
                | "OR"
                | "NOT"
                | "DEFINED"
                | "COMMAND"
                | "TARGET"
                | "POLICY"
                | "TEST"
                | "EXISTS"
                | "IS_READABLE"
                | "IS_WRITABLE"
                | "IS_EXECUTABLE"
                | "IS_DIRECTORY"
                | "IS_SYMLINK"
                | "IS_ABSOLUTE"
                | "MATCHES"
                | "LESS"
                | "GREATER"
                | "EQUAL"
                | "LESS_EQUAL"
                | "GREATER_EQUAL"
                | "STRLESS"
                | "STRGREATER"
                | "STREQUAL"
                | "STRLESS_EQUAL"
                | "STRGREATER_EQUAL"
                | "VERSION_LESS"
                | "VERSION_GREATER"
                | "VERSION_EQUAL"
                | "VERSION_LESS_EQUAL"
                | "VERSION_GREATER_EQUAL"
                | "IN_LIST"
                | "IS_NEWER_THAN"
                | "PATH_EQUAL"
                | "ON"
                | "OFF"
                | "TRUE"
                | "FALSE"
                | "YES"
                | "NO"
                | "Y"
                | "N"
                | "IGNORE"
                | "NOTFOUND"
        ) || keyword.ends_with("-NOTFOUND")
            || text.parse::<f64>().is_ok();
        if !constant
            && !matches!(
                previous,
                "POLICY"
                    | "TEST"
                    | "EXISTS"
                    | "IS_READABLE"
                    | "IS_WRITABLE"
                    | "IS_EXECUTABLE"
                    | "IS_DIRECTORY"
                    | "IS_SYMLINK"
                    | "IS_ABSOLUTE"
            )
        {
            let namespace = match previous {
                "COMMAND" => Namespace::Command,
                "TARGET" => Namespace::Target,
                _ => Namespace::Variable,
            };
            refer(facts, parsed, command, argument, scope, namespace);
            if previous == "DEFINED" && literal(&arg.content).is_some() {
                let special = if text.starts_with("ENV{") && text.ends_with('}') {
                    Some((Namespace::Environment, 4))
                } else if text.starts_with("CACHE{") && text.ends_with('}') {
                    Some((Namespace::Cache, 6))
                } else {
                    None
                };
                if let Some((namespace, start)) = special {
                    if let Some(reference) = facts.references.last_mut().filter(|reference| {
                        reference.name.command == command
                            && reference.name.argument == Some(argument)
                    }) {
                        reference.namespace = namespace;
                        reference.name.bytes = start..text.len() - 1;
                    }
                }
            }
        }
        previous = match text.as_str() {
            "COMMAND" => "COMMAND",
            "TARGET" => "TARGET",
            "DEFINED" => "DEFINED",
            "POLICY" => "POLICY",
            "TEST" => "TEST",
            "EXISTS" => "EXISTS",
            "IS_DIRECTORY" => "IS_DIRECTORY",
            "IS_SYMLINK" => "IS_SYMLINK",
            "IS_ABSOLUTE" => "IS_ABSOLUTE",
            "IS_READABLE" => "IS_READABLE",
            "IS_WRITABLE" => "IS_WRITABLE",
            "IS_EXECUTABLE" => "IS_EXECUTABLE",
            _ => "",
        };
    }
}

fn lists(facts: &mut Facts, parsed: &NParsedBuffer, command: usize, scope: usize) {
    let args = &parsed.commands[command].args;
    let operation = args
        .first()
        .and_then(|arg| argument_text(&arg.content))
        .unwrap_or("");
    refer(facts, parsed, command, 1, scope, Namespace::Variable);
    let min = match operation {
        "LENGTH" => 3, "GET" | "JOIN" | "FIND" => 4, "SUBLIST" => 5, _ => 0,
    };
    if args.len() < min {
        if !dynamic_arity(parsed, command) {
            diagnostic(facts, parsed, command, DiagnosticKind::InvalidArity { min, max: None, actual: args.len() });
        }
        return;
    }
    if matches!(operation, "LENGTH" | "GET" | "JOIN" | "FIND" | "SUBLIST") {
        if let Some(last) = args.len().checked_sub(1).filter(|last| *last >= 2) {
            define(
                facts,
                parsed,
                command,
                last,
                scope,
                Namespace::Variable,
                SymbolKind::Variable,
            );
        }
    } else if matches!(
        operation,
        "APPEND"
            | "PREPEND"
            | "INSERT"
            | "REMOVE_ITEM"
            | "REMOVE_AT"
            | "REMOVE_DUPLICATES"
            | "FILTER"
            | "REVERSE"
            | "SORT"
            | "TRANSFORM"
            | "POP_FRONT"
            | "POP_BACK"
    ) {
        let output = if operation == "TRANSFORM" {
            args.iter()
                .position(|arg| argument_text(&arg.content) == Some("OUTPUT_VARIABLE"))
                .map(|argument| argument + 1)
        } else {
            None
        };
        define(
            facts,
            parsed,
            command,
            output.unwrap_or(1),
            scope,
            Namespace::Variable,
            SymbolKind::Variable,
        );
        if matches!(operation, "POP_FRONT" | "POP_BACK") {
            for argument in 2..args.len() {
                define(
                    facts,
                    parsed,
                    command,
                    argument,
                    scope,
                    Namespace::Variable,
                    SymbolKind::Variable,
                );
            }
        }
    }
}

fn strings(facts: &mut Facts, parsed: &NParsedBuffer, command: usize, scope: usize) {
    let args = &parsed.commands[command].args;
    let operation = args
        .first()
        .and_then(|arg| argument_text(&arg.content))
        .unwrap_or("");
    let min = match operation {
        "APPEND" | "PREPEND" | "CONCAT" | "TIMESTAMP" => 2,
        "MAKE_C_IDENTIFIER" | "JOIN" | "CONFIGURE" | "TOLOWER" | "TOUPPER" | "LENGTH" | "STRIP" | "HEX" => 3,
        "FIND" => 4, "REPLACE" | "SUBSTRING" => 5,
        "REGEX" => match args.get(1).and_then(|arg| argument_text(&arg.content)) { Some("MATCH" | "MATCHALL") => 5, Some("REPLACE") => 6, _ => 0 },
        _ => 0,
    };
    if args.len() < min {
        if !dynamic_arity(parsed, command) {
            diagnostic(facts, parsed, command, DiagnosticKind::InvalidArity { min, max: None, actual: args.len() });
        }
        return;
    }
    let output = match operation {
        "APPEND" | "PREPEND" => {
            refer(facts, parsed, command, 1, scope, Namespace::Variable);
            Some(1)
        }
        "CONCAT" | "TIMESTAMP" | "MAKE_C_IDENTIFIER" => Some(if operation == "MAKE_C_IDENTIFIER" {
            2
        } else {
            1
        }),
        "JOIN" | "CONFIGURE" => Some(2),
        "REPLACE" | "FIND" => Some(3),
        "REGEX" => match args.get(1).and_then(|arg| argument_text(&arg.content)) {
            Some("MATCH" | "MATCHALL") => Some(3),
            Some("REPLACE") => Some(4),
            _ => None,
        },
        "TOLOWER" | "TOUPPER" | "LENGTH" | "STRIP" | "HEX" => Some(2),
        "SUBSTRING" => Some(4),
        _ => None,
    };
    if let Some(argument) = output {
        define(
            facts,
            parsed,
            command,
            argument,
            scope,
            Namespace::Variable,
            SymbolKind::Variable,
        );
    }
}

fn diagnostic(facts: &mut Facts, parsed: &NParsedBuffer, command: usize, kind: DiagnosticKind) {
    facts.diagnostics.push(Spanned {
        content: Diagnostic {
            severity: Severity::Error,
            kind,
        },
        span: parsed.commands[command].name.span,
    });
}

fn structural(name: &str) -> bool {
    matches!(
        name,
        "if" | "elseif"
            | "else"
            | "endif"
            | "foreach"
            | "endforeach"
            | "while"
            | "endwhile"
            | "function"
            | "endfunction"
            | "macro"
            | "endmacro"
            | "block"
            | "endblock"
    )
}
