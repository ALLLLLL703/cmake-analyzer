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
        if entries.len() > 32 {
            return Err(ServerError::invalid("Workspace limit (32) exceeded"));
        }
        let hierarchical = params
            .capabilities
            .text_document
            .and_then(|caps| caps.document_symbol)
            .and_then(|caps| caps.hierarchical_document_symbol_support)
            .unwrap_or(false);
        let mut state = self
            .state
            .lock()
            .map_err(|_| ServerError::new(-32603, "Server state poisoned"))?;
        state.roots = entries;
        state.hierarchical = hierarchical;
        Ok(InitializeResult {
            capabilities: ServerCapabilities {
                // UTF-16 is mandatory LSP fallback; TextIndex also supports other encodings.
                position_encoding: Some(PositionEncodingKind("utf-16".into())),
                text_document_sync: Some(TextDocumentSyncCapability::Options(
                    TextDocumentSyncOptions {
                        open_close: Some(true),
                        change: Some(TextDocumentSyncKind(2)),
                        will_save: None,
                        will_save_wait_until: None,
                        save: Some(TextDocumentSyncSaveOptions::Supported(true)),
                    },
                )),
                document_symbol_provider: Some(OneOf::Left(true)),
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
