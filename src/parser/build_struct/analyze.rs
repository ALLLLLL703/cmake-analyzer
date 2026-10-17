// AI-generated implementation.
use super::model::*;
use crate::parser::semanticer::{
    model::{ArgKind, BlockKind, SemanticArg},
    schema::CommandSchemaFrom,
};

fn argument(arg: &SemanticArg) -> Argument {
    Argument {
        text: arg.content.clone(),
        literal: arg.literal_text().is_some(),
    }
}
impl BuildStructAnalyzer<'_> {
    // AI-authored extraction shared by full and incremental analysis.
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
    pub fn analyze(&self) -> BuildStruct {
        let mut output = self.analyze_selected(|_| true);
        super::target_references::bind_local(&mut output, self.buffer);
        output
    }
    // AI-authored incremental extraction: unchanged effects need only fresh source references.
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
