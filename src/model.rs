pub mod trait1;

/// Half-open UTF-8 byte range; row/column locate its start (zero-based).
/// Columns count Unicode scalar values, not LSP UTF-16 code units.
#[derive(Default, Debug)]
pub struct TextSpan {
    pub column: u64,
    pub row: u64,
    pub start_byte: usize,
    pub end_byte: usize,
}
