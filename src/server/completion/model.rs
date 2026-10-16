use crate::{parser::semanticer::model::ReferenceIndex, workspace::model::DocumentSnapshot};
use std::{ops::Range, sync::Arc};

pub(super) struct Input {
    pub snapshot: Arc<DocumentSnapshot>,
    pub index: Option<Arc<ReferenceIndex>>,
    pub sources: Vec<Source>,
}
pub(super) struct Source {
    pub snapshot: Arc<DocumentSnapshot>,
    pub variables_visible: bool,
}
pub(super) struct ArgumentSite<'a> {
    pub command: &'a crate::parser::semanticer::model::SemanticCommandRepresent,
    pub argument: usize,
    pub payload: Range<usize>,
    pub form: crate::parser::semanticer::model::SemanticArgForm,
}
pub(super) struct Site {
    pub replace: Range<usize>,
    pub prefix: String,
    pub has_parentheses: bool,
}
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(super) enum Kind {
    Keyword,
    Module,
    Function,
    Variable,
    EnumMember,
}
#[derive(Clone, PartialEq, Eq)]
pub(super) enum Parameter {
    Literal(String),
    Placeholder(String),
}
pub(super) struct Candidate {
    pub name: String,
    pub kind: Kind,
    pub parameters: Option<Vec<Parameter>>,
    pub source_defined: bool,
}
