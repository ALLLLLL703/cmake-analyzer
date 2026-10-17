//! Select top-level commands and blocks for document symbol presentation. No I/O.
// Contains AI-generated documentation additions.
// AI-generated implementation.
use crate::parser::semanticer::model::{
    SemanticSymbol, SemanticSymbolKind, SemanticedBuffer, Semanticer,
};

impl Semanticer {
    /// Replace root presentation symbols with top-level blocks and commands outside blocks. No I/O or branch evaluation.
    ///
    /// # Arguments
    /// - `self` `[in]`: analyzer, unchanged.
    /// - `result` `[in,out]`: clears/replaces symbols using commands and block ranges.
    ///
    /// # Returns
    /// () after source-order root symbol selection; nested commands are not separate root entries.
    ///
    /// # Panics
    /// Requires command.block values to index the supplied block arena.
    ///
    /// # Examples
    /// ```
    /// use cmake_analyzer::parser::semanticer::model::*;
    /// use cmake_analyzer::parser::nparser::model::NParsedBuffer;
    /// let analyzer = Semanticer::new(NParsedBuffer::default(), Default::default());
    /// let mut buffer = SemanticedBuffer::default();
    /// analyzer.collect_symbols(&mut buffer);
    /// assert!(buffer.symbols.is_empty());
    /// ```
    pub fn collect_symbols(&self, result: &mut SemanticedBuffer) {
        result.symbols.clear();
        for command in &result.commands {
            match command.block {
                Some(id) => {
                    let block = &result.blocks[id.id as usize];
                    if block.parent.is_none() && block.opening == command.id {
                        result.symbols.push(SemanticSymbol {
                            kind: SemanticSymbolKind::Block(id),
                            span: block.span,
                        });
                    }
                }
                None => result.symbols.push(SemanticSymbol {
                    kind: SemanticSymbolKind::Command(command.id),
                    span: command.full_span,
                }),
            }
        }
    }
}
