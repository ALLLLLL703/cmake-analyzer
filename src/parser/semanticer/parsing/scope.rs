//! Assign file-local variable scopes to functions and variable-scoped blocks.
//!
//! Macros and control-flow blocks inherit their surrounding scope. Declaration openers
//! remain in the parent scope; bodies and closers use the created scope. No execution.
// Contains AI-generated documentation additions.
// AI-generated implementation.
use crate::parser::semanticer::model::{
    BlockKind, ScopeId, ScopeKind, SemanticCommandRepresent, SemanticScope, SemanticedBuffer,
    Semanticer,
};

impl Semanticer {
    /// Rebuild file-local variable scopes and overwrite command scope links. Functions and qualifying blocks create scopes; macros/control-flow blocks inherit. No I/O or execution.
    ///
    /// # Arguments
    /// - `self` `[in]`: analyzer, unchanged.
    /// - `result` `[in,out]`: clears/replaces scopes and overwrites each command.scope; blocks and arguments are read only.
    ///
    /// # Returns
    /// () after assigning parent-scope openers and body-scope commands.
    ///
    /// # Panics
    /// Requires parent-first dense block IDs and valid opening command indices.
    ///
    /// # Algorithm
    /// Walk parent-first blocks, retaining inherited and newly created scopes separately. A function or variable block creates a scope with its inherited parent. A second command pass assigns that scope except on scope-creating openers, which stay in the surrounding scope.
    ///
    /// # Flow
    /// ```text
    ///  [parent block scope]      [block kind/arguments]
    ///           |                         |
    ///           +------------+------------+
    ///                        v
    ///                 <creates scope?>
    ///                    /        \
    ///                  yes        no
    ///                   |          |
    ///             [new scope]  [inherit parent]
    ///                   |          |
    ///                   +-----+----+
    ///                         v
    ///                   <opening command?>
    ///                      /       \
    ///             [parent scope] [body scope]
    /// ```
    ///
    /// # Examples
    /// ```
    /// use cmake_analyzer::parser::semanticer::model::*;
    /// use cmake_analyzer::parser::nparser::model::NParsedBuffer;
    /// let analyzer = Semanticer::new(NParsedBuffer::default(), Default::default());
    /// let mut buffer = SemanticedBuffer::default();
    /// analyzer.scan_scopes(&mut buffer);
    /// assert!(buffer.scopes.is_empty());
    /// ```
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
/// Read whether block arguments request a variable scope. No side effects.
///
/// # Arguments
/// - `command` `[in]`: block opener arguments, compared by exact payload spelling.
///
/// # Returns
/// True if SCOPE_FOR is absent or VARIABLES appears after it before PROPAGATE; otherwise false.
///
/// # Examples
/// ```text
/// assert!(variable_scope(&buffer.commands[0]));
/// ```
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
