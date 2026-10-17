#!/usr/bin/env python3
# AI-authored generator.
"""Emit the analyzer's explicit trait implementation, including per-hook contracts.

Read the 73-method trait inventory and supported-handler table, then overwrite only
src/server/handlers/implementation.rs. Generated docs describe forwarding, ignored
inputs and unsupported hooks; they do not advertise additional capabilities. Run
``python scripts/gen/cmake_server.py`` from a checkout, then format the output.
This generator is authoritative only for that implementation, not model/adapters.
"""
from pathlib import Path
import re
root = Path(__file__).resolve().parents[2]
source = (root / "crates/lspally/src/server/model.rs").read_text()
trait = source.split("pub trait ILanguageServer", 1)[1].split("#[derive(Debug)]", 1)[0]
methods = re.findall(r"fn (\w+)\(\s*&self,\s*context: Context,\s*params: (.*?)\s*\) -> impl Future<Output = ServerResult<(.*?)>> \+ Send;", trait, re.S)
handlers = {
    "initialize": "self.initialize_server(context, params).await",
    "custom_request": "self.custom_server_request(context, params).await",
    "initialized": "let _ = params; self.start(context)",
    "shutdown": "let _ = (context, params); self.stop().await",
    "exit": "let _ = (context, params); self.stop().await",
    "did_open_text_document": "let _ = context; self.send(ServiceCommand::Open(params.text_document)).await",
    "did_change_text_document": "let _ = context; self.send(ServiceCommand::Change(params)).await",
    "did_close_text_document": "let _ = context; self.send(ServiceCommand::Close(params.text_document.uri)).await",
    "did_save_text_document": "let _ = context; self.send(ServiceCommand::Save(params)).await",
    "did_change_workspace_folders": "let _ = context; self.send(ServiceCommand::Folders(params.event)).await",
    "formatting": "self.query_formatting(context, params.text_document.uri, Some(params.options), false).await",
    "will_save_wait_until": "self.query_formatting(context, params.text_document.uri, None, true).await",
    "completion": "self.query_completion(context, params).await",
    "hover_request": "self.hover_variable(context, params).await",
    "goto_definition": "self.query_definition(context, params).await",
    "references": "self.query_references(context, params).await",
    "document_symbol_request": "self.query_symbols(context, params.text_document.uri).await",
    "workspace_symbol_request": "let _ = params; self.query_workspace_symbols(context).await",
    "code_action_request": "self.code_actions(context, params).await",
    "execute_command": "self.execute_action(context, params).await",
    "cancel": "let _ = (context, params); Ok(())",
    "set_trace": "let _ = (context, params); Ok(())",
    "work_done_progress_cancel": "let _ = (context, params); Ok(())",
}
contracts = {
'initialize': (
    'Discover roots, load configuration and cache client presentation preferences through `initialize_server`.',
    'Shared initialization state receives selected roots, configuration and client preferences. Reads configuration and root paths. With modules enabled and no explicit builtin directory, best-effort installed-module discovery may run native `cmake --version`; failed discovery leaves the builtin directory absent. This inspection does not configure or run project scripts. Does not start the service.',
    'Client folders/root, capabilities and presentation preferences; explicit command-line roots take priority.',
    'Ignored by `initialize_server`; no context services are used.',
    '`Ok(result)` advertises the analyzer capabilities and server name/version. No output parameters.',
    'Invalid URIs or workspace limits, configuration/root I/O failures, or a poisoned state lock propagate as server errors.'),
'shutdown': (
    'Stop and join the workspace service through `stop`.',
    'Removes the shared service handle, cancels the worker and waits for it. Worker shutdown also cancels running CMake jobs; an absent service is already stopped.',
    'Unit shutdown input; ignored.', 'Ignored; stopping uses the shared analyzer state.',
    '`Ok(())` after the service joins, or immediately if it was absent.',
    'A poisoned state lock or failed worker join yields a server error. Removal/cancellation is not rolled back.'),
'exit': (
    'Stop and join the workspace service on the exit notification through `stop`.',
    'Removes the shared service handle and cancels/joins the worker, including shutdown cleanup of CMake jobs. Does not terminate the process itself.',
    'Unit exit input; ignored.', 'Ignored; stopping uses the shared analyzer state.',
    '`Ok(())` after joining, or immediately if no service is present.',
    'A poisoned state lock or failed worker join yields a server error; the handle is not restored.'),
'initialized': (
    'Start the configured workspace worker through `start`.',
    'Takes cached roots/configuration from shared initialization state, creates the bounded command queue and spawns the service. The worker may read/analyze roots and publish diagnostics.',
    'Empty acknowledgment; ignored.', 'Moved into the worker for client access and negotiated coordinates.',
    '`Ok(())` once the worker is spawned, not after initial analysis completes.',
    'A poisoned lock, already-running service or absent initialization configuration returns a server error.'),
'completion': (
    'Query completion candidates through the ordered workspace service using `query_completion`.',
    'Queues a cancellable completion query. The service may read module/package catalogs and prepare snapshot-based candidates; it does not execute CMake project code or edit text.',
    'Document URI and position; completion trigger context is not used by the query helper.',
    'Supplies cancellation; cached client snippet support is read from analyzer state.',
    '`Ok(Some(CompletionResponse::Array(items)))` contains candidates, possibly empty. The helper currently does not return `Ok(None)`.',
    'Poisoned state, unavailable service/reply, cancellation, invalid coordinates/URIs, catalog I/O or candidate-conversion failures propagate.'),
'hover_request': (
    'Query variable or command hover content through `hover_variable`.',
    'Queues an ordered hover query. The service may invoke bounded native CMake help inspection and read source documentation, but never configures or runs project scripts. Unavailable or failed help inspection is omitted; source content can still produce a hover, otherwise content may be absent.',
    'Document URI and hover position; work-done metadata is not used by the helper.',
    'Supplies cooperative cancellation for lookup and help inspection.',
    '`Ok(Some(hover))` contains available content and range; `Ok(None)` means no snapshot, target or nonempty content.',
    'Unavailable service/reply, cancellation or invalid URIs/coordinates propagate. Help spawn/read/status/timeout/overflow/decoding failures yield absent help, not an error; only cancellation propagates from help inspection.'),
'goto_definition': (
    'Query definition locations through `query_definition`.',
    'Queues a cancellable ordered navigation query over source snapshots and dependency candidates. May inspect dependency paths; does not execute CMake project code.',
    'Document URI and position forwarded to definition lookup.', 'Supplies cancellation to navigation.',
    '`Ok(Some(GotoDefinitionResponse::Array(locations)))` contains matches; `Ok(None)` means no definition location was found.',
    'Unavailable service/reply, cancellation, invalid URIs/positions and navigation conversion/I/O failures propagate.'),
'references': (
    'Query symbol references through `query_references`.',
    'Queues a cancellable ordered navigation query. Uses snapshots and owner-qualified indexes; does not edit text or execute project code.',
    'Document position and `include_declaration` flag forwarded to reference lookup.', 'Supplies cancellation to navigation.',
    '`Ok(Some(locations))` contains references, possibly empty; the helper currently does not return `Ok(None)`.',
    'Unavailable service/reply, cancellation, invalid URIs/positions and navigation conversion failures propagate.'),
'document_symbol_request': (
    'Query a document outline through `query_symbols`.',
    'Queues an ordered symbol request using only `params.text_document.uri`; the worker ensures document ownership, flushes analysis and may load the source snapshot from disk, without editing source.',
    'Only the document URI is forwarded; progress metadata is not used.', 'Supplies cancellation while waiting for the reply.',
    '`Ok(Some(symbols))` is flat or nested according to cached client support; `Ok(None)` means document symbols are disabled.',
    'Unavailable service/reply, cancellation, invalid URI, snapshot I/O or symbol-range conversion failures propagate.'),
'workspace_symbol_request': (
    'Return all indexed workspace symbols through `query_workspace_symbols`.',
    'Queues an ordered cancellable query over retained snapshots. The entire `params` value, including its search query, is ignored; the client filters/ranks results. No filesystem scan or project execution.',
    'Ignored, including the search query and progress metadata.', 'Supplies cancellation for snapshot collection and conversion.',
    '`Ok(Some(WorkspaceSymbolResponse::Flat(symbols)))` contains all enabled snapshots\' symbols, possibly empty; the helper currently never returns `Ok(None)`.',
    'Unavailable service/reply, cancellation, snapshot access or symbol conversion/worker failures propagate.'),
'execute_command': (
    'Validate and execute an analyzer action through `execute_action`.',
    'Dispatches configure, script, expanded-argument view or cancellation actions without bypassing service ownership/runtime checks. Authorized configure/script actions start background project-code execution and acknowledge the start, not completion. Each job has a separate stop token and can outlive this request; explicit cancellation, runtime invalidation or shutdown can stop it. The service sends a start message and later completion/failure messages for still-current runs; invalidated or superseded results are discarded. Expanded-view actions send recorded output to the client.',
    'Analyzer command plus document URI, root path, source line and optional execution profile arguments.',
    'Supplies cancellation checked before a configure/script job starts and while waiting for the action reply. Cancelling this request does not cancel an already-started job or undo action effects.',
    '`Ok(Some(data))` has an action-specific shape: configure/script returns `Data::Object` with `started: Data::Bool(true)` and `run: Data::Unsigned(id)` after launching the job; it does not wait for the process result. Cancellation returns `Data::Bool(true)` when a matching queued save-run was removed or a matching running job was signalled, otherwise `false`; it does not join the job. Expanded view returns an object with `run` (unsigned run ID), `events` (list) and `truncated` (boolean). Each event object contains `event` and `frame` unsigned numbers, `command` text, `arguments` text list, `observed_values` objects with `name`, `namespace` and nullable `value`, and `dependencies` objects with `kind` and nullable `resolved` text. Current actions always return `Some`; `None` is not produced.',
    'Unknown commands return MethodNotFound. Argument/ownership/runtime-policy validation, input preparation, unavailable service/reply, request cancellation and immediate action/client-message failures propagate. A start-message failure can occur after the job is launched; the job is not rolled back. Later configure/script execution or runtime-installation failures are reported by client messages, not by this request\'s result.'),
'code_action_request': (
    'List available analyzer actions through `code_actions` without executing them.',
    'Automatic triggers or incompatible action-kind filters return an empty list immediately. Otherwise queues an ordered listing query for the URI and start line; no project code is executed.',
    'Trigger/filter context plus document URI and range start line; other range and diagnostic details are not used.',
    'Supplies cancellation while waiting for the service reply.',
    '`Ok(Some(actions))` lists explicit source actions, possibly empty; the helper currently never returns `Ok(None)`.',
    'Unavailable service/reply, cancellation, invalid URI or non-UTF-8 project paths propagate as server errors.'),
'formatting': (
    'Request whole-document formatting through `query_formatting` with client style options.',
    'Queues a cancellable formatting barrier with `save = false` and `Some(params.options)`. Returns client-applied edits; never writes the source file or executes project code.',
    'Document URI and client formatting options; progress metadata is not used.', 'Supplies cancellation for rendering and the reply wait.',
    '`Ok(Some(edits))` is empty for unchanged/invalid/unavailable text, otherwise contains a whole-document replacement. The helper currently never returns `Ok(None)`.',
    'Unavailable service/reply, cancellation, invalid URI/style/coordinates, conflicting shared-owner styles or worker failures propagate.'),
'will_save_wait_until': (
    'Request pre-save formatting through `query_formatting` using configured style.',
    'Passes `None` for client options and `save = true`; any owner disabling on-save formatting suppresses edits. Returns client-applied edits without saving a file or executing project code.',
    'Only the document URI is forwarded; the save reason is not used.', 'Supplies cancellation for rendering and the reply wait.',
    '`Ok(Some(edits))` is empty when formatting is suppressed, unchanged or unavailable; otherwise contains a whole-document replacement. The helper currently never returns `Ok(None)`.',
    'Unavailable service/reply, cancellation, invalid URI/style/coordinates, conflicting shared-owner styles or worker failures propagate.'),
'custom_request': (
    'Handle the analyzer configuration-reload extension through `custom_server_request`.',
    'Recognizes only `cmake-analyzer/reloadConfig`. Queues a transactional reload that may read configuration and rebuild workspaces. A fresh configuration store repeats best-effort installed-module discovery: with modules enabled and no explicit builtin directory, native `cmake --version` may run; failed discovery leaves the builtin directory absent. This inspection does not configure or run project scripts, and reload does not itself authorize project execution.',
    'Extension method and optional strict JSON object with a configuration override `path`.',
    'Supplies cancellation while awaiting the reload result.',
    '`Ok(value)` is a JSON object with `reloaded: true` and the workspace count after success.',
    'Other methods return MethodNotFound. Invalid JSON fields, unavailable service/reply, cancellation or reload failures propagate; failed reload validation preserves the live configuration.'),
'did_open_text_document': (
    'Queue an opened document through `ServiceCommand::Open`.',
    'Sends the full document to the bounded service queue; the worker later owns overlay updates, analysis and diagnostics. Does not await analysis completion.',
    'Only `text_document` is moved into the Open command.', 'Ignored; the service already retains its session context.',
    '`Ok(())` when the command is enqueued, not when document analysis completes.',
    'A poisoned state lock, absent service or closed command queue returns a server error.'),
'did_change_text_document': (
    'Queue a document version and content changes through `ServiceCommand::Change`.',
    'Moves all change parameters into the bounded service queue. The worker later applies/coalesces changes and refreshes analysis/diagnostics; this hook does not await those effects.',
    'Versioned document and ordered content changes moved into the Change command.', 'Ignored; the service already retains its session context.',
    '`Ok(())` once the command is enqueued; it does not acknowledge successful edit application.',
    'A poisoned state lock, absent service or closed command queue returns a server error.'),
'did_close_text_document': (
    'Queue document closure through `ServiceCommand::Close`.',
    'Moves only the URI into the bounded queue. The worker later removes the overlay and clears diagnostics; this hook does not await that cleanup.',
    'Only `text_document.uri` is moved into the Close command.', 'Ignored; the service already retains its session context.',
    '`Ok(())` once closure is enqueued, not when cleanup completes.',
    'A poisoned state lock, absent service or closed command queue returns a server error.'),
'did_save_text_document': (
    'Queue a reported save through `ServiceCommand::Save`.',
    'Moves all save parameters into the bounded queue. The worker handles invalidation and may schedule globally authorized on-save CMake jobs. This hook never writes the document.',
    'Document URI and optional saved text moved into the Save command.', 'Ignored; the service already retains its session context.',
    '`Ok(())` after enqueueing; worker save handling or scheduled jobs have not necessarily completed.',
    'A poisoned state lock, absent service or closed command queue returns a server error.'),
'did_change_workspace_folders': (
    'Queue workspace folder membership changes through `ServiceCommand::Folders`.',
    'Moves the folder event into the bounded queue. The worker later changes projects and may read/analyze roots or invalidate running jobs; the hook only awaits enqueueing.',
    'Only the added/removed `event` is moved into the Folders command.', 'Ignored; the service already retains its session context.',
    '`Ok(())` once the event is enqueued, not when project refresh completes.',
    'A poisoned state lock, absent service or closed command queue returns a server error.'),
}


