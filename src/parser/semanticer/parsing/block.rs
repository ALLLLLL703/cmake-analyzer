// AI-generated implementation.
use crate::{
    model::{Spanned, TextSpan},
    parser::semanticer::model::{
        BlockId, BlockKind, BuiltinCommand, CommandKind, SemanticBlock, SemanticError,
        SemanticedBuffer, Semanticer,
    },
};

impl Semanticer {
    pub fn scan_blocks(&self, result: &mut SemanticedBuffer) {
        result.blocks.clear();
        let mut stack: Vec<BlockId> = Vec::new();
        let mut seen_else = Vec::new();
        for index in 0..result.commands.len() {
            let command = &result.commands[index];
            let id = command.id;
            let span = command.full_span;
            let keyword = command.closed
                && matches!(command.kind, CommandKind::Builtin(BuiltinCommand::KeyWord));
            let name = command.name.to_ascii_lowercase();
            if let Some(kind) = keyword.then(|| opening(&name)).flatten() {
                let block = BlockId {
                    id: result.blocks.len() as u32,
                };
                let parent = stack.last().copied();
                if let Some(parent) = parent {
                    result.blocks[parent.id as usize].children.push(block);
                }
                result.blocks.push(SemanticBlock {
                    id: block,
                    kind,
                    contains: vec![id],
                    children: Vec::new(),
                    parent,
                    opening: id,
                    closing: None,
                    closed: false,
                    span,
                });
                seen_else.push(false);
                result.commands[index].block = Some(block);
                stack.push(block);
                continue;
            }
            if let Some(kind) = keyword.then(|| ending(&name)).flatten() {
                if let Some(position) = stack
                    .iter()
                    .rposition(|block| result.blocks[block.id as usize].kind == kind)
                {
                    let previous = if index > 0 {
                        result.commands[index - 1].full_span
                    } else {
                        span
                    };
                    while stack.len() > position + 1 {
                        if let Some(block) = stack.pop() {
                            unclosed(result, block, previous);
                        }
                    }
                    if let Some(block) = stack.pop() {
                        let entry = &mut result.blocks[block.id as usize];
                        entry.contains.push(id);
                        entry.closing = Some(id);
                        entry.closed = true;
                        entry.span = TextSpan::combine_with_middle(entry.span, span);
                        result.commands[index].block = Some(block);
                    }
                    continue;
                }
                result.diagnostics.push(Spanned::new(
                    SemanticError::UnexpectedBlockEnd(command.name.clone()),
                    command.span,
                ));
            }
            if keyword && (name == "else" || name == "elseif") {
                match stack.last().copied() {
                    Some(block) if result.blocks[block.id as usize].kind == BlockKind::If => {
                        if seen_else[block.id as usize] {
                            result.diagnostics.push(Spanned::new(
                                SemanticError::InvalidIfBranch(command.name.clone()),
                                command.span,
                            ));
                        }
                        if name == "else" {
                            seen_else[block.id as usize] = true;
                        }
                    }
                    _ => result.diagnostics.push(Spanned::new(
                        SemanticError::UnexpectedBranch(command.name.clone()),
                        command.span,
                    )),
                }
            }
            let owner = stack.last().copied();
            result.commands[index].block = owner;
            if let Some(block) = owner {
                result.blocks[block.id as usize].contains.push(id);
            }
        }
        if let Some(end) = result.commands.last().map(|command| command.full_span) {
            for block in stack {
                unclosed(result, block, end);
            }
        }
    }
}
fn opening(name: &str) -> Option<BlockKind> {
    match name {
        "if" => Some(BlockKind::If),
        "while" => Some(BlockKind::While),
        "foreach" => Some(BlockKind::Foreach),
        "function" => Some(BlockKind::Function),
        "macro" => Some(BlockKind::Macro),
        "block" => Some(BlockKind::Block),
        _ => None,
    }
}
fn ending(name: &str) -> Option<BlockKind> {
    match name {
        "endif" => Some(BlockKind::If),
        "endwhile" => Some(BlockKind::While),
        "endforeach" => Some(BlockKind::Foreach),
        "endfunction" => Some(BlockKind::Function),
        "endmacro" => Some(BlockKind::Macro),
        "endblock" => Some(BlockKind::Block),
        _ => None,
    }
}
fn unclosed(result: &mut SemanticedBuffer, id: BlockId, end: TextSpan) {
    let block = &mut result.blocks[id.id as usize];
    block.span = TextSpan::combine_with_middle(block.span, end);
    let opening = result.commands[block.opening.id as usize].span;
    result.diagnostics.push(Spanned::new(
        SemanticError::UnclosedBlock(block.kind),
        opening,
    ));
}
