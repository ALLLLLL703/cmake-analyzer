use super::model::{SemanticBuffer, Severity};
use owo_colors::{OwoColorize, Style};
use std::fmt::{self, Display};

pub struct SemanticDisplay<'a> {
    buffer: &'a SemanticBuffer,
    colored: bool,
}

impl SemanticBuffer {
    pub fn display(&self, colored: bool) -> SemanticDisplay<'_> {
        SemanticDisplay {
            buffer: self,
            colored,
        }
    }
}

impl Display for SemanticBuffer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.display(false))
    }
}

impl Display for SemanticDisplay<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let buffer = self.buffer;
        write!(f, "{}", buffer.parsed.display(self.colored))?;
        line(
            f,
            format_args!("Blocks ({})", buffer.blocks.len()),
            Style::new().magenta().bold(),
            self.colored,
        )?;
        for (id, block) in buffer.blocks.iter().enumerate() {
            line(
                f,
                format_args!(
                    "  #{id} {:?} parent={:?} open={} close={:?} branches={:?} scope=#{}",
                    block.kind,
                    block.parent,
                    block.opening,
                    block.closing,
                    block.branches,
                    block.scope
                ),
                Style::new().magenta(),
                self.colored,
            )?;
        }
        line(
            f,
            format_args!("Scopes ({})", buffer.scopes.len()),
            Style::new().blue().bold(),
            self.colored,
        )?;
        for (id, scope) in buffer.scopes.iter().enumerate() {
            line(
                f,
                format_args!(
                    "  #{id} {:?} parent={:?} block={:?}",
                    scope.kind, scope.parent, scope.block
                ),
                Style::new().blue(),
                self.colored,
            )?;
        }
        line(
            f,
            format_args!("Symbols ({})", buffer.symbols.len()),
            Style::new().green().bold(),
            self.colored,
        )?;
        for (id, symbol) in buffer.symbols.iter().enumerate() {
            line(
                f,
                format_args!(
                    "  #{id} {:?}/{:?} {:?} scope=#{} at {:?}",
                    symbol.namespace,
                    symbol.kind,
                    buffer.name(&symbol.name).unwrap_or("<invalid-name>"),
                    symbol.scope,
                    symbol.span
                ),
                Style::new().green(),
                self.colored,
            )?;
        }
        line(
            f,
            format_args!("References ({})", buffer.references.len()),
            Style::new().cyan().bold(),
            self.colored,
        )?;
        for reference in &buffer.references {
            line(
                f,
                format_args!(
                    "  {:?} {:?} scope=#{} candidates={:?} dynamic={} at {:?}",
                    reference.namespace,
                    buffer.name(&reference.name).unwrap_or("<invalid-name>"),
                    reference.scope,
                    reference.resolved,
                    reference.dynamic,
                    reference.span
                ),
                Style::new().cyan(),
                self.colored,
            )?;
        }
        for diagnostic in &buffer.diagnostics {
            let style = match diagnostic.content.severity {
                Severity::Error => Style::new().red().bold(),
                Severity::Warning => Style::new().yellow(),
            };
            line(
                f,
                format_args!(
                    "{:?}: {:?} at {:?}",
                    diagnostic.content.severity, diagnostic.content.kind, diagnostic.span
                ),
                style,
                self.colored,
            )?;
        }
        Ok(())
    }
}

fn line(
    f: &mut fmt::Formatter<'_>,
    text: fmt::Arguments<'_>,
    style: Style,
    colored: bool,
) -> fmt::Result {
    if colored {
        writeln!(f, "{}", text.style(style))
    } else {
        writeln!(f, "{text}")
    }
}