def method_docs(name, params):
    """Render Rustdoc for one concrete analyzer hook without changing its body.

    Args:
        name: Trait method spelling used for links and a concrete call example.
        params: Rust parameter type used in the compiling example's argument.
    Returns:
        Indented declaration documentation. There is no file I/O or mutation.
    Example:
        method_docs("cancel", "CancelParams") documents an ignored notification.
    """
    if name in contracts:
        summary, effects, parameter, context, result, errors = contracts[name]
        receiver = "Shared analyzer state/service used by the forwarded helper; effects are described above."
    elif name in {"cancel", "set_trace", "work_done_progress_cancel"}:
        summary = f"Acknowledge [`ILanguageServer::{name}`] without changing analyzer state."
        effects = ("No side effects. Both arguments are discarded. Request cancellation and trace "
                   "state are handled by the wrapper reader; this hook performs no extra action.")
        if name == "work_done_progress_cancel":
            effects = ("No side effects. Both arguments are discarded; this analyzer hook does not "
                       "cancel work associated with the progress token.")
        receiver = "Unused analyzer; no state is changed."
        context = "Ignored, including its cancellation and client handle."
        parameter = f"Ignored `{params}` notification payload."
        result = "Always `Ok(())`; no output parameters."
        errors = None
    else:
        summary = f"Reject [`ILanguageServer::{name}`] with MethodNotFound; no side effects."
        effects = "This feature is not implemented here. Neither argument is inspected; no work is queued."
        receiver = "Unused analyzer; no state is changed."
        context = "Ignored, including its cancellation and client handle."
        parameter = f"Ignored `{params}` payload; no fields are read or changed."
        result = "Always `Err(ServerError::unsupported())` (-32601); no successful value or output parameters."
        errors = "Every call returns MethodNotFound, regardless of the payload or session state."
    lines = [summary, effects, "", "# Arguments",
             f"- `self` `{'[in,out]' if name in contracts else '[in]'}`: {receiver}",
             f"- `context` `{'[in,out]' if name in contracts and not context.startswith('Ignored') else '[in]'}`: {context}",
             f"- `params` `[in]`: {parameter}",
             "# Returns", result, "No caller-owned output parameter is overwritten or appended."]
    if errors:
        lines.extend(["# Errors", errors])
    # Forwarding is a single helper call; the containing module explains the
    # queue/reply split. The methods themselves add no branching algorithm.
    lines.extend(["# Examples"])
    if name in contracts:
        lines.extend(["Requires a Tokio runtime and a wrapper-supplied call context. Initialize/start",
                      "the service before queries or events; use an authorized project for actions.",
                      "```no_run"])
    else:
        lines.append("```")
    lines.extend(["use cmake_analyzer::server::CmakeAnalyzer;",
                  "use lspally::server::model::*;",
                  f"async fn example(server: &CmakeAnalyzer, context: Context, params: {params}) {{",
                  f"    let result = server.{name}(context, params).await;"])
    if name not in handlers:
        lines.append("    assert!(matches!(result, Err(error) if error.code == -32601));")
    elif name not in contracts:
        lines.append("    assert!(result.is_ok());")
    else:
        lines.append("    let _reply = result; // The caller handles the documented success/error outcomes.")
    lines.extend(["}", "```"])
    return "\n".join("    ///" + (" " + line if line else "") for line in lines)


