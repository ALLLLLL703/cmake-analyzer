//! Convert semantic root symbols and direct block membership to protocol symbols.
//! Blocks are assembled from descendants to ancestors without recursion. Flat output
//! retains each immediate parent name. Conversion has no I/O or snapshot mutation.
// AI-authored symbols from semantic root symbols and direct block membership.
mod workspace;
use super::{model::SymbolInput, text::TextIndex};
use crate::{model::TextSpan, parser::semanticer::model::*};
use lspally::server::model::*;

/// Render semantic commands/blocks as nested or flat protocol symbols. No I/O or analysis mutation.
///
/// # Arguments
/// - `input` `[in]`: Consumed snapshot, coordinate encoding, URI and full/functions-only presentation flags.
///
/// # Returns
/// `Ok(Some(response))`, even for no symbols; restricted function output is always Flat, otherwise hierarchical chooses Nested versus Flat. `Err` on range conversion.
///
/// # Errors
/// Invalid semantic byte spans or protocol coordinate overflow.
///
/// # Panics
/// Malformed semantic arenas with invalid command/block IDs can panic; input must be a consistent analyzed snapshot.
///
/// # Algorithm
/// For restricted inputs collect only defined functions/macros whose body block has no parent. Otherwise build block symbols in reverse arena order, gather direct commands and assembled child blocks, sort children by source position, then select semantic root symbols. Flatten iteratively for clients without hierarchy.
///
/// # Flow
/// ```text
///  [SymbolInput]
///        |
///  <Functions only?>
///     /           \
///   yes           no
///    |             |
///  [Top-level] [Reverse block assembly]
///    |             |
///  [Flat]      [Select root symbols]
///                  |
///           <Hierarchical client?>
///              /            \
///          [Nested]       [Flatten]
/// ```
///
/// # Examples
/// ```text
/// let response = document_symbols(SymbolInput { functions_only: false, uri,
///     snapshot, encoding: CoordinateEncoding::Utf16, hierarchical: true })?;
/// ```
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
/// Classify a command by native spelling and choose an argument name for variables/projects/targets.
/// No side effects; classification is display-only and does not evaluate the command.
///
/// # Arguments
/// - `command` `[in]`: Semantic command with name, argument and full ranges.
/// - `index` `[in]`: Coordinate index for the owning snapshot.
///
/// # Returns
/// `Ok(DocumentSymbol)` without children, with command-full range and selected name/argument range; `Err` on coordinate conversion.
///
/// # Errors
/// Invalid byte endpoints or coordinate overflow.
///
/// # Algorithm
/// set/option become Variable, project becomes Module, target constructors become Class; all others become Function. Non-Function kinds use the first argument if present, otherwise the command name.
///
/// # Flow
/// ```text
///  [Command name]
///         |
///  <set/option/project/target?>
///        /               \
///      yes               no
///       |                 |
///  [Typed + first arg] [Function + command]
///       +-----------------+
///                v
///           [make_symbol]
/// ```
///
/// # Examples
/// ```text
/// let symbol = command_symbol(&buffer.commands[0], &index)?;
/// ```
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
/// Create a childless symbol by converting full and selection byte spans. No I/O or input mutation.
///
/// # Arguments
/// - `name` `[in]`: Consumed display label.
/// - `kind` `[in]`: LSP symbol kind number selected by the caller.
/// - `span` `[in]`: Full command/block range.
/// - `selection` `[in]`: Name range selected when navigating to the symbol.
/// - `index` `[in]`: Index for the exact containing source text.
///
/// # Returns
/// `Ok(DocumentSymbol)` with absent detail/tags/children; `Err` if either range cannot be converted.
///
/// # Errors
/// Invalid byte endpoints or coordinate overflow; containment between the two ranges is not separately validated.
///
/// # Examples
/// ```text
/// let symbol = make_symbol("demo".into(), 12, block_span, name_span, &index)?;
/// ```
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
/// Consume a symbol tree in source/preorder and attach immediate parent names to flat entries.
/// No I/O; nested input is consumed, not mutated through borrowed output parameters.
///
/// # Arguments
/// - `symbols` `[in]`: Owned root symbols whose child trees are consumed.
/// - `uri` `[in]`: Consumed URI cloned into every flat location.
///
/// # Returns
/// Flat symbols in root/preorder; empty input yields an empty vector. Root container_name is None and children carry only their immediate parent name.
///
/// # Algorithm
/// Use a reversed explicit stack rather than recursion. Pop a symbol, push its children in reverse with its name as container, then append a flat entry using the full range.
///
/// # Flow
/// ```text
///  [Roots reversed on stack]
///              |
///          <Pop node?> -- none --> [Return output]
///              | some
///  [Push reversed children + parent name]
///              |
///      [Append flat location]
///              |
///        [Continue stack loop]
/// ```
///
/// # Examples
/// ```text
/// let flat = flatten(nested_symbols, uri);
/// assert!(flat.first().is_none_or(|symbol| symbol.container_name.is_none()));
/// ```
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
