use crate::parser::semanticer::model::ReferenceLocation;
use lspally::server::model::Range;
pub(super) struct Target {
    pub name: String,
    pub location: ReferenceLocation,
    pub range: Range,
    pub kind: TargetKind,
}
pub(super) enum TargetKind {
    Variable,
    Command,
}
