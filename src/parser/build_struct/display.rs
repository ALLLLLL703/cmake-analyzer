//! Stream escaped, optionally colored build facts and borrowed diagnostics to a formatter.
//! Formatting does not load dependencies, evaluate CMake, or write files directly.
// Contains AI-generated documentation additions.
// AI-generated implementation.
use super::model::{Argument, BuildDisplay, BuildEffect, BuildFact, BuildStruct};
use crate::{
    model::{Spanned, TextSpan},
    parser::semanticer::model::{SemanticError, SemanticWarn},
};
use owo_colors::{OwoColorize, Style};
use std::fmt::{self, Display, Formatter};

/// Report title style; ignored when color output is disabled.
const TITLE: Style = Style::new().bright_magenta().bold();
/// Section heading style; ignored when color output is disabled.
const HEADING: Style = Style::new().bright_blue().bold();
/// File, command and visit identifiers style; ignored when color output is disabled.
const ID: Style = Style::new().bright_cyan();
/// Declared names style; ignored when color output is disabled.
const NAME: Style = Style::new().cyan().bold();
/// Literal metadata values style; ignored when color output is disabled.
const VALUE: Style = Style::new().bright_green();
/// Source paths and block labels style; ignored when color output is disabled.
const PATH: Style = Style::new().blue().bold();
/// Possible variable output names style; ignored when color output is disabled.
const WRITE: Style = Style::new().green().bold();
/// Scope and deferred-context labels style; ignored when color output is disabled.
const SCOPE: Style = Style::new().magenta();
/// Unresolved or conditional metadata style; ignored when color output is disabled.
const UNKNOWN: Style = Style::new().bright_yellow().bold();
/// Error labels style; ignored when color output is disabled.
const ERROR: Style = Style::new().bright_red().bold();
/// Coordinates and tree guides style; ignored when color output is disabled.
const MUTED: Style = Style::new().bright_black();
/// Successful or local-state labels style; ignored when color output is disabled.
const GOOD: Style = Style::new().green();

