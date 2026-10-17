//! Recover nested blocks from closed keyword commands, retaining structural errors.
//!
//! All branches are structural candidates; conditions are not evaluated.
// Contains AI-generated documentation additions.
// AI-generated implementation.
use crate::{
    model::{Spanned, TextSpan},
    parser::semanticer::model::{
        BlockId, BlockKind, BuiltinCommand, CommandKind, SemanticBlock, SemanticError,
        SemanticedBuffer, Semanticer,
    },
};

impl Semanticer {
    /// Rebuild block nesting from closed schema keywords and append recovery diagnostics. Mutates only the supplied snapshot; never evaluates branches.
    ///
    /// # Arguments
    /// - `self` `[in]`: analyzer, unchanged.
    /// - `result` `[in,out]`: clears/replaces blocks, overwrites command.block links and appends structural diagnostics.
    ///
    /// # Returns
    /// () after closing compatible blocks or marking remaining openers unclosed.
    ///
    /// # Panics
    /// Requires dense command IDs and internally consistent arenas; direct indexing assumes commands were extracted in order.
    ///
    /// # Algorithm
    /// Maintain an opener stack. A closer searches backward for a compatible opener, diagnosing skipped unclosed blocks. Track else occurrence per if block. Ordinary commands belong to the current innermost block; leftover openers extend through the last command.
    ///
    /// # Flow
    /// ```text
    ///  [closed keyword]         [ordinary/unclosed command]
    ///         |                          |
    ///     <opener/closer?>                |
    ///       /         \                 |
    ///  [push block] [find compatible]     |
    ///       |         |                  |
    ///       |    [close/recover]          |
    ///       +---------+------------------+
    ///                 v
    ///        [assign current owner]
    ///                 |
    ///                 v
    ///        [end: diagnose leftovers]
    /// ```
    ///
    /// # Examples
    /// ```
    /// use cmake_analyzer::parser::semanticer::model::*;
    /// use cmake_analyzer::parser::nparser::model::NParsedBuffer;
    /// let analyzer = Semanticer::new(NParsedBuffer::default(), Default::default());
    /// let mut buffer = SemanticedBuffer::default();
    /// analyzer.scan_blocks(&mut buffer);
    /// assert!(buffer.blocks.is_empty());
    /// ```
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
/// Classify a lowercase opening keyword spelling. No side effects.
///
/// # Arguments
/// - `name` `[in]`: lowercase command spelling; no normalization is performed here.
///
/// # Returns
/// Some block family for a supported keyword, otherwise None.
///
/// # Examples
/// ```text
/// assert_eq!(opening("if"), Some(BlockKind::If));
/// assert_eq!(opening("set"), None);
/// ```
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
/// Classify a lowercase closing keyword spelling. No side effects.
///
/// # Arguments
/// - `name` `[in]`: lowercase command spelling; no normalization is performed here.
///
/// # Returns
/// Some block family for a supported keyword, otherwise None.
///
/// # Examples
/// ```text
/// assert_eq!(ending("endif"), Some(BlockKind::If));
/// assert_eq!(ending("set"), None);
/// ```
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
/// Extend an unclosed block through the recovery endpoint and append its diagnostic. No I/O.
///
/// # Arguments
/// - `result` `[in,out]`: overwrites the selected block span and appends a diagnostic at its opening name.
/// - `id` `[in]`: valid block index in result.
/// - `end` `[in]`: last source extent included by recovery.
///
/// # Returns
/// (); closed/closing fields remain unchanged.
///
/// # Panics
/// Panics if id or the block opening command is outside its owning arena.
///
/// # Examples
/// ```text
/// unclosed(&mut buffer, BlockId { id: 0 }, end);
/// ```
fn unclosed(result: &mut SemanticedBuffer, id: BlockId, end: TextSpan) {
    let block = &mut result.blocks[id.id as usize];
    block.span = TextSpan::combine_with_middle(block.span, end);
    let opening = result.commands[block.opening.id as usize].span;
    result.diagnostics.push(Spanned::new(
        SemanticError::UnclosedBlock(block.kind),
        opening,
    ));
}
