// AI-generated implementation.
use crate::parser::semanticer::model::{
    SemanticSymbol, SemanticSymbolKind, SemanticedBuffer, Semanticer,
};

impl Semanticer {
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