impl BuildStruct {
    /// Create a borrowed build-fact display adapter with no attached semantic diagnostics. Does not
    /// render, copy facts or perform I/O.
    ///
    /// # Arguments
    /// - `self` `[in]`: Build output borrowed for the adapter lifetime.
    /// - `colored` `[in]`: Whether later formatting uses terminal styles.
    ///
    /// # Returns
    /// BuildDisplay borrowing this output; use Display formatting to render it.
    ///
    /// # Examples
    /// ```
    /// use cmake_analyzer::parser::build_struct::model::BuildStruct;
    /// fn render(build: &BuildStruct) -> String { build.display(false).to_string() }
    /// ```
    pub fn display(&self, colored: bool) -> BuildDisplay<'_> {
        BuildDisplay {
            buffer: self,
            colored,
            semantic_diagnostics: &[],
            semantic_warnings: &[],
        }
    }
}
impl<'a> BuildDisplay<'a> {
    /// Include errors from earlier passes without copying them.
    /// Attach borrowed semantic errors to the display adapter without copying them or writing
    /// output.
    ///
    /// # Arguments
    /// - `self` `[in,out]` : Consumed adapter; its semantic error slice is overwritten in the
    ///   returned value.
    /// - `diagnostics` `[in]`: Borrowed errors that outlive the adapter.
    ///
    /// # Returns
    /// Updated adapter retaining the same build facts and color setting.
    ///
    /// # Examples
    /// ```
    /// use cmake_analyzer::parser::build_struct::model::BuildStruct;
    /// use cmake_analyzer::{model::Spanned, parser::semanticer::model::SemanticError};
    /// fn render(build: &BuildStruct, errors: &[Spanned<SemanticError>]) -> String {
    ///     build.display(false).with_semantic_diagnostics(errors).to_string()
    /// }
    /// ```
    pub fn with_semantic_diagnostics(mut self, diagnostics: &'a [Spanned<SemanticError>]) -> Self {
        self.semantic_diagnostics = diagnostics;
        self
    }
    /// Attach borrowed semantic warnings without copying them or writing output.
    ///
    /// # Arguments
    /// - `self` `[in,out]` : Consumed adapter; its warning slice is overwritten in the returned
    ///   value.
    /// - `warnings` `[in]`: Borrowed warnings that outlive the adapter.
    ///
    /// # Returns
    /// Updated adapter retaining build facts, errors and color setting.
    ///
    /// # Examples
    /// ```
    /// use cmake_analyzer::parser::build_struct::model::BuildStruct;
    /// use cmake_analyzer::{model::Spanned, parser::semanticer::model::SemanticWarn};
    /// fn render(build: &BuildStruct, warnings: &[Spanned<SemanticWarn>]) -> String {
    ///     build.display(false).with_semantic_warnings(warnings).to_string()
    /// }
    /// ```
    pub fn with_semantic_warnings(mut self, warnings: &'a [Spanned<SemanticWarn>]) -> Self {
        self.semantic_warnings = warnings;
        self
    }
    /// Append a styled or plain value according to the view's color flag. No filesystem/network
    /// access.
    ///
    /// # Arguments
    /// - `self` `[in]`: Borrowed build view and style policy.
    /// - `f` `[in,out]`: Formatter; appends rendered text without clearing earlier output.
    /// - `style` `[in]`: Terminal style applied only when colored is true.
    /// - `text` `[in]`: Display value written once to the formatter.
    ///
    /// # Returns
    /// Ok(()) after appending the requested text; Err(fmt::Error) on formatter failure.
    ///
    /// # Errors
    /// Returns fmt::Error if writing any fragment to the formatter fails; already appended output
    /// is not rolled back.
    ///
    /// # Examples
    /// ```text
    /// view.paint(f, MUTED, "scope")?;
    /// ```
    fn paint(&self, f: &mut Formatter<'_>, style: Style, text: impl Display) -> fmt::Result {
        if self.colored {
            write!(f, "{}", text.style(style))
        } else {
            write!(f, "{text}")
        }
    }
    /// Append one-based row/column plus the original half-open byte range. No analysis mutation or
    /// I/O beyond formatter output.
    ///
    /// # Arguments
    /// - `self` `[in]`: Borrowed build view and style policy.
    /// - `f` `[in,out]`: Formatter; appends rendered text without clearing earlier output.
    /// - `span` `[in]`: Source coordinates; row/column are converted from zero-based for display.
    ///
    /// # Returns
    /// Ok(()) after appending the requested text; Err(fmt::Error) on formatter failure.
    ///
    /// # Errors
    /// Returns fmt::Error if writing any fragment to the formatter fails; already appended output
    /// is not rolled back.
    ///
    /// # Examples
    /// ```text
    /// view.span(f, fact.source.span)?;
    /// ```
    fn span(&self, f: &mut Formatter<'_>, span: TextSpan) -> fmt::Result {
        self.paint(
            f,
            MUTED,
            format_args!(
                "@ {}:{} bytes {}..{}",
                span.row + 1,
                span.column + 1,
                span.start_byte,
                span.end_byte
            ),
        )
    }
    /// Append debug-escaped operand text and mark nonliteral values unresolved. No source mutation
    /// or filesystem access.
    ///
    /// # Arguments
    /// - `self` `[in]`: Borrowed build view and style policy.
    /// - `f` `[in,out]`: Formatter; appends rendered text without clearing earlier output.
    /// - `arg` `[in]`: Operand whose text is escaped to prevent terminal-control injection.
    /// - `style` `[in]`: Requested literal style; unresolved operands use warning style instead.
    ///
    /// # Returns
    /// Ok(()) after appending the requested text; Err(fmt::Error) on formatter failure.
    ///
    /// # Errors
    /// Returns fmt::Error if writing any fragment to the formatter fails; already appended output
    /// is not rolled back.
    ///
    /// # Examples
    /// ```text
    /// view.argument(f, &target.name, NAME)?;
    /// ```
    fn argument(&self, f: &mut Formatter<'_>, arg: &Argument, style: Style) -> fmt::Result {
        // Escape source text so a filename or expression cannot inject terminal controls.
        self.paint(
            f,
            if arg.literal { style } else { UNKNOWN },
            format_args!("{:?}", arg.text),
        )?;
        if !arg.literal {
            write!(f, " ")?;
            self.paint(f, UNKNOWN, "[unresolved]")?;
        }
        Ok(())
    }
    /// Append a labeled bracketed argument list and newline. No analysis mutation or filesystem
    /// access.
    ///
    /// # Arguments
    /// - `self` `[in]`: Borrowed build view and style policy.
    /// - `f` `[in,out]`: Formatter; appends rendered text without clearing earlier output.
    /// - `label` `[in]`: Trusted field label.
    /// - `args` `[in]`: Ordered operands formatted with escaping and unresolved markers.
    /// - `style` `[in]`: Style requested for literal operands.
    ///
    /// # Returns
    /// Ok(()) after appending the requested text; Err(fmt::Error) on formatter failure.
    ///
    /// # Errors
    /// Returns fmt::Error if writing any fragment to the formatter fails; already appended output
    /// is not rolled back.
    ///
    /// # Examples
    /// ```text
    /// view.arguments(f, "sources", &target.sources, PATH)?;
    /// ```
    fn arguments(
        &self,
        f: &mut Formatter<'_>,
        label: &str,
        args: &[Argument],
        style: Style,
    ) -> fmt::Result {
        write!(f, "    {label}=[")?;
        for (index, arg) in args.iter().enumerate() {
            if index > 0 {
                write!(f, ", ")?;
            }
            self.argument(f, arg, style)?;
        }
        writeln!(f, "]")
    }
    /// Append a labeled optional operand and newline, or nothing for None. No analysis mutation or
    /// filesystem access.
    ///
    /// # Arguments
    /// - `self` `[in]`: Borrowed build view and style policy.
    /// - `f` `[in,out]`: Formatter; appends rendered text without clearing earlier output.
    /// - `label` `[in]`: Trusted field label.
    /// - `value` `[in]`: Optional operand to print with the value style.
    ///
    /// # Returns
    /// Ok(()) after appending the requested text; Err(fmt::Error) on formatter failure.
    ///
    /// # Errors
    /// Returns fmt::Error if writing any fragment to the formatter fails; already appended output
    /// is not rolled back.
    ///
    /// # Examples
    /// ```text
    /// view.field(f, "version", project.version.as_ref())?;
    /// ```
    fn field(&self, f: &mut Formatter<'_>, label: &str, value: Option<&Argument>) -> fmt::Result {
        if let Some(value) = value {
            write!(f, "    {label}=")?;
            self.argument(f, value, VALUE)?;
            writeln!(f)?;
        }
        Ok(())
    }
    /// Append one fact's source context and effect-specific payload. Escapes source text; never
    /// loads dependencies or changes analysis.
    ///
    /// # Arguments
    /// - `self` `[in]`: Borrowed build view and style policy.
    /// - `f` `[in,out]`: Formatter; appends rendered text without clearing earlier output.
    /// - `fact` `[in]`: Ordered structural fact with qualified command origin.
    ///
    /// # Returns
    /// Ok(()) after appending the requested text; Err(fmt::Error) on formatter failure.
    ///
    /// # Errors
    /// Returns fmt::Error if writing any fragment to the formatter fails; already appended output
    /// is not rolled back.
    ///
    /// # Algorithm
    /// Print command/scope/block IDs, conditional/deferred markers and source coordinates, then
    /// dispatch by effect kind to typed metadata, raw operands or possible writes. Nested source
    /// sections/file sets retain order.
    ///
    /// # Flow
    /// ```text
    /// [fact origin]
    ///        |
    ///  [IDs/context/span]
    ///        |
    ///   <effect kind>
    ///    /       |        \
    ///  project  target   sources/links/dependency/writes
    ///    |       |                 |
    ///  [fields] [flags/args] [ordered groups/operands]
    ///    +-------+-----------------+
    ///                 |
    ///           [formatter output]
    /// ```
    ///
    /// # Examples
    /// ```text
    /// view.fact(f, &build.facts[0])?;
    /// ```
    fn fact(&self, f: &mut Formatter<'_>, fact: &BuildFact) -> fmt::Result {
        let label = match fact.effect {
            BuildEffect::Project(_) => "PROJECT",
            BuildEffect::Minimum { .. } => "MINIMUM",
            BuildEffect::Languages { .. } => "LANGUAGES",
            BuildEffect::Target(_) => "TARGET",
            BuildEffect::Sources(_) => "SOURCES",
            BuildEffect::Links { .. } => "LINKS",
            BuildEffect::Dependency(_) => "DEPENDENCY",
            BuildEffect::VariableWrites { .. } => "WRITES",
        };
        write!(f, "  ")?;
        self.paint(f, ID, format_args!("C{}", fact.source.command.id))?;
        write!(f, " ")?;
        self.paint(f, HEADING, label)?;
        write!(f, " scope=")?;
        match fact.source.scope {
            Some(scope) => self.paint(f, SCOPE, format_args!("S{}", scope.id))?,
            None => self.paint(f, MUTED, "file/caller")?,
        }
        if let Some(block) = fact.source.block {
            write!(f, " block=")?;
            self.paint(f, PATH, format_args!("B{}", block.id))?;
        }
        // These flags describe where the command sits, not whether CMake ran it.
        if fact.source.conditional {
            write!(f, " ")?;
            self.paint(f, UNKNOWN, "[conditional]")?;
        }
        if fact.source.deferred {
            write!(f, " ")?;
            self.paint(f, SCOPE, "[deferred]")?;
        }
        write!(f, "  ")?;
        self.span(f, fact.source.span)?;
        writeln!(f)?;
        match &fact.effect {
            BuildEffect::Project(project) => {
                write!(f, "    name=")?;
                self.argument(f, &project.name, NAME)?;
                writeln!(f)?;
                self.field(f, "version", project.version.as_ref())?;
                self.field(f, "description", project.description.as_ref())?;
                self.field(f, "homepage", project.homepage.as_ref())?;
                self.arguments(f, "languages", &project.languages, VALUE)?;
                self.arguments(f, "raw", &project.arguments, MUTED)?;
            }
            BuildEffect::Minimum { arguments } | BuildEffect::Languages { arguments } => {
                self.arguments(f, "args", arguments, VALUE)?
            }
            BuildEffect::Target(target) => {
                write!(f, "    name=")?;
                self.argument(f, &target.name, NAME)?;
                write!(f, " kind=")?;
                self.paint(f, NAME, format_args!("{:?}", target.kind))?;
                if target.imported {
                    write!(f, " ")?;
                    self.paint(f, UNKNOWN, "IMPORTED")?;
                }
                if target.global {
                    write!(f, " ")?;
                    self.paint(f, SCOPE, "GLOBAL")?;
                }
                writeln!(f)?;
                self.field(f, "alias", target.alias.as_ref())?;
                self.arguments(f, "sources", &target.sources, PATH)?;
                self.arguments(f, "raw", &target.arguments, MUTED)?;
            }
            BuildEffect::Sources(sources) => {
                write!(f, "    target=")?;
                self.argument(f, &sources.target, NAME)?;
                writeln!(f)?;
                for section in &sources.sections {
                    write!(f, "    visibility=")?;
                    match section.visibility {
                        Some(visibility) => self.paint(f, SCOPE, format_args!("{visibility:?}"))?,
                        None => self.paint(f, MUTED, "unspecified")?,
                    }
                    writeln!(f)?;
                    self.arguments(f, "files", &section.files, PATH)?;
                    for set in &section.file_sets {
                        write!(f, "    FILE_SET ")?;
                        self.argument(f, &set.name, NAME)?;
                        writeln!(f)?;
                        self.field(f, "type", set.kind.as_ref())?;
                        self.arguments(f, "base_dirs", &set.base_dirs, PATH)?;
                        self.arguments(f, "files", &set.files, PATH)?;
                    }
                }
                self.arguments(f, "raw", &sources.arguments, MUTED)?;
            }
            BuildEffect::Links { target, arguments } => {
                write!(f, "    target=")?;
                self.argument(f, target, NAME)?;
                writeln!(f)?;
                self.arguments(f, "items", arguments, VALUE)?;
            }
            BuildEffect::Dependency(dependency) => {
                write!(f, "    ")?;
                self.paint(f, NAME, format_args!("{:?}", dependency.kind))?;
                write!(f, " path=")?;
                self.argument(f, &dependency.path, PATH)?;
                if dependency.optional {
                    write!(f, " ")?;
                    self.paint(f, GOOD, "OPTIONAL")?;
                }
                if dependency.no_policy_scope {
                    write!(f, " ")?;
                    self.paint(f, SCOPE, "NO_POLICY_SCOPE")?;
                }
                writeln!(f)?;
                self.field(f, "binary_dir", dependency.binary.as_ref())?;
            }
            BuildEffect::VariableWrites { names, local } => {
                write!(f, "    names=[")?;
                for (index, name) in names.iter().enumerate() {
                    if index > 0 {
                        write!(f, ", ")?;
                    }
                    self.paint(f, WRITE, format_args!("{name:?}"))?;
                }
                write!(f, "] ")?;
                self.paint(
                    f,
                    if *local { GOOD } else { UNKNOWN },
                    if *local {
                        "local destination"
                    } else {
                        "nonlocal destination unresolved"
                    },
                )?;
                writeln!(f)?;
            }
        }
        Ok(())
    }
    /// Append semantic errors, build errors and semantic warnings with a total count. Prints a
    /// clean marker when empty; no analysis mutation or filesystem access.
    ///
    /// # Arguments
    /// - `self` `[in]`: Borrowed build view and style policy.
    /// - `f` `[in,out]`: Formatter; appends rendered text without clearing earlier output.
    ///
    /// # Returns
    /// Ok(()) after appending the requested text; Err(fmt::Error) on formatter failure.
    ///
    /// # Errors
    /// Returns fmt::Error if writing any fragment to the formatter fails; already appended output
    /// is not rolled back.
    ///
    /// # Algorithm
    /// Count borrowed and build diagnostics, write each category in order with spans and escaped
    /// messages, then emit No diagnostics only if all categories are empty.
    ///
    /// # Flow
    /// ```text
    /// [semantic errors]   [build errors]   [warnings]
    ///         |                  |             |
    ///         +-----> [total + heading] <------+
    ///                           |
    ///                   [render categories]
    ///                           |
    ///                      <empty?>
    ///                       /    \
    ///                  clean     done
    /// ```
    ///
    /// # Examples
    /// ```text
    /// view.diagnostics(f)?;
    /// ```
    fn diagnostics(&self, f: &mut Formatter<'_>) -> fmt::Result {
        writeln!(f)?;
        self.paint(f, HEADING, "DIAGNOSTICS")?;
        write!(f, " ")?;
        self.paint(
            f,
            ID,
            format_args!(
                "({})",
                self.semantic_diagnostics.len()
                    + self.buffer.diagnostics.len()
                    + self.semantic_warnings.len()
            ),
        )?;
        writeln!(f)?;
        for diagnostic in self.semantic_diagnostics {
            write!(f, "  ")?;
            self.paint(f, ERROR, "SEMANTIC")?;
            write!(f, " ")?;
            self.span(f, diagnostic.span)?;
            write!(f, " ")?;
            self.paint(f, ERROR, format_args!("{:?}", diagnostic.content))?;
            writeln!(f)?;
        }
        for diagnostic in &self.buffer.diagnostics {
            write!(f, "  ")?;
            self.paint(f, ERROR, "BUILD")?;
            write!(f, " ")?;
            self.span(f, diagnostic.source.span)?;
            write!(f, " ")?;
            self.paint(f, ERROR, format_args!("{:?}", diagnostic.message))?;
            writeln!(f)?;
        }
        for warning in self.semantic_warnings {
            self.paint(f, UNKNOWN, "  WARNING ")?;
            self.span(f, warning.span)?;
            writeln!(f, " {:?}", warning.content.message())?;
        }
        if self.semantic_diagnostics.is_empty()
            && self.buffer.diagnostics.is_empty()
            && self.semantic_warnings.is_empty()
        {
            write!(f, "  ")?;
            self.paint(f, GOOD, "No diagnostics")?;
            writeln!(f)?;
        }
        Ok(())
    }
}
impl Display for BuildDisplay<'_> {
    /// Append the full build-structure report through Display. Does not execute CMake, load
    /// dependencies or change facts.
    ///
    /// # Arguments
    /// - `self` `[in]`: Borrowed build view and style policy.
    /// - `f` `[in,out]`: Formatter; appends rendered text without clearing earlier output.
    ///
    /// # Returns
    /// Ok(()) after appending the requested text; Err(fmt::Error) on formatter failure.
    ///
    /// # Errors
    /// Returns fmt::Error if writing any fragment to the formatter fails; already appended output
    /// is not rolled back.
    ///
    /// # Algorithm
    /// Write identity and static-analysis notice, then ordered facts and all attached diagnostics.
    ///
    /// # Flow
    /// ```text
    /// [header + revision]
    ///           |
    ///       [fact loop] <---+
    ///           |          |
    ///        <more?> -- yes-+
    ///           | no
    ///      [diagnostics]
    ///           |
    ///        [return]
    /// ```
    ///
    /// # Examples
    /// ```
    /// use cmake_analyzer::parser::build_struct::model::BuildStruct;
    /// fn report(build: &BuildStruct) -> String {
    ///     format!("{}", build.display(false))
    /// }
    /// ```
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.paint(f, TITLE, "CMAKE BUILD STRUCTURE")?;
        writeln!(
            f,
            "  file={} revision={}",
            self.buffer.analysis.file, self.buffer.analysis.revision
        )?;
        writeln!(
            f,
            "Static facts, not configured targets. No dependency loading."
        )?;
        writeln!(f)?;
        self.paint(f, HEADING, "FACTS")?;
        write!(f, " ")?;
        self.paint(f, ID, format_args!("({})", self.buffer.facts.len()))?;
        writeln!(f)?;
        for fact in &self.buffer.facts {
            self.fact(f, fact)?;
        }
        self.diagnostics(f)
    }
}
