use super::{
    model::{Block, BlockKind, Diagnostic, DiagnosticKind, Scope, ScopeKind, Severity, Structure},
    text::argument_text,
};
use crate::{model::Spanned, parser::nparser::model::NParsedBuffer};

pub(super) fn parse(parsed: &NParsedBuffer) -> Structure {
    let mut output = Structure {
        blocks: Vec::new(),
        command_blocks: vec![None; parsed.commands.len()],
        scopes: vec![Scope {
            kind: ScopeKind::Directory,
            parent: None,
            block: None,
        }],
        command_scopes: vec![0; parsed.commands.len()],
        diagnostics: Vec::new(),
    };
    let mut stack: Vec<usize> = Vec::new();
    let mut else_seen = Vec::new();
    let mut openings: [Vec<usize>; 6] = std::array::from_fn(|_| Vec::new());
    for (index, command) in parsed.commands.iter().enumerate() {
        let parent = stack.last().copied();
        let scope = parent.map_or(0, |block| output.blocks[block].scope);
        output.command_blocks[index] = parent;
        output.command_scopes[index] = scope;
        let name = command.name.content.to_ascii_lowercase();
        let opening = match name.as_str() {
            "if" => Some(BlockKind::If),
            "foreach" => Some(BlockKind::Foreach),
            "while" => Some(BlockKind::While),
            "function" => Some(BlockKind::Function),
            "macro" => Some(BlockKind::Macro),
            "block" => Some(BlockKind::Block),
            _ => None,
        };
        if let Some(kind) = opening {
            let id = output.blocks.len();
            let scope_kind = match kind {
                BlockKind::Function => Some(ScopeKind::Function),
                BlockKind::Macro => Some(ScopeKind::Macro),
                BlockKind::Block
                    if !command
                        .args
                        .iter()
                        .any(|arg| argument_text(&arg.content) == Some("SCOPE_FOR"))
                        || command
                            .args
                            .iter()
                            .any(|arg| argument_text(&arg.content) == Some("VARIABLES")) =>
                {
                    Some(ScopeKind::Variables)
                }
                _ => None,
            };
            let body_scope = if let Some(kind) = scope_kind {
                let id_scope = output.scopes.len();
                output.scopes.push(Scope {
                    kind,
                    parent: Some(scope),
                    block: Some(id),
                });
                id_scope
            } else {
                scope
            };
            output.blocks.push(Block {
                kind,
                parent,
                opening: index,
                closing: None,
                branches: Vec::new(),
                scope: body_scope,
            });
            output.command_blocks[index] = Some(id);
            else_seen.push(false);
            stack.push(id);
            openings[kind as usize].push(id);
            continue;
        }
        if matches!(name.as_str(), "else" | "elseif") {
            let problem =
                if let Some(id) = parent.filter(|id| output.blocks[*id].kind == BlockKind::If) {
                    let block = &mut output.blocks[id];
                    let had_else = else_seen[id];
                    if name == "else" {
                        else_seen[id] = true;
                    }
                    block.branches.push(index);
                    match (had_else, name.as_str()) {
                        (true, "else") => Some(DiagnosticKind::DuplicateElse),
                        (true, "elseif") => Some(DiagnosticKind::ElseIfAfterElse),
                        _ => None,
                    }
                } else {
                    Some(DiagnosticKind::UnexpectedBranch)
                };
            if let Some(kind) = problem {
                error(&mut output, parsed, index, kind);
            }
            continue;
        }
        let ending = match name.as_str() {
            "endif" => Some(BlockKind::If),
            "endforeach" => Some(BlockKind::Foreach),
            "endwhile" => Some(BlockKind::While),
            "endfunction" => Some(BlockKind::Function),
            "endmacro" => Some(BlockKind::Macro),
            "endblock" => Some(BlockKind::Block),
            _ => None,
        };
        if let Some(kind) = ending {
            let Some(&matching) = openings[kind as usize].last() else {
                error(
                    &mut output,
                    parsed,
                    index,
                    DiagnosticKind::UnmatchedBlockEnd(kind),
                );
                continue;
            };
            while let Some(id) = stack.pop() {
                let block_kind = output.blocks[id].kind;
                openings[block_kind as usize].pop();
                if id == matching {
                    output.blocks[id].closing = Some(index);
                    output.command_blocks[index] = Some(id);
                    output.command_scopes[index] = output.blocks[id].scope;
                    break;
                }
                let opening = output.blocks[id].opening;
                error(
                    &mut output,
                    parsed,
                    opening,
                    DiagnosticKind::UnclosedBlock(block_kind),
                );
            }
        }
    }
    for id in stack {
        let block = &output.blocks[id];
        let (opening, kind) = (block.opening, block.kind);
        error(
            &mut output,
            parsed,
            opening,
            DiagnosticKind::UnclosedBlock(kind),
        );
    }
    output
}

fn error(output: &mut Structure, parsed: &NParsedBuffer, command: usize, kind: DiagnosticKind) {
    output.diagnostics.push(Spanned {
        content: Diagnostic {
            severity: Severity::Error,
            kind,
        },
        span: parsed.commands[command].name.span,
    });
}
