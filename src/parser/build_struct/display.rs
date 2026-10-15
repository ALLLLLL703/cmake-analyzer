// AI-generated implementation.
use super::model::{Argument, BuildDisplay, BuildEffect, BuildFact, BuildStruct};
use crate::{
    model::{Spanned, TextSpan},
    parser::semanticer::model::{SemanticError, SemanticWarn},
};
use owo_colors::{OwoColorize, Style};
use std::fmt::{self, Display, Formatter};

const TITLE: Style = Style::new().bright_magenta().bold();
const HEADING: Style = Style::new().bright_blue().bold();
const ID: Style = Style::new().bright_cyan();
const NAME: Style = Style::new().cyan().bold();
const VALUE: Style = Style::new().bright_green();
const PATH: Style = Style::new().blue().bold();
const WRITE: Style = Style::new().green().bold();
const SCOPE: Style = Style::new().magenta();
const UNKNOWN: Style = Style::new().bright_yellow().bold();
const ERROR: Style = Style::new().bright_red().bold();
const MUTED: Style = Style::new().bright_black();
const GOOD: Style = Style::new().green();

impl BuildStruct {
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
    pub fn with_semantic_diagnostics(mut self, diagnostics: &'a [Spanned<SemanticError>]) -> Self {
        self.semantic_diagnostics = diagnostics;
        self
    }
    pub fn with_semantic_warnings(mut self, warnings: &'a [Spanned<SemanticWarn>]) -> Self {
        self.semantic_warnings = warnings;
        self
    }
    fn paint(&self, f: &mut Formatter<'_>, style: Style, text: impl Display) -> fmt::Result {
        if self.colored {
            write!(f, "{}", text.style(style))
        } else {
            write!(f, "{text}")
        }
    }
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
    fn field(&self, f: &mut Formatter<'_>, label: &str, value: Option<&Argument>) -> fmt::Result {
        if let Some(value) = value {
            write!(f, "    {label}=")?;
            self.argument(f, value, VALUE)?;
            writeln!(f)?;
        }
        Ok(())
    }
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
