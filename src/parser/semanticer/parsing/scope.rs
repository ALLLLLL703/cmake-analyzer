// AI-generated implementation.
use crate::parser::semanticer::model::{
    BlockKind, ScopeId, ScopeKind, SemanticCommandRepresent, SemanticScope, SemanticedBuffer,
    Semanticer,
};

impl Semanticer {
    pub fn scan_scopes(&self, result: &mut SemanticedBuffer) {
        result.scopes.clear();
        let mut inherited = vec![None; result.blocks.len()];
        let mut created = vec![None; result.blocks.len()];
        for block in &result.blocks {
            let parent = block.parent.and_then(|id| inherited[id.id as usize]);
            let kind = match block.kind {
                BlockKind::Function => Some(ScopeKind::Function),
                BlockKind::Block if variable_scope(&result.commands[block.opening.id as usize]) => {
                    Some(ScopeKind::VariablesBlock)
                }
                _ => None,
            };
            let scope = if let Some(kind) = kind {
                let id = ScopeId {
                    id: result.scopes.len() as u32,
                };
                result.scopes.push(SemanticScope {
                    id,
                    kind,
                    block: block.id,
                    parent,
                });
                created[block.id.id as usize] = Some(id);
                Some(id)
            } else {
                parent
            };
            inherited[block.id.id as usize] = scope;
        }
        for command in &mut result.commands {
            command.scope = command.block.and_then(|id| {
                let block = &result.blocks[id.id as usize];
                if command.id == block.opening && created[id.id as usize].is_some() {
                    block
                        .parent
                        .and_then(|parent| inherited[parent.id as usize])
                } else {
                    inherited[id.id as usize]
                }
            });
        }
    }
}
fn variable_scope(command: &SemanticCommandRepresent) -> bool {
    let Some(position) = command
        .args
        .iter()
        .position(|arg| arg.content == "SCOPE_FOR")
    else {
        return true;
    };
    command.args[position + 1..]
        .iter()
        .take_while(|arg| arg.content != "PROPAGATE")
        .any(|arg| arg.content == "VARIABLES")
}
