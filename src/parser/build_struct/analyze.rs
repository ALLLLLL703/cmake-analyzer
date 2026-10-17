//! Translate closed, schema-known semantic commands into ordered facts without I/O.
//! Native schemas supply build effects; any schema may supply output-variable writes.
// Contains AI-generated documentation additions.
// AI-generated implementation.
use super::model::*;
use crate::parser::semanticer::{
    model::{ArgKind, BlockKind, SemanticArg},
    schema::CommandSchemaFrom,
};

/// Copy semantic content and its literal classification; no expansion, mutation or I/O.
///
/// # Arguments
/// - `arg` `[in]`: Operand whose semantic content is retained.
///
/// # Returns
/// Owned text and a literal flag.
///
/// # Examples
/// ```text
/// let operand = argument(&command.args[0]);
/// ```
fn argument(arg: &SemanticArg) -> Argument {
    Argument {
        text: arg.content.clone(),
        literal: arg.literal_text().is_some(),
    }
}
impl BuildStructAnalyzer<'_> {
    // AI-authored extraction shared by full and incremental analysis.
    /// Qualify a command and walk its block ancestors to flag conditional/deferred context. No
    /// mutation or I/O.
    ///
    /// # Arguments
    /// - `self` `[in]`: Analyzer supplying block ancestry and origin.
    /// - `command` `[in]`: Command whose full span, scope and immediate block are retained.
    ///
    /// # Returns
    /// Source reference; invalid ancestor IDs terminate the walk.
    ///
    /// # Algorithm
    /// Walk immediate block then parents, accumulating loop/if and function/macro flags
    /// independently. Stop at file level or a missing ancestor; preserve the original immediate
    /// block and scope in the returned origin.
    ///
    /// # Flow
    /// ```text
    /// [command block]
    ///        |
    ///  <ancestor exists?> -- no --> [qualified origin]
    ///        | yes                        ^
    ///  [merge context flags]              |
    ///        |                            |
    ///  [follow parent] -------------------+
    ///        |
    ///        +----> [next ancestor]
    /// ```
    ///
    /// # Examples
    /// ```text
    /// let source = analyzer.source_for(command);
    /// ```
    pub(crate) fn source_for(
        &self,
        command: &crate::parser::semanticer::model::SemanticCommandRepresent,
    ) -> SourceRef {
        let mut block = command.block;
        let mut conditional = false;
        let mut deferred = false;
        while let Some(id) = block {
            let Some(owner) = self.buffer.blocks.get(id.id as usize) else {
                break;
            };
            conditional |= matches!(
                owner.kind,
                BlockKind::If | BlockKind::While | BlockKind::Foreach
            );
            deferred |= matches!(owner.kind, BlockKind::Function | BlockKind::Macro);
            block = owner.parent;
        }
        SourceRef {
            analysis: self.origin,
            command: command.id,
            span: command.full_span,
            block: command.block,
            scope: command.scope,
            conditional,
            deferred,
        }
    }
    /// Extract ordered facts, not executed build state.
    /// Extract ordered facts and bind file-local target references. Does not execute CMake, read
    /// files or mutate input.
    ///
    /// # Arguments
    /// - `self` `[in]`: Borrowed semantic snapshot and file revision.
    ///
    /// # Returns
    /// Owned facts and extraction diagnostics, with target references local to this file.
    ///
    /// # Algorithm
    /// Extract all eligible commands, then bind literal target declarations and schema-identified
    /// reads in one local directory.
    ///
    /// # Flow
    /// ```text
    /// [semantic input]
    ///        |
    /// [analyze_selected(all)]
    ///        |
    /// [owned facts] ------> [bind local targets]
    ///                              |
    ///                        [return output]
    /// ```
    ///
    /// # Examples
    /// ```
    /// use cmake_analyzer::parser::build_struct::model::{AnalysisKey, BuildStructAnalyzer};
    /// use cmake_analyzer::parser::semanticer::model::SemanticedBuffer;
    /// let semantic = SemanticedBuffer::default();
    /// let analyzer = BuildStructAnalyzer::new(&semantic, AnalysisKey { file: 0, revision: 1 });
    /// let build = analyzer.analyze();
    /// ```
    pub fn analyze(&self) -> BuildStruct {
        let mut output = self.analyze_selected(|_| true);
        super::target_references::bind_local(&mut output, self.buffer);
        output
    }
    // AI-authored incremental extraction: unchanged effects need only fresh source references.
    /// Extract selected command effects without binding target reads. Input is unchanged; the
    /// selection callback may have its own side effects.
    ///
    /// # Arguments
    /// - `self` `[in]`: Semantic snapshot and origin.
    /// - `selected` `[in,out]` : Callback invoked per command; its captured state may change. False
    ///   skips extraction.
    ///
    /// # Returns
    /// Owned selected facts and missing-first-argument diagnostics, in command order.
    ///
    /// # Algorithm
    /// Skip unselected, unclosed or schema-less commands. Native schemas supply typed build
    /// effects; schema Out arguments supply literal writes. Cache/parent writes are retained as
    /// nonlocal. Missing required first operands emit a diagnostic and skip that command.
    ///
    /// # Flow
    /// ```text
    /// [command loop]
    ///       |
    ///  <selected/closed/schema?> -- no --> [next]
    ///       | yes
    ///  [native effect]        [Out names]
    ///       |                    |
    ///       +----> [append facts]+
    ///                    |
    ///                 [next]
    /// ```
    ///
    /// # Examples
    /// ```text
    /// let changed = analyzer.analyze_selected(|command| command.id.id == 2);
    /// ```
    pub(crate) fn analyze_selected(
        &self,
        mut selected: impl FnMut(&crate::parser::semanticer::model::SemanticCommandRepresent) -> bool,
    ) -> BuildStruct {
        let mut output = BuildStruct {
            analysis: self.origin,
            facts: Vec::new(),
            diagnostics: Vec::new(),
        };
        for command in &self.buffer.commands {
            if !selected(command) || !command.closed || command.schema.is_none() {
                continue;
            }
            // An injected schema may reuse a native name without its build semantics.
            let native = command.schema.as_ref().is_some_and(|s| {
                matches!(
                    s.from,
                    CommandSchemaFrom::Builtin | CommandSchemaFrom::KeyWord
                )
            });
            let source = self.source_for(command);
            let args: Vec<_> = command.args.iter().map(argument).collect();
            let name = command.name.to_ascii_lowercase();
            let effect = match if native { name.as_str() } else { "" } {
                "cmake_minimum_required" => Some(BuildEffect::Minimum { arguments: args }),
                "enable_language" => Some(BuildEffect::Languages { arguments: args }),
                "project"
                | "add_executable"
                | "add_library"
                | "add_custom_target"
                | "target_sources"
                | "target_link_libraries"
                | "include"
                | "find_package"
                | "add_subdirectory" => {
                    let mut arguments = args.into_iter();
                    let Some(first) = arguments.next() else {
                        output.diagnostics.push(BuildDiagnostic {
                            source,
                            message: format!("Missing first argument for {name}"),
                        });
                        continue;
                    };
                    let rest: Vec<_> = arguments.collect();
                    Some(match name.as_str() {
                        "project" => BuildEffect::Project(super::project::project(first, rest)),
                        "target_sources" => {
                            BuildEffect::Sources(super::sources::sources(first, rest))
                        }
                        "target_link_libraries" => BuildEffect::Links {
                            target: first,
                            arguments: rest,
                        },
                        "include" | "find_package" | "add_subdirectory" => BuildEffect::Dependency(
                            super::dependencies::from_arguments(&name, first, rest),
                        ),
                        _ => BuildEffect::Target(super::targets::target(&name, first, rest)),
                    })
                }
                _ => None,
            };
            if let Some(effect) = effect {
                output.facts.push(BuildFact {
                    source: source.clone(),
                    effect,
                });
            }
            let names: Vec<_> = command
                .args
                .iter()
                .filter(|arg| arg.kind.iter().any(|kind| matches!(kind, ArgKind::Out)))
                .filter_map(|arg| arg.literal_text().map(str::to_owned))
                .collect();
            if !names.is_empty() {
                // Keep cache/parent writes as facts; their destination is not modeled yet.
                let nonlocal = native
                    && (name == "option"
                        || (name == "set"
                            && (command.args.last().and_then(|arg| arg.literal_text())
                                == Some("PARENT_SCOPE")
                                || command
                                    .args
                                    .iter()
                                    .skip(2)
                                    .any(|arg| arg.literal_text() == Some("CACHE")))));
                output.facts.push(BuildFact {
                    source,
                    effect: BuildEffect::VariableWrites {
                        names,
                        local: !nonlocal,
                    },
                });
            }
        }
        output
    }
}
