//! Build semantic snapshots from parser nodes and source text.
//!
//! The pass order is version/schema selection, commands, documentation, blocks/scopes,
//! functions, template names, possible writes, references, values and root symbols.
//! Only version fallback starts a process (`cmake --version`); commands are not run.
// Contains AI-generated documentation additions.
// AI-generated implementation.
pub mod block;
mod functions;
pub(super) mod outputs;
pub(crate) use outputs::{
    completion_keywords, completion_roles, has_target_arguments, target_arguments,
};
pub(crate) mod references;
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
    /// Build a static semantic snapshot and refresh schemas/version/cursor in the analyzer. May query `cmake --version` if no declaration supplies a version; does not execute project code.
    ///
    /// # Arguments
    /// - `self` `[in,out]`: analyzer; overwrites active version/schemas and sets cursor offset to node count.
    /// - `source` `[in]`: source snapshot matching self.buffer spans.
    /// - `file_id` `[in]`: workspace identity stored in the result.
    ///
    /// # Returns
    /// New populated snapshot; version/parse/structure failures are retained as diagnostics rather than returned errors.
    ///
    /// # Panics
    /// Pass source matching parser spans. Documentation binding and structural passes assume valid parser ranges and internally generated dense IDs.
    ///
    /// # Algorithm
    /// Select the versioned catalog plus injected overrides, seed builtin names, extract commands and then run documentation, blocks, scopes, signatures, template specialization, writes, references, values and symbol passes in dependency order.
    ///
    /// # Flow
    /// ```text
    ///  [nodes + source]        [injected schemas]
    ///         |                        |
    ///         v                        |
    ///  [version + catalog] <-----------+
    ///         |
    ///         v
    ///  [commands + comments]
    ///         |
    ///         v
    ///  [blocks -> scopes -> signatures]
    ///         |
    ///         v
    ///  [template names + writes]
    ///         |
    ///         v
    ///  [references + values + symbols]
    ///         |
    ///         +----------------------> [new snapshot]
    /// ```
    ///
    /// # Examples
    /// ```
    /// use cmake_analyzer::parser::semanticer::model::*;
    /// use cmake_analyzer::parser::{lexer::model::{Lexer, RawBuffer}, nparser::model::NParser};
    /// # tokio::runtime::Runtime::new().unwrap().block_on(async {
    /// let source = "cmake_minimum_required(VERSION 3.31)\nset(x yes)";
    /// let mut lexer = Lexer::new(RawBuffer::new(source));
    /// let mut parser = NParser::new(lexer.parse());
    /// let parsed = parser.parse(source);
    /// let mut analyzer = Semanticer::new(parsed, Default::default());
    /// let buffer = analyzer.parse(source, FileId { id: 1 }).await;
    /// assert_eq!(buffer.commands.len(), 2);
    /// # });
    /// ```
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
        self.bind_documentation(source, &self.buffer.comments, &mut result);
        self.scan_blocks(&mut result);
        self.scan_scopes(&mut result);
        self.scan_functions(source, &mut result);
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
        self.collect_references(&mut result, source);
        self.collect_values(&mut result);
        self.collect_symbols(&mut result);
        self.cursor.offset = self.buffer.nodes.len();
        result
    }

    /// Append extracted commands and parser/command diagnostics for all parser nodes. Does not clear existing output or execute commands.
    ///
    /// # Arguments
    /// - `self` `[in]`: analyzer with selected schemas and parser nodes.
    /// - `source` `[in]`: source matching parser spans.
    /// - `result` `[in,out]`: appends commands/diagnostics, assigning IDs from its current command length.
    ///
    /// # Returns
    /// () after extraction; missing command-name slices are skipped.
    ///
    /// # Examples
    /// ```
    /// use cmake_analyzer::parser::semanticer::model::*;
    /// use cmake_analyzer::parser::nparser::model::NParsedBuffer;
    /// let analyzer = Semanticer::new(NParsedBuffer::default(), Default::default());
    /// let mut result = SemanticedBuffer::default();
    /// analyzer.scan_commands("", &mut result);
    /// assert!(result.commands.is_empty());
    /// ```
    pub fn scan_commands(&self, source: &str, result: &mut SemanticedBuffer) {
        self.scan_nodes(source, self.buffer.nodes.iter(), result);
    }
    // AI-authored extraction for changed parser nodes.
    /// Append command records for selected parser nodes and extract their unbound reads/path roles. Used for changed nodes during incremental updates; no I/O.
    ///
    /// # Arguments
    /// - `self` `[in]`: selected command schemas.
    /// - `source` `[in]`: source snapshot for node and argument ranges.
    /// - `nodes` `[in]`: iterator of parser-node references, consumed in order.
    /// - `result` `[in,out]`: appends commands and diagnostics; new commands receive fresh reads/path roles.
    ///
    /// # Returns
    /// () after all supplied nodes; invalid name spans are skipped and invalid argument text becomes empty.
    ///
    /// # Algorithm
    /// Forward parser errors, resolve command names case-insensitively against schemas, preserve raw ranges and lexical argument forms, then scan each new command for reads and path slots.
    ///
    /// # Flow
    /// ```text
    ///  [node iterator]      [active schemas]
    ///         |                    |
    ///         v                    |
    ///     <node kind?>             |
    ///      /       \              |
    ///    error    command <--------+
    ///      |         |
    ///  [diagnostic] [record + reads/paths]
    ///      |         |
    ///      +----+----+
    ///           v
    ///     [append to result]
    /// ```
    ///
    /// # Examples
    /// ```text
    /// analyzer.scan_nodes(source, parsed.nodes.iter().take(1), &mut result);
    /// ```
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
                        documentation: None,
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
                        variable_references: Vec::new(),
                        paths: Vec::new(),
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
                    if let Some(command) = result.commands.last_mut() {
                        references::scan_command(source, result.id, command);
                    }
                }
            }
        }
    }

    /// Clone the parser node at the inspection cursor without advancing it. No side effects.
    ///
    /// # Arguments
    /// - `self` `[in]`: analyzer and current node offset.
    ///
    /// # Returns
    /// Some cloned node at offset, or None when offset is outside the node vector.
    ///
    /// # Examples
    /// ```
    /// use cmake_analyzer::parser::semanticer::model::*;
    /// use cmake_analyzer::parser::nparser::model::NParsedBuffer;
    /// let analyzer = Semanticer::new(NParsedBuffer::default(), Default::default());
    /// assert!(analyzer.peek().is_none());
    /// ```
    pub fn peek(&self) -> Option<Spanned<NParsedNode>> {
        self.buffer.nodes.get(self.cursor.offset).cloned()
    }
}
