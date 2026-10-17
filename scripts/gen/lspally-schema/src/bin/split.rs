// AI-authored generator.
//! Split a monolithic common.rs seed into responsibility-specific models.
//! Args: seed common.rs, output directory outside target/.
use quote::ToTokens;
use std::{collections::BTreeMap, fs, path::PathBuf};
/// Split model seed declarations into feature files and format those files.
/// Reads the input, creates the destination, overwrites grouped outputs and starts `rustfmt`.
/// Item documentation attributes remain attached to their declarations; no adapted
/// production file should be used as the destination.
///
/// # Arguments
/// No Rust parameters. CLI argument 1 names the common seed file; argument 2 names
/// an output directory outside `target/`.
/// # Returns
/// `()` after all grouped files have been written and formatted.
/// # Panics
/// Missing arguments, a destination containing a `target` component, read/write or
/// directory creation failures, invalid Rust input, malformed empty implementation
/// type paths, or a missing/failing formatter terminate the command.
/// # Algorithm
/// Parse declarations once. Group structs, enums, aliases and inherent implementations
/// by their type name. Ignore imports and other item kinds. Preserve each item's
/// tokens, including its documentation, then add imports and generation provenance.
/// # Flow
/// ```text
/// [common seed]           [destination]
///       |                       |
///   [parse Rust]         [reject target/]
///       |                       |
///       v                       |
/// <model declaration?>          |
///   /             \\             |
/// yes              no           |
///  |                |           |
/// [group by name] [skip]         |
///  |                            |
///  +--------------+-------------+
///                 v
///        [write feature files]
///                 |
///                 v
///          [rustfmt outputs]
/// ```
/// # Examples
/// ```sh
/// cargo run --manifest-path scripts/gen/lspally-schema/Cargo.toml --bin split -- /tmp/lsp-seeds/model/common.rs /tmp/lsp-split
/// ```
fn main() {
    let input = PathBuf::from(std::env::args().nth(1).expect("seed common.rs"));
    let base = PathBuf::from(std::env::args().nth(2).expect("split output directory"));
    assert!(
        !base.components().any(|c| c.as_os_str() == "target"),
        "Generated sources must stay outside target/"
    );
    fs::create_dir_all(&base).unwrap();
    let source = syn::parse_file(&fs::read_to_string(input).unwrap()).unwrap();
    let mut groups = BTreeMap::<&str, String>::new();
    for item in source.items {
        let name = match &item {
            syn::Item::Struct(i) => i.ident.to_string(),
            syn::Item::Enum(i) => i.ident.to_string(),
            syn::Item::Type(i) => i.ident.to_string(),
            syn::Item::Impl(i) => {
                if let syn::Type::Path(p) = &*i.self_ty {
                    p.path.segments.last().unwrap().ident.to_string()
                } else {
                    continue;
                }
            }
            _ => continue,
        };
        let group = if name.starts_with("Diagnostic")
            || name.starts_with("PublishDiagnostics")
            || name == "CodeDescription"
        {
            "diagnostic"
        } else if name.starts_with("Initialize")
            || name == "ClientInfo"
            || name == "ClientCapabilities"
            || name == "ServerCapabilities"
            || name == "ServerInfo"
            || name == "PositionEncodingKind"
            || name == "GeneralClientCapabilities"
        {
            "lifecycle"
        } else if name.contains("ClientCapabilities")
            || name.starts_with("DynamicRegistration")
            || name.starts_with("GotoCapability")
            || name.ends_with("Capability")
            || name == "TagSupport"
            || name == "StaleRequestSupportClientCapabilities"
        {
            "capabilities"
        } else if name == "WorkspaceEdit"
            || name == "DocumentChanges"
            || name == "DocumentChangeOperation"
            || name == "ResourceOp"
            || name.starts_with("CreateFile")
            || name.starts_with("RenameFile")
            || name.starts_with("DeleteFile")
            || name == "TextEdit"
            || name == "AnnotatedTextEdit"
            || name == "TextDocumentEdit"
            || name.starts_with("ChangeAnnotation")
        {
            "edit"
        } else if name.starts_with("TextDocument")
            || name == "VersionedTextDocumentIdentifier"
            || name == "OptionalVersionedTextDocumentIdentifier"
            || name == "Position"
            || name == "Range"
            || name == "Location"
            || name == "LocationLink"
            || name.starts_with("DidOpenText")
            || name.starts_with("DidChangeText")
            || name.starts_with("DidCloseText")
            || name.starts_with("DidSaveText")
            || name == "WillSaveTextDocumentParams"
            || name == "DocumentFilter"
            || name == "DocumentSelector"
        {
            "document"
        } else if name.starts_with("Workspace")
            || name.starts_with("Configuration")
            || name.starts_with("FileEvent")
            || name.starts_with("FileSystem")
            || name.starts_with("FileChange")
            || name.starts_with("DidChange")
            || name == "RelativePattern"
            || name == "GlobPattern"
        {
            "workspace"
        } else {
            "common"
        };
        groups
            .entry(group)
            .or_default()
            .push_str(&format!("{}\n", item.into_token_stream()));
    }
    let mut format = std::process::Command::new("rustfmt");
    format.args(["--edition", "2024"]);
    for (name, body) in groups {
        let path = base.join(format!("{name}.rs"));
        fs::write(&path,format!("// Script-generated by scripts/gen/lspally-schema/src/bin/split.rs from main.rs seeds.\n//! Grouped protocol model seeds; construction alone performs no I/O.\nuse super::*;\n{body}")).unwrap();
        format.arg(path);
    }
    assert!(
        format.status().expect("rustfmt").success(),
        "Could not format generated Rust"
    );
}
