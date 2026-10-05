use std::collections::HashMap;

use super::{
    model::{
        Diagnostic, DiagnosticKind, Namespace, Scope, SemanticBuffer, Severity, Symbol, SymbolKind,
    },
    text::{dynamic_arity, name_text},
};
use crate::model::Spanned;

type Bindings<'a> = HashMap<(usize, Namespace, &'a str), Vec<usize>>;

pub(super) fn bind(buffer: &mut SemanticBuffer) {
    let SemanticBuffer {
        parsed,
        symbols,
        references,
        scopes,
        diagnostics,
        ..
    } = buffer;
    let mut bindings: Bindings<'_> = HashMap::new();
    let mut callables: HashMap<usize, HashMap<String, Vec<usize>>> = HashMap::new();
    for (id, symbol) in symbols.iter().enumerate() {
        if symbol.kind == SymbolKind::ParentWrite {
            continue;
        }
        let Some(name) = name_text(parsed, &symbol.name) else {
            continue;
        };
        if symbol.namespace == Namespace::Command {
            callables
                .entry(symbol.scope)
                .or_default()
                .entry(name.to_ascii_lowercase())
                .or_default()
                .push(id);
        } else {
            let candidates = bindings
                .entry((symbol.scope, symbol.namespace, name))
                .or_default();
            if symbol.namespace == Namespace::Target && !candidates.is_empty() {
                diagnostics.push(Spanned {
                    content: Diagnostic {
                        severity: Severity::Warning,
                        kind: DiagnosticKind::DuplicateTarget,
                    },
                    span: symbol.span,
                });
            }
            candidates.push(id);
        }
    }
    for reference in references {
        if reference.dynamic {
            continue;
        }
        let Some(name) = name_text(parsed, &reference.name) else {
            continue;
        };
        if reference.namespace == Namespace::Command {
            let folded = name.to_ascii_lowercase();
            let mut scope = Some(reference.scope);
            while let Some(id) = scope {
                if let Some(candidates) = callables.get(&id).and_then(|names| names.get(&folded)) {
                    reference.resolved.extend(candidates);
                    break;
                }
                scope = scopes[id].parent;
            }
            if reference.name.argument.is_none()
                && reference.resolved.len() == 1
                && !dynamic_arity(parsed, reference.name.command)
            {
                let declaration = &symbols[reference.resolved[0]];
                if dynamic_arity(parsed, declaration.name.command) {
                    continue;
                }
                let min = parsed.commands[declaration.name.command]
                    .args
                    .len()
                    .saturating_sub(1);
                let actual = parsed.commands[reference.name.command].args.len();
                if actual < min {
                    diagnostics.push(Spanned {
                        content: Diagnostic {
                            severity: Severity::Error,
                            kind: DiagnosticKind::InvalidArity {
                                min,
                                max: None,
                                actual,
                            },
                        },
                        span: reference.span,
                    });
                }
            }
        } else if reference.namespace == Namespace::Target {
            let mut scope = Some(reference.scope);
            while let Some(id) = scope {
                if let Some(candidates) = bindings.get(&(id, Namespace::Target, name)) {
                    reference.resolved.extend(candidates);
                    break;
                }
                scope = scopes[id].parent;
            }
        } else {
            reference.resolved = variable(
                &bindings,
                symbols,
                scopes,
                reference.scope,
                reference.namespace,
                name,
                reference.name.command,
            );
            if reference.namespace == Namespace::Variable && reference.resolved.is_empty() {
                reference.resolved = variable(
                    &bindings,
                    symbols,
                    scopes,
                    reference.scope,
                    Namespace::Cache,
                    name,
                    reference.name.command,
                );
            }
        }
    }
}

fn variable(
    bindings: &Bindings<'_>,
    symbols: &[Symbol],
    scopes: &[Scope],
    initial: usize,
    namespace: Namespace,
    name: &str,
    command: usize,
) -> Vec<usize> {
    let mut scope = Some(initial);
    while let Some(id) = scope {
        if let Some(candidates) = bindings.get(&(id, namespace, name)) {
            // Definitions on the same command do not bind reads of that command's inputs.
            let position =
                candidates.partition_point(|candidate| symbols[*candidate].name.command < command);
            if let Some(candidate) = position.checked_sub(1).map(|position| candidates[position]) {
                return if symbols[candidate].kind == SymbolKind::Unset {
                    Vec::new()
                } else {
                    vec![candidate]
                };
            }
        }
        scope = scopes[id].parent;
    }
    Vec::new()
}
