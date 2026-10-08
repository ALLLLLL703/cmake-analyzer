use crate::parser::nparser::model::NParsedBuffer;

pub struct Semanticer {
    pub buffer: NParsedBuffer,
}

#[derive(Default, Debug)]
pub struct SemanticCursor {
    pub offset: usize,
    pub max_length: usize,
}

#[derive(Default, Clone, Debug)]
pub struct SemanticedBuffer {}