assert len(methods) == 73, "Review the changed trait inventory"
output = ["// Script-generated by scripts/gen/cmake_server.py.",
          "//! Implement analyzer hooks by forwarding supported work and explicitly rejecting unsupported features.",
          "//! The wrapper calls this implementation; capability advertisement remains in `initialize_server`.",
          "//! Unsupported hooks have no side effects. Supported hooks may read files, mutate shared state,",
          "//! publish messages or execute authorized project actions as documented on each method.",
          "//!",
          "//! # Flow",
          "//! ```text",
          "//! [wrapper: typed call]          [analyzer shared state]",
          "//!          |                              |",
          "//!    <supported hook?>                    |",
          "//!      /          \\",
          "//!     no          yes                     |",
          "//!      |           |                      |",
          "//! [unsupported] [helper] -----------------+",
          "//!                  |",
          "//!       +----------+------------+",
          "//!       v                       v",
          "//! [direct init/start/stop] [bounded service queue]",
          "//!       |                       |",
          "//!       |                [ordered worker + I/O]",
          "//!       |                       |",
          "//!       +---------+-------------+",
          "//!                 v",
          "//!       [result or enqueue acknowledgment]",
          "//! ```",
          "use crate::server::model::{CmakeAnalyzer, ServiceCommand};", "use lspally::server::model::*;", "impl ILanguageServer for CmakeAnalyzer {"]
for name, params, result in methods:
    params = params.strip().rstrip(',')
    body = handlers.get(name, "let _ = (context, params); Err(ServerError::unsupported())")
    output.append(method_docs(name, params))
    output.append(f"async fn {name}(&self, context: Context, params: {params}) -> ServerResult<{result}> {{ {body} }}")
output.append("}")
path = root / "src/server/handlers/implementation.rs"
path.parent.mkdir(parents=True, exist_ok=True)
path.write_text('\n'.join(output) + '\n')
