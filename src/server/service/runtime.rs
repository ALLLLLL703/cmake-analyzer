// AI-authored service actor: ordered queries are barriers; queued edits coalesce.
use super::super::model::*;
use lspally::server::model::*;
use std::collections::BTreeMap;
use tokio::sync::mpsc;

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
