use super::{
    blocks, commands,
    model::{SemanticBuffer, Semanticer},
    resolve,
};
use crate::parser::{
    lexer::model::{Lexer, RawBuffer},
    nparser::model::NParser,
};

impl Semanticer {
    /// Moves the parsed commands into one structural and semantic snapshot.
    pub fn parse(&mut self) -> SemanticBuffer {
        let parsed = std::mem::take(&mut self.buffer);
        let structure = blocks::parse(&parsed);
        let facts = commands::extract(&parsed, &structure);
        let mut buffer = SemanticBuffer {
            parsed,
            blocks: structure.blocks,
            command_blocks: structure.command_blocks,
            scopes: structure.scopes,
            command_scopes: structure.command_scopes,
            symbols: facts.symbols,
            references: facts.references,
            diagnostics: structure.diagnostics,
        };
        buffer.diagnostics.extend(facts.diagnostics);
        resolve::bind(&mut buffer);
        buffer
    }

    pub fn analyze(source: &str) -> SemanticBuffer {
        let lexed = Lexer::new(RawBuffer::new(source)).parse();
        Self::new(NParser::new(lexed).parse()).parse()
    }
}
