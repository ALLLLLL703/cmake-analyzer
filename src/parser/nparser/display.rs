use std::fmt::Display;

use owo_colors::OwoColorize;
use owo_colors::Style;

use crate::parser::lexer::model::LexError;
use crate::parser::nparser::model::NParsedError;
use crate::parser::nparser::model::{NParsedArgument, NParsedBuffer, NParsedNode};

pub struct NParsedDisplay<'a> {
    buffer: &'a NParsedBuffer,
    source: &'a str,
    colored: bool,
}

impl NParsedBuffer {
    pub fn display<'a>(&'a self, colored: bool, source: &'a str) -> NParsedDisplay<'a> {
        NParsedDisplay {
            buffer: self,
            source,
            colored,
        }
    }
}

impl Display for NParsedDisplay<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for node in &self.buffer.nodes {
            let label = match &node.content {
                NParsedNode::Command(command) => {
                    let mut command_label = match command.name.span.text(self.source) {
                        Some(s) => s,
                        None => "command name lost,,,",
                    };
                    let mut args_label = format!("");
                    for arg in &command.args {
                        let mut style = match arg.content {
                            NParsedArgument::Unquoted => Style::new().blue(),
                            NParsedArgument::Quoted => Style::new().green(),
                            NParsedArgument::Bracked => Style::new().white(),
                            NParsedArgument::LeftParen => Style::new().yellow(),
                            NParsedArgument::RightParen => Style::new().yellow(),
                        };
                        let arg_label = match arg.span.text(self.source) {
                            Some(s) => s.to_owned(),
                            None => {
                                style = Style::new().red();
                                "command arg name lost,,,".to_string()
                            }
                        };
                        args_label = format!("{}, {}", args_label.clone(), arg_label.style(style));
                    }

                    format!("{}: [ {} ]\n", command_label.cyan(), args_label)
                }
                NParsedNode::Error(e) => {
                    let span = node.span;
                    let source = match span.text(self.source) {
                        Some(s) => s,
                        None => "error source not found,,,",
                    };

                    let label = match e {
                        NParsedError::LexError(lexe) => Some(match lexe {
                            LexError::ReachTheEof => "ReachTheEof(LexError)",
                            LexError::UnclosedStringLiteral => "UnclosedStringLitera(LexError)",
                            LexError::UnclosedParentheses => "UnclosedParentheses(LexError)",
                            LexError::UnclosedBracketArgument => {
                                "UnclosedBracketArgument(LexError)"
                            }
                        }),
                        NParsedError::ShouldBeCommand => Some("ShouldBeCommand"),
                        NParsedError::UnclosedLeftParentheses => Some("UnclosedLeftParentheses"),
                        NParsedError::UnArgStringLiteral => Some("UnArgStringLiteral"),
                        NParsedError::ShouldHaveLeftParentheses => {
                            Some("ShouldHaveLeftParentheses")
                        }
                        _ => None,
                    };

                    let loc_info = span.loc_info();

                    if let Some(label) = label {
                        format!(
                            "({}: [ loc: {},  source: {} ])\n",
                            label.red(),
                            loc_info.yellow(),
                            source.white()
                        )
                    } else {
                        format!("")
                    }
                }
            };
            writeln!(f, "{}", label);
        }
        Ok(())
    }
}
