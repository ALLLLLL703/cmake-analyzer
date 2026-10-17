//! Run the single mutable owner of projects, editor text and publications.
//! Receive order is authoritative. Up to the edit-batch limit of notifications are
//! applied before a flush; a query stops that batch and runs after the flush, before
//! later queued notifications. Diagnostic publication waits until queued edits settle.
//! Runtime jobs run independently and their completions re-enter this loop. Reports
//! are prepared only when the queue looks idle, then discarded or interrupted when
//! new commands arrive. Already-sent reports cannot be recalled.
// AI-authored service actor: ordered queries are barriers; queued edits coalesce.
use super::super::model::*;
use lspally::server::model::*;
use std::collections::BTreeMap;
use tokio::sync::mpsc;

/// Build service-owned state, admit initial projects and drive messages until stop/EOF.
/// Initial project/flush errors are logged to stderr; runtime jobs are cancelled and joined on exit.
///
/// # Arguments
/// - `roots` `[in]`: Consumed initial normalized entry paths; admission is attempted for each.
/// - `config` `[in]`: Loaded store moved into the actor.
/// - `context` `[in]`: Request context consumed for cancellation and/or client session data.
/// - `hierarchical` `[in]`: Whether document symbols may be nested.
/// - `hover_markdown` `[in]`: Whether hover text should use Markdown.
/// - `receiver` `[in]`: Consumed bounded command receiver.
/// - `stop` `[in]`: Service cancellation signal observed by the loop.
///
/// # Returns
/// `()` after the command loop exits and all runtime jobs have been reaped.
///
/// # Algorithm
/// Construct caches/maps, add roots, flush once, run the serialized loop, then stop all configures. Root errors do not abort startup.
///
/// # Flow
/// ```text
///  [Roots + session]
///          |
///    [Construct Service]
///          |
///    [Add roots + flush] ----> [Log failures]
///          |
///       [Drive loop]
///          |
///    [Cancel + join jobs]
/// ```
///
/// # Examples
/// ```text
/// run(roots, config, context, hierarchical, hover_markdown, receiver, stop).await;
/// ```
pub(super) async fn run(
    roots: Vec<std::path::PathBuf>,
    config: crate::config::ConfigStore,
    context: Context,
    hierarchical: bool,
    hover_markdown: bool,
    receiver: mpsc::Receiver<ServiceCommand>,
    stop: Cancellation,
) {
    let mut service = Service {
        local_help: super::super::hovers::LocalHelp::new(
            config.sources.cwd.clone(),
            config.global_runtime(),
        ),
        hover_markdown,
        roots: roots.iter().cloned().collect(),
        config,
        projects: Vec::new(),
        documents: BTreeMap::new(),
        clears: Vec::new(),
        published: BTreeMap::new(),
        client: context.client,
        encoding: context.encoding,
        hierarchical,
        jobs: tokio::task::JoinSet::new(),
        running: BTreeMap::new(),
        run_counter: 0,
        saved_runs: Default::default(),
    };
    for root in roots {
        if let Err(error) = service.add_project(root, false, None).await {
            eprintln!("{error}");
        }
    }
    if let Err(error) = service.flush().await {
        eprintln!("{error}");
    }
    drive(&mut service, receiver, stop).await;
    service.stop_configures().await;
}
/// Process bounded edit batches and query barriers, then publish only idle report batches.
/// Mutates service state, logs recoverable failures and sends replies/diagnostics; exits on stop, closed queue or publication failure.
///
/// # Arguments
/// - `service` `[in,out]`: Actor state changed by edits, queries, runtime completions and successful publication-cache writes.
/// - `receiver` `[in]`: Consumed FIFO queue, drained in receive order up to the edit-batch bound.
/// - `stop` `[in]`: Cancellation signal checked between joined edit/flush/query phases and during publication.
///
/// # Returns
/// `()` on loop termination; edit/query/report errors are logged or replied rather than returned.
///
/// # Algorithm
/// Select a pending command, job completion or queue receive. Apply up to EDIT_BATCH_SIZE notifications, retaining the first query/overflow command. Flush before answering that query; skip replies whose receivers closed. If more commands exist, continue without reports. Otherwise start pending saves, prepare reports, recheck arrivals before each send, and update publication keys only after success. Already-sent reports remain sent; unfinished clear URIs are retained for retry.
///
/// # Flow
/// ```text
///  [Queue / job completion] <----------------------+
///              |                                  |
///        <Edit or query?>                          |
///         /          \                            |
///   [Apply batch]   [Hold query]                    |
///         \          /                            |
///           [Flush + reply]                        |
///                  |                              |
///           <More commands?> -- yes ---------------+
///                  | no                           |
///         [Start saves + reports]                  |
///                  |                              |
///          <Arrival before send?> -- yes ----------+
///                  | no                           |
///          [Publish + cache key] ------------------+
/// ```
///
/// # Examples
/// ```text
/// drive(&mut service, receiver, stop).await;
/// ```
async fn drive(
    service: &mut Service,
    mut receiver: mpsc::Receiver<ServiceCommand>,
    stop: Cancellation,
) {
    let mut pending = None;
    loop {
        if stop.is_cancelled() {
            return;
        }
        let command = match pending.take() {
            Some(command) => command,
            None => {
                tokio::select! {_=stop.cancelled()=>return,
                completion=service.jobs.join_next(),if !service.jobs.is_empty()=>{if let Some(completion)=completion {service.configure_finished(completion).await;}ServiceCommand::RuntimeRefresh},
                command=receiver.recv()=>match command {Some(command)=>command,None=>return}}
            }
        };
        let mut query = None;
        let mut command = Some(command);
        for _ in 0..super::EDIT_BATCH_SIZE {
            let Some(current) = command.take() else {
                break;
            };
            if matches!(
                current,
                ServiceCommand::Format { .. }
                    | ServiceCommand::WorkspaceSymbols { .. }
                    | ServiceCommand::Symbols { .. }
                    | ServiceCommand::Hover { .. }
                    | ServiceCommand::Navigate { .. }
                    | ServiceCommand::Complete { .. }
                    | ServiceCommand::ReloadConfig { .. }
                    | ServiceCommand::Actions { .. }
                    | ServiceCommand::RunAction { .. }
            ) {
                query = Some(current);
                break;
            }
            // Join bounded edit work rather than abandoning a running blocking task on shutdown.
            let applied = service.apply(current).await;
            if stop.is_cancelled() {
                return;
            }
            if let Err(error) = applied {
                eprintln!("{error}");
            }
            command = receiver.try_recv().ok();
        }
        pending = command;
        if let Err(error) = service.flush().await {
            eprintln!("{error}");
        }
        if stop.is_cancelled() {
            return;
        }
        match query {
            Some(ServiceCommand::Format {
                uri,
                options,
                save,
                cancel,
                reply,
            }) if !reply.is_closed() => {
                let result = service.format_document(uri, options, save, cancel).await;
                let _ = reply.send(result);
            }
            Some(ServiceCommand::Complete {
                uri,
                position,
                snippets,
                cancel,
                reply,
            }) if !reply.is_closed() => {
                let result = service.complete(uri, position, snippets, cancel).await;
                let _ = reply.send(result);
            }
            Some(ServiceCommand::Navigate {
                uri,
                position,
                query,
                cancel,
                reply,
            }) if !reply.is_closed() => {
                let result = service.navigate(uri, position, query, cancel).await;
                let _ = reply.send(result);
            }
            Some(ServiceCommand::Hover {
                cancel,
                uri,
                position,
                reply,
            }) if !reply.is_closed() => {
                let result = service.variable_hover(uri, position, cancel).await;
                let _ = reply.send(result);
            }
            Some(ServiceCommand::WorkspaceSymbols { cancel, reply }) if !reply.is_closed() => {
                let result = service.workspace_symbols(cancel).await;
                let _ = reply.send(result);
            }
            Some(ServiceCommand::Symbols { uri, reply }) if !reply.is_closed() => {
                let result = service.symbols(uri).await;
                let _ = reply.send(result);
            }
            Some(ServiceCommand::ReloadConfig { path, reply }) if !reply.is_closed() => {
                let result = service.reload_config(path).await;
                let _ = reply.send(result);
            }
            Some(ServiceCommand::Actions { uri, line, reply }) if !reply.is_closed() => {
                let _ = reply.send(service.actions(uri, line));
            }
            Some(ServiceCommand::RunAction { request, reply }) if !reply.is_closed() => {
                let result = service.run_action(request).await;
                let _ = reply.send(result);
            }
            _ => {}
        }
        if pending.is_none() {
            pending = receiver.try_recv().ok();
        }
        if pending.is_some() {
            continue;
        }
        service.start_saved_runs().await;
        let reports = match service.reports().await {
            Ok(reports) => reports,
            Err(error) => {
                eprintln!("{error}");
                continue;
            }
        };
        // Edits arriving while analysis/conversion ran supersede this report batch.
        pending = receiver.try_recv().ok();
        if pending.is_some() {
            continue;
        }
        for report in reports {
            pending = receiver.try_recv().ok();
            if pending.is_some() {
                break;
            }
            let published = tokio::select! {_=stop.cancelled()=>return,result=service.client.publish_diagnostics(report.report)=>result};
            if let Err(error) = published {
                eprintln!("Diagnostics: {error}");
                return;
            }
            if let Some(key) = report.key {
                service.published.insert(report.path, key);
            } else {
                service.published.remove(&report.path);
            }
        }
        if pending.is_none() {
            service.clears.clear();
        }
    }
}
