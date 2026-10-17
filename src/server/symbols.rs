// AI-authored symbols from semantic root symbols and direct block membership.
mod workspace;
use super::{model::SymbolInput, text::TextIndex};
use crate::{model::TextSpan, parser::semanticer::model::*};
use lspally::server::model::*;

pub(super) fn document_symbols(input: SymbolInput) -> ServerResult<Option<DocumentSymbolResponse>> {
    let buffer = &input.snapshot.semantics;
    let index = TextIndex::new(&input.snapshot.text, input.encoding);
    if input.functions_only {
        let mut symbols = Vec::new();
        for function in &buffer.functions {
            let Some(definition) = &function.definition else {
                continue;
            };
            let Some(block) = function
                .block
                .and_then(|block| buffer.blocks.get(block.id as usize))
                .filter(|block| block.parent.is_none())
            else {
                continue;
            };
            symbols.push(make_symbol(
                function.name.clone(),
                12,
                block.span,
                definition.span,
                &index,
            )?);
        }
        return Ok(Some(DocumentSymbolResponse::Flat(flatten(
            symbols, input.uri,
        ))));
    }
    let mut blocks: Vec<Option<DocumentSymbol>> = (0..buffer.blocks.len()).map(|_| None).collect();
    // Blocks are created before their descendants; assemble bottom-up without recursive traversal.
    for block in buffer.blocks.iter().rev() {
        let opening = &buffer.commands[block.opening.id as usize];
        let mut children = Vec::new();
        for id in &block.contains {
            if *id != block.opening && Some(*id) != block.closing {
                children.push(command_symbol(&buffer.commands[id.id as usize], &index)?);
            }
        }
        for id in &block.children {
            if let Some(symbol) = blocks[id.id as usize].take() {
                children.push(symbol);
            }
        }
        children.sort_by_key(|symbol| (symbol.range.start.line, symbol.range.start.character));
        let name = match block.kind {
            BlockKind::Function | BlockKind::Macro => opening
                .args
                .first()
                .map(|arg| arg.content.clone())
                .unwrap_or_else(|| opening.name.clone()),
            _ => opening.name.clone(),
        };
        let mut symbol = make_symbol(
            name,
            if matches!(block.kind, BlockKind::Function | BlockKind::Macro) {
                12
            } else {
                3
            },
            block.span,
            if matches!(block.kind, BlockKind::Function | BlockKind::Macro) {
                opening.args.first().map_or(opening.span, |arg| arg.span)
            } else {
                opening.span
            },
            &index,
        )?;
        symbol.children = (!children.is_empty()).then_some(children);
        blocks[block.id.id as usize] = Some(symbol);
    }
    let mut symbols = Vec::with_capacity(buffer.symbols.len());
    for symbol in &buffer.symbols {
        match symbol.kind {
            SemanticSymbolKind::Command(id) => {
                symbols.push(command_symbol(&buffer.commands[id.id as usize], &index)?)
            }
            SemanticSymbolKind::Block(id) => {
                if let Some(symbol) = blocks[id.id as usize].take() {
                    symbols.push(symbol);
                }
            }
        }
    }
    if input.hierarchical {
        Ok(Some(DocumentSymbolResponse::Nested(symbols)))
    } else {
        Ok(Some(DocumentSymbolResponse::Flat(flatten(
            symbols, input.uri,
        ))))
    }
}
fn command_symbol(
    command: &SemanticCommandRepresent,
    index: &TextIndex<'_>,
) -> ServerResult<DocumentSymbol> {
    // LSP kinds: Module=2, Class=5, Function=12, Variable=13.
    let kind = match command.name.to_ascii_lowercase().as_str() {
        "set" | "option" => 13,
        "project" => 2,
        "add_library" | "add_executable" | "add_custom_target" => 5,
        _ => 12,
    };
    let named = if kind != 12 {
        command.args.first()
    } else {
        None
    };
    let name = named.map_or_else(|| command.name.clone(), |arg| arg.content.clone());
    let selection = named.map_or(command.span, |arg| arg.span);
    make_symbol(name, kind, command.full_span, selection, index)
}
fn make_symbol(
    name: String,
    kind: i32,
    span: TextSpan,
    selection: TextSpan,
    index: &TextIndex<'_>,
) -> ServerResult<DocumentSymbol> {
    Ok(DocumentSymbol {
        name,
        detail: None,
        kind: SymbolKind(kind),
        tags: None,
        deprecated: None,
        range: index.range(span.start_byte..span.end_byte)?,
        selection_range: index.range(selection.start_byte..selection.end_byte)?,
        children: None,
    })
}
fn flatten(symbols: Vec<DocumentSymbol>, uri: Uri) -> Vec<SymbolInformation> {
    let mut pending: Vec<_> = symbols
        .into_iter()
        .rev()
        .map(|symbol| (symbol, None))
        .collect();
    let mut result = Vec::new();
    while let Some((symbol, container_name)) = pending.pop() {
        if let Some(children) = symbol.children {
            pending.extend(
                children
                    .into_iter()
                    .rev()
                    .map(|child| (child, Some(symbol.name.clone()))),
            );
        }
        result.push(SymbolInformation {
            name: symbol.name,
            kind: symbol.kind,
            tags: None,
            deprecated: None,
            location: Location {
                uri: Uri(uri.0.clone()),
                range: symbol.range,
            },
            container_name,
        });
    }
    result
}
