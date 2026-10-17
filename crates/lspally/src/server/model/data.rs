// AI-generated implementation.
//! Application payloads and URI text used by business handlers; no I/O.
use std::collections::BTreeMap;

/// Application data where the protocol explicitly allows an arbitrary JSON value.
/// Business handlers use this instead of depending on JSON serialization types.
/// [`super::Extension`] is the separate escape hatch for raw extension messages.
/// Construction has no side effects. Floating-point values must be finite when sent.
///
/// # Examples
/// ```
/// use lspally::server::model::Data;
/// let arguments = Data::List(vec![Data::Text("build".into()), Data::Integer(2)]);
/// ```
#[derive(Debug, Default)]
pub enum Data {
    /// JSON null and the default payload. Example: `Data::Null`.
    #[default]
    Null,
    /// Boolean payload. Example: `Data::Bool(true)`.
    Bool(
        /// Boolean value. Example: `Data::Bool(false)`.
        bool,
    ),
    /// Signed whole number. Example: `Data::Integer(-1)`.
    Integer(
        /// Signed value. Example: `Data::Integer(42)`.
        i64,
    ),
    /// Nonnegative whole number, including values above `i64::MAX`.
    /// Example: `Data::Unsigned(u64::MAX)`.
    Unsigned(
        /// Unsigned value. Example: `Data::Unsigned(42)`.
        u64,
    ),
    /// Floating-point payload; sending NaN or infinity returns an error.
    /// Example: `Data::Float(0.5)`.
    Float(
        /// Numeric value. Example: `Data::Float(1.25)`.
        f64,
    ),
    /// Unicode string payload. Example: `Data::Text("build".into())`.
    Text(
        /// Owned text. Example: `Data::Text(String::new())`.
        String,
    ),
    /// Ordered JSON array. Example: `Data::List(vec![Data::Null])`.
    List(
        /// Elements in transmission order. Example: `Data::List(vec![])`.
        Vec<Data>,
    ),
    /// JSON object with sorted string keys. Example: `Data::Object(Default::default())`.
    Object(
        /// Key/value entries. Example: `Data::Object(BTreeMap::from([("ok".into(), Data::Bool(true))]))`.
        BTreeMap<String, Data>,
    ),
}
/// One of two protocol representations, selected by the sender; no side effects.
/// Capability and edit records use this when either representation is legal.
/// Unlike [`Data`], both payload types are fixed by the containing field.
///
/// # Examples
/// ```
/// use lspally::server::model::OneOf;
/// let supported: OneOf<bool, String> = OneOf::Left(true);
/// ```
#[derive(Debug)]
pub enum OneOf<L, R> {
    /// First representation. Example: `OneOf::<bool, String>::Left(true)`.
    Left(
        /// First-type payload. Example: `OneOf::<bool, String>::Left(false)`.
        L,
    ),
    /// Second representation. Example: `OneOf::<bool, String>::Right("options".into())`.
    Right(
        /// Second-type payload. Example: `OneOf::<bool, String>::Right(String::new())`.
        R,
    ),
}
/// Owned resource URI text used by [`super::TextDocumentIdentifier`] and locations.
/// Construction does not validate the text or access the resource. The outgoing
/// adapter validates URI syntax before sending it; this is not a filesystem path.
///
/// # Examples
/// ```
/// use lspally::server::model::Uri;
/// let uri = Uri("file:///tmp/CMakeLists.txt".into());
/// assert!(uri.0.starts_with("file:"));
/// ```
#[derive(Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Uri(
    /// URI spelling, including scheme and escaping. Example: `Uri("file:///tmp/a%20b".into()).0`.
    pub String,
);
