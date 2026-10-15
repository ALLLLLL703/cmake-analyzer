// AI-authored filesystem roles; generic schema Path remains unspecified.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PathKind {
    File,
    Directory,
    Input,
    Module,
    Output,
}
#[derive(Clone, Debug)]
pub struct PathArgument {
    pub argument: usize,
    pub kind: PathKind,
}
