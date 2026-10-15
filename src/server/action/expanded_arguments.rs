// AI-authored on-demand presentation of recorded command expansions and value observations.
use super::{Service, model::ActionRequest, uri};
use lspally::server::model::*;
use std::collections::BTreeMap;
pub(super) async fn run(
    service: &mut Service,
    request: ActionRequest,
) -> ServerResult<Option<Data>> {
    let owner = service.action_project(&request)?;
    service.require_saved(owner).await?;
    let project = &service.projects[owner];
    let layer = project
        .workspace
        .runtime()
        .ok_or_else(|| ServerError::invalid("No current runtime result; run CMake first"))?;
    let file = uri::to_path(&request.uri)?;
    let source = layer
        .documents
        .get(&file)
        .ok_or_else(|| ServerError::invalid("File was not observed"))?;
    let position = Position {
        line: request.line as u32,
        character: 0,
    };
    let offset = super::super::text::TextIndex::new(&source.text, service.encoding)
        .edit_range(&Range {
            start: Position {
                line: position.line,
                character: position.character,
            },
            end: position,
        })?
        .start;
    let line = source
        .semantics
        .commands
        .iter()
        .find(|command| {
            command.span.row == request.line
                || command.full_span.start_byte <= offset && command.full_span.end_byte > offset
        })
        .map_or(request.line + 1, |command| command.span.row + 1);
    let mut events = Vec::new();
    let mut display = String::new();
    let mut truncated = false;
    for event in layer
        .trace
        .events
        .iter()
        .filter(|event| event.file == file && u64::from(event.line) == line)
    {
        let observations = &layer.values[layer.event_values[event.id.index()].clone()];
        let raw_size = event.arguments.iter().fold(0usize, |size, text| {
            size.saturating_add(text.len()).saturating_add(1)
        });
        let raw_size = observations.iter().fold(raw_size, |size, value| {
            size.saturating_add(value.name.len()).saturating_add(
                value
                    .possible_value
                    .as_ref()
                    .map_or(0, |value| value.text().len()),
            )
        });
        if raw_size
            > project
                .settings
                .runtime
                .max_view_bytes
                .saturating_sub(display.len())
        {
            truncated = true;
            break;
        }
        let arguments = serde_json::to_string(&event.arguments)
            .map_err(|error| ServerError::new(-32603, error.to_string()))?;
        let mut text = format!(
            "#{} frame {} {} {}\n",
            event.id.index(),
            event.frame.depth(),
            event.command,
            arguments
        );
        let mut observed = Vec::new();
        for value in observations {
            text.push_str(&format!(
                "  {:?} {:?} = {:?}\n",
                value.namespace,
                value.name,
                value.possible_value.as_ref().map(|value| value.text())
            ));
            observed.push(Data::Object(BTreeMap::from([
                ("name".into(), Data::Text(value.name.clone())),
                (
                    "namespace".into(),
                    Data::Text(format!("{:?}", value.namespace)),
                ),
                (
                    "value".into(),
                    value
                        .possible_value
                        .as_ref()
                        .map_or(Data::Null, |value| Data::Text(value.text().into())),
                ),
            ])));
        }
        let mut dependencies = Vec::new();
        let dependency = layer
            .trace
            .dependencies
            .binary_search_by_key(&event.id.index(), |dependency| dependency.event.index())
            .ok()
            .map(|index| &layer.trace.dependencies[index]);
        if let Some(dependency) = dependency {
            text.push_str(&format!(
                "  {:?}: {:?}\n",
                dependency.kind, dependency.resolved
            ));
            dependencies.push(Data::Object(BTreeMap::from([
                ("kind".into(), Data::Text(format!("{:?}", dependency.kind))),
                (
                    "resolved".into(),
                    dependency
                        .resolved
                        .as_ref()
                        .map_or(Data::Null, |path| Data::Text(path.display().to_string())),
                ),
            ])));
        }
        if text.len()
            > project
                .settings
                .runtime
                .max_view_bytes
                .saturating_sub(display.len())
        {
            truncated = true;
            break;
        }
        display.push_str(&text);
        events.push(Data::Object(BTreeMap::from([
            ("event".into(), Data::Unsigned(event.id.index() as u64)),
            ("frame".into(), Data::Unsigned(event.frame.depth())),
            ("command".into(), Data::Text(event.command.clone())),
            (
                "arguments".into(),
                Data::List(event.arguments.iter().cloned().map(Data::Text).collect()),
            ),
            ("observed_values".into(), Data::List(observed)),
            ("dependencies".into(), Data::List(dependencies)),
        ])));
    }
    if events.is_empty() {
        return Err(ServerError::invalid(if truncated {
            "Expanded command exceeds runtime.max_view_bytes"
        } else {
            "This command has no observed execution"
        }));
    }
    if truncated {
        let notice = "\nFurther executions omitted by runtime.max_view_bytes.";
        if notice.len()
            <= project
                .settings
                .runtime
                .max_view_bytes
                .saturating_sub(display.len())
        {
            display.push_str(notice);
        }
    }
    service
        .client
        .show_message(ShowMessageParams {
            typ: MessageType(3),
            message: display,
        })
        .await?;
    Ok(Some(Data::Object(BTreeMap::from([
        ("run".into(), Data::Unsigned(layer.trace.run.value())),
        ("events".into(), Data::List(events)),
        ("truncated".into(), Data::Bool(truncated)),
    ]))))
}
