//! Initialize roots, configuration and advertised client-dependent features.
//! The generated child implementation forwards protocol calls to these adapters.
//! Initialization prepares lifecycle state only; `initialized` starts the service.
//! Root discovery and configuration can read disk and inspect native CMake defaults,
//! but do not configure projects or execute project scripts.
// AI-authored lifecycle and server capabilities.
mod implementation;
use super::{model::*, roots, service::internal, uri};
use lspally::server::model::*;

impl CmakeAnalyzer {
    /// Discover initial roots, load settings and retain negotiated feature flags for startup.
    /// Reads filesystem metadata/configuration and may inspect native defaults; does not start the actor or run project code.
    ///
    /// # Arguments
    /// - `self` `[in,out]`: Shared lifecycle state is overwritten with prepared roots, configuration and capability flags.
    /// - `_context` `[in]`: Unused initialization context; this function does not consult its cancellation.
    /// - `params` `[in]`: Consumed client roots and capabilities; explicit option roots take precedence.
    ///
    /// # Returns
    /// `Ok(InitializeResult)` advertising UTF-16, incremental sync and implemented features; `Err` on discovery/configuration/admission failure.
    ///
    /// # Errors
    /// Invalid file URIs, explicit-root/discovery failures, configuration read/parse/validation errors, root count above max_workspaces, or poisoned lifecycle mutex. State is replaced only after preparation succeeds.
    ///
    /// # Algorithm
    /// Prefer explicit roots, otherwise folders then root URI/path then startup cwd. Discover nearest entries, sort/deduplicate, derive client flags and load each entry/cwd configuration before locking state.
    ///
    /// # Flow
    /// ```text
    ///  [Options roots]          [Client folders/root/cwd]
    ///        |                           |
    ///        +--------- <Explicit?> -----+
    ///                         |
    ///                   [Discover entries]
    ///                         |
    ///                  [Load + check limits]
    ///                    /           \
    ///                 failure       success
    ///                    |             |
    ///                 [Error]    [Store state + capabilities]
    /// ```
    ///
    /// # Examples
    /// ```text
    /// let initialized = server.initialize_server(context, params).await?;
    /// // The later initialized notification calls server.start(context).
    /// ```
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
                workspace_symbol_provider: Some(OneOf::Left(true)),
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
