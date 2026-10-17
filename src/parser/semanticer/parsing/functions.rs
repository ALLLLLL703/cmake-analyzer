//! Discover literal function/macro signatures from recovered blocks.
//!
//! Definitions and parameters get source name spans, not runtime call bindings.
// Contains AI-generated documentation additions.
// AI-authored function/macro definitions discovered from semantic blocks.
use super::references::argument_name_span;
use crate::parser::semanticer::model::*;

impl Semanticer {
    /// Replace source function/macro signatures using recovered declaration blocks. Copies literal nonempty names/parameters and leading documentation; no execution or I/O.
    ///
    /// # Arguments
    /// - `self` `[in]`: analyzer, unchanged.
    /// - `source` `[in]`: owning source used to remove argument delimiters from definition spans.
    /// - `result` `[in,out]`: clears/replaces functions; other arenas are read only.
    ///
    /// # Returns
    /// () after recording valid literal declarations, including unclosed body blocks.
    ///
    /// # Panics
    /// Requires block opening command IDs to index result.commands.
    ///
    /// # Algorithm
    /// Visit function/macro blocks, accept a literal nonempty first argument as the name, then collect literal nonempty remaining arguments as parameter declarations. Each record begins with its definition reference.
    ///
    /// # Flow
    /// ```text
    ///  [recovered blocks]     [source text]
    ///          |                    |
    ///          v                    |
    ///   <function/macro?> --no--> [skip]
    ///          | yes                |
    ///          +----------+---------+
    ///                     v
    ///          <literal nonempty name?>
    ///               /            \
    ///             no             yes
    ///              |              |
    ///            [skip]   [parameters + definition]
    ///                             |
    ///                             v
    ///                      [append signature]
    /// ```
    ///
    /// # Examples
    /// ```text
    /// analyzer.scan_functions(source, &mut buffer);
    /// ```
    pub(crate) fn scan_functions(&self, source: &str, result: &mut SemanticedBuffer) {
        result.functions.clear();
        for block in &result.blocks {
            let kind = match block.kind {
                BlockKind::Function => FunctionKind::Function,
                BlockKind::Macro => FunctionKind::Macro,
                _ => continue,
            };
            let command = &result.commands[block.opening.id as usize];
            let Some(arg) = command.args.first() else {
                continue;
            };
            let Some(name) = arg.literal_text().filter(|name| !name.is_empty()) else {
                continue;
            };
            let location = |arg: &SemanticArg| ReferenceLocation {
                file: result.id,
                command: command.id,
                span: argument_name_span(source, arg),
                kind: ReferenceKind::Definition,
                possible_value: None,
            };
            let definition = location(arg);
            let parameters = command
                .args
                .iter()
                .skip(1)
                .filter_map(|arg| {
                    arg.literal_text()
                        .filter(|name| !name.is_empty())
                        .map(|name| FunctionParameter {
                            name: name.to_owned(),
                            definition: location(arg),
                        })
                })
                .collect();
            result.functions.push(SemanticFunction {
                documentation: command.documentation.clone(),
                name: name.to_owned(),
                kind,
                block: Some(block.id),
                scope: command.scope,
                definition: Some(definition.clone()),
                parameters,
                references: vec![definition],
            });
        }
    }
}
