pub mod builtin;

use crate::parser::semanticer::model::CommandKind;

#[derive(Clone, Debug)]
pub struct CommandSchema {
    pub args: Vec<ArgSchema>,
    pub command_name: String,
    pub from: CommandSchemaFrom,
    pub kind: CommandKind,
}

#[derive(Clone, Debug)]
pub struct ArgSchema {
    pub kind: ArgSchemaKind,
}

#[derive(Clone, Debug)]
pub enum ArgSchemaKind {
    Path,
    TargetExe,
    TargetLib,
    CustomTarget(String),
    String,
    Action,
    Variable,
    List,
    Out,
    Unset,
    BindValue { var_place: usize },
    Unknown,
}

#[derive(Clone, Debug)]
pub enum CommandSchemaFrom {
    External,
    Builtin,
    KeyWord,
}
