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

pub struct SemanticVariable {
    pub value: String,
    pub id: VariableId,
    pub block: BlockId,
    pub scope: ScopeId,
}

pub struct SemanticSymbol {}

pub struct VariableId {
    pub id: usize,
}

pub struct BlockId {
    pub id: usize,
}

pub struct ScopeId {
    pub id: usize,
}

pub struct FunctionId {
    pub id: usize,
}

// 需要干什么?
//首先需要虑出所有symbol
// and then extract all variable and its !Span and where it might be changed bind the scope with id
// then extract all function and its !Span(also should know the builtin function sign)
// then find all blocks and its Span
// then find all out args function and mark the variable changes
// need a scope struct to represent the scope of a variable (can be invert!)
// scope should not own the variable struct
// need a struct to represent the sign of a command(can be inject via lua script)
//
//
// writing in the paper is extremely important,,, i see
