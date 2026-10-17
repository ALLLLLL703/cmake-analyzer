// AI-authored lifecycle and server capabilities.
mod implementation;
use super::{model::*, roots, service::internal, uri};
use lspally::server::model::*;

impl CmakeAnalyzer {
    pub(super) async fn initialize_server(
        &self,
        _context: Context,
        params: InitializeParams,
    ) -> ServerResult<InitializeResult> {
        let mut entries = Vec::new();
        if !self.options.roots.is_empty() {
            for path in &self.options.roots {
                entries.push(roots::explicit_root(path).await.map_err(internal)?);
            }
        } else {
            let mut paths = Vec::new();
            if let Some(folders) = params.workspace_folders {
                for folder in folders {
                    paths.push(uri::to_path(&folder.uri)?);
                }
            }
            if paths.is_empty() {
                if let Some(root) = params.root_uri {
                    paths.push(uri::to_path(&root)?);
                } else if let Some(root) = params.root_path {
                    paths.push(root.into());
                }
            }
            if paths.is_empty() {
                paths.push(self.options.start_directory.clone());
            }
            for path in paths {
                if let Some(entry) = roots::find_project(&path).await.map_err(internal)? {
                    entries.push(entry);
                }
            }
        }
        entries.sort();
        entries.dedup();
        let hover_markdown = params
            .capabilities
            .text_document
            .as_ref()
            .and_then(|caps| caps.hover.as_ref())
            .and_then(|caps| caps.content_format.as_ref())
            .and_then(|formats| formats.first())
            .is_some_and(|format| matches!(format, MarkupKind::Markdown));
        let completion_snippets = params
            .capabilities
            .text_document
            .as_ref()
            .and_then(|caps| caps.completion.as_ref())
            .and_then(|caps| caps.completion_item.as_ref())
            .and_then(|caps| caps.snippet_support)
            .unwrap_or(false);
        let hierarchical = params
            .capabilities
            .text_document
            .and_then(|caps| caps.document_symbol)
            .and_then(|caps| caps.hierarchical_document_symbol_support)
            .unwrap_or(false);
        let mut config = crate::config::ConfigStore::load(self.options.config.clone())
            .await
            .map_err(internal)?;
        if entries.len() > config.limits.max_workspaces {
            return Err(ServerError::invalid(format!(
                "Workspace limit ({}) exceeded",
                config.limits.max_workspaces
            )));
        }
        for entry in &entries {
            config
                .for_directory(entry.parent().unwrap_or(&self.options.start_directory))
                .await
                .map_err(internal)?;
        }
        config
            .for_directory(&self.options.start_directory)
            .await
            .map_err(internal)?;
        let mut state = self
            .state
            .lock()
            .map_err(|_| ServerError::new(-32603, "Server state poisoned"))?;
        state.roots = entries;
        state.config = Some(config);
        state.hierarchical = hierarchical;
        state.hover_markdown = hover_markdown;
        state.completion_snippets = completion_snippets;
        Ok(InitializeResult {
            capabilities: ServerCapabilities {
                // UTF-16 is mandatory LSP fallback; TextIndex also supports other encodings.
                position_encoding: Some(PositionEncodingKind("utf-16".into())),
                text_document_sync: Some(TextDocumentSyncCapability::Options(
                    TextDocumentSyncOptions {
                        open_close: Some(true),
                        change: Some(TextDocumentSyncKind(2)),
                        will_save: None,
                        will_save_wait_until: Some(true),
                        save: Some(TextDocumentSyncSaveOptions::Supported(true)),
                    },
                )),
                document_symbol_provider: Some(OneOf::Left(true)),
                document_formatting_provider: Some(OneOf::Left(true)),
                hover_provider: Some(HoverProviderCapability::Simple(true)),
                completion_provider: Some(CompletionOptions {
                    resolve_provider: Some(false),
                    trigger_characters: Some(vec!["(".into(), "{".into()]),
                    all_commit_characters: None,
                    work_done_progress_options: WorkDoneProgressOptions {
                        work_done_progress: None,
                    },
                    completion_item: None,
                }),
                definition_provider: Some(OneOf::Left(true)),
                references_provider: Some(OneOf::Left(true)),
                code_action_provider: Some(CodeActionProviderCapability::Simple(true)),
                execute_command_provider: Some(ExecuteCommandOptions {
                    commands: [
                        super::action::model::ActionKind::Configure,
                        super::action::model::ActionKind::RunScript,
                        super::action::model::ActionKind::ExpandedArguments,
                        super::action::model::ActionKind::CancelConfigure,
                    ]
                    .into_iter()
                    .map(|kind| kind.command().to_owned())
                    .collect(),
                    work_done_progress_options: WorkDoneProgressOptions {
                        work_done_progress: None,
                    },
                }),
                workspace: Some(WorkspaceServerCapabilities {
                    workspace_folders: Some(WorkspaceFoldersServerCapabilities {
                        supported: Some(true),
                        change_notifications: Some(OneOf::Left(true)),
                    }),
                    file_operations: None,
                }),
                ..Default::default()
            },
            server_info: Some(ServerInfo {
                name: "cmake-analyzer".into(),
                version: Some(env!("CARGO_PKG_VERSION").into()),
            }),
        })
    }
}
