// AI-generated implementation.
use super::model::{
    ArgKind, CommandId, ScopeId, SemanticDisplay, SemanticError, SemanticSymbolKind,
    SemanticedBuffer,
};
use super::schema::CommandSchemaFrom;
use crate::model::TextSpan;
use owo_colors::{OwoColorize, Style};
use std::fmt::{self, Display, Formatter};

const TITLE: Style = Style::new().bright_magenta().bold();
const HEADING: Style = Style::new().bright_blue().bold();
const NUMBER: Style = Style::new().bright_cyan();
const COMMAND: Style = Style::new().cyan().bold();
const KEYWORD: Style = Style::new().magenta().bold();
const VARIABLE: Style = Style::new().green().bold();
const OUTPUT: Style = Style::new().bright_yellow().bold();
const SCOPE: Style = Style::new().bright_magenta();
const BLOCK: Style = Style::new().blue();
const TEXT: Style = Style::new().bright_white();
const STRING: Style = Style::new().bright_green();
const BRACKET: Style = Style::new().cyan();
const POSITION: Style = Style::new().bright_black();
const ERROR: Style = Style::new().bright_red().bold();
const SUCCESS: Style = Style::new().green();

impl SemanticedBuffer {
    pub fn display(&self, colored: bool) -> SemanticDisplay<'_> {
        SemanticDisplay {
            buffer: self,
            colored,
            all_variables: false,
        }
    }
}
impl SemanticDisplay<'_> {
    pub fn with_all_variables(mut self, enabled: bool) -> Self {
        self.all_variables = enabled;
        self
    }
    fn paint(&self, f: &mut Formatter<'_>, style: Style, text: impl Display) -> fmt::Result {
        if self.colored {
            write!(f, "{}", text.style(style))
        } else {
            write!(f, "{text}")
        }
    }
    fn heading(&self, f: &mut Formatter<'_>, name: &str, count: usize) -> fmt::Result {
        writeln!(f)?;
        self.paint(f, HEADING, name)?;
        write!(f, " ")?;
        self.paint(f, NUMBER, format_args!("({count})"))?;
        writeln!(f)
    }
    fn span(&self, f: &mut Formatter<'_>, span: TextSpan) -> fmt::Result {
        self.paint(
            f,
            POSITION,
            format_args!(
                "@ {}:{} bytes {}..{}",
                span.row + 1,
                span.column + 1,
                span.start_byte,
                span.end_byte
            ),
        )
    }
    fn scope(&self, f: &mut Formatter<'_>, scope: Option<ScopeId>) -> fmt::Result {
        match scope {
            Some(id) => self.paint(f, SCOPE, format_args!("S{}", id.id)),
            None => self.paint(f, POSITION, "global"),
        }
    }
    fn commands(&self, f: &mut Formatter<'_>, ids: impl Iterator<Item = CommandId>) -> fmt::Result {
        write!(f, "[")?;
        for (index, id) in ids.enumerate() {
            if index > 0 {
                write!(f, ", ")?;
            }
            self.paint(f, NUMBER, format_args!("C{}", id.id))?;
        }
        write!(f, "]")
    }
}
impl Display for SemanticDisplay<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let buffer = self.buffer;
        self.paint(f, TITLE, "CMAKE SEMANTICS")?;
        write!(f, "  version=")?;
        match buffer.version {
            Some(version) => self.paint(
                f,
                NUMBER,
                format_args!("{}.{}.{}", version.major, version.minor, version.patch),
            )?,
            None => self.paint(f, ERROR, "unavailable")?,
        };
        writeln!(f, "  file={}", buffer.id.id)?;
        writeln!(
            f,
            "Static structure and possible writes; no CMake execution."
        )?;

        self.heading(f, "ROOT SYMBOLS", buffer.symbols.len())?;
        for symbol in &buffer.symbols {
            write!(f, "  ")?;
            match symbol.kind {
                SemanticSymbolKind::Command(id) => {
                    self.paint(f, NUMBER, format_args!("C{}", id.id))?;
                    if let Some(command) = buffer.commands.get(id.id as usize) {
                        write!(f, " ")?;
                        self.paint(f, COMMAND, format_args!("{:?}", command.name))?;
                    }
                }
                SemanticSymbolKind::Block(id) => {
                    self.paint(f, BLOCK, format_args!("B{}", id.id))?;
                    if let Some(block) = buffer.blocks.get(id.id as usize) {
                        write!(f, " ")?;
                        self.paint(f, KEYWORD, format_args!("{:?}", block.kind))?;
                    }
                }
            }
            write!(f, "  ")?;
            self.span(f, symbol.span)?;
            writeln!(f)?;
        }

        self.heading(f, "COMMANDS", buffer.commands.len())?;
        for command in &buffer.commands {
            let (color, role) = match command.schema.as_ref().map(|schema| &schema.from) {
                Some(CommandSchemaFrom::KeyWord) => (KEYWORD, "keyword"),
                Some(CommandSchemaFrom::Builtin) => (COMMAND, "builtin"),
                Some(CommandSchemaFrom::External) => (VARIABLE, "external"),
                None => (ERROR, "unknown"),
            };
            write!(f, "  ")?;
            self.paint(f, NUMBER, format_args!("C{}", command.id.id))?;
            write!(f, " ")?;
            self.paint(f, color, format_args!("{:?}", command.name))?;
            write!(f, " ")?;
            self.paint(f, color, role)?;
            write!(f, " scope=")?;
            self.scope(f, command.scope)?;
            if let Some(block) = command.block {
                write!(f, " block=")?;
                self.paint(f, BLOCK, format_args!("B{}", block.id))?;
            }
            write!(f, " ")?;
            self.paint(
                f,
                if command.closed { SUCCESS } else { ERROR },
                if command.closed { "closed" } else { "unclosed" },
            )?;
            write!(f, "  ")?;
            self.span(f, command.full_span)?;
            writeln!(f)?;
            for (index, arg) in command.args.iter().enumerate() {
                let output = arg.kind.iter().any(|kind| matches!(kind, ArgKind::Out));
                let color = if output {
                    OUTPUT
                } else {
                    match arg.form {
                        super::model::SemanticArgForm::Quoted => STRING,
                        super::model::SemanticArgForm::Bracket => BRACKET,
                        super::model::SemanticArgForm::Unquoted => TEXT,
                        _ => POSITION,
                    }
                };
                write!(f, "    ")?;
                self.paint(f, NUMBER, format_args!("[{index}]"))?;
                write!(f, " ")?;
                self.paint(f, color, format_args!("{:?}", arg.content))?;
                write!(f, " ")?;
                self.paint(f, POSITION, format_args!("{:?}", arg.form))?;
                if output {
                    write!(f, " ")?;
                    self.paint(f, OUTPUT, "OUT")?;
                }
                write!(f, "  ")?;
                self.span(f, arg.span)?;
                writeln!(f)?;
            }
        }

        self.heading(f, "BLOCKS", buffer.blocks.len())?;
        for block in &buffer.blocks {
            write!(f, "  ")?;
            self.paint(f, BLOCK, format_args!("B{}", block.id.id))?;
            write!(f, " ")?;
            self.paint(f, KEYWORD, format_args!("{:?}", block.kind))?;
            write!(f, " ")?;
            self.paint(
                f,
                if block.closed { SUCCESS } else { ERROR },
                if block.closed { "closed" } else { "UNCLOSED" },
            )?;
            if let Some(parent) = block.parent {
                write!(f, " parent=")?;
                self.paint(f, BLOCK, format_args!("B{}", parent.id))?;
            }
            write!(f, "  ")?;
            self.span(f, block.span)?;
            writeln!(f)?;
            write!(f, "    direct=")?;
            self.commands(f, block.contains.iter().copied())?;
            write!(f, " children=[")?;
            for (index, child) in block.children.iter().enumerate() {
                if index > 0 {
                    write!(f, ", ")?;
                }
                self.paint(f, BLOCK, format_args!("B{}", child.id))?;
            }
            writeln!(f, "]")?;
        }

        self.heading(f, "SCOPES", buffer.scopes.len())?;
        for scope in &buffer.scopes {
            write!(f, "  ")?;
            self.scope(f, Some(scope.id))?;
            write!(f, " ")?;
            self.paint(f, SCOPE, format_args!("{:?}", scope.kind))?;
            write!(f, " block=")?;
            self.paint(f, BLOCK, format_args!("B{}", scope.block.id))?;
            write!(f, " parent=")?;
            self.scope(f, scope.parent)?;
            writeln!(f)?;
        }

        let visible = buffer
            .variables
            .iter()
            .filter(|variable| {
                self.all_variables
                    || !variable.changing.is_empty()
                    || !variable.references.is_empty()
            })
            .count();
        self.heading(f, "VARIABLES", visible)?;
        if !self.all_variables {
            self.paint(
                f,
                POSITION,
                format_args!(
                    "  {} unchanged records hidden; use --all-variables to include them",
                    buffer.variables.len() - visible
                ),
            )?;
            writeln!(f)?;
        }
        for variable in &buffer.variables {
            if !self.all_variables && variable.changing.is_empty() && variable.references.is_empty()
            {
                continue;
            }
            write!(f, "  ")?;
            self.paint(f, VARIABLE, format_args!("{:?}", variable.name))?;
            write!(f, " scope=")?;
            self.scope(f, variable.scope)?;
            write!(f, " initial=")?;
            self.paint(f, STRING, format_args!("{:?}", variable.init_value))?;
            write!(f, " writes=")?;
            self.commands(f, variable.changing.iter().map(|change| change.command))?;
            writeln!(f)?;
            for reference in &variable.references {
                write!(
                    f,
                    "    {:?} F{} C{} ",
                    reference.kind, reference.file.id, reference.command.id
                )?;
                self.span(f, reference.span)?;
                writeln!(f)?;
            }
        }

        self.heading(f, "FUNCTIONS", buffer.functions.len())?;
        for function in &buffer.functions {
            write!(f, "  {:?} ", function.kind)?;
            self.paint(f, COMMAND, format_args!("{:?}", function.name))?;
            writeln!(f)?;
            for reference in &function.references {
                write!(
                    f,
                    "    {:?} F{} C{} ",
                    reference.kind, reference.file.id, reference.command.id
                )?;
                self.span(f, reference.span)?;
                writeln!(f)?;
            }
        }
        self.heading(f, "WARNINGS", buffer.warnings.len())?;
        for warning in &buffer.warnings {
            self.paint(f, OUTPUT, "  WARNING ")?;
            self.span(f, warning.span)?;
            writeln!(f, " {:?}", warning.content.message())?;
        }
        self.heading(f, "DIAGNOSTICS", buffer.diagnostics.len())?;
        if buffer.diagnostics.is_empty() {
            write!(f, "  ")?;
            self.paint(
                f,
                SUCCESS,
                if buffer.warnings.is_empty() {
                    "No diagnostics"
                } else {
                    "No errors"
                },
            )?;
            writeln!(f)?;
        }
        for diagnostic in &buffer.diagnostics {
            write!(f, "  ")?;
            self.paint(f, ERROR, "ERROR")?;
            write!(f, " ")?;
            self.span(f, diagnostic.span)?;
            write!(f, " ")?;
            match &diagnostic.content {
                SemanticError::CommandNotFound(name) => {
                    self.paint(f, ERROR, format_args!("Unknown command {:?}", name))?
                }
                SemanticError::UnclosedBlock(kind) => {
                    self.paint(f, ERROR, format_args!("Unclosed {:?} block", kind))?
                }
                SemanticError::UnexpectedBlockEnd(name) => {
                    self.paint(f, ERROR, format_args!("Unexpected end command {:?}", name))?
                }
                SemanticError::UnexpectedBranch(name) => {
                    self.paint(f, ERROR, format_args!("Unexpected branch {:?}", name))?
                }
                SemanticError::InvalidIfBranch(name) => self.paint(
                    f,
                    ERROR,
                    format_args!("Invalid branch {:?} after else", name),
                )?,
                SemanticError::Version(message) => self.paint(
                    f,
                    ERROR,
                    format_args!("Version detection failed: {:?}", message),
                )?,
                other => self.paint(f, ERROR, format_args!("{:?}", other))?,
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
