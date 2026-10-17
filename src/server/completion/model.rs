use crate::{parser::semanticer::model::ReferenceIndex, workspace::model::DocumentSnapshot};
use std::{ops::Range, sync::Arc};

pub(super) enum Prepared {
    Commands(Site),
    Arguments {
        site: Site,
        expansion: bool,
        requests: Vec<crate::workspace::completion::model::Request>,
    },
}
pub(super) struct Input {
    pub catalog: crate::workspace::completion::model::Catalog,
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
#[derive(Default)]
pub(super) struct ArgumentChoices<'a> {
    pub roles: Vec<crate::parser::semanticer::schema::NameRole>,
    pub keywords: Vec<&'a str>,
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
