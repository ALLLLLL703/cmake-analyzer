// AI-generated implementation.
pub mod block;
mod outputs;
pub mod scope;
pub mod symbols;
mod version;
// AI-generated implementation.
mod templates;

use crate::{
    model::{Spanned, TextSpan},
    parser::{
        nparser::model::{CommandState, NParsedArgument, NParsedError, NParsedNode},
        semanticer::{
            model::{
                ArgKind, CommandId, CommandKind, FileId, SemanticArg, SemanticArgForm,
                SemanticCommandRepresent, SemanticError, SemanticedBuffer, Semanticer,
            },
            schema::builtin,
            variables,
        },
    },
};
use std::sync::Arc;

impl Semanticer {
    pub async fn parse(&mut self, source: &str, file_id: FileId) -> SemanticedBuffer {
        let mut result = SemanticedBuffer::new_with_file_id(file_id);
        self.version = None;
        match self.scan_version(source).await {
            Ok(version) => {
                self.version = Some(version);
                result.version = Some(version);
            }
            Err(message) => result.diagnostics.push(Spanned::new(
                SemanticError::Version(message),
                TextSpan::default(),
            )),
        }
        let schemas = match self.version {
            Some(version) => match builtin::schemas_for_version(version.major, version.minor) {
                Some(schemas) => schemas,
                None => {
                    result.diagnostics.push(Spanned::new(
                        SemanticError::UnsupportedVersion(version),
                        TextSpan::default(),
                    ));
                    builtin::BASE_SCHEMAS
                        .iter()
                        .map(|s| (s.command_name.clone(), s.clone()))
                        .collect()
                }
            },
            None => builtin::BASE_SCHEMAS
                .iter()
                .map(|s| (s.command_name.clone(), s.clone()))
                .collect(),
        };
        self.schemas = schemas
            .into_iter()
            .map(|(name, schema)| (name, Arc::new(schema)))
            .collect();
        self.schemas.extend(
            self.injected_schemas
                .iter()
                .map(|(name, schema)| (name.to_ascii_lowercase(), schema.clone())),
        );
        result.variables = self
            .version
            .and_then(|v| variables::builtin::variables_for_version(v.major, v.minor))
            .unwrap_or_else(|| variables::builtin::BASE_VARIABLES.clone());
        self.scan_commands(source, &mut result);
        self.scan_blocks(&mut result);
        self.scan_scopes(&mut result);
        // AI-generated template specialization.
        let specialized = self.detect_specialized_variables(&result);
        result
            .variables
            .retain(|variable| !variable.name.contains(['<', '>']));
        let mut names: std::collections::HashSet<_> = result
            .variables
            .iter()
            .map(|variable| variable.name.clone())
            .collect();
        result.variables.extend(
            specialized
                .into_iter()
                .filter(|variable| names.insert(variable.name.clone())),
        );
        self.track_variable_changes(&mut result);
        self.collect_symbols(&mut result);
        self.cursor.offset = self.buffer.nodes.len();
        result
    }

    pub fn scan_commands(&self, source: &str, result: &mut SemanticedBuffer) {
        self.scan_nodes(source, self.buffer.nodes.iter(), result);
    }
    // AI-authored extraction for changed parser nodes.
    pub(crate) fn scan_nodes<'node>(
        &self,
        source: &str,
        nodes: impl Iterator<Item = &'node Spanned<NParsedNode>>,
        result: &mut SemanticedBuffer,
    ) {
        for node in nodes {
            match &node.content {
                NParsedNode::Error(error) => result.diagnostics.push(Spanned::new(
                    SemanticError::NParsedError(error.clone()),
                    node.span,
                )),
                NParsedNode::Command(command) => {
                    let Some(name) = command.name.span.text(source) else {
                        continue;
                    };
                    let schema = self.schemas.get(&name.to_ascii_lowercase()).cloned();
                    let kind = match &schema {
                        Some(schema) => schema.kind.clone(),
                        None => {
                            result.diagnostics.push(Spanned::new(
                                SemanticError::CommandNotFound(name.to_owned()),
                                command.name.span,
                            ));
                            CommandKind::Other(name.to_owned())
                        }
                    };
                    if let CommandState::Unclosed { opening_span } = command.closed {
                        result.diagnostics.push(Spanned::new(
                            SemanticError::NParsedError(NParsedError::UnclosedLeftParentheses),
                            opening_span,
                        ));
                    }
                    result.commands.push(SemanticCommandRepresent {
                        id: CommandId {
                            id: result.commands.len() as u32,
                        },
                        kind,
                        span: command.name.span,
                        name: name.to_owned(),
                        closed: command.closed == CommandState::Closed,
                        schema,
                        full_span: node.span,
                        block: None,
                        scope: None,
                        args: command
                            .args
                            .iter()
                            .map(|arg| SemanticArg {
                                kind: Vec::<ArgKind>::new(),
                                content: arg.text(source).unwrap_or_default().to_owned(),
                                span: arg.span,
                                form: match arg.content {
                                    NParsedArgument::Unquoted => SemanticArgForm::Unquoted,
                                    NParsedArgument::Quoted => SemanticArgForm::Quoted,
                                    NParsedArgument::Bracked => SemanticArgForm::Bracket,
                                    NParsedArgument::LeftParen => SemanticArgForm::LeftParen,
                                    NParsedArgument::RightParen => SemanticArgForm::RightParen,
                                },
                            })
                            .collect(),
                    });
                }
            }
        }
    }

    pub fn peek(&self) -> Option<Spanned<NParsedNode>> {
        self.buffer.nodes.get(self.cursor.offset).cloned()
    }
}
