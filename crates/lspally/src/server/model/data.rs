// AI-generated implementation.
use std::collections::BTreeMap;

/// Only used where LSP explicitly allows arbitrary application data.
#[derive(Debug, Default)]
pub enum Data {
    #[default]
    Null,
    Bool(bool),
    Integer(i64),
    Unsigned(u64),
    Float(f64),
    Text(String),
    List(Vec<Data>),
    Object(BTreeMap<String, Data>),
}
#[derive(Debug)]
pub enum OneOf<L, R> {
    Left(L),
    Right(R),
}
#[derive(Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Uri(pub String);
