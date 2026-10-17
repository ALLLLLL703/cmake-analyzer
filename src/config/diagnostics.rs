//! Protocol-independent diagnostic switches and ordered severities. Constructing or
//! comparing these values performs no I/O; the server decides which reports to publish.
//!
//! Contains AI-generated documentation additions.
/// Strongest first; shared owners select the minimum, with None meaning disabled.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
/// Ordered report severity; smaller values are stronger, with None disabled.
/// Shared enabled owners take the minimum. [`DiagnosticRules`] assigns levels to rules.
/// Comparison and construction have no side effects.
///
/// # Examples
/// ```
/// # use cmake_analyzer::config::DiagnosticLevel;
/// assert!(DiagnosticLevel::Error < DiagnosticLevel::Warn);
/// ```
pub enum DiagnosticLevel {
    /// Report an error; strongest level. Example: `let level = DiagnosticLevel::Error`.
    Error,
    /// Report a warning. Example: `let level = DiagnosticLevel::Warn`.
    Warn,
    /// Report informational guidance. Example: `let level = DiagnosticLevel::Info`.
    Info,
    /// Report a low-priority hint. Example: `let level = DiagnosticLevel::Hint`.
    Hint,
    /// Disable this rule; weaker than every report level. Example: `let level = DiagnosticLevel::None`.
    None,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
/// Independent severities for unresolved functions and unset/possibly-unset variables.
/// [`Diagnostics`] holds the master switch; constructing rules performs no publication.
///
/// # Examples
/// ```
/// # use cmake_analyzer::config::{DiagnosticRules, DiagnosticLevel};
/// let rules = DiagnosticRules::default();
/// assert_eq!(rules.unknown_function, DiagnosticLevel::Error);
/// ```
pub struct DiagnosticRules {
    /// Unresolved command severity; default Error. Example: `rules.unknown_function = DiagnosticLevel::None`.
    pub unknown_function: DiagnosticLevel,
    /// Unset/possibly-unset read severity; default Warn; known-empty warnings are separate. Example: `rules.unknown_variable = DiagnosticLevel::Hint`.
    pub unknown_variable: DiagnosticLevel,
}
impl Default for DiagnosticRules {
    /// Build default diagnostic policy in memory. No publication or I/O.
    ///
    /// # Arguments
    /// None.
    ///
    /// # Returns
    /// Error for unknown functions and Warn for unknown variables.
    ///
    /// # Examples
    /// ```
    /// # use cmake_analyzer::config::DiagnosticRules;
    /// let policy = DiagnosticRules::default();
    /// ```
    fn default() -> Self {
        Self {
            unknown_function: DiagnosticLevel::Error,
            unknown_variable: DiagnosticLevel::Warn,
        }
    }
}
#[derive(Clone, Debug)]
/// Master diagnostic publication policy with per-rule levels in [`DiagnosticRules`].
/// Stored in [`super::Settings`]; construction changes no client or filesystem state.
///
/// # Examples
/// ```
/// # use cmake_analyzer::config::Diagnostics;
/// assert!(Diagnostics::default().enable);
/// ```
pub struct Diagnostics {
    /// Master report switch; defaults true. Example: `diagnostics.enable = false`.
    pub enable: bool,
    /// Independent unresolved-name rule choices. Example: `diagnostics.rules.unknown_function = DiagnosticLevel::Warn`.
    pub rules: DiagnosticRules,
}
impl Default for Diagnostics {
    /// Build default diagnostic policy in memory. No publication or I/O.
    ///
    /// # Arguments
    /// None.
    ///
    /// # Returns
    /// Enabled reports with default unknown-name levels.
    ///
    /// # Examples
    /// ```
    /// # use cmake_analyzer::config::Diagnostics;
    /// let policy = Diagnostics::default();
    /// ```
    fn default() -> Self {
        Self {
            enable: true,
            rules: DiagnosticRules::default(),
        }
    }
}
