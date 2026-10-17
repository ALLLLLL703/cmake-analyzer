/// Strongest first; shared owners select the minimum, with None meaning disabled.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum DiagnosticLevel {
    Error,
    Warn,
    Info,
    Hint,
    None,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DiagnosticRules {
    pub unknown_function: DiagnosticLevel,
    pub unknown_variable: DiagnosticLevel,
}
impl Default for DiagnosticRules {
    fn default() -> Self {
        Self {
            unknown_function: DiagnosticLevel::Error,
            unknown_variable: DiagnosticLevel::Warn,
        }
    }
}
#[derive(Clone, Debug)]
pub struct Diagnostics {
    pub enable: bool,
    pub rules: DiagnosticRules,
}
impl Default for Diagnostics {
    fn default() -> Self {
        Self {
            enable: true,
            rules: DiagnosticRules::default(),
        }
    }
}
