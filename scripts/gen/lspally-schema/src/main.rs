// AI-authored generator.
//! Generate lsp-types 0.97 seed declarations, adapters, and inventory.
//! Args: lsp-types source directory, output directory outside target/.
//! Outputs need review; do not overwrite the adapted production files.
//! Incoming-hook docs are copied from the tracked, adapted `ILanguageServer` trait.
use quote::ToTokens;
use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    path::PathBuf,
};
use syn::{Fields, GenericArgument, Item, PathArguments, Type};

/// Parsed upstream declaration with its source module, used by dependency discovery.
/// Cloning copies the syntax tree; it does not read files. [`types`] reads its
/// payload types and [`domain_type`] selects business-model spellings.
///
/// # Examples
/// ```text
/// let entry = Entry { module: "document".into(), item: syn::parse_quote!(pub struct Position { pub line: u32 }) };
/// assert_eq!(types(&entry).len(), 1);
/// ```
#[derive(Clone)]
struct Entry {
    /// Source filename without `.rs`; selects the seed output group. Example: `entry.module == "completion"`.
    module: String,
    /// Parsed struct, enum or alias, including upstream attributes. Example: `matches!(entry.item, Item::Struct(_))`.
    item: Item,
}
/// Render syntax tokens as Rust source text without I/O.
///
/// # Arguments
/// - `t` `[in]`: syntax node to consume into a token stream.
/// # Returns
/// Source text with token-stream spacing; not original formatting.
/// # Examples
/// ```text
/// let code = text(syn::parse_str::<syn::ItemStruct>("struct Empty;").unwrap());
/// ```
fn text(t: impl ToTokens) -> String {
    t.into_token_stream().to_string()
}
/// Render only Rust documentation attributes for a seed declaration; no I/O.
/// Serialization, conditional-compilation and representation attributes are not copied.
/// Upstream examples and links still need review after business-type adaptations.
///
/// # Arguments
/// - `attrs` `[in]`: upstream declaration, field or variant attributes to inspect.
/// # Returns
/// Rust `#[doc = ...]` source text in original order, empty when no docs exist.
/// No caller state or output parameter is changed.
/// # Examples
/// ```text
/// let item: syn::ItemStruct = syn::parse_quote!(#[doc = "A flag."] struct Flag;);
/// assert!(documentation(&item.attrs).contains("A flag."));
/// ```
fn documentation(attrs: &[syn::Attribute]) -> String {
    attrs
        .iter()
        .filter(|attribute| attribute.path().is_ident("doc"))
        .map(|attribute| format!("{}\n", text(attribute)))
        .collect()
}
/// Copy authored incoming-hook docs from the adapted production trait; no runtime I/O.
/// Only documentation attributes are retained, never signatures or implementation code.
/// This keeps reviewed business contracts/examples in seed declarations without
/// regenerating production models or overwriting custom hook behavior.
///
/// # Arguments
/// None. The tracked `crates/lspally/src/server/model.rs` is embedded at compile time.
/// # Returns
/// Method-name keys mapped to rendered documentation attributes from `ILanguageServer`.
/// An absent trait or undocumented method contributes no entry; other item kinds
/// are ignored. There are no output parameters or shared state changes.
/// # Panics
/// Panics if the embedded production source is not valid Rust syntax. Its dependency
/// on application types need not resolve: this helper parses syntax only.
/// # Algorithm
/// Parse the embedded source, select `ILanguageServer`, then collect docs from its
/// function declarations. The emission loops use matching handler names for incoming
/// methods and retain upstream descriptions as fallback for unknown seed methods.
/// # Flow
/// ```text
/// [embedded model.rs]       [upstream method names]
///          |                          |
///     [parse syntax]                  |
///          |                          |
/// [ILanguageServer function docs]     |
///          +------------+-------------+
///                       v
///                 <name match?>
///                 /           \
///               yes           no
///                |             |
///       [authored hook docs] [upstream fallback]
///                |             |
///                +------+------+
///                       v
///             [seed declaration docs]
/// ```
/// # Examples
/// ```text
/// let docs = incoming_trait_documentation();
/// assert!(docs["did_change_text_document"].contains("ordered batch"));
/// ```
fn incoming_trait_documentation() -> BTreeMap<String, String> {
    let source = include_str!("../../../../crates/lspally/src/server/model.rs");
    let syntax = syn::parse_file(source).expect("production handler trait must parse");
    syntax
        .items
        .iter()
        .filter_map(|item| match item {
            Item::Trait(item) if item.ident == "ILanguageServer" => Some(item),
            _ => None,
        })
        .flat_map(|item| item.items.iter())
        .filter_map(|item| match item {
            syn::TraitItem::Fn(method) => {
                let docs = documentation(&method.attrs);
                (!docs.is_empty()).then(|| (method.sig.ident.to_string(), docs))
            }
            _ => None,
        })
        .collect()
}
/// Recognize upstream conditional attributes mentioning proposed protocol features; no I/O.
///
/// # Arguments
/// - `attrs` `[in]`: attributes to inspect without changing them.
/// # Returns
/// `true` when a `cfg` attribute contains `proposed`; otherwise `false`.
/// # Examples
/// ```text
/// assert!(!proposed(&[]));
/// ```
fn proposed(attrs: &[syn::Attribute]) -> bool {
    attrs
        .iter()
        .any(|a| a.path().is_ident("cfg") && text(a).contains("proposed"))
}
/// Remove upstream attributes and expose struct fields in a seed syntax tree; no I/O.
/// This helper is currently unused by the emission path.
///
/// # Arguments
/// - `item` `[in]`: owned declaration to clean; the caller's syntax tree is not mutated.
/// # Returns
/// Cleaned struct, enum or alias; other item kinds are returned unchanged.
/// # Algorithm
/// Match the declaration kind. Clear declaration and nested field/variant attributes;
/// only struct fields have their visibility replaced with public visibility.
/// # Flow
/// ```text
///                <item kind?>
///           /        |        \
///       struct      enum     alias/other
///          |          |          |
/// [clear attributes] [clear] [clear alias attrs]
/// [publish fields]    |     [or keep other item]
///          |          |          |
///          +----------+----------+
///                     v
///                [owned item]
/// ```
/// # Examples
/// ```text
/// let item = clean(syn::parse_quote!(struct Flag(bool);));
/// ```
fn clean(mut item: Item) -> Item {
    match &mut item {
        Item::Struct(s) => {
            s.attrs.clear();
            for f in &mut s.fields {
                f.attrs.clear();
                f.vis = syn::parse_quote!(pub);
            }
        }
        Item::Enum(e) => {
            e.attrs.clear();
            for v in &mut e.variants {
                v.attrs.clear();
                for f in &mut v.fields {
                    f.attrs.clear();
                }
            }
        }
        Item::Type(a) => a.attrs.clear(),
        _ => {}
    }
    item
}
/// Read the last path component of a Rust type; no mutation or I/O.
///
/// # Arguments
/// - `t` `[in]`: type syntax to inspect.
/// # Returns
/// `Some(name)` for a nonempty path; `None` for other type forms or an empty path.
/// # Examples
/// ```text
/// assert_eq!(name(&syn::parse_quote!(Vec<String>)), Some("Vec".into()));
/// ```
fn name(t: &Type) -> Option<String> {
    if let Type::Path(p) = t {
        p.path.segments.last().map(|s| s.ident.to_string())
    } else {
        None
    }
}
/// Collect names mentioned by supported type shapes; no I/O.
/// Generic type arguments, tuple members and array elements are visited recursively.
///
/// # Arguments
/// - `t` `[in]`: type whose named dependencies are needed.
/// - `out` `[in,out]`: set extended with discovered names; existing entries are retained and duplicates collapse.
/// # Returns
/// `()` after extending `out`. Unsupported type forms add nothing.
/// # Algorithm
/// For paths, insert each segment name and recurse into its type arguments. Recurse into tuple
/// members and array elements; other type shapes leave the set unchanged.
/// # Flow
/// ```text
///                    [input type]
///                          |
///                       <shape>
///           +--------------+-------------+-------------+
///           v              v             v             v
///         path           tuple         array         other
///           |              |             |             |
///   [insert segments] [each member] [element type] [skip]
///           |              |             |             |
///   [type arguments]       |             |             |
///           +--------------+-------------+             |
///                          v                           |
///                  [recurse into types]                |
///                          +---------------------------+
///                          v
///                  [extended shared set]
/// ```
/// # Examples
/// ```text
/// let mut names = BTreeSet::new();
/// references(&syn::parse_quote!(Vec<String>), &mut names);
/// assert!(names.contains("String"));
/// ```
fn references(t: &Type, out: &mut BTreeSet<String>) {
    match t {
        Type::Path(p) => {
            for s in &p.path.segments {
                out.insert(s.ident.to_string());
                if let PathArguments::AngleBracketed(args) = &s.arguments {
                    for arg in &args.args {
                        if let GenericArgument::Type(t) = arg {
                            references(t, out);
                        }
                    }
                }
            }
        }
        Type::Tuple(t) => {
            for x in &t.elems {
                references(x, out)
            }
        }
        Type::Array(a) => references(&a.elem, out),
        _ => {}
    }
}
/// Extract payload type syntax from a declaration; no I/O or shared mutation.
///
/// # Arguments
/// - `e` `[in]`: entry whose fields or alias target are inspected.
/// # Returns
/// Cloned struct field types, enum payload types, or a single alias target; empty for other items.
/// # Examples
/// ```text
/// let entry = Entry { module: "common".into(), item: syn::parse_quote!(type Names = Vec<String>;) };
/// assert_eq!(types(&entry).len(), 1);
/// ```
fn types(e: &Entry) -> Vec<Type> {
    match &e.item {
        Item::Struct(s) => s.fields.iter().map(|f| f.ty.clone()).collect(),
        Item::Enum(e) => e
            .variants
            .iter()
            .flat_map(|v| v.fields.iter().map(|f| f.ty.clone()))
            .collect(),
        Item::Type(t) => vec![*t.ty.clone()],
        _ => vec![],
    }
}
/// Spell an upstream type using business payload types; no I/O.
/// JSON values become `Data`; maps, owned strings and nested containers are mapped recursively.
///
/// # Arguments
/// - `t` `[in]`: upstream type syntax, normally a path, tuple or array.
/// # Returns
/// Rust type text for seed declarations; unsupported syntax keeps its token spelling.
/// # Panics
/// Panics on a malformed path with no segments; parsed upstream Rust paths are nonempty.
/// # Algorithm
/// Map Value, Map and Cow paths to their fixed business spellings. For other paths, map type
/// arguments recursively; map tuple members and array elements likewise. Keep other syntax as text.
/// # Flow
/// ```text
///                     [input type]
///                           |
///                        <shape>
///          +----------------+----------------+---------------+
///          v                v                v               v
///        path             tuple            array           other
///          |                |                |               |
///   <Value/Map/Cow?>  [map members]   [map element; keep len]  |
///     /        \            |                |               |
/// [fixed text] [last name; map type args]     |               |
///     |                 |   |                |               |
///     +-----------------+---+----------------+------[token text]
///                           v
///                  [business type spelling]
/// ```
/// # Examples
/// ```text
/// assert_eq!(domain_type(&syn::parse_quote!(Option<Value>)), "Option<Data>");
/// ```
fn domain_type(t: &Type) -> String {
    match t {
        Type::Path(p) => {
            let s = p.path.segments.last().unwrap();
            let n = s.ident.to_string();
            if n == "Value" {
                return "Data".into();
            }
            if n == "Map" {
                return "BTreeMap<String, Data>".into();
            }
            if n == "Cow" {
                return "std::borrow::Cow<'static,str>".into();
            }
            if let PathArguments::AngleBracketed(a) = &s.arguments {
                let args: Vec<_> = a
                    .args
                    .iter()
                    .filter_map(|x| {
                        if let GenericArgument::Type(t) = x {
                            Some(domain_type(t))
                        } else {
                            None
                        }
                    })
                    .collect();
                format!("{n}<{}>", args.join(","))
            } else {
                n
            }
        }
        Type::Tuple(t) => format!(
            "({})",
            t.elems
                .iter()
                .map(domain_type)
                .collect::<Vec<_>>()
                .join(",")
        ),
        Type::Array(a) => format!("[{};{}]", domain_type(&a.elem), text(&a.len)),
        _ => text(t),
    }
}
/// Build a fallible seed conversion expression, not execute a conversion or perform I/O.
/// Containers recurse into their members. Aliases resolve through the declaration table;
/// remaining named types call generated adapters. These seeds still require manual review.
///
/// # Arguments
/// - `t` `[in]`: upstream field type to convert.
/// - `x` `[in]`: source expression text, which may be moved by the emitted expression.
/// - `to_wire` `[in]`: `true` for business-to-LSP output; `false` for incoming LSP data.
/// - `entries` `[in]`: declarations used to resolve type aliases.
/// # Returns
/// Rust expression text that propagates `ServerError` with `?` where needed.
/// # Panics
/// Panics for malformed empty paths or containers lacking the expected type arguments; expects valid upstream declarations.
/// # Algorithm
/// Keep primitive expressions and emit direction-specific Cow conversions. For known containers,
/// emit wrapping, iteration or matching around recursive member conversions. Keep unit values,
/// emit JSON conversion calls, resolve aliases recursively, or emit a named adapter call.
/// # Flow
/// ```text
/// [type + source expression]
///             |
///     <primitive or Cow?> ---- yes ---> [keep / emit ownership conversion]
///             | no                                      |
///     <known container?> ---- yes ---> [recurse on members; emit wrapper]
///             | no                                      |
///       <unit or Value?> ---- yes ---> [keep unit / emit JSON conversion]
///             | no                                      |
///       <declared alias?> ---- yes ---> [recurse into alias target]
///             | no                                      |
///   [emit named adapter call]                           |
///             +-----------------------------------------+
///             v
///    [Rust expression text; nothing executed]
/// ```
/// # Examples
/// ```text
/// assert_eq!(convert(&syn::parse_quote!(u32), "v.line", true, &BTreeMap::new()), "v.line");
/// ```
fn convert(t: &Type, x: &str, to_wire: bool, entries: &BTreeMap<String, Entry>) -> String {
    let n = name(t).unwrap_or_default();
    if [
        "String", "bool", "u8", "u16", "u32", "u64", "i8", "i16", "i32", "i64", "usize", "f32",
        "f64",
    ]
    .contains(&n.as_str())
    {
        return x.into();
    }
    if n == "Cow" {
        return if to_wire {
            format!("({x}).into()")
        } else {
            format!("({x}).into_owned()")
        };
    }
    if let Type::Path(p) = t {
        let s = p.path.segments.last().unwrap();
        if let PathArguments::AngleBracketed(a) = &s.arguments {
            let args: Vec<_> = a
                .args
                .iter()
                .filter_map(|a| {
                    if let GenericArgument::Type(t) = a {
                        Some(t)
                    } else {
                        None
                    }
                })
                .collect();
            match n.as_str() {
                "Option" => {
                    return format!(
                        "({x}).map(|v| Ok({})).transpose()?",
                        convert(args[0], "v", to_wire, entries)
                    );
                }
                "Vec" => {
                    return format!(
                        "({x}).into_iter().map(|v| Ok({})).collect::<Result<Vec<_>, ServerError>>()?",
                        convert(args[0], "v", to_wire, entries)
                    );
                }
                "Box" => {
                    return format!(
                        "Box::new({})",
                        convert(args[0], &format!("*({x})"), to_wire, entries)
                    );
                }
                "HashMap" | "BTreeMap" | "Map" => {
                    let val = args.last().unwrap();
                    let key = if n == "Map" {
                        "k".into()
                    } else {
                        convert(args[0], "k", to_wire, entries)
                    };
                    return format!(
                        "({x}).into_iter().map(|(k,v)| Ok(({key},{}))).collect::<Result<_, ServerError>>()?",
                        convert(val, "v", to_wire, entries)
                    );
                }
                "TagSupport" => {
                    let dest = if to_wire {
                        "lsp_types::TagSupport"
                    } else {
                        "d::TagSupport"
                    };
                    return format!(
                        "{dest} {{value_set: ({x}).value_set.into_iter().map(|v| Ok({})).collect::<Result<_,ServerError>>()?}}",
                        convert(args[0], "v", to_wire, entries)
                    );
                }
                "OneOf" => {
                    let src = if to_wire {
                        "d::OneOf"
                    } else {
                        "lsp_types::OneOf"
                    };
                    let dest = if to_wire {
                        "lsp_types::OneOf"
                    } else {
                        "d::OneOf"
                    };
                    return format!(
                        "match {x} {{ {src}::Left(v)=>{dest}::Left({}), {src}::Right(v)=>{dest}::Right({}) }}",
                        convert(args[0], "v", to_wire, entries),
                        convert(args[1], "v", to_wire, entries)
                    );
                }
                _ => {}
            }
        }
    }
    if let Type::Tuple(t) = t {
        if t.elems.is_empty() {
            return x.into();
        }
    }
    if n == "Value" {
        return if to_wire {
            format!("data_to_wire({x})?")
        } else {
            format!("data_from_wire({x})?")
        };
    }
    if entries
        .get(&n)
        .is_some_and(|e| matches!(e.item, Item::Type(_)))
    {
        let Item::Type(a) = &entries[&n].item else {
            unreachable!()
        };
        return convert(&a.ty, x, to_wire, entries);
    }
    let function = if to_wire { "to_wire" } else { "from_wire" };
    format!("{function}_{n}({x})?")
}
/// Insert underscores before uppercase characters for generated method names; no I/O.
/// Consecutive capitals each receive an underscore, so this is not a general acronym formatter.
///
/// # Arguments
/// - `s` `[in]`: upstream request or notification marker name.
/// # Returns
/// Lowercase ASCII spelling with inserted underscores.
/// # Examples
/// ```text
/// assert_eq!(snake("ShowDocument"), "show_document");
/// ```
fn snake(s: &str) -> String {
    let mut out = String::new();
    for (i, c) in s.chars().enumerate() {
        if c.is_uppercase() && i > 0 {
            out.push('_');
        }
        out.push(c.to_ascii_lowercase());
    }
    out
}
/// Collect non-module declarations from nested inline modules; no I/O.
/// Out-of-line modules are skipped rather than opened.
///
/// # Arguments
/// - `items` `[in]`: owned parsed declarations to flatten recursively.
/// # Returns
/// Declarations in traversal order, with inline module wrappers removed.
/// # Algorithm
/// Visit items in order. Append non-module items; recurse into inline module bodies and append
/// their flattened items. Skip out-of-line modules without opening files.
/// # Flow
/// ```text
///                 [next item] <-------------------------+
///                      |                                |
///                  <module?>                            |
///               /             \                         |
///             no              yes                       |
///              |               |                        |
///       [append item]     <inline body?>                 |
///              |           /          \                 |
///              |         yes           no               |
///              |          |             |               |
///              | [recurse; append] [skip external file]  |
///              +----------+-------------+---------------+
///                      |
///                 [return ordered items when exhausted]
/// ```
/// # Examples
/// ```text
/// let items = flatten(vec![syn::parse_quote!(mod inner { struct Flag; })]);
/// assert_eq!(items.len(), 1);
/// ```
fn flatten(items: Vec<Item>) -> Vec<Item> {
    let mut out = Vec::new();
    for item in items {
        if let Item::Mod(m) = &item {
            if let Some((_, body)) = &m.content {
                out.extend(flatten(body.clone()));
            }
        } else {
            out.push(item);
        }
    }
    out
}
/// Read upstream LSP declarations and write review-only seed models, adapters and inventories.
/// Reads `.rs` files, creates output directories, overwrites seed outputs and runs `rustfmt`.
/// Never point this command at the adapted production tree.
///
/// # Arguments
/// No Rust parameters. Command-line arguments 1 and 2 are the upstream source
/// directory and the seed destination, respectively.
/// # Returns
/// `()` after writing and formatting all seed outputs; prints inventory counts to stdout.
/// # Panics
/// Panics on missing arguments, a destination containing a `target` component, failed file access, malformed upstream syntax, incomplete method markers or failed formatting.
/// # Algorithm
/// Parse stable declarations and method markers, compute the reachable type set,
/// then emit grouped models and adapters in both directions. Read embedded production
/// hook documentation for incoming trait fragments. Emit method fragments
/// and a direction inventory separately. Pass every `.rs` file in both model and conversion output
/// directories to `rustfmt`, including pre-existing files.
/// # Flow
/// ```text
/// [upstream .rs files]       [CLI destination]
///          |                        |
///    [parse and filter]      [reject target/]
///          |                        |
///          v                        |
/// [method roots + type closure]     |
///          |                        |
///          +-----------+------------+
///                      v
///           [write model/adapter seeds]
///                      |
///          +-----------+-------------+
///          v                         v
///   [method fragments]        [inventory table]
///          |                         |
///          +------------+------------+
///                       v
///              [rustfmt seed files]
/// ```
/// # Examples
/// ```sh
/// cargo run --manifest-path scripts/gen/lspally-schema/Cargo.toml --bin lspally-schema-gen -- /path/to/lsp-types/src /tmp/lsp-seeds
/// ```
fn main() {
    let incoming_docs = incoming_trait_documentation();
    let base = PathBuf::from(std::env::args().nth(1).expect("lsp-types source directory"));
    let dest = PathBuf::from(std::env::args().nth(2).expect("seed output directory"));
    assert!(
        !dest.components().any(|c| c.as_os_str() == "target"),
        "Generated sources must stay outside target/"
    );
    let mut entries = BTreeMap::<String, Entry>::new();
    let mut impls = Vec::new();
    let mut requests = Vec::new();
    let mut notifications = Vec::new();
    for path in fs::read_dir(&base)
        .unwrap()
        .map(|p| p.unwrap().path())
        .filter(|p| p.extension().is_some_and(|e| e == "rs"))
    {
        let module = path.file_stem().unwrap().to_str().unwrap().to_owned();
        let ast = syn::parse_file(&fs::read_to_string(&path).unwrap()).unwrap();
        for item in flatten(ast.items) {
            let id = match &item {
                Item::Struct(s) if !proposed(&s.attrs) => Some(s.ident.to_string()),
                Item::Enum(e) if !proposed(&e.attrs) => Some(e.ident.to_string()),
                Item::Type(a) if !proposed(&a.attrs) => Some(a.ident.to_string()),
                _ => None,
            };
            if let Some(id) = id {
                let mut entry = item.clone();
                match &mut entry {
                    Item::Struct(s) => match &mut s.fields {
                        Fields::Named(f) => {
                            f.named = f
                                .named
                                .clone()
                                .into_iter()
                                .filter(|f| !proposed(&f.attrs))
                                .collect()
                        }
                        Fields::Unnamed(f) => {
                            f.unnamed = f
                                .unnamed
                                .clone()
                                .into_iter()
                                .filter(|f| !proposed(&f.attrs))
                                .collect()
                        }
                        _ => {}
                    },
                    Item::Enum(e) => {
                        e.variants = e
                            .variants
                            .clone()
                            .into_iter()
                            .filter(|v| !proposed(&v.attrs))
                            .collect()
                    }
                    _ => {}
                }
                entries.insert(
                    id,
                    Entry {
                        module: module.clone(),
                        item: entry,
                    },
                );
            }
            if let Item::Impl(i) = item {
                if proposed(&i.attrs) {
                    continue;
                }
                if let Some((_, tr, _)) = &i.trait_ {
                    let kind = tr.segments.last().unwrap().ident.to_string();
                    if kind == "Request" || kind == "Notification" {
                        let n = name(&i.self_ty).unwrap();
                        let mut p = None;
                        let mut r = None;
                        let mut m = None;
                        for it in &i.items {
                            match it {
                                syn::ImplItem::Type(t) if t.ident == "Params" => {
                                    p = Some(t.ty.clone())
                                }
                                syn::ImplItem::Type(t) if t.ident == "Result" => {
                                    r = Some(t.ty.clone())
                                }
                                syn::ImplItem::Const(c) if c.ident == "METHOD" => {
                                    m = Some(text(&c.expr))
                                }
                                _ => {}
                            }
                        }
                        if kind == "Request" {
                            requests.push((n, p.unwrap(), r.unwrap(), m.unwrap()));
                        } else {
                            notifications.push((n, p.unwrap(), m.unwrap()));
                        }
                    }
                } else {
                    impls.push(i);
                }
            }
        }
    }
    let mut needed = BTreeSet::new();
    for (_, p, r, _) in &requests {
        references(p, &mut needed);
        references(r, &mut needed);
    }
    for (_, p, _) in &notifications {
        references(p, &mut needed);
    }
    loop {
        let old = needed.len();
        for n in needed.clone() {
            if let Some(e) = entries.get(&n) {
                for t in types(e) {
                    references(&t, &mut needed);
                }
            }
        }
        if old == needed.len() {
            break;
        }
    }
    let model = dest.join("model");
    let conversion = dest.join("wrapper/convert");
    fs::create_dir_all(&model).unwrap();
    fs::create_dir_all(&conversion).unwrap();
    let mut models = BTreeMap::<String, String>::new();
    let mut conversions = BTreeMap::<String, String>::new();
    for n in &needed {
        if ["Uri", "Value", "Map", "Cow", "OneOf"].contains(&n.as_str()) {
            continue;
        }
        let Some(e) = entries.get(n) else {
            continue;
        };
        if e.module == "request" || e.module == "notification" {
            if let Item::Type(a) = &e.item {
                models
                    .entry("common".into())
                    .or_default()
                    .push_str(&format!(
                        "{}pub type {n} = {};\n",
                        documentation(&a.attrs),
                        domain_type(&a.ty)
                    ));
            }
            continue;
        }
        let module = if e.module == "lib" {
            "common"
        } else {
            &e.module
        };
        let out = models.entry(module.into()).or_default();
        let c = conversions.entry(module.into()).or_default();
        match &e.item {
            Item::Type(a) => out.push_str(&format!(
                "{}pub type {n} = {};\n",
                documentation(&a.attrs),
                domain_type(&a.ty)
            )),
            Item::Struct(s) => {
                out.push_str(&documentation(&s.attrs));
                if n == "TagSupport" {
                    out.push_str(
                        "#[derive(Debug)]\npub struct TagSupport<T> {\n/// Tags supported by the client. Example: `support.value_set.len()`.\npub value_set:Vec<T>}\n",
                    );
                    continue;
                }
                out.push_str("#[derive(Debug)]\n");
                match &s.fields {
                    Fields::Named(f) => {
                        out.push_str(&format!("pub struct {n} {{\n"));
                        for f in &f.named {
                            out.push_str(&format!(
                                "{}pub {}: {},\n",
                                documentation(&f.attrs),
                                f.ident.as_ref().unwrap(),
                                domain_type(&f.ty)
                            ));
                        }
                        out.push_str("}\n");
                    }
                    Fields::Unnamed(f) => {
                        out.push_str(&format!(
                            "pub struct {n}({});\n",
                            f.unnamed
                                .iter()
                                .map(|f| format!(
                                    "{}pub {}",
                                    documentation(&f.attrs),
                                    domain_type(&f.ty)
                                ))
                                .collect::<Vec<_>>()
                                .join(",")
                        ));
                    }
                    Fields::Unit => out.push_str(&format!("pub struct {n};\n")),
                }
                let private = s
                    .fields
                    .iter()
                    .any(|f| !matches!(f.vis, syn::Visibility::Public(_)));
                if private {
                    let ty = &s.fields.iter().next().unwrap().ty;
                    c.push_str(&format!("pub(super) fn from_wire_{n}(v: lsp_types::{n})->Result<d::{n},ServerError>{{ let value=serde_json::to_value(v).map_err(codec_error)?; Ok(d::{n}(serde_json::from_value(value).map_err(codec_error)?)) }}\n"));
                    c.push_str(&format!("pub(super) fn to_wire_{n}(v:d::{n})->Result<lsp_types::{n},ServerError>{{ serde_json::from_value(serde_json::to_value(v.0).map_err(codec_error)?).map_err(codec_error) }}\n"));
                    for i in &impls {
                        if name(&i.self_ty).as_deref() == Some(n) {
                            for item in &i.items {
                                if let syn::ImplItem::Const(k) = item {
                                    let mut k = k.clone();
                                    k.attrs.retain(|attribute| attribute.path().is_ident("doc"));
                                    if name(ty).as_deref() == Some("Cow") {
                                        continue;
                                    }
                                    out.push_str(&format!("impl {n} {{ {} }}\n", text(k)));
                                }
                            }
                        }
                    }
                } else {
                    for direction in [false, true] {
                        let function = if direction { "to_wire" } else { "from_wire" };
                        let (src, dst) = if direction {
                            ("d", "lsp_types")
                        } else {
                            ("lsp_types", "d")
                        };
                        c.push_str(&format!("pub(super) fn {function}_{n}(v:{src}::{n})->Result<{dst}::{n},ServerError>{{ Ok({dst}::{n}"));
                        match &s.fields {
                            Fields::Named(f) => {
                                c.push('{');
                                for f in &f.named {
                                    let field = f.ident.as_ref().unwrap();
                                    c.push_str(&format!(
                                        "{field}: {},",
                                        convert(&f.ty, &format!("v.{field}"), direction, &entries)
                                    ));
                                }
                                c.push('}');
                            }
                            Fields::Unnamed(f) => {
                                c.push('(');
                                for (index, f) in f.unnamed.iter().enumerate() {
                                    c.push_str(&convert(
                                        &f.ty,
                                        &format!("v.{index}"),
                                        direction,
                                        &entries,
                                    ));
                                    c.push(',');
                                }
                                c.push(')');
                            }
                            Fields::Unit => {}
                        }
                        c.push_str(") }\n");
                    }
                }
            }
            Item::Enum(en) => {
                out.push_str(&documentation(&en.attrs));
                out.push_str(&format!("#[derive(Debug)]\npub enum {n} {{\n"));
                for v in &en.variants {
                    out.push_str(&documentation(&v.attrs));
                    out.push_str(&v.ident.to_string());
                    match &v.fields {
                        Fields::Unnamed(f) => {
                            out.push('(');
                            out.push_str(
                                &f.unnamed
                                    .iter()
                                    .map(|f| {
                                        format!("{}{}", documentation(&f.attrs), domain_type(&f.ty))
                                    })
                                    .collect::<Vec<_>>()
                                    .join(","),
                            );
                            out.push(')');
                        }
                        Fields::Named(f) => {
                            out.push('{');
                            for f in &f.named {
                                out.push_str(&format!(
                                    "{}{}:{},",
                                    documentation(&f.attrs),
                                    f.ident.as_ref().unwrap(),
                                    domain_type(&f.ty)
                                ));
                            }
                            out.push('}');
                        }
                        _ => {}
                    }
                    out.push_str(",\n");
                }
                out.push_str("}\n");
                for direction in [false, true] {
                    let function = if direction { "to_wire" } else { "from_wire" };
                    let (src, dst) = if direction {
                        ("d", "lsp_types")
                    } else {
                        ("lsp_types", "d")
                    };
                    c.push_str(&format!("pub(super) fn {function}_{n}(v:{src}::{n})->Result<{dst}::{n},ServerError>{{Ok(match v {{"));
                    for v in &en.variants {
                        let vn = &v.ident;
                        c.push_str(&format!("{src}::{n}::{vn}"));
                        let vars: Vec<_> = v
                            .fields
                            .iter()
                            .enumerate()
                            .map(|(i, f)| {
                                f.ident
                                    .as_ref()
                                    .map(ToString::to_string)
                                    .unwrap_or(format!("f{i}"))
                            })
                            .collect();
                        let (open, close) = match &v.fields {
                            Fields::Unnamed(_) => ("(", ")"),
                            Fields::Named(_) => ("{", "}"),
                            _ => ("", ""),
                        };
                        c.push_str(open);
                        c.push_str(&vars.join(","));
                        c.push_str(close);
                        c.push_str(&format!("=>{dst}::{n}::{vn}{open}"));
                        for (f, var) in v.fields.iter().zip(vars) {
                            if matches!(v.fields, Fields::Named(_)) {
                                c.push_str(&format!("{var}:"));
                            }
                            c.push_str(&convert(&f.ty, &var, direction, &entries));
                            c.push(',');
                        }
                        c.push_str(close);
                        c.push(',');
                    }
                    c.push_str("})}\n");
                }
            }
            _ => {}
        }
    }
    let mut modelmods = String::new();
    let mut convertmods = String::new();
    for (module, body) in models {
        fs::write(model.join(format!("{module}.rs")),format!("// Script-generated by scripts/gen/lspally-schema/src/main.rs.\n//! Review-only protocol seeds; adapt upstream links and examples before production use.\nuse super::*;\n{body}")).unwrap();
        modelmods.push_str(&format!("mod {module};\npub use {module}::*;\n"));
    }
    for (module, body) in conversions {
        let body = body
            .replace("pub(super) fn", "pub(in crate::server::wrapper) fn")
            .replace("Ok(", "Ok::<_,ServerError>(");
        fs::write(conversion.join(format!("{module}.rs")),format!("// Script-generated by scripts/gen/lspally-schema/src/main.rs.\n//! Review-only protocol seeds; adapt upstream links and examples before production use.\nuse super::*;\n{body}")).unwrap();
        convertmods.push_str(&format!("mod {module};\nuse {module}::*;\n"));
    }
    fs::write(model.join("modules.inc"), modelmods).unwrap();
    fs::write(conversion.join("modules.inc"), convertmods).unwrap();
    let outbound: [&str; 13] = [
        "ShowMessageRequest",
        "RegisterCapability",
        "UnregisterCapability",
        "ApplyWorkspaceEdit",
        "WorkspaceConfiguration",
        "WorkspaceFoldersRequest",
        "WorkDoneProgressCreate",
        "SemanticTokensRefresh",
        "CodeLensRefresh",
        "ShowDocument",
        "InlayHintRefreshRequest",
        "InlineValueRefreshRequest",
        "WorkspaceDiagnosticRefresh",
    ];
    let outnotif = [
        "ShowMessage",
        "LogMessage",
        "TelemetryEvent",
        "PublishDiagnostics",
        "Progress",
        "LogTrace",
    ];
    let mut traittext = String::new();
    let mut routing = String::new();
    let mut client = String::new();
    let mut inventory = String::from(
        "> Script-generated content. Generator: scripts/gen/lspally-schema/src/main.rs\n\n# Stable method inventory\n\n| Method | Direction | Kind | Seed input | Seed result | Rust handler |\n| --- | --- | --- | --- | --- | --- |\n",
    );
    for (n, p, r, m) in &requests {
        let method_docs = entries
            .get(n)
            .map(|entry| match &entry.item {
                Item::Enum(marker) => documentation(&marker.attrs),
                _ => String::new(),
            })
            .unwrap_or_default();
        let handler = snake(n);
        let domp = domain_type(p);
        let domr = domain_type(r);
        let wiremarker = format!("lsp_types::request::{n}");
        inventory.push_str(&format!(
            "| `{}` | {} | Request | `{domp}` | `{domr}` | `{handler}` |\n",
            m.trim_matches('"'),
            if outbound.contains(&n.as_str()) {
                "Server → client"
            } else {
                "Client → server"
            }
        ));
        if outbound.contains(&n.as_str()) {
            // Normalize generated expressions, not the upstream documentation text.
            client.push_str(&method_docs);
            client.push_str(&format!("pub async fn {handler}(&self,params:{domp})->ServerResult<{domr}>{{let params={};let result=self.request::<{wiremarker}>(params).await?;Ok({})}}\n",convert(p,"params",true,&entries),convert(r,"result",false,&entries)).replace("Ok(", "Ok::<_,ServerError>("));
        } else {
            traittext.push_str(incoming_docs.get(&handler).unwrap_or(&method_docs));
            traittext.push_str(&format!("fn {handler}(&self, context:Context, params:{domp})->impl Future<Output=ServerResult<{domr}>>+Send;\n"));
            routing.push_str(&format!("name if name==<{wiremarker} as lsp_types::request::Request>::METHOD=>{{let params=request.take_params::<{wiremarker}>().map_err(invalid_params)?;let params={};let result=server.{handler}(context,params).await?;let result={};Response::success::<{wiremarker}>(request.id,result).map_err(codec_error)}},\n",convert(p,"params",false,&entries),convert(r,"result",true,&entries)));
        }
    }
    fs::write(dest.join("request-trait.inc"), traittext).unwrap();
    fs::write(
        dest.join("request-routes.inc"),
        routing.replace("Ok(", "Ok::<_,ServerError>("),
    )
    .unwrap();
    fs::write(dest.join("client-requests.inc"), client).unwrap();
    traittext = String::new();
    routing = String::new();
    client = String::new();
    for (n, p, m) in &notifications {
        let method_docs = entries
            .get(n)
            .map(|entry| match &entry.item {
                Item::Enum(marker) => documentation(&marker.attrs),
                _ => String::new(),
            })
            .unwrap_or_default();
        let handler = snake(n);
        let domp = domain_type(p);
        let marker = format!("lsp_types::notification::{n}");
        inventory.push_str(&format!(
            "| `{}` | {} | Notification | `{domp}` | `()` | `{handler}` |\n",
            m.trim_matches('"'),
            if outnotif.contains(&n.as_str()) {
                "Server → client"
            } else {
                "Client → server"
            }
        ));
        if outnotif.contains(&n.as_str()) {
            // Normalize generated expressions, not the upstream documentation text.
            client.push_str(&method_docs);
            client.push_str(&format!("pub async fn {handler}(&self, params:{domp})->ServerResult<()>{{let params={};self.notify::<{marker}>(params).await}}\n",convert(p,"params",true,&entries)).replace("Ok(", "Ok::<_,ServerError>("));
        } else {
            traittext.push_str(incoming_docs.get(&handler).unwrap_or(&method_docs));
            traittext.push_str(&format!("fn {handler}(&self,context:Context,params:{domp})->impl Future<Output=ServerResult<()>>+Send;\n"));
            routing.push_str(&format!("name if name==<{marker} as lsp_types::notification::Notification>::METHOD=>{{let params=notification.take_params::<{marker}>().map_err(invalid_params)?;let params={};server.{handler}(context,params).await}},\n",convert(p,"params",false,&entries)));
        }
    }
    fs::write(dest.join("notification-trait.inc"), traittext).unwrap();
    fs::write(
        dest.join("notification-routes.inc"),
        routing.replace("Ok(", "Ok::<_,ServerError>("),
    )
    .unwrap();
    fs::write(dest.join("client-notifications.inc"), client).unwrap();
    fs::write(dest.join("inventory.md"), inventory).unwrap();
    for folder in [&dest, &model, &conversion] {
        for entry in fs::read_dir(folder).unwrap() {
            let path = entry.unwrap().path();
            if path.extension().is_some_and(|extension| extension == "inc") {
                let body = fs::read_to_string(&path).unwrap();
                fs::write(
                    &path,
                    format!(
                        "// Script-generated by scripts/gen/lspally-schema/src/main.rs.\n{body}"
                    ),
                )
                .unwrap();
            }
        }
    }
    let mut format = std::process::Command::new("rustfmt");
    format.args(["--edition", "2024"]);
    for folder in [&model, &conversion] {
        for entry in fs::read_dir(folder).unwrap() {
            let path = entry.unwrap().path();
            if path.extension().is_some_and(|extension| extension == "rs") {
                format.arg(path);
            }
        }
    }
    assert!(
        format.status().expect("rustfmt").success(),
        "Could not format generated Rust"
    );
    println!(
        "{} model names, {} requests, {} notifications",
        needed.len(),
        requests.len(),
        notifications.len()
    );
}
