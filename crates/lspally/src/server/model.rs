// Contains AI-authored additions or edits.
//! Business protocol models, handler contracts and per-connection transport state.
//! Model construction has no I/O; running a [`Wrapper`] starts a framed LSP session.
use std::{
    collections::{BTreeMap, HashMap, HashSet},
    future::Future,
    sync::{
        Arc,
        atomic::{AtomicBool, AtomicI32, AtomicU8},
    },
};
use tokio::sync::{Mutex, mpsc, oneshot};
mod client;
pub use client::*;
mod context;
mod data;
mod extension;
pub use context::*;
pub use data::*;
pub use extension::*;
mod call_hierarchy;
pub use call_hierarchy::*;
mod code_action;
pub use code_action::*;
mod code_lens;
pub use code_lens::*;
mod color;
pub use color::*;
mod document;
pub use document::*;
mod diagnostic;
pub use diagnostic::*;
mod lifecycle;
pub use lifecycle::*;
mod capabilities;
pub use capabilities::*;
mod edit;
pub use edit::*;
mod workspace;
pub use workspace::*;
mod common;
pub use common::*;
mod completion;
pub use completion::*;
mod document_diagnostic;
pub use document_diagnostic::*;
mod document_highlight;
pub use document_highlight::*;
mod document_link;
pub use document_link::*;
mod document_symbols;
pub use document_symbols::*;
mod file_operations;
pub use file_operations::*;
mod folding_range;
pub use folding_range::*;
mod formatting;
pub use formatting::*;
mod hover;
pub use hover::*;
mod inlay_hint;
pub use inlay_hint::*;
mod inline_value;
pub use inline_value::*;
mod linked_editing;
pub use linked_editing::*;
mod moniker;
pub use moniker::*;
mod notebook;
pub use notebook::*;
mod progress;
pub use progress::*;
mod references;
pub use references::*;
mod rename;
pub use rename::*;
mod selection_range;
pub use selection_range::*;
mod semantic_tokens;
pub use semantic_tokens::*;
mod signature_help;
pub use signature_help::*;
mod trace;
pub use trace::*;
mod type_hierarchy;
pub use type_hierarchy::*;
mod window;
pub use window::*;
mod workspace_diagnostic;
pub use workspace_diagnostic::*;
mod workspace_folders;
pub use workspace_folders::*;
mod workspace_symbols;
pub use workspace_symbols::*;

// Script-generated standard method declarations by scripts/gen/lspally-schema/src/main.rs; custom hooks are AI-authored.
/// Business hooks for all incoming standard LSP methods and non-reserved extensions.
/// [`Wrapper`] adapts RPC values to these models; implementations own analysis,
/// document state, configuration and authorized work. All 73 hooks require explicit
/// bodies: 51 standard requests, 20 standard notifications and two extension hooks.
/// There are no default bodies, implicit capabilities or inherent I/O in this trait.
/// Advertise supported features in [`InitializeResult::capabilities`]; unsupported
/// hooks can return [`ServerError::unsupported`].
///
/// Use a concrete implementation type with `Wrapper<S>`: methods return sendable
/// futures and the trait is not a `dyn` dispatch interface. [`Context`] supplies
/// client communication, cooperative cancellation and the coordinate unit;
/// [`ClientHandle`] provides typed server-to-client calls, not additional hooks here.
///
/// # Dispatch and effects
/// A running wrapper invokes one business hook at a time in arrival order, with
/// independent reader/writer tasks. Notifications can schedule asynchronous business
/// work, so serialization of hooks alone does not guarantee that scheduled work has
/// completed before a later query. Implementations provide their own ordering barriers.
/// The reader handles request cancellation and trace changes before their queued hooks.
/// The wrapper does not store document text, translate byte offsets, apply returned
/// edits, merge settings or execute commands on behalf of an implementation.
///
/// Query results are owned values, not output buffers. Handlers may update caches or
/// perform application I/O; only their implementation can define those side effects.
/// Returned edits/actions are proposals for the client. Explicit command execution
/// and state-change notifications are separate from discovery. The `context` and
/// `params` arguments are consumed; their contained client/cancellation handles can
/// share state with other tasks. Positions/ranges use [`Context::encoding`].
/// Direct Rust calls to these methods bypass the wrapper's lifecycle, decoding and
/// reply handling; the examples show typed calls, not a second protocol dispatcher.
///
/// # Errors and cancellation
/// A standard request's `Err(ServerError)` becomes an RPC error with the original
/// request ID. Ordinary notification errors are logged on stderr; there is no reply.
/// The explicit-exit cleanup call is different: its error becomes `Wrapper::run`'s
/// error. Invalid standard wire parameters are rejected before calling a hook and
/// never fall through to the extension hooks. Result conversion can also fail after
/// a hook has succeeded; this does not roll back its application side effects.
///
/// Request cancellation or connection closure can drop a pending handler future.
/// Check [`Cancellation::is_cancelled`] or await [`Cancellation::cancelled`] when
/// implementing cooperative subwork. Dropping a future does not undo prior mutations,
/// stop detached tasks or stop already-running blocking work. Cancellation can win
/// before the hook is polled at all; cleanup must not depend solely on a later hook.
/// Optional work/partial-result tokens request explicit progress publications through
/// [`ClientHandle::work_done_progress`] or [`ClientHandle::progress`]; the wrapper
/// does not generate progress or assemble partial results automatically.
///
/// # Lifecycle
/// Initialization caches client permissions before [`Self::initialize`]; the selected
/// output position encoding takes effect only for later contexts. Once initialization
/// produces a successful response, the reader can accept `initialized` and marks the session running
/// before [`Self::initialized`]. Successful [`Self::shutdown`] blocks normal calls and
/// retires pending outgoing requests; [`Self::exit`] runs after explicit-exit transport
/// cleanup, not on EOF. The owner must also release resources when `Wrapper::run`
/// returns after disconnection. See each hook for its exact before/after effects.
///
/// # Flow
/// ```text
/// [peer messages]                     [application implementation]
///       |                                          ^
///    [reader] -- request/notification --> [queue] ---+
///       |                                  |       |
///       |                           [await one hook]
///       |                                  |
///       |                              <outcome?>
///       |                              /       \
///       |                         request    notification
///       |                            |             |
///       |                     [result/error] [log errors only]
///       |                            |
///       |                      [writer queue] ---> [peer]
///       |                                          |
///       +-- peer responses --> [client-call waits] <+
/// ```
/// # Examples
/// A live wrapper supplies the context; `S` implements all hooks explicitly.
/// No network is contacted merely by declaring this caller.
/// ```no_run
/// use lspally::server::model::*;
/// async fn outline<S: ILanguageServer>(server: &S, context: Context) -> ServerResult<Option<DocumentSymbolResponse>> {
///     let params = DocumentSymbolParams {
///         text_document: TextDocumentIdentifier { uri: Uri("file:///tmp/CMakeLists.txt".into()) },
///         work_done_progress_params: WorkDoneProgressParams { work_done_token: None },
///         partial_result_params: PartialResultParams::default(),
///     };
///     server.document_symbol_request(context, params).await
/// }
/// ```
pub trait ILanguageServer: Send + Sync + 'static {
    /// Negotiate this server's features and session setup for the `initialize` request.
    /// The wrapper caches client permissions before calling this hook and validates the returned
    /// position encoding afterward. The implementation owns workspace/configuration setup; no feature
    /// is enabled merely by implementing a method.
    ///
    /// # Arguments
    /// - `self` `[in,out]`: implementation receiving this call; application state/cache changes are
    /// implementation-owned, not an output buffer.
    /// - `context` `[in]`: owned client handle, cancellation signal and negotiated coordinate unit for
    /// this call; see the [dispatch contract](ILanguageServer#dispatch-and-effects).
    /// - `params` `[in]`: client capabilities, initial roots/folders, options, process identity,
    /// locale, requested trace mode and optional initialization-progress token.
    /// # Returns
    /// A sendable future yielding `Ok(result)` with advertised capabilities and optional server
    /// identity; `Err(error)` if application initialization fails.
    /// No direct output parameters are overwritten or appended; the returned value carries the result.
    /// # Errors
    /// Return an application initialization error when setup cannot complete. The wrapper separately
    /// rejects an unsupported/unoffered output position encoding and resets failed initialization to
    /// its pre-initialize phase.
    /// # Algorithm
    /// Inspect the client capabilities, initialize application state and choose only supported server
    /// features. An omitted output encoding means UTF-16; otherwise choose an offered UTF-8/16/32
    /// encoding. The current context predates this choice; later contexts use it.
    /// # Flow
    /// ```text
    /// [client initialize]       [application setup]
    ///           |                        |
    ///  [cache client permissions]        |
    ///           +------------+-----------+
    ///                        v
    ///                 [hook reply]
    ///                        |
    ///              <valid chosen encoding?>
    ///                 /             \
    ///               no              yes
    ///                |               |
    ///         [error/reset phase] [await initialized]
    /// ```
    /// # Examples
    /// Requires an implementation and a context supplied by a live wrapper; this may perform
    /// application or client I/O.
    /// ```no_run
    /// use lspally::server::model::*;
    /// async fn example<S: ILanguageServer>(server: &S, context: Context, mut params: InitializeParams) -> ServerResult<InitializeResult> {
    ///     params.locale = Some("en".into());
    ///     server.initialize(context, params).await
    /// }
    /// ```
    fn initialize(
        &self,
        context: Context,
        params: InitializeParams,
    ) -> impl Future<Output = ServerResult<InitializeResult>> + Send;
    /// Release application resources for the `shutdown` request without terminating the process.
    /// The wrapper enters its shutting-down phase before the hook. On success it retires outgoing
    /// response waiters and waits for `exit`; it does not itself stop implementation-owned background
    /// work.
    ///
    /// # Arguments
    /// - `self` `[in,out]`: implementation receiving this call; application state/cache changes are
    /// implementation-owned, not an output buffer.
    /// - `context` `[in]`: owned client handle, cancellation signal and negotiated coordinate unit for
    /// this call; see the [dispatch contract](ILanguageServer#dispatch-and-effects).
    /// - `params` `[in]`: unit payload; shutdown has no method-specific options.
    /// # Returns
    /// A sendable future yielding `Ok(())` when shutdown processing completes; `Err(error)` if
    /// application cleanup fails.
    /// No direct output parameters are overwritten or appended; the returned value carries the result.
    /// # Errors
    /// Return cleanup failures as `Err(error)`. The wrapper restores its running phase after a failed
    /// shutdown; it cannot restore resources already released by the implementation.
    /// # Algorithm
    /// Stop or join application-owned work while the transport remains alive. Return successfully
    /// rather than exiting the process. Do not issue a new client request from this hook: its phase
    /// forbids such calls.
    /// # Flow
    /// ```text
    /// [shutdown admitted]      [application work]
    ///           |                       |
    ///  [block new requests]             |
    ///           +-----------+-----------+
    ///                       v
    ///                  [cleanup hook]
    ///                       |
    ///                  <success?>
    ///                  /        \
    ///                no         yes
    ///                 |          |
    ///       [restore RUNNING] [retire waits]
    ///                            |
    ///                       [await exit]
    /// ```
    /// # Examples
    /// Requires an implementation and a context supplied by a live wrapper; this may perform
    /// application or client I/O.
    /// ```no_run
    /// use lspally::server::model::*;
    /// async fn example<S: ILanguageServer>(server: &S, context: Context) -> ServerResult<()> {
    ///     server.shutdown(context, ()).await
    /// }
    /// ```
    fn shutdown(
        &self,
        context: Context,
        params: (),
    ) -> impl Future<Output = ServerResult<()>> + Send;
    /// Suggest completion candidates at the requested document position.
    /// Returns candidates, not an applied text edit or executed completion command. Preserve
    /// lazy-resolution data for a later [`Self::resolve_completion_item`] call.
    ///
    /// # Arguments
    /// - `self` `[in,out]`: implementation receiving this call; application state/cache changes are
    /// implementation-owned, not an output buffer.
    /// - `context` `[in]`: owned client handle, cancellation signal and negotiated coordinate unit for
    /// this call; see the [dispatch contract](ILanguageServer#dispatch-and-effects).
    /// - `params` `[in]`: document/cursor, optional trigger information and optional
    /// work/partial-result tokens.
    /// # Returns
    /// A sendable future yielding `Ok(Some(response))` with an item array or a list carrying its
    /// incomplete flag; `Ok(None)` when no completion result is available; an empty collection is a
    /// valid result.
    /// `Err(error)` reports unsupported functionality or an implementation failure; see Errors.
    /// No direct output parameters are overwritten or appended; the returned value carries the result.
    /// # Errors
    /// An implementation may return `ServerError::unsupported()` for an unsupported feature, an
    /// invalid-input error, or an analysis/I/O failure. [Cancellation and partial side
    /// effects](ILanguageServer#errors-and-cancellation) follow the shared contract.
    /// # Examples
    /// Requires an implementation and a context supplied by a live wrapper; this may perform
    /// application or client I/O.
    /// ```no_run
    /// use lspally::server::model::*;
    /// async fn example<S: ILanguageServer>(server: &S, context: Context, at: TextDocumentPositionParams) -> ServerResult<Option<CompletionResponse>> {
    ///     let params = CompletionParams { text_document_position: at, context: None, work_done_progress_params: WorkDoneProgressParams { work_done_token: None }, partial_result_params: PartialResultParams::default() };
    ///     server.completion(context, params).await
    /// }
    /// ```
    fn completion(
        &self,
        context: Context,
        params: CompletionParams,
    ) -> impl Future<Output = ServerResult<Option<CompletionResponse>>> + Send;
    /// Fill lazily supplied details for an item previously returned by [`Self::completion`].
    /// Returns the enriched item; the client still decides whether to insert its text or run its
    /// command. Respect the client's advertised resolvable properties and retain stable
    /// insertion/filtering identity.
    ///
    /// # Arguments
    /// - `self` `[in,out]`: implementation receiving this call; application state/cache changes are
    /// implementation-owned, not an output buffer.
    /// - `context` `[in]`: owned client handle, cancellation signal and negotiated coordinate unit for
    /// this call; see the [dispatch contract](ILanguageServer#dispatch-and-effects).
    /// - `params` `[in]`: the selected completion item, including opaque data previously supplied by
    /// the implementation.
    /// # Returns
    /// A sendable future yielding `Ok(item)` with resolved documentation/details or other negotiated
    /// lazy properties; `Err(error)` if the item cannot be resolved.
    /// No direct output parameters are overwritten or appended; the returned value carries the result.
    /// # Errors
    /// An implementation may return `ServerError::unsupported()` for an unsupported feature, an
    /// invalid-input error, or an analysis/I/O failure. [Cancellation and partial side
    /// effects](ILanguageServer#errors-and-cancellation) follow the shared contract.
    /// # Examples
    /// Requires an implementation and a context supplied by a live wrapper; this may perform
    /// application or client I/O.
    /// ```no_run
    /// use lspally::server::model::*;
    /// async fn example<S: ILanguageServer>(server: &S, context: Context, item: CompletionItem) -> ServerResult<CompletionItem> {
    ///     server.resolve_completion_item(context, item).await
    /// }
    /// ```
    fn resolve_completion_item(
        &self,
        context: Context,
        params: CompletionItem,
    ) -> impl Future<Output = ServerResult<CompletionItem>> + Send;
    /// Describe the symbol or text under the requested cursor for a hover popup.
    /// Returns display contents and an optional highlighted source range; does not navigate or change
    /// the document.
    ///
    /// # Arguments
    /// - `self` `[in,out]`: implementation receiving this call; application state/cache changes are
    /// implementation-owned, not an output buffer.
    /// - `context` `[in]`: owned client handle, cancellation signal and negotiated coordinate unit for
    /// this call; see the [dispatch contract](ILanguageServer#dispatch-and-effects).
    /// - `params` `[in]`: document/cursor and optional work-done token; format choices come from
    /// initialization capabilities.
    /// # Returns
    /// A sendable future yielding `Ok(Some(hover))` when hover information exists; `Ok(None)` when
    /// nothing should be shown.
    /// `Err(error)` reports unsupported functionality or an implementation failure; see Errors.
    /// No direct output parameters are overwritten or appended; the returned value carries the result.
    /// # Errors
    /// An implementation may return `ServerError::unsupported()` for an unsupported feature, an
    /// invalid-input error, or an analysis/I/O failure. [Cancellation and partial side
    /// effects](ILanguageServer#errors-and-cancellation) follow the shared contract.
    /// # Examples
    /// Requires an implementation and a context supplied by a live wrapper; this may perform
    /// application or client I/O.
    /// ```no_run
    /// use lspally::server::model::*;
    /// async fn example<S: ILanguageServer>(server: &S, context: Context, at: TextDocumentPositionParams) -> ServerResult<Option<Hover>> {
    ///     let params = HoverParams { text_document_position_params: at, work_done_progress_params: WorkDoneProgressParams { work_done_token: None } };
    ///     server.hover_request(context, params).await
    /// }
    /// ```
    fn hover_request(
        &self,
        context: Context,
        params: HoverParams,
    ) -> impl Future<Output = ServerResult<Option<Hover>>> + Send;
    /// Describe callable signatures and the active argument at the requested cursor.
    /// Returns signature-help state for the client UI; does not insert arguments or execute the
    /// callable.
    ///
    /// # Arguments
    /// - `self` `[in,out]`: implementation receiving this call; application state/cache changes are
    /// implementation-owned, not an output buffer.
    /// - `context` `[in]`: owned client handle, cancellation signal and negotiated coordinate unit for
    /// this call; see the [dispatch contract](ILanguageServer#dispatch-and-effects).
    /// - `params` `[in]`: document/cursor, optional trigger/retrigger context and optional work-done
    /// token.
    /// # Returns
    /// A sendable future yielding `Ok(Some(help))` with signatures and optional active
    /// signature/parameter indices; `Ok(None)` when signature help is unavailable.
    /// `Err(error)` reports unsupported functionality or an implementation failure; see Errors.
    /// No direct output parameters are overwritten or appended; the returned value carries the result.
    /// # Errors
    /// An implementation may return `ServerError::unsupported()` for an unsupported feature, an
    /// invalid-input error, or an analysis/I/O failure. [Cancellation and partial side
    /// effects](ILanguageServer#errors-and-cancellation) follow the shared contract.
    /// # Examples
    /// Requires an implementation and a context supplied by a live wrapper; this may perform
    /// application or client I/O.
    /// ```no_run
    /// use lspally::server::model::*;
    /// async fn example<S: ILanguageServer>(server: &S, context: Context, at: TextDocumentPositionParams) -> ServerResult<Option<SignatureHelp>> {
    ///     let params = SignatureHelpParams { text_document_position_params: at, context: None, work_done_progress_params: WorkDoneProgressParams { work_done_token: None } };
    ///     server.signature_help_request(context, params).await
    /// }
    /// ```
    fn signature_help_request(
        &self,
        context: Context,
        params: SignatureHelpParams,
    ) -> impl Future<Output = ServerResult<Option<SignatureHelp>>> + Send;
    /// Find declaration sites of the symbol at a document cursor.
    /// Returns target locations or location links; navigation is performed by the client, not by the
    /// wrapper.
    ///
    /// # Arguments
    /// - `self` `[in,out]`: implementation receiving this call; application state/cache changes are
    /// implementation-owned, not an output buffer.
    /// - `context` `[in]`: owned client handle, cancellation signal and negotiated coordinate unit for
    /// this call; see the [dispatch contract](ILanguageServer#dispatch-and-effects).
    /// - `params` `[in]`: source document/cursor and optional work-done and partial-result tokens.
    /// # Returns
    /// A sendable future yielding `Ok(Some(targets))` with a single location, location array or
    /// location-link array; `Ok(None)` when no navigation result exists. An empty array is also a
    /// valid no-target result.
    /// `Err(error)` reports unsupported functionality or an implementation failure; see Errors.
    /// No direct output parameters are overwritten or appended; the returned value carries the result.
    /// # Errors
    /// An implementation may return `ServerError::unsupported()` for an unsupported feature, an
    /// invalid-input error, or an analysis/I/O failure. [Cancellation and partial side
    /// effects](ILanguageServer#errors-and-cancellation) follow the shared contract.
    /// # Examples
    /// Requires an implementation and a context supplied by a live wrapper; this may perform
    /// application or client I/O.
    /// ```no_run
    /// use lspally::server::model::*;
    /// async fn example<S: ILanguageServer>(server: &S, context: Context, at: TextDocumentPositionParams) -> ServerResult<Option<GotoDeclarationResponse>> {
    ///     let params = GotoDeclarationParams { text_document_position_params: at, work_done_progress_params: WorkDoneProgressParams { work_done_token: None }, partial_result_params: PartialResultParams::default() };
    ///     server.goto_declaration(context, params).await
    /// }
    /// ```
    fn goto_declaration(
        &self,
        context: Context,
        params: GotoDeclarationParams,
    ) -> impl Future<Output = ServerResult<Option<GotoDeclarationResponse>>> + Send;
    /// Find definition sites of the symbol at a document cursor.
    /// Returns target locations or location links; navigation is performed by the client, not by the
    /// wrapper.
    ///
    /// # Arguments
    /// - `self` `[in,out]`: implementation receiving this call; application state/cache changes are
    /// implementation-owned, not an output buffer.
    /// - `context` `[in]`: owned client handle, cancellation signal and negotiated coordinate unit for
    /// this call; see the [dispatch contract](ILanguageServer#dispatch-and-effects).
    /// - `params` `[in]`: source document/cursor and optional work-done and partial-result tokens.
    /// # Returns
    /// A sendable future yielding `Ok(Some(targets))` with a single location, location array or
    /// location-link array; `Ok(None)` when no navigation result exists. An empty array is also a
    /// valid no-target result.
    /// `Err(error)` reports unsupported functionality or an implementation failure; see Errors.
    /// No direct output parameters are overwritten or appended; the returned value carries the result.
    /// # Errors
    /// An implementation may return `ServerError::unsupported()` for an unsupported feature, an
    /// invalid-input error, or an analysis/I/O failure. [Cancellation and partial side
    /// effects](ILanguageServer#errors-and-cancellation) follow the shared contract.
    /// # Examples
    /// Requires an implementation and a context supplied by a live wrapper; this may perform
    /// application or client I/O.
    /// ```no_run
    /// use lspally::server::model::*;
    /// async fn example<S: ILanguageServer>(server: &S, context: Context, at: TextDocumentPositionParams) -> ServerResult<Option<GotoDefinitionResponse>> {
    ///     let params = GotoDefinitionParams { text_document_position_params: at, work_done_progress_params: WorkDoneProgressParams { work_done_token: None }, partial_result_params: PartialResultParams::default() };
    ///     server.goto_definition(context, params).await
    /// }
    /// ```
    fn goto_definition(
        &self,
        context: Context,
        params: GotoDefinitionParams,
    ) -> impl Future<Output = ServerResult<Option<GotoDefinitionResponse>>> + Send;
    /// Find references to the symbol at the requested document position.
    /// Returns source locations only; it does not rename the symbol or navigate the editor.
    ///
    /// # Arguments
    /// - `self` `[in,out]`: implementation receiving this call; application state/cache changes are
    /// implementation-owned, not an output buffer.
    /// - `context` `[in]`: owned client handle, cancellation signal and negotiated coordinate unit for
    /// this call; see the [dispatch contract](ILanguageServer#dispatch-and-effects).
    /// - `params` `[in]`: document/cursor, include-declaration policy and optional work/partial-result
    /// tokens.
    /// # Returns
    /// A sendable future yielding `Ok(Some(locations))` with matching references (possibly empty);
    /// `Ok(None)` when no reference result is available.
    /// `Err(error)` reports unsupported functionality or an implementation failure; see Errors.
    /// No direct output parameters are overwritten or appended; the returned value carries the result.
    /// # Errors
    /// An implementation may return `ServerError::unsupported()` for an unsupported feature, an
    /// invalid-input error, or an analysis/I/O failure. [Cancellation and partial side
    /// effects](ILanguageServer#errors-and-cancellation) follow the shared contract.
    /// # Examples
    /// Requires an implementation and a context supplied by a live wrapper; this may perform
    /// application or client I/O.
    /// ```no_run
    /// use lspally::server::model::*;
    /// async fn example<S: ILanguageServer>(server: &S, context: Context, at: TextDocumentPositionParams) -> ServerResult<Option<Vec<Location>>> {
    ///     let params = ReferenceParams { text_document_position: at, context: ReferenceContext { include_declaration: false }, work_done_progress_params: WorkDoneProgressParams { work_done_token: None }, partial_result_params: PartialResultParams::default() };
    ///     server.references(context, params).await
    /// }
    /// ```
    fn references(
        &self,
        context: Context,
        params: ReferenceParams,
    ) -> impl Future<Output = ServerResult<Option<Vec<Location>>>> + Send;
    /// Find definitions of the type of the symbol at a document cursor.
    /// Returns target locations or location links; navigation is performed by the client, not by the
    /// wrapper.
    ///
    /// # Arguments
    /// - `self` `[in,out]`: implementation receiving this call; application state/cache changes are
    /// implementation-owned, not an output buffer.
    /// - `context` `[in]`: owned client handle, cancellation signal and negotiated coordinate unit for
    /// this call; see the [dispatch contract](ILanguageServer#dispatch-and-effects).
    /// - `params` `[in]`: source document/cursor and optional work-done and partial-result tokens.
    /// # Returns
    /// A sendable future yielding `Ok(Some(targets))` with a single location, location array or
    /// location-link array; `Ok(None)` when no navigation result exists. An empty array is also a
    /// valid no-target result.
    /// `Err(error)` reports unsupported functionality or an implementation failure; see Errors.
    /// No direct output parameters are overwritten or appended; the returned value carries the result.
    /// # Errors
    /// An implementation may return `ServerError::unsupported()` for an unsupported feature, an
    /// invalid-input error, or an analysis/I/O failure. [Cancellation and partial side
    /// effects](ILanguageServer#errors-and-cancellation) follow the shared contract.
    /// # Examples
    /// Requires an implementation and a context supplied by a live wrapper; this may perform
    /// application or client I/O.
    /// ```no_run
    /// use lspally::server::model::*;
    /// async fn example<S: ILanguageServer>(server: &S, context: Context, at: TextDocumentPositionParams) -> ServerResult<Option<GotoTypeDefinitionResponse>> {
    ///     let params = GotoTypeDefinitionParams { text_document_position_params: at, work_done_progress_params: WorkDoneProgressParams { work_done_token: None }, partial_result_params: PartialResultParams::default() };
    ///     server.goto_type_definition(context, params).await
    /// }
    /// ```
    fn goto_type_definition(
        &self,
        context: Context,
        params: GotoTypeDefinitionParams,
    ) -> impl Future<Output = ServerResult<Option<GotoTypeDefinitionResponse>>> + Send;
    /// Find implementation sites for the symbol at a document cursor.
    /// Returns target locations or location links; navigation is performed by the client, not by the
    /// wrapper.
    ///
    /// # Arguments
    /// - `self` `[in,out]`: implementation receiving this call; application state/cache changes are
    /// implementation-owned, not an output buffer.
    /// - `context` `[in]`: owned client handle, cancellation signal and negotiated coordinate unit for
    /// this call; see the [dispatch contract](ILanguageServer#dispatch-and-effects).
    /// - `params` `[in]`: source document/cursor and optional work-done and partial-result tokens.
    /// # Returns
    /// A sendable future yielding `Ok(Some(targets))` with a single location, location array or
    /// location-link array; `Ok(None)` when no navigation result exists. An empty array is also a
    /// valid no-target result.
    /// `Err(error)` reports unsupported functionality or an implementation failure; see Errors.
    /// No direct output parameters are overwritten or appended; the returned value carries the result.
    /// # Errors
    /// An implementation may return `ServerError::unsupported()` for an unsupported feature, an
    /// invalid-input error, or an analysis/I/O failure. [Cancellation and partial side
    /// effects](ILanguageServer#errors-and-cancellation) follow the shared contract.
    /// # Examples
    /// Requires an implementation and a context supplied by a live wrapper; this may perform
    /// application or client I/O.
    /// ```no_run
    /// use lspally::server::model::*;
    /// async fn example<S: ILanguageServer>(server: &S, context: Context, at: TextDocumentPositionParams) -> ServerResult<Option<GotoImplementationResponse>> {
    ///     let params = GotoImplementationParams { text_document_position_params: at, work_done_progress_params: WorkDoneProgressParams { work_done_token: None }, partial_result_params: PartialResultParams::default() };
    ///     server.goto_implementation(context, params).await
    /// }
    /// ```
    fn goto_implementation(
        &self,
        context: Context,
        params: GotoImplementationParams,
    ) -> impl Future<Output = ServerResult<Option<GotoImplementationResponse>>> + Send;
    /// Find related read/write/text spans within the requested document.
    /// Unlike [`Self::references`], highlights carry ranges rather than cross-document locations; the
    /// client renders them without editing text.
    ///
    /// # Arguments
    /// - `self` `[in,out]`: implementation receiving this call; application state/cache changes are
    /// implementation-owned, not an output buffer.
    /// - `context` `[in]`: owned client handle, cancellation signal and negotiated coordinate unit for
    /// this call; see the [dispatch contract](ILanguageServer#dispatch-and-effects).
    /// - `params` `[in]`: document/cursor and optional work-done and partial-result tokens.
    /// # Returns
    /// A sendable future yielding `Ok(Some(highlights))` with same-document spans and optional kinds,
    /// possibly empty; `Ok(None)` when no highlight result is available.
    /// `Err(error)` reports unsupported functionality or an implementation failure; see Errors.
    /// No direct output parameters are overwritten or appended; the returned value carries the result.
    /// # Errors
    /// An implementation may return `ServerError::unsupported()` for an unsupported feature, an
    /// invalid-input error, or an analysis/I/O failure. [Cancellation and partial side
    /// effects](ILanguageServer#errors-and-cancellation) follow the shared contract.
    /// # Examples
    /// Requires an implementation and a context supplied by a live wrapper; this may perform
    /// application or client I/O.
    /// ```no_run
    /// use lspally::server::model::*;
    /// async fn example<S: ILanguageServer>(server: &S, context: Context, at: TextDocumentPositionParams) -> ServerResult<Option<Vec<DocumentHighlight>>> {
    ///     let params = DocumentHighlightParams { text_document_position_params: at, work_done_progress_params: WorkDoneProgressParams { work_done_token: None }, partial_result_params: PartialResultParams::default() };
    ///     server.document_highlight_request(context, params).await
    /// }
    /// ```
    fn document_highlight_request(
        &self,
        context: Context,
        params: DocumentHighlightParams,
    ) -> impl Future<Output = ServerResult<Option<Vec<DocumentHighlight>>>> + Send;
    /// List the named symbols declared in one document for its outline.
    /// Returns flat symbol information or a nested outline, respecting the client's
    /// hierarchical-symbol support; does not alter declarations.
    ///
    /// # Arguments
    /// - `self` `[in,out]`: implementation receiving this call; application state/cache changes are
    /// implementation-owned, not an output buffer.
    /// - `context` `[in]`: owned client handle, cancellation signal and negotiated coordinate unit for
    /// this call; see the [dispatch contract](ILanguageServer#dispatch-and-effects).
    /// - `params` `[in]`: document identity and optional work-done and partial-result tokens.
    /// # Returns
    /// A sendable future yielding `Ok(Some(symbols))` with flat or nested symbols, possibly empty;
    /// `Ok(None)` when no outline result is available.
    /// `Err(error)` reports unsupported functionality or an implementation failure; see Errors.
    /// No direct output parameters are overwritten or appended; the returned value carries the result.
    /// # Errors
    /// An implementation may return `ServerError::unsupported()` for an unsupported feature, an
    /// invalid-input error, or an analysis/I/O failure. [Cancellation and partial side
    /// effects](ILanguageServer#errors-and-cancellation) follow the shared contract.
    /// # Examples
    /// Requires an implementation and a context supplied by a live wrapper; this may perform
    /// application or client I/O.
    /// ```no_run
    /// use lspally::server::model::*;
    /// async fn example<S: ILanguageServer>(server: &S, context: Context, document: TextDocumentIdentifier) -> ServerResult<Option<DocumentSymbolResponse>> {
    ///     let params = DocumentSymbolParams { text_document: document, work_done_progress_params: WorkDoneProgressParams { work_done_token: None }, partial_result_params: PartialResultParams::default() };
    ///     server.document_symbol_request(context, params).await
    /// }
    /// ```
    fn document_symbol_request(
        &self,
        context: Context,
        params: DocumentSymbolParams,
    ) -> impl Future<Output = ServerResult<Option<DocumentSymbolResponse>>> + Send;
    /// Search workspace symbols using the client's query text.
    /// Returns legacy symbol information or workspace-symbol records; URI-only targets can be
    /// completed by [`Self::workspace_symbol_resolve`]. The wrapper does not search files itself.
    ///
    /// # Arguments
    /// - `self` `[in,out]`: implementation receiving this call; application state/cache changes are
    /// implementation-owned, not an output buffer.
    /// - `context` `[in]`: owned client handle, cancellation signal and negotiated coordinate unit for
    /// this call; see the [dispatch contract](ILanguageServer#dispatch-and-effects).
    /// - `params` `[in]`: query text and optional work-done and partial-result tokens.
    /// # Returns
    /// A sendable future yielding `Ok(Some(symbols))` with matching flat/workspace symbols, possibly
    /// empty; `Ok(None)` when no search result is available.
    /// `Err(error)` reports unsupported functionality or an implementation failure; see Errors.
    /// No direct output parameters are overwritten or appended; the returned value carries the result.
    /// # Errors
    /// An implementation may return `ServerError::unsupported()` for an unsupported feature, an
    /// invalid-input error, or an analysis/I/O failure. [Cancellation and partial side
    /// effects](ILanguageServer#errors-and-cancellation) follow the shared contract.
    /// # Examples
    /// Requires an implementation and a context supplied by a live wrapper; this may perform
    /// application or client I/O.
    /// ```no_run
    /// use lspally::server::model::*;
    /// async fn example<S: ILanguageServer>(server: &S, context: Context) -> ServerResult<Option<WorkspaceSymbolResponse>> {
    ///     let params = WorkspaceSymbolParams { query: "target".into(), work_done_progress_params: WorkDoneProgressParams { work_done_token: None }, partial_result_params: PartialResultParams::default() };
    ///     server.workspace_symbol_request(context, params).await
    /// }
    /// ```
    fn workspace_symbol_request(
        &self,
        context: Context,
        params: WorkspaceSymbolParams,
    ) -> impl Future<Output = ServerResult<Option<WorkspaceSymbolResponse>>> + Send;
    /// Resolve the full target location of a previously returned workspace symbol.
    /// Returns the completed symbol without navigating the client. Use preserved opaque data to
    /// recover identity rather than treating the display name as unique.
    ///
    /// # Arguments
    /// - `self` `[in,out]`: implementation receiving this call; application state/cache changes are
    /// implementation-owned, not an output buffer.
    /// - `context` `[in]`: owned client handle, cancellation signal and negotiated coordinate unit for
    /// this call; see the [dispatch contract](ILanguageServer#dispatch-and-effects).
    /// - `params` `[in]`: workspace symbol previously supplied by [`Self::workspace_symbol_request`],
    /// including its URI-only/full location and optional data.
    /// # Returns
    /// A sendable future yielding `Ok(symbol)` with its resolvable location filled; `Err(error)` if
    /// the symbol can no longer be resolved.
    /// No direct output parameters are overwritten or appended; the returned value carries the result.
    /// # Errors
    /// An implementation may return `ServerError::unsupported()` for an unsupported feature, an
    /// invalid-input error, or an analysis/I/O failure. [Cancellation and partial side
    /// effects](ILanguageServer#errors-and-cancellation) follow the shared contract.
    /// # Examples
    /// Requires an implementation and a context supplied by a live wrapper; this may perform
    /// application or client I/O.
    /// ```no_run
    /// use lspally::server::model::*;
    /// async fn example<S: ILanguageServer>(server: &S, context: Context, symbol: WorkspaceSymbol) -> ServerResult<WorkspaceSymbol> {
    ///     server.workspace_symbol_resolve(context, symbol).await
    /// }
    /// ```
    fn workspace_symbol_resolve(
        &self,
        context: Context,
        params: WorkspaceSymbol,
    ) -> impl Future<Output = ServerResult<WorkspaceSymbol>> + Send;
    /// Execute an implementation-defined command requested by the client.
    /// Unlike discovery hooks such as code actions, this may change application state, request client
    /// edits or run authorized work. Command validation and authorization belong to the
    /// implementation, not the wrapper.
    ///
    /// # Arguments
    /// - `self` `[in,out]`: implementation receiving this call; application state/cache changes are
    /// implementation-owned, not an output buffer.
    /// - `context` `[in]`: owned client handle, cancellation signal and negotiated coordinate unit for
    /// this call; see the [dispatch contract](ILanguageServer#dispatch-and-effects).
    /// - `params` `[in]`: command identifier, ordered arbitrary-data arguments and optional work-done
    /// token.
    /// # Returns
    /// A sendable future yielding `Ok(Some(data))` for a command result, `Ok(None)` for a null result,
    /// or `Err(error)` on rejection/failure; `Some(Data::Null)` also encodes JSON null.
    /// No direct output parameters are overwritten or appended; the returned value carries the result.
    /// # Errors
    /// Return an error for unknown commands, invalid arguments, denied execution or command failures
    /// as appropriate. The wrapper does not roll back partial command side effects on error or
    /// cancellation.
    /// # Examples
    /// Requires an implementation and a context supplied by a live wrapper; this may perform
    /// application or client I/O.
    /// ```no_run
    /// use lspally::server::model::*;
    /// async fn example<S: ILanguageServer>(server: &S, context: Context) -> ServerResult<Option<Data>> {
    ///     let params = ExecuteCommandParams { command: "example.reindex".into(), arguments: vec![], work_done_progress_params: WorkDoneProgressParams { work_done_token: None } };
    ///     server.execute_command(context, params).await
    /// }
    /// ```
    fn execute_command(
        &self,
        context: Context,
        params: ExecuteCommandParams,
    ) -> impl Future<Output = ServerResult<Option<Data>>> + Send;
    /// Compute edits for the client to apply before saving a document.
    /// The client waits subject to its own save timeout, applies returned edits and performs the save.
    /// Returning edits does not write disk or mutate the client buffer through the wrapper.
    ///
    /// # Arguments
    /// - `self` `[in,out]`: implementation receiving this call; application state/cache changes are
    /// implementation-owned, not an output buffer.
    /// - `context` `[in]`: owned client handle, cancellation signal and negotiated coordinate unit for
    /// this call; see the [dispatch contract](ILanguageServer#dispatch-and-effects).
    /// - `params` `[in]`: document identity and save reason, not the document contents; use
    /// synchronized application text.
    /// # Returns
    /// A sendable future yielding `Ok(Some(edits))` with pre-save replacements (possibly empty);
    /// `Ok(None)` when no pre-save edits are provided.
    /// `Err(error)` reports unsupported functionality or an implementation failure; see Errors.
    /// No direct output parameters are overwritten or appended; the returned value carries the result.
    /// # Errors
    /// An implementation may return `ServerError::unsupported()` for an unsupported feature, an
    /// invalid-input error, or an analysis/I/O failure. [Cancellation and partial side
    /// effects](ILanguageServer#errors-and-cancellation) follow the shared contract.
    /// # Examples
    /// Requires an implementation and a context supplied by a live wrapper; this may perform
    /// application or client I/O.
    /// ```no_run
    /// use lspally::server::model::*;
    /// async fn example<S: ILanguageServer>(server: &S, context: Context, document: TextDocumentIdentifier) -> ServerResult<Option<Vec<TextEdit>>> {
    ///     let params = WillSaveTextDocumentParams { text_document: document, reason: TextDocumentSaveReason(1) };
    ///     server.will_save_wait_until(context, params).await
    /// }
    /// ```
    fn will_save_wait_until(
        &self,
        context: Context,
        params: WillSaveTextDocumentParams,
    ) -> impl Future<Output = ServerResult<Option<Vec<TextEdit>>>> + Send;
    /// Propose commands or edits for the selected range and diagnostic context.
    /// Discovery does not apply edits or execute commands. Honor requested action-kind filters and
    /// trigger kind; deferred details can be supplied by [`Self::code_action_resolve_request`].
    ///
    /// # Arguments
    /// - `self` `[in,out]`: implementation receiving this call; application state/cache changes are
    /// implementation-owned, not an output buffer.
    /// - `context` `[in]`: owned client handle, cancellation signal and negotiated coordinate unit for
    /// this call; see the [dispatch contract](ILanguageServer#dispatch-and-effects).
    /// - `params` `[in]`: document/range, diagnostics, optional action-kind filter and trigger kind,
    /// plus work/partial-result tokens.
    /// # Returns
    /// A sendable future yielding `Ok(Some(actions))` with commands or code-action records (possibly
    /// empty); `Ok(None)` when no action result is available.
    /// `Err(error)` reports unsupported functionality or an implementation failure; see Errors.
    /// No direct output parameters are overwritten or appended; the returned value carries the result.
    /// # Errors
    /// An implementation may return `ServerError::unsupported()` for an unsupported feature, an
    /// invalid-input error, or an analysis/I/O failure. [Cancellation and partial side
    /// effects](ILanguageServer#errors-and-cancellation) follow the shared contract.
    /// # Examples
    /// Requires an implementation and a context supplied by a live wrapper; this may perform
    /// application or client I/O.
    /// ```no_run
    /// use lspally::server::model::*;
    /// async fn example<S: ILanguageServer>(server: &S, context: Context, document: TextDocumentIdentifier, range: Range) -> ServerResult<Option<CodeActionResponse>> {
    ///     let params = CodeActionParams { text_document: document, range, context: CodeActionContext { diagnostics: vec![], only: None, trigger_kind: None }, work_done_progress_params: WorkDoneProgressParams { work_done_token: None }, partial_result_params: PartialResultParams::default() };
    ///     server.code_action_request(context, params).await
    /// }
    /// ```
    fn code_action_request(
        &self,
        context: Context,
        params: CodeActionParams,
    ) -> impl Future<Output = ServerResult<Option<CodeActionResponse>>> + Send;
    /// Fill deferred properties of an action previously returned by [`Self::code_action_request`].
    /// Returns an enriched action without applying its edit or executing its command. Only fill
    /// properties the client negotiated for lazy resolution.
    ///
    /// # Arguments
    /// - `self` `[in,out]`: implementation receiving this call; application state/cache changes are
    /// implementation-owned, not an output buffer.
    /// - `context` `[in]`: owned client handle, cancellation signal and negotiated coordinate unit for
    /// this call; see the [dispatch contract](ILanguageServer#dispatch-and-effects).
    /// - `params` `[in]`: code action with its title, identity and implementation-supplied opaque
    /// data.
    /// # Returns
    /// A sendable future yielding `Ok(action)` with resolved properties, such as its edit;
    /// `Err(error)` when resolution fails.
    /// No direct output parameters are overwritten or appended; the returned value carries the result.
    /// # Errors
    /// An implementation may return `ServerError::unsupported()` for an unsupported feature, an
    /// invalid-input error, or an analysis/I/O failure. [Cancellation and partial side
    /// effects](ILanguageServer#errors-and-cancellation) follow the shared contract.
    /// # Examples
    /// Requires an implementation and a context supplied by a live wrapper; this may perform
    /// application or client I/O.
    /// ```no_run
    /// use lspally::server::model::*;
    /// async fn example<S: ILanguageServer>(server: &S, context: Context, action: CodeAction) -> ServerResult<CodeAction> {
    ///     server.code_action_resolve_request(context, action).await
    /// }
    /// ```
    fn code_action_resolve_request(
        &self,
        context: Context,
        params: CodeAction,
    ) -> impl Future<Output = ServerResult<CodeAction>> + Send;
    /// List command lenses associated with source ranges in one document.
    /// Returns lens placements; a lens can omit its command until [`Self::code_lens_resolve`] is
    /// called. Listing does not execute a command.
    ///
    /// # Arguments
    /// - `self` `[in,out]`: implementation receiving this call; application state/cache changes are
    /// implementation-owned, not an output buffer.
    /// - `context` `[in]`: owned client handle, cancellation signal and negotiated coordinate unit for
    /// this call; see the [dispatch contract](ILanguageServer#dispatch-and-effects).
    /// - `params` `[in]`: document identity and optional work-done and partial-result tokens.
    /// # Returns
    /// A sendable future yielding `Ok(Some(lenses))` with resolved or unresolved lenses (possibly
    /// empty); `Ok(None)` when no lens result is available.
    /// `Err(error)` reports unsupported functionality or an implementation failure; see Errors.
    /// No direct output parameters are overwritten or appended; the returned value carries the result.
    /// # Errors
    /// An implementation may return `ServerError::unsupported()` for an unsupported feature, an
    /// invalid-input error, or an analysis/I/O failure. [Cancellation and partial side
    /// effects](ILanguageServer#errors-and-cancellation) follow the shared contract.
    /// # Examples
    /// Requires an implementation and a context supplied by a live wrapper; this may perform
    /// application or client I/O.
    /// ```no_run
    /// use lspally::server::model::*;
    /// async fn example<S: ILanguageServer>(server: &S, context: Context, document: TextDocumentIdentifier) -> ServerResult<Option<Vec<CodeLens>>> {
    ///     let params = CodeLensParams { text_document: document, work_done_progress_params: WorkDoneProgressParams { work_done_token: None }, partial_result_params: PartialResultParams::default() };
    ///     server.code_lens_request(context, params).await
    /// }
    /// ```
    fn code_lens_request(
        &self,
        context: Context,
        params: CodeLensParams,
    ) -> impl Future<Output = ServerResult<Option<Vec<CodeLens>>>> + Send;
    /// Fill the command of a lens returned by [`Self::code_lens_request`].
    /// Resolution produces a command for the client to display or invoke; it does not invoke that
    /// command.
    ///
    /// # Arguments
    /// - `self` `[in,out]`: implementation receiving this call; application state/cache changes are
    /// implementation-owned, not an output buffer.
    /// - `context` `[in]`: owned client handle, cancellation signal and negotiated coordinate unit for
    /// this call; see the [dispatch contract](ILanguageServer#dispatch-and-effects).
    /// - `params` `[in]`: lens range and opaque data from an earlier lens result, with its possibly
    /// missing command.
    /// # Returns
    /// A sendable future yielding `Ok(lens)` carrying the resolved command; `Err(error)` if the lens
    /// is stale or cannot be resolved.
    /// No direct output parameters are overwritten or appended; the returned value carries the result.
    /// # Errors
    /// An implementation may return `ServerError::unsupported()` for an unsupported feature, an
    /// invalid-input error, or an analysis/I/O failure. [Cancellation and partial side
    /// effects](ILanguageServer#errors-and-cancellation) follow the shared contract.
    /// # Examples
    /// Requires an implementation and a context supplied by a live wrapper; this may perform
    /// application or client I/O.
    /// ```no_run
    /// use lspally::server::model::*;
    /// async fn example<S: ILanguageServer>(server: &S, context: Context, lens: CodeLens) -> ServerResult<CodeLens> {
    ///     server.code_lens_resolve(context, lens).await
    /// }
    /// ```
    fn code_lens_resolve(
        &self,
        context: Context,
        params: CodeLens,
    ) -> impl Future<Output = ServerResult<CodeLens>> + Send;
    /// Find clickable source spans and their resource targets in a document.
    /// Returns links; a missing target can be resolved by [`Self::document_link_resolve`]. Discovery
    /// does not open the target resource.
    ///
    /// # Arguments
    /// - `self` `[in,out]`: implementation receiving this call; application state/cache changes are
    /// implementation-owned, not an output buffer.
    /// - `context` `[in]`: owned client handle, cancellation signal and negotiated coordinate unit for
    /// this call; see the [dispatch contract](ILanguageServer#dispatch-and-effects).
    /// - `params` `[in]`: document identity and optional work-done and partial-result tokens.
    /// # Returns
    /// A sendable future yielding `Ok(Some(links))` with link ranges and optional targets (possibly
    /// empty); `Ok(None)` when no link result is available.
    /// `Err(error)` reports unsupported functionality or an implementation failure; see Errors.
    /// No direct output parameters are overwritten or appended; the returned value carries the result.
    /// # Errors
    /// An implementation may return `ServerError::unsupported()` for an unsupported feature, an
    /// invalid-input error, or an analysis/I/O failure. [Cancellation and partial side
    /// effects](ILanguageServer#errors-and-cancellation) follow the shared contract.
    /// # Examples
    /// Requires an implementation and a context supplied by a live wrapper; this may perform
    /// application or client I/O.
    /// ```no_run
    /// use lspally::server::model::*;
    /// async fn example<S: ILanguageServer>(server: &S, context: Context, document: TextDocumentIdentifier) -> ServerResult<Option<Vec<DocumentLink>>> {
    ///     let params = DocumentLinkParams { text_document: document, work_done_progress_params: WorkDoneProgressParams { work_done_token: None }, partial_result_params: PartialResultParams::default() };
    ///     server.document_link_request(context, params).await
    /// }
    /// ```
    fn document_link_request(
        &self,
        context: Context,
        params: DocumentLinkParams,
    ) -> impl Future<Output = ServerResult<Option<Vec<DocumentLink>>>> + Send;
    /// Resolve a deferred target for a previously returned document link.
    /// Returns the link with its target; resource opening remains a client action. Preserve the
    /// original source range and opaque identity.
    ///
    /// # Arguments
    /// - `self` `[in,out]`: implementation receiving this call; application state/cache changes are
    /// implementation-owned, not an output buffer.
    /// - `context` `[in]`: owned client handle, cancellation signal and negotiated coordinate unit for
    /// this call; see the [dispatch contract](ILanguageServer#dispatch-and-effects).
    /// - `params` `[in]`: link from [`Self::document_link_request`], including its range, optional
    /// target and data.
    /// # Returns
    /// A sendable future yielding `Ok(link)` with the target resolved; `Err(error)` when the target
    /// cannot be determined.
    /// No direct output parameters are overwritten or appended; the returned value carries the result.
    /// # Errors
    /// An implementation may return `ServerError::unsupported()` for an unsupported feature, an
    /// invalid-input error, or an analysis/I/O failure. [Cancellation and partial side
    /// effects](ILanguageServer#errors-and-cancellation) follow the shared contract.
    /// # Examples
    /// Requires an implementation and a context supplied by a live wrapper; this may perform
    /// application or client I/O.
    /// ```no_run
    /// use lspally::server::model::*;
    /// async fn example<S: ILanguageServer>(server: &S, context: Context, link: DocumentLink) -> ServerResult<DocumentLink> {
    ///     server.document_link_resolve(context, link).await
    /// }
    /// ```
    fn document_link_resolve(
        &self,
        context: Context,
        params: DocumentLink,
    ) -> impl Future<Output = ServerResult<DocumentLink>> + Send;
    /// Compute replacements that format an entire synchronized document.
    /// The client applies the returned edits. The wrapper neither applies them to server text nor
    /// writes the file.
    ///
    /// # Arguments
    /// - `self` `[in,out]`: implementation receiving this call; application state/cache changes are
    /// implementation-owned, not an output buffer.
    /// - `context` `[in]`: owned client handle, cancellation signal and negotiated coordinate unit for
    /// this call; see the [dispatch contract](ILanguageServer#dispatch-and-effects).
    /// - `params` `[in]`: document identity, tab/space and optional formatting policies, and optional
    /// work-done token.
    /// # Returns
    /// A sendable future yielding `Ok(Some(edits))` with formatting replacements (possibly empty);
    /// `Ok(None)` when no formatting result is provided.
    /// `Err(error)` reports unsupported functionality or an implementation failure; see Errors.
    /// No direct output parameters are overwritten or appended; the returned value carries the result.
    /// # Errors
    /// An implementation may return `ServerError::unsupported()` for an unsupported feature, an
    /// invalid-input error, or an analysis/I/O failure. [Cancellation and partial side
    /// effects](ILanguageServer#errors-and-cancellation) follow the shared contract.
    /// # Examples
    /// Requires an implementation and a context supplied by a live wrapper; this may perform
    /// application or client I/O.
    /// ```no_run
    /// use lspally::server::model::*;
    /// async fn example<S: ILanguageServer>(server: &S, context: Context, document: TextDocumentIdentifier, options: FormattingOptions) -> ServerResult<Option<Vec<TextEdit>>> {
    ///     let params = DocumentFormattingParams { text_document: document, options, work_done_progress_params: WorkDoneProgressParams { work_done_token: None } };
    ///     server.formatting(context, params).await
    /// }
    /// ```
    fn formatting(
        &self,
        context: Context,
        params: DocumentFormattingParams,
    ) -> impl Future<Output = ServerResult<Option<Vec<TextEdit>>>> + Send;
    /// Compute formatting replacements for the requested source range.
    /// Returns client-applied edits rather than editing text or disk; ranges use the negotiated
    /// coordinate unit.
    ///
    /// # Arguments
    /// - `self` `[in,out]`: implementation receiving this call; application state/cache changes are
    /// implementation-owned, not an output buffer.
    /// - `context` `[in]`: owned client handle, cancellation signal and negotiated coordinate unit for
    /// this call; see the [dispatch contract](ILanguageServer#dispatch-and-effects).
    /// - `params` `[in]`: document, selected span, formatting policies and optional work-done token.
    /// # Returns
    /// A sendable future yielding `Ok(Some(edits))` with range-formatting replacements (possibly
    /// empty); `Ok(None)` when no formatting result is provided.
    /// `Err(error)` reports unsupported functionality or an implementation failure; see Errors.
    /// No direct output parameters are overwritten or appended; the returned value carries the result.
    /// # Errors
    /// An implementation may return `ServerError::unsupported()` for an unsupported feature, an
    /// invalid-input error, or an analysis/I/O failure. [Cancellation and partial side
    /// effects](ILanguageServer#errors-and-cancellation) follow the shared contract.
    /// # Examples
    /// Requires an implementation and a context supplied by a live wrapper; this may perform
    /// application or client I/O.
    /// ```no_run
    /// use lspally::server::model::*;
    /// async fn example<S: ILanguageServer>(server: &S, context: Context, document: TextDocumentIdentifier, range: Range, options: FormattingOptions) -> ServerResult<Option<Vec<TextEdit>>> {
    ///     let params = DocumentRangeFormattingParams { text_document: document, range, options, work_done_progress_params: WorkDoneProgressParams { work_done_token: None } };
    ///     server.range_formatting(context, params).await
    /// }
    /// ```
    fn range_formatting(
        &self,
        context: Context,
        params: DocumentRangeFormattingParams,
    ) -> impl Future<Output = ServerResult<Option<Vec<TextEdit>>>> + Send;
    /// Compute local formatting after the client types a trigger character.
    /// Returns edits for client application; it does not insert the trigger character or save the
    /// document.
    ///
    /// # Arguments
    /// - `self` `[in,out]`: implementation receiving this call; application state/cache changes are
    /// implementation-owned, not an output buffer.
    /// - `context` `[in]`: owned client handle, cancellation signal and negotiated coordinate unit for
    /// this call; see the [dispatch contract](ILanguageServer#dispatch-and-effects).
    /// - `params` `[in]`: document/cursor after typing, the typed character and formatting policies.
    /// # Returns
    /// A sendable future yielding `Ok(Some(edits))` with on-type replacements (possibly empty);
    /// `Ok(None)` when no adjustment is provided.
    /// `Err(error)` reports unsupported functionality or an implementation failure; see Errors.
    /// No direct output parameters are overwritten or appended; the returned value carries the result.
    /// # Errors
    /// An implementation may return `ServerError::unsupported()` for an unsupported feature, an
    /// invalid-input error, or an analysis/I/O failure. [Cancellation and partial side
    /// effects](ILanguageServer#errors-and-cancellation) follow the shared contract.
    /// # Examples
    /// Requires an implementation and a context supplied by a live wrapper; this may perform
    /// application or client I/O.
    /// ```no_run
    /// use lspally::server::model::*;
    /// async fn example<S: ILanguageServer>(server: &S, context: Context, at: TextDocumentPositionParams, options: FormattingOptions) -> ServerResult<Option<Vec<TextEdit>>> {
    ///     let params = DocumentOnTypeFormattingParams { text_document_position: at, ch: "\n".into(), options };
    ///     server.on_type_formatting(context, params).await
    /// }
    /// ```
    fn on_type_formatting(
        &self,
        context: Context,
        params: DocumentOnTypeFormattingParams,
    ) -> impl Future<Output = ServerResult<Option<Vec<TextEdit>>>> + Send;
    /// Find matching ranges the client should edit together as text changes.
    /// Returns linked ranges and an optional word pattern; it does not apply edits. All ranges belong
    /// to the requested document.
    ///
    /// # Arguments
    /// - `self` `[in,out]`: implementation receiving this call; application state/cache changes are
    /// implementation-owned, not an output buffer.
    /// - `context` `[in]`: owned client handle, cancellation signal and negotiated coordinate unit for
    /// this call; see the [dispatch contract](ILanguageServer#dispatch-and-effects).
    /// - `params` `[in]`: document/cursor and optional work-done token.
    /// # Returns
    /// A sendable future yielding `Ok(Some(ranges))` with linked spans and optional matching pattern;
    /// `Ok(None)` when no linked-editing group exists.
    /// `Err(error)` reports unsupported functionality or an implementation failure; see Errors.
    /// No direct output parameters are overwritten or appended; the returned value carries the result.
    /// # Errors
    /// An implementation may return `ServerError::unsupported()` for an unsupported feature, an
    /// invalid-input error, or an analysis/I/O failure. [Cancellation and partial side
    /// effects](ILanguageServer#errors-and-cancellation) follow the shared contract.
    /// # Examples
    /// Requires an implementation and a context supplied by a live wrapper; this may perform
    /// application or client I/O.
    /// ```no_run
    /// use lspally::server::model::*;
    /// async fn example<S: ILanguageServer>(server: &S, context: Context, at: TextDocumentPositionParams) -> ServerResult<Option<LinkedEditingRanges>> {
    ///     let params = LinkedEditingRangeParams { text_document_position_params: at, work_done_progress_params: WorkDoneProgressParams { work_done_token: None } };
    ///     server.linked_editing_range(context, params).await
    /// }
    /// ```
    fn linked_editing_range(
        &self,
        context: Context,
        params: LinkedEditingRangeParams,
    ) -> impl Future<Output = ServerResult<Option<LinkedEditingRanges>>> + Send;
    /// Compute a workspace edit renaming the symbol at the requested cursor.
    /// Returns a proposed edit for the client to apply, not a server-side rewrite. The implementation
    /// validates the new name and discovers affected references.
    ///
    /// # Arguments
    /// - `self` `[in,out]`: implementation receiving this call; application state/cache changes are
    /// implementation-owned, not an output buffer.
    /// - `context` `[in]`: owned client handle, cancellation signal and negotiated coordinate unit for
    /// this call; see the [dispatch contract](ILanguageServer#dispatch-and-effects).
    /// - `params` `[in]`: document/cursor, replacement name and optional work-done token.
    /// # Returns
    /// A sendable future yielding `Ok(Some(edit))` with coordinated text/resource changes; `Ok(None)`
    /// when no rename edit is available.
    /// `Err(error)` reports unsupported functionality or an implementation failure; see Errors.
    /// No direct output parameters are overwritten or appended; the returned value carries the result.
    /// # Errors
    /// Return an implementation-defined error for an invalid new name, unsafe rename or failed
    /// analysis. Returning a workspace edit does not apply it; an error cannot undo unrelated
    /// application side effects.
    /// # Examples
    /// Requires an implementation and a context supplied by a live wrapper; this may perform
    /// application or client I/O.
    /// ```no_run
    /// use lspally::server::model::*;
    /// async fn example<S: ILanguageServer>(server: &S, context: Context, at: TextDocumentPositionParams) -> ServerResult<Option<WorkspaceEdit>> {
    ///     let params = RenameParams { text_document_position: at, new_name: "new_target".into(), work_done_progress_params: WorkDoneProgressParams { work_done_token: None } };
    ///     server.rename(context, params).await
    /// }
    /// ```
    fn rename(
        &self,
        context: Context,
        params: RenameParams,
    ) -> impl Future<Output = ServerResult<Option<WorkspaceEdit>>> + Send;
    /// Find color literals and their source ranges in a document.
    /// Returns RGBA values for client color decorations; does not rewrite literals or open a color
    /// picker itself.
    ///
    /// # Arguments
    /// - `self` `[in,out]`: implementation receiving this call; application state/cache changes are
    /// implementation-owned, not an output buffer.
    /// - `context` `[in]`: owned client handle, cancellation signal and negotiated coordinate unit for
    /// this call; see the [dispatch contract](ILanguageServer#dispatch-and-effects).
    /// - `params` `[in]`: document identity and optional work-done and partial-result tokens.
    /// # Returns
    /// A sendable future yielding `Ok(colors)` with color/range pairs; an empty vector means none were
    /// found. `Err(error)` reports an implementation failure.
    /// No direct output parameters are overwritten or appended; the returned value carries the result.
    /// # Errors
    /// An implementation may return `ServerError::unsupported()` for an unsupported feature, an
    /// invalid-input error, or an analysis/I/O failure. [Cancellation and partial side
    /// effects](ILanguageServer#errors-and-cancellation) follow the shared contract.
    /// # Examples
    /// Requires an implementation and a context supplied by a live wrapper; this may perform
    /// application or client I/O.
    /// ```no_run
    /// use lspally::server::model::*;
    /// async fn example<S: ILanguageServer>(server: &S, context: Context, document: TextDocumentIdentifier) -> ServerResult<Vec<ColorInformation>> {
    ///     let params = DocumentColorParams { text_document: document, work_done_progress_params: WorkDoneProgressParams { work_done_token: None }, partial_result_params: PartialResultParams::default() };
    ///     server.document_color(context, params).await
    /// }
    /// ```
    fn document_color(
        &self,
        context: Context,
        params: DocumentColorParams,
    ) -> impl Future<Output = ServerResult<Vec<ColorInformation>>> + Send;
    /// Propose textual spellings for a chosen color at a source range.
    /// Returns labels and optional replacement/additional edits; only the client applies a selected
    /// presentation.
    ///
    /// # Arguments
    /// - `self` `[in,out]`: implementation receiving this call; application state/cache changes are
    /// implementation-owned, not an output buffer.
    /// - `context` `[in]`: owned client handle, cancellation signal and negotiated coordinate unit for
    /// this call; see the [dispatch contract](ILanguageServer#dispatch-and-effects).
    /// - `params` `[in]`: document, RGBA color, original color span and optional work/partial-result
    /// tokens.
    /// # Returns
    /// A sendable future yielding `Ok(presentations)` with candidate spellings and edits; an empty
    /// vector means none are offered. `Err(error)` reports an implementation failure.
    /// No direct output parameters are overwritten or appended; the returned value carries the result.
    /// # Errors
    /// An implementation may return `ServerError::unsupported()` for an unsupported feature, an
    /// invalid-input error, or an analysis/I/O failure. [Cancellation and partial side
    /// effects](ILanguageServer#errors-and-cancellation) follow the shared contract.
    /// # Examples
    /// Requires an implementation and a context supplied by a live wrapper; this may perform
    /// application or client I/O.
    /// ```no_run
    /// use lspally::server::model::*;
    /// async fn example<S: ILanguageServer>(server: &S, context: Context, document: TextDocumentIdentifier, range: Range) -> ServerResult<Vec<ColorPresentation>> {
    ///     let params = ColorPresentationParams { text_document: document, color: Color { red: 1.0, green: 0.0, blue: 0.0, alpha: 1.0 }, range, work_done_progress_params: WorkDoneProgressParams { work_done_token: None }, partial_result_params: PartialResultParams::default() };
    ///     server.color_presentation_request(context, params).await
    /// }
    /// ```
    fn color_presentation_request(
        &self,
        context: Context,
        params: ColorPresentationParams,
    ) -> impl Future<Output = ServerResult<Vec<ColorPresentation>>> + Send;
    /// Find collapsible source regions for the client's folding UI.
    /// Returns fold boundaries and optional kinds/collapsed text; it does not hide or remove source
    /// text itself. Respect the client's line-only and range-limit capabilities.
    ///
    /// # Arguments
    /// - `self` `[in,out]`: implementation receiving this call; application state/cache changes are
    /// implementation-owned, not an output buffer.
    /// - `context` `[in]`: owned client handle, cancellation signal and negotiated coordinate unit for
    /// this call; see the [dispatch contract](ILanguageServer#dispatch-and-effects).
    /// - `params` `[in]`: document identity and optional work-done and partial-result tokens.
    /// # Returns
    /// A sendable future yielding `Ok(Some(ranges))` with fold regions (possibly empty); `Ok(None)`
    /// when no folding result is available.
    /// `Err(error)` reports unsupported functionality or an implementation failure; see Errors.
    /// No direct output parameters are overwritten or appended; the returned value carries the result.
    /// # Errors
    /// An implementation may return `ServerError::unsupported()` for an unsupported feature, an
    /// invalid-input error, or an analysis/I/O failure. [Cancellation and partial side
    /// effects](ILanguageServer#errors-and-cancellation) follow the shared contract.
    /// # Examples
    /// Requires an implementation and a context supplied by a live wrapper; this may perform
    /// application or client I/O.
    /// ```no_run
    /// use lspally::server::model::*;
    /// async fn example<S: ILanguageServer>(server: &S, context: Context, document: TextDocumentIdentifier) -> ServerResult<Option<Vec<FoldingRange>>> {
    ///     let params = FoldingRangeParams { text_document: document, work_done_progress_params: WorkDoneProgressParams { work_done_token: None }, partial_result_params: PartialResultParams::default() };
    ///     server.folding_range_request(context, params).await
    /// }
    /// ```
    fn folding_range_request(
        &self,
        context: Context,
        params: FoldingRangeParams,
    ) -> impl Future<Output = ServerResult<Option<Vec<FoldingRange>>>> + Send;
    /// Check whether rename is valid at the cursor before the client asks for a new name.
    /// Returns the rename span/placeholder or default-behavior choice without renaming anything;
    /// [`Self::rename`] computes the later edit.
    ///
    /// # Arguments
    /// - `self` `[in,out]`: implementation receiving this call; application state/cache changes are
    /// implementation-owned, not an output buffer.
    /// - `context` `[in]`: owned client handle, cancellation signal and negotiated coordinate unit for
    /// this call; see the [dispatch contract](ILanguageServer#dispatch-and-effects).
    /// - `params` `[in]`: document identity and cursor position only.
    /// # Returns
    /// A sendable future yielding `Ok(Some(response))` with a range, range/placeholder or
    /// default-behavior flag; `Ok(None)` if rename is not valid here.
    /// `Err(error)` reports unsupported functionality or an implementation failure; see Errors.
    /// No direct output parameters are overwritten or appended; the returned value carries the result.
    /// # Errors
    /// An implementation may return `ServerError::unsupported()` for an unsupported feature, an
    /// invalid-input error, or an analysis/I/O failure. [Cancellation and partial side
    /// effects](ILanguageServer#errors-and-cancellation) follow the shared contract.
    /// # Examples
    /// Requires an implementation and a context supplied by a live wrapper; this may perform
    /// application or client I/O.
    /// ```no_run
    /// use lspally::server::model::*;
    /// async fn example<S: ILanguageServer>(server: &S, context: Context, at: TextDocumentPositionParams) -> ServerResult<Option<PrepareRenameResponse>> {
    ///     server.prepare_rename_request(context, at).await
    /// }
    /// ```
    fn prepare_rename_request(
        &self,
        context: Context,
        params: TextDocumentPositionParams,
    ) -> impl Future<Output = ServerResult<Option<PrepareRenameResponse>>> + Send;
    /// Build enclosing selection chains for each requested cursor position.
    /// Returns source ranges with optional parent ranges; the client chooses and changes its
    /// selection. Results correspond to input positions in order.
    ///
    /// # Arguments
    /// - `self` `[in,out]`: implementation receiving this call; application state/cache changes are
    /// implementation-owned, not an output buffer.
    /// - `context` `[in]`: owned client handle, cancellation signal and negotiated coordinate unit for
    /// this call; see the [dispatch contract](ILanguageServer#dispatch-and-effects).
    /// - `params` `[in]`: document, ordered cursor positions and optional work-done and partial-result
    /// tokens.
    /// # Returns
    /// A sendable future yielding `Ok(Some(ranges))` with one nested selection chain per position;
    /// `Ok(None)` when no selection-range result is available.
    /// `Err(error)` reports unsupported functionality or an implementation failure; see Errors.
    /// No direct output parameters are overwritten or appended; the returned value carries the result.
    /// # Errors
    /// An implementation may return `ServerError::unsupported()` for an unsupported feature, an
    /// invalid-input error, or an analysis/I/O failure. [Cancellation and partial side
    /// effects](ILanguageServer#errors-and-cancellation) follow the shared contract.
    /// # Examples
    /// Requires an implementation and a context supplied by a live wrapper; this may perform
    /// application or client I/O.
    /// ```no_run
    /// use lspally::server::model::*;
    /// async fn example<S: ILanguageServer>(server: &S, context: Context, document: TextDocumentIdentifier) -> ServerResult<Option<Vec<SelectionRange>>> {
    ///     let params = SelectionRangeParams { text_document: document, positions: vec![Position { line: 0, character: 0 }], work_done_progress_params: WorkDoneProgressParams { work_done_token: None }, partial_result_params: PartialResultParams::default() };
    ///     server.selection_range_request(context, params).await
    /// }
    /// ```
    fn selection_range_request(
        &self,
        context: Context,
        params: SelectionRangeParams,
    ) -> impl Future<Output = ServerResult<Option<Vec<SelectionRange>>>> + Send;
    /// Identify callable items at the cursor as roots for later call-hierarchy queries.
    /// Returns items and optional opaque data for incoming/outgoing queries; it does not
    /// execute any callable.
    ///
    /// # Arguments
    /// - `self` `[in,out]`: implementation receiving this call; application state/cache changes are
    /// implementation-owned, not an output buffer.
    /// - `context` `[in]`: owned client handle, cancellation signal and negotiated coordinate unit for
    /// this call; see the [dispatch contract](ILanguageServer#dispatch-and-effects).
    /// - `params` `[in]`: document/cursor and optional work-done token.
    /// # Returns
    /// A sendable future yielding `Ok(Some(items))` with candidate callable roots (possibly empty);
    /// `Ok(None)` when no root is available.
    /// `Err(error)` reports unsupported functionality or an implementation failure; see Errors.
    /// No direct output parameters are overwritten or appended; the returned value carries the result.
    /// # Errors
    /// An implementation may return `ServerError::unsupported()` for an unsupported feature, an
    /// invalid-input error, or an analysis/I/O failure. [Cancellation and partial side
    /// effects](ILanguageServer#errors-and-cancellation) follow the shared contract.
    /// # Examples
    /// Requires an implementation and a context supplied by a live wrapper; this may perform
    /// application or client I/O.
    /// ```no_run
    /// use lspally::server::model::*;
    /// async fn example<S: ILanguageServer>(server: &S, context: Context, at: TextDocumentPositionParams) -> ServerResult<Option<Vec<CallHierarchyItem>>> {
    ///     let params = CallHierarchyPrepareParams { text_document_position_params: at, work_done_progress_params: WorkDoneProgressParams { work_done_token: None } };
    ///     server.call_hierarchy_prepare(context, params).await
    /// }
    /// ```
    fn call_hierarchy_prepare(
        &self,
        context: Context,
        params: CallHierarchyPrepareParams,
    ) -> impl Future<Output = ServerResult<Option<Vec<CallHierarchyItem>>>> + Send;
    /// Find callers of an item produced by [`Self::call_hierarchy_prepare`].
    /// Returns caller items and ranges within each caller where it calls the requested item; no
    /// callable is executed.
    ///
    /// # Arguments
    /// - `self` `[in,out]`: implementation receiving this call; application state/cache changes are
    /// implementation-owned, not an output buffer.
    /// - `context` `[in]`: owned client handle, cancellation signal and negotiated coordinate unit for
    /// this call; see the [dispatch contract](ILanguageServer#dispatch-and-effects).
    /// - `params` `[in]`: target callable item with preserved opaque data and optional
    /// work/partial-result tokens.
    /// # Returns
    /// A sendable future yielding `Ok(Some(calls))` with callers and their call-site ranges (possibly
    /// empty); `Ok(None)` when no incoming-call result is available.
    /// `Err(error)` reports unsupported functionality or an implementation failure; see Errors.
    /// No direct output parameters are overwritten or appended; the returned value carries the result.
    /// # Errors
    /// An implementation may return `ServerError::unsupported()` for an unsupported feature, an
    /// invalid-input error, or an analysis/I/O failure. [Cancellation and partial side
    /// effects](ILanguageServer#errors-and-cancellation) follow the shared contract.
    /// # Examples
    /// Requires an implementation and a context supplied by a live wrapper; this may perform
    /// application or client I/O.
    /// ```no_run
    /// use lspally::server::model::*;
    /// async fn example<S: ILanguageServer>(server: &S, context: Context, item: CallHierarchyItem) -> ServerResult<Option<Vec<CallHierarchyIncomingCall>>> {
    ///     let params = CallHierarchyIncomingCallsParams { item, work_done_progress_params: WorkDoneProgressParams { work_done_token: None }, partial_result_params: PartialResultParams::default() };
    ///     server.call_hierarchy_incoming_calls(context, params).await
    /// }
    /// ```
    fn call_hierarchy_incoming_calls(
        &self,
        context: Context,
        params: CallHierarchyIncomingCallsParams,
    ) -> impl Future<Output = ServerResult<Option<Vec<CallHierarchyIncomingCall>>>> + Send;
    /// Find callees referenced by a call-hierarchy item.
    /// Returns callee items and call-site ranges inside the requested caller, not inside each callee;
    /// it does not execute calls.
    ///
    /// # Arguments
    /// - `self` `[in,out]`: implementation receiving this call; application state/cache changes are
    /// implementation-owned, not an output buffer.
    /// - `context` `[in]`: owned client handle, cancellation signal and negotiated coordinate unit for
    /// this call; see the [dispatch contract](ILanguageServer#dispatch-and-effects).
    /// - `params` `[in]`: source callable item with preserved opaque data and optional
    /// work/partial-result tokens.
    /// # Returns
    /// A sendable future yielding `Ok(Some(calls))` with callees and source-item call sites (possibly
    /// empty); `Ok(None)` when no outgoing-call result is available.
    /// `Err(error)` reports unsupported functionality or an implementation failure; see Errors.
    /// No direct output parameters are overwritten or appended; the returned value carries the result.
    /// # Errors
    /// An implementation may return `ServerError::unsupported()` for an unsupported feature, an
    /// invalid-input error, or an analysis/I/O failure. [Cancellation and partial side
    /// effects](ILanguageServer#errors-and-cancellation) follow the shared contract.
    /// # Examples
    /// Requires an implementation and a context supplied by a live wrapper; this may perform
    /// application or client I/O.
    /// ```no_run
    /// use lspally::server::model::*;
    /// async fn example<S: ILanguageServer>(server: &S, context: Context, item: CallHierarchyItem) -> ServerResult<Option<Vec<CallHierarchyOutgoingCall>>> {
    ///     let params = CallHierarchyOutgoingCallsParams { item, work_done_progress_params: WorkDoneProgressParams { work_done_token: None }, partial_result_params: PartialResultParams::default() };
    ///     server.call_hierarchy_outgoing_calls(context, params).await
    /// }
    /// ```
    fn call_hierarchy_outgoing_calls(
        &self,
        context: Context,
        params: CallHierarchyOutgoingCallsParams,
    ) -> impl Future<Output = ServerResult<Option<Vec<CallHierarchyOutgoingCall>>>> + Send;
    /// Classify semantic tokens for the whole document using the advertised legend.
    /// Returns relative token coordinates and optional result identity for later delta requests. The
    /// wrapper serializes records but neither tokenizes text nor stores a token cache.
    ///
    /// # Arguments
    /// - `self` `[in,out]`: implementation receiving this call; application state/cache changes are
    /// implementation-owned, not an output buffer.
    /// - `context` `[in]`: owned client handle, cancellation signal and negotiated coordinate unit for
    /// this call; see the [dispatch contract](ILanguageServer#dispatch-and-effects).
    /// - `params` `[in]`: document identity and optional work-done and partial-result tokens.
    /// # Returns
    /// A sendable future yielding `Ok(Some(Tokens(tokens)))` with a full stream and optional result
    /// ID, or `Ok(Some(Partial(partial)))` with token data; `Ok(None)` when no token result is
    /// available.
    /// `Err(error)` reports unsupported functionality or an implementation failure; see Errors.
    /// No direct output parameters are overwritten or appended; the returned value carries the result.
    /// # Errors
    /// An implementation may return `ServerError::unsupported()` for an unsupported feature, an
    /// invalid-input error, or an analysis/I/O failure. [Cancellation and partial side
    /// effects](ILanguageServer#errors-and-cancellation) follow the shared contract.
    /// # Examples
    /// Requires an implementation and a context supplied by a live wrapper; this may perform
    /// application or client I/O.
    /// ```no_run
    /// use lspally::server::model::*;
    /// async fn example<S: ILanguageServer>(server: &S, context: Context, document: TextDocumentIdentifier) -> ServerResult<Option<SemanticTokensResult>> {
    ///     let params = SemanticTokensParams { text_document: document, work_done_progress_params: WorkDoneProgressParams { work_done_token: None }, partial_result_params: PartialResultParams::default() };
    ///     server.semantic_tokens_full_request(context, params).await
    /// }
    /// ```
    fn semantic_tokens_full_request(
        &self,
        context: Context,
        params: SemanticTokensParams,
    ) -> impl Future<Output = ServerResult<Option<SemanticTokensResult>>> + Send;
    /// Return changed semantic-token data relative to a previous token result.
    /// The implementation owns previous-result lookup and any token cache. Returned edits address the
    /// flattened five-integer token stream, not text bytes; the wrapper only converts them.
    ///
    /// # Arguments
    /// - `self` `[in,out]`: implementation receiving this call; application state/cache changes are
    /// implementation-owned, not an output buffer.
    /// - `context` `[in]`: owned client handle, cancellation signal and negotiated coordinate unit for
    /// this call; see the [dispatch contract](ILanguageServer#dispatch-and-effects).
    /// - `params` `[in]`: document, previous result ID and optional work-done and partial-result
    /// tokens.
    /// # Returns
    /// A sendable future yielding `Ok(Some(Tokens(tokens)))` for a replacement stream,
    /// `Ok(Some(TokensDelta(delta)))` for edits and optional new ID, or `Ok(Some(PartialTokensDelta {
    /// edits }))` for delta edits; `Ok(None)` when no result is available.
    /// `Err(error)` reports unsupported functionality or an implementation failure; see Errors.
    /// No direct output parameters are overwritten or appended; the returned value carries the result.
    /// # Errors
    /// An implementation may return `ServerError::unsupported()` for an unsupported feature, an
    /// invalid-input error, or an analysis/I/O failure. [Cancellation and partial side
    /// effects](ILanguageServer#errors-and-cancellation) follow the shared contract.
    /// # Algorithm
    /// Resolve the previous result ID against implementation-owned state. Return compatible stream
    /// edits when available, otherwise a complete replacement stream. The wrapper does not validate
    /// edit offsets or reconstruct the previous result.
    /// # Flow
    /// ```text
    /// [document snapshot]      [previous result ID]
    ///           |                        |
    ///           +-----------+------------+
    ///                       v
    ///               <usable baseline?>
    ///                 /           \
    ///               yes           no
    ///                |             |
    ///           [delta edits] [replacement tokens]
    ///                |             |
    ///                +------+------+
    ///                       v
    ///                 [client result]
    /// ```
    /// # Examples
    /// Requires an implementation and a context supplied by a live wrapper; this may perform
    /// application or client I/O.
    /// ```no_run
    /// use lspally::server::model::*;
    /// async fn example<S: ILanguageServer>(server: &S, context: Context, document: TextDocumentIdentifier, previous_result_id: String) -> ServerResult<Option<SemanticTokensFullDeltaResult>> {
    ///     let params = SemanticTokensDeltaParams { text_document: document, previous_result_id, work_done_progress_params: WorkDoneProgressParams { work_done_token: None }, partial_result_params: PartialResultParams::default() };
    ///     server.semantic_tokens_full_delta_request(context, params).await
    /// }
    /// ```
    fn semantic_tokens_full_delta_request(
        &self,
        context: Context,
        params: SemanticTokensDeltaParams,
    ) -> impl Future<Output = ServerResult<Option<SemanticTokensFullDeltaResult>>> + Send;
    /// Classify semantic tokens intersecting the requested document range.
    /// Uses the same advertised token legend as full requests. Return coordinates relative to the
    /// document, not to the requested range; the wrapper does not rebase them.
    ///
    /// # Arguments
    /// - `self` `[in,out]`: implementation receiving this call; application state/cache changes are
    /// implementation-owned, not an output buffer.
    /// - `context` `[in]`: owned client handle, cancellation signal and negotiated coordinate unit for
    /// this call; see the [dispatch contract](ILanguageServer#dispatch-and-effects).
    /// - `params` `[in]`: document, requested span and optional work-done and partial-result tokens.
    /// # Returns
    /// A sendable future yielding `Ok(Some(Tokens(tokens)))` for token records with optional result
    /// ID, or `Ok(Some(Partial(partial)))` for token data; `Ok(None)` when no range result is
    /// available.
    /// `Err(error)` reports unsupported functionality or an implementation failure; see Errors.
    /// No direct output parameters are overwritten or appended; the returned value carries the result.
    /// # Errors
    /// An implementation may return `ServerError::unsupported()` for an unsupported feature, an
    /// invalid-input error, or an analysis/I/O failure. [Cancellation and partial side
    /// effects](ILanguageServer#errors-and-cancellation) follow the shared contract.
    /// # Examples
    /// Requires an implementation and a context supplied by a live wrapper; this may perform
    /// application or client I/O.
    /// ```no_run
    /// use lspally::server::model::*;
    /// async fn example<S: ILanguageServer>(server: &S, context: Context, document: TextDocumentIdentifier, range: Range) -> ServerResult<Option<SemanticTokensRangeResult>> {
    ///     let params = SemanticTokensRangeParams { text_document: document, range, work_done_progress_params: WorkDoneProgressParams { work_done_token: None }, partial_result_params: PartialResultParams::default() };
    ///     server.semantic_tokens_range_request(context, params).await
    /// }
    /// ```
    fn semantic_tokens_range_request(
        &self,
        context: Context,
        params: SemanticTokensRangeParams,
    ) -> impl Future<Output = ServerResult<Option<SemanticTokensRangeResult>>> + Send;
    /// Compute workspace edits the client should apply before creating resources.
    /// This pre-operation hook does not perform the file operation. The client applies returned edits
    /// before its own operation and may discard slow results under its timeout policy.
    ///
    /// # Arguments
    /// - `self` `[in,out]`: implementation receiving this call; application state/cache changes are
    /// implementation-owned, not an output buffer.
    /// - `context` `[in]`: owned client handle, cancellation signal and negotiated coordinate unit for
    /// this call; see the [dispatch contract](ILanguageServer#dispatch-and-effects).
    /// - `params` `[in]`: URIs the client intends to create; URI fields are strings in these
    /// file-operation models.
    /// # Returns
    /// A sendable future yielding `Ok(Some(edit))` with preparatory workspace changes; `Ok(None)` when
    /// no preparatory edit is needed.
    /// `Err(error)` reports unsupported functionality or an implementation failure; see Errors.
    /// No direct output parameters are overwritten or appended; the returned value carries the result.
    /// # Errors
    /// An implementation may return `ServerError::unsupported()` for an unsupported feature, an
    /// invalid-input error, or an analysis/I/O failure. [Cancellation and partial side
    /// effects](ILanguageServer#errors-and-cancellation) follow the shared contract.
    /// # Examples
    /// Requires an implementation and a context supplied by a live wrapper; this may perform
    /// application or client I/O.
    /// ```no_run
    /// use lspally::server::model::*;
    /// async fn example<S: ILanguageServer>(server: &S, context: Context) -> ServerResult<Option<WorkspaceEdit>> {
    ///     let params = CreateFilesParams { files: vec![FileCreate { uri: "file:///tmp/new.cmake".into() }] };
    ///     server.will_create_files(context, params).await
    /// }
    /// ```
    fn will_create_files(
        &self,
        context: Context,
        params: CreateFilesParams,
    ) -> impl Future<Output = ServerResult<Option<WorkspaceEdit>>> + Send;
    /// Compute workspace edits the client should apply before renaming resources.
    /// This pre-operation hook does not perform the file operation. The client applies returned edits
    /// before its own operation and may discard slow results under its timeout policy.
    ///
    /// # Arguments
    /// - `self` `[in,out]`: implementation receiving this call; application state/cache changes are
    /// implementation-owned, not an output buffer.
    /// - `context` `[in]`: owned client handle, cancellation signal and negotiated coordinate unit for
    /// this call; see the [dispatch contract](ILanguageServer#dispatch-and-effects).
    /// - `params` `[in]`: old/new URI pairs the client intends to rename; URI fields are strings in
    /// these file-operation models.
    /// # Returns
    /// A sendable future yielding `Ok(Some(edit))` with preparatory workspace changes; `Ok(None)` when
    /// no preparatory edit is needed.
    /// `Err(error)` reports unsupported functionality or an implementation failure; see Errors.
    /// No direct output parameters are overwritten or appended; the returned value carries the result.
    /// # Errors
    /// An implementation may return `ServerError::unsupported()` for an unsupported feature, an
    /// invalid-input error, or an analysis/I/O failure. [Cancellation and partial side
    /// effects](ILanguageServer#errors-and-cancellation) follow the shared contract.
    /// # Examples
    /// Requires an implementation and a context supplied by a live wrapper; this may perform
    /// application or client I/O.
    /// ```no_run
    /// use lspally::server::model::*;
    /// async fn example<S: ILanguageServer>(server: &S, context: Context) -> ServerResult<Option<WorkspaceEdit>> {
    ///     let params = RenameFilesParams { files: vec![FileRename { old_uri: "file:///tmp/old.cmake".into(), new_uri: "file:///tmp/new.cmake".into() }] };
    ///     server.will_rename_files(context, params).await
    /// }
    /// ```
    fn will_rename_files(
        &self,
        context: Context,
        params: RenameFilesParams,
    ) -> impl Future<Output = ServerResult<Option<WorkspaceEdit>>> + Send;
    /// Compute workspace edits the client should apply before deleting resources.
    /// This pre-operation hook does not perform the file operation. The client applies returned edits
    /// before its own operation and may discard slow results under its timeout policy.
    ///
    /// # Arguments
    /// - `self` `[in,out]`: implementation receiving this call; application state/cache changes are
    /// implementation-owned, not an output buffer.
    /// - `context` `[in]`: owned client handle, cancellation signal and negotiated coordinate unit for
    /// this call; see the [dispatch contract](ILanguageServer#dispatch-and-effects).
    /// - `params` `[in]`: URIs the client intends to delete; URI fields are strings in these
    /// file-operation models.
    /// # Returns
    /// A sendable future yielding `Ok(Some(edit))` with preparatory workspace changes; `Ok(None)` when
    /// no preparatory edit is needed.
    /// `Err(error)` reports unsupported functionality or an implementation failure; see Errors.
    /// No direct output parameters are overwritten or appended; the returned value carries the result.
    /// # Errors
    /// An implementation may return `ServerError::unsupported()` for an unsupported feature, an
    /// invalid-input error, or an analysis/I/O failure. [Cancellation and partial side
    /// effects](ILanguageServer#errors-and-cancellation) follow the shared contract.
    /// # Examples
    /// Requires an implementation and a context supplied by a live wrapper; this may perform
    /// application or client I/O.
    /// ```no_run
    /// use lspally::server::model::*;
    /// async fn example<S: ILanguageServer>(server: &S, context: Context) -> ServerResult<Option<WorkspaceEdit>> {
    ///     let params = DeleteFilesParams { files: vec![FileDelete { uri: "file:///tmp/old.cmake".into() }] };
    ///     server.will_delete_files(context, params).await
    /// }
    /// ```
    fn will_delete_files(
        &self,
        context: Context,
        params: DeleteFilesParams,
    ) -> impl Future<Output = ServerResult<Option<WorkspaceEdit>>> + Send;
    /// Identify symbols at the cursor with cross-tool symbol identities.
    /// Returns scheme-qualified identifiers and their uniqueness/import/export roles. It does not
    /// resolve external indexes or navigate the client automatically.
    ///
    /// # Arguments
    /// - `self` `[in,out]`: implementation receiving this call; application state/cache changes are
    /// implementation-owned, not an output buffer.
    /// - `context` `[in]`: owned client handle, cancellation signal and negotiated coordinate unit for
    /// this call; see the [dispatch contract](ILanguageServer#dispatch-and-effects).
    /// - `params` `[in]`: document/cursor and optional work-done and partial-result tokens.
    /// # Returns
    /// A sendable future yielding `Ok(Some(monikers))` with identities (possibly empty); `Ok(None)`
    /// when no moniker result is available.
    /// `Err(error)` reports unsupported functionality or an implementation failure; see Errors.
    /// No direct output parameters are overwritten or appended; the returned value carries the result.
    /// # Errors
    /// An implementation may return `ServerError::unsupported()` for an unsupported feature, an
    /// invalid-input error, or an analysis/I/O failure. [Cancellation and partial side
    /// effects](ILanguageServer#errors-and-cancellation) follow the shared contract.
    /// # Examples
    /// Requires an implementation and a context supplied by a live wrapper; this may perform
    /// application or client I/O.
    /// ```no_run
    /// use lspally::server::model::*;
    /// async fn example<S: ILanguageServer>(server: &S, context: Context, at: TextDocumentPositionParams) -> ServerResult<Option<Vec<Moniker>>> {
    ///     let params = MonikerParams { text_document_position_params: at, work_done_progress_params: WorkDoneProgressParams { work_done_token: None }, partial_result_params: PartialResultParams::default() };
    ///     server.moniker_request(context, params).await
    /// }
    /// ```
    fn moniker_request(
        &self,
        context: Context,
        params: MonikerParams,
    ) -> impl Future<Output = ServerResult<Option<Vec<Moniker>>>> + Send;
    /// Compute inline labels, such as inferred types or parameter names, for a document range.
    /// Hints are display suggestions, not inserted text. The client may later request lazy properties
    /// through [`Self::inlay_hint_resolve_request`].
    ///
    /// # Arguments
    /// - `self` `[in,out]`: implementation receiving this call; application state/cache changes are
    /// implementation-owned, not an output buffer.
    /// - `context` `[in]`: owned client handle, cancellation signal and negotiated coordinate unit for
    /// this call; see the [dispatch contract](ILanguageServer#dispatch-and-effects).
    /// - `params` `[in]`: document, requested range and optional work-done token.
    /// # Returns
    /// A sendable future yielding `Ok(Some(hints))` with display labels and optional
    /// edits/tooltips/data (possibly empty); `Ok(None)` when no hint result is available.
    /// `Err(error)` reports unsupported functionality or an implementation failure; see Errors.
    /// No direct output parameters are overwritten or appended; the returned value carries the result.
    /// # Errors
    /// An implementation may return `ServerError::unsupported()` for an unsupported feature, an
    /// invalid-input error, or an analysis/I/O failure. [Cancellation and partial side
    /// effects](ILanguageServer#errors-and-cancellation) follow the shared contract.
    /// # Examples
    /// Requires an implementation and a context supplied by a live wrapper; this may perform
    /// application or client I/O.
    /// ```no_run
    /// use lspally::server::model::*;
    /// async fn example<S: ILanguageServer>(server: &S, context: Context, document: TextDocumentIdentifier, range: Range) -> ServerResult<Option<Vec<InlayHint>>> {
    ///     let params = InlayHintParams { text_document: document, range, work_done_progress_params: WorkDoneProgressParams { work_done_token: None } };
    ///     server.inlay_hint_request(context, params).await
    /// }
    /// ```
    fn inlay_hint_request(
        &self,
        context: Context,
        params: InlayHintParams,
    ) -> impl Future<Output = ServerResult<Option<Vec<InlayHint>>>> + Send;
    /// Fill negotiated lazy properties of a previously returned inlay hint.
    /// Returns an enriched hint, such as a tooltip, label-part location or text edit; resolving does
    /// not apply text edits or execute label commands.
    ///
    /// # Arguments
    /// - `self` `[in,out]`: implementation receiving this call; application state/cache changes are
    /// implementation-owned, not an output buffer.
    /// - `context` `[in]`: owned client handle, cancellation signal and negotiated coordinate unit for
    /// this call; see the [dispatch contract](ILanguageServer#dispatch-and-effects).
    /// - `params` `[in]`: hint from [`Self::inlay_hint_request`] with its position, label and opaque
    /// data.
    /// # Returns
    /// A sendable future yielding `Ok(hint)` with resolved properties; `Err(error)` if the original
    /// hint cannot be resolved.
    /// No direct output parameters are overwritten or appended; the returned value carries the result.
    /// # Errors
    /// An implementation may return `ServerError::unsupported()` for an unsupported feature, an
    /// invalid-input error, or an analysis/I/O failure. [Cancellation and partial side
    /// effects](ILanguageServer#errors-and-cancellation) follow the shared contract.
    /// # Examples
    /// Requires an implementation and a context supplied by a live wrapper; this may perform
    /// application or client I/O.
    /// ```no_run
    /// use lspally::server::model::*;
    /// async fn example<S: ILanguageServer>(server: &S, context: Context, hint: InlayHint) -> ServerResult<InlayHint> {
    ///     server.inlay_hint_resolve_request(context, hint).await
    /// }
    /// ```
    fn inlay_hint_resolve_request(
        &self,
        context: Context,
        params: InlayHint,
    ) -> impl Future<Output = ServerResult<InlayHint>> + Send;
    /// Describe debugger inline values for the visible range and stopped frame.
    /// Returns literal text, variable-lookup instructions or evaluatable-expression instructions.
    /// Returning lookup/expression descriptors does not itself evaluate them through the wrapper.
    ///
    /// # Arguments
    /// - `self` `[in,out]`: implementation receiving this call; application state/cache changes are
    /// implementation-owned, not an output buffer.
    /// - `context` `[in]`: owned client handle, cancellation signal and negotiated coordinate unit for
    /// this call; see the [dispatch contract](ILanguageServer#dispatch-and-effects).
    /// - `params` `[in]`: document, visible range, debugger frame ID, stopped location and optional
    /// work-done token.
    /// # Returns
    /// A sendable future yielding `Ok(Some(values))` with text/lookup/expression entries (possibly
    /// empty); `Ok(None)` when no inline-value result is available.
    /// `Err(error)` reports unsupported functionality or an implementation failure; see Errors.
    /// No direct output parameters are overwritten or appended; the returned value carries the result.
    /// # Errors
    /// An implementation may return `ServerError::unsupported()` for an unsupported feature, an
    /// invalid-input error, or an analysis/I/O failure. [Cancellation and partial side
    /// effects](ILanguageServer#errors-and-cancellation) follow the shared contract.
    /// # Examples
    /// Requires an implementation and a context supplied by a live wrapper; this may perform
    /// application or client I/O.
    /// ```no_run
    /// use lspally::server::model::*;
    /// async fn example<S: ILanguageServer>(server: &S, context: Context, document: TextDocumentIdentifier, range: Range, stopped: InlineValueContext) -> ServerResult<Option<Vec<InlineValue>>> {
    ///     let params = InlineValueParams { text_document: document, range, context: stopped, work_done_progress_params: WorkDoneProgressParams { work_done_token: None } };
    ///     server.inline_value_request(context, params).await
    /// }
    /// ```
    fn inline_value_request(
        &self,
        context: Context,
        params: InlineValueParams,
    ) -> impl Future<Output = ServerResult<Option<Vec<InlineValue>>>> + Send;
    /// Pull diagnostics for one document, optionally reusing a previous report identity.
    /// Returns reports rather than sending a push publication. The implementation decides whether
    /// cached diagnostics are unchanged and may include related-document reports.
    ///
    /// # Arguments
    /// - `self` `[in,out]`: implementation receiving this call; application state/cache changes are
    /// implementation-owned, not an output buffer.
    /// - `context` `[in]`: owned client handle, cancellation signal and negotiated coordinate unit for
    /// this call; see the [dispatch contract](ILanguageServer#dispatch-and-effects).
    /// - `params` `[in]`: document, optional provider identifier and previous result ID, plus
    /// work/partial-result tokens.
    /// # Returns
    /// A sendable future yielding `Ok(Report(Full(report)))` for replacement diagnostics,
    /// `Ok(Report(Unchanged(report)))` to reuse the previous result, or `Ok(Partial(partial))` for
    /// related-document reports. `Err(error)` reports failure; there is no outer `Option`.
    /// No direct output parameters are overwritten or appended; the returned value carries the result.
    /// # Errors
    /// An implementation may return `ServerError::unsupported()` for an unsupported feature, an
    /// invalid-input error, or an analysis/I/O failure. [Cancellation and partial side
    /// effects](ILanguageServer#errors-and-cancellation) follow the shared contract.
    /// # Examples
    /// Requires an implementation and a context supplied by a live wrapper; this may perform
    /// application or client I/O.
    /// ```no_run
    /// use lspally::server::model::*;
    /// async fn example<S: ILanguageServer>(server: &S, context: Context, document: TextDocumentIdentifier) -> ServerResult<DocumentDiagnosticReportResult> {
    ///     let params = DocumentDiagnosticParams { text_document: document, identifier: None, previous_result_id: None, work_done_progress_params: WorkDoneProgressParams { work_done_token: None }, partial_result_params: PartialResultParams::default() };
    ///     server.document_diagnostic_request(context, params).await
    /// }
    /// ```
    fn document_diagnostic_request(
        &self,
        context: Context,
        params: DocumentDiagnosticParams,
    ) -> impl Future<Output = ServerResult<DocumentDiagnosticReportResult>> + Send;
    /// Pull workspace-wide diagnostics with per-document previous-result identities.
    /// The implementation owns workspace discovery and diagnostic caches. Return URI/version-qualified
    /// full or unchanged document reports, optionally using the supplied partial-result token.
    ///
    /// # Arguments
    /// - `self` `[in,out]`: implementation receiving this call; application state/cache changes are
    /// implementation-owned, not an output buffer.
    /// - `context` `[in]`: owned client handle, cancellation signal and negotiated coordinate unit for
    /// this call; see the [dispatch contract](ILanguageServer#dispatch-and-effects).
    /// - `params` `[in]`: optional provider identifier, previous result IDs by URI, and optional
    /// work/partial-result tokens.
    /// # Returns
    /// A sendable future yielding `Ok(Report(report))` for workspace report items or
    /// `Ok(Partial(partial))` for a batch; each item is full or unchanged. An empty item list is
    /// valid; `Err(error)` reports failure.
    /// No direct output parameters are overwritten or appended; the returned value carries the result.
    /// # Errors
    /// An implementation may return `ServerError::unsupported()` for an unsupported feature, an
    /// invalid-input error, or an analysis/I/O failure. [Cancellation and partial side
    /// effects](ILanguageServer#errors-and-cancellation) follow the shared contract.
    /// # Examples
    /// Requires an implementation and a context supplied by a live wrapper; this may perform
    /// application or client I/O.
    /// ```no_run
    /// use lspally::server::model::*;
    /// async fn example<S: ILanguageServer>(server: &S, context: Context) -> ServerResult<WorkspaceDiagnosticReportResult> {
    ///     let params = WorkspaceDiagnosticParams { identifier: None, previous_result_ids: vec![], work_done_progress_params: WorkDoneProgressParams { work_done_token: None }, partial_result_params: PartialResultParams::default() };
    ///     server.workspace_diagnostic_request(context, params).await
    /// }
    /// ```
    fn workspace_diagnostic_request(
        &self,
        context: Context,
        params: WorkspaceDiagnosticParams,
    ) -> impl Future<Output = ServerResult<WorkspaceDiagnosticReportResult>> + Send;
    /// Identify type-hierarchy roots at the requested cursor.
    /// Returns items with identity/data for later supertype/subtype queries; it does not compute a
    /// full transitive hierarchy unless the implementation chooses to cache one.
    ///
    /// # Arguments
    /// - `self` `[in,out]`: implementation receiving this call; application state/cache changes are
    /// implementation-owned, not an output buffer.
    /// - `context` `[in]`: owned client handle, cancellation signal and negotiated coordinate unit for
    /// this call; see the [dispatch contract](ILanguageServer#dispatch-and-effects).
    /// - `params` `[in]`: document/cursor and optional work-done token.
    /// # Returns
    /// A sendable future yielding `Ok(Some(items))` with candidate type roots (possibly empty);
    /// `Ok(None)` when no type root exists.
    /// `Err(error)` reports unsupported functionality or an implementation failure; see Errors.
    /// No direct output parameters are overwritten or appended; the returned value carries the result.
    /// # Errors
    /// An implementation may return `ServerError::unsupported()` for an unsupported feature, an
    /// invalid-input error, or an analysis/I/O failure. [Cancellation and partial side
    /// effects](ILanguageServer#errors-and-cancellation) follow the shared contract.
    /// # Examples
    /// Requires an implementation and a context supplied by a live wrapper; this may perform
    /// application or client I/O.
    /// ```no_run
    /// use lspally::server::model::*;
    /// async fn example<S: ILanguageServer>(server: &S, context: Context, at: TextDocumentPositionParams) -> ServerResult<Option<Vec<TypeHierarchyItem>>> {
    ///     let params = TypeHierarchyPrepareParams { text_document_position_params: at, work_done_progress_params: WorkDoneProgressParams { work_done_token: None } };
    ///     server.type_hierarchy_prepare(context, params).await
    /// }
    /// ```
    fn type_hierarchy_prepare(
        &self,
        context: Context,
        params: TypeHierarchyPrepareParams,
    ) -> impl Future<Output = ServerResult<Option<Vec<TypeHierarchyItem>>>> + Send;
    /// Find direct base types of a type-hierarchy item.
    /// Returns the next hierarchy level rather than navigating or modifying declarations. Preserve
    /// opaque item data when expanding results from [`Self::type_hierarchy_prepare`].
    ///
    /// # Arguments
    /// - `self` `[in,out]`: implementation receiving this call; application state/cache changes are
    /// implementation-owned, not an output buffer.
    /// - `context` `[in]`: owned client handle, cancellation signal and negotiated coordinate unit for
    /// this call; see the [dispatch contract](ILanguageServer#dispatch-and-effects).
    /// - `params` `[in]`: type item to expand and optional work-done and partial-result tokens.
    /// # Returns
    /// A sendable future yielding `Ok(Some(items))` with direct base types (possibly empty);
    /// `Ok(None)` when no hierarchy result is available.
    /// `Err(error)` reports unsupported functionality or an implementation failure; see Errors.
    /// No direct output parameters are overwritten or appended; the returned value carries the result.
    /// # Errors
    /// An implementation may return `ServerError::unsupported()` for an unsupported feature, an
    /// invalid-input error, or an analysis/I/O failure. [Cancellation and partial side
    /// effects](ILanguageServer#errors-and-cancellation) follow the shared contract.
    /// # Examples
    /// Requires an implementation and a context supplied by a live wrapper; this may perform
    /// application or client I/O.
    /// ```no_run
    /// use lspally::server::model::*;
    /// async fn example<S: ILanguageServer>(server: &S, context: Context, item: TypeHierarchyItem) -> ServerResult<Option<Vec<TypeHierarchyItem>>> {
    ///     let params = TypeHierarchySupertypesParams { item, work_done_progress_params: WorkDoneProgressParams { work_done_token: None }, partial_result_params: PartialResultParams::default() };
    ///     server.type_hierarchy_supertypes(context, params).await
    /// }
    /// ```
    fn type_hierarchy_supertypes(
        &self,
        context: Context,
        params: TypeHierarchySupertypesParams,
    ) -> impl Future<Output = ServerResult<Option<Vec<TypeHierarchyItem>>>> + Send;
    /// Find direct derived types of a type-hierarchy item.
    /// Returns the next hierarchy level rather than navigating or modifying declarations. Preserve
    /// opaque item data when expanding results from [`Self::type_hierarchy_prepare`].
    ///
    /// # Arguments
    /// - `self` `[in,out]`: implementation receiving this call; application state/cache changes are
    /// implementation-owned, not an output buffer.
    /// - `context` `[in]`: owned client handle, cancellation signal and negotiated coordinate unit for
    /// this call; see the [dispatch contract](ILanguageServer#dispatch-and-effects).
    /// - `params` `[in]`: type item to expand and optional work-done and partial-result tokens.
    /// # Returns
    /// A sendable future yielding `Ok(Some(items))` with direct derived types (possibly empty);
    /// `Ok(None)` when no hierarchy result is available.
    /// `Err(error)` reports unsupported functionality or an implementation failure; see Errors.
    /// No direct output parameters are overwritten or appended; the returned value carries the result.
    /// # Errors
    /// An implementation may return `ServerError::unsupported()` for an unsupported feature, an
    /// invalid-input error, or an analysis/I/O failure. [Cancellation and partial side
    /// effects](ILanguageServer#errors-and-cancellation) follow the shared contract.
    /// # Examples
    /// Requires an implementation and a context supplied by a live wrapper; this may perform
    /// application or client I/O.
    /// ```no_run
    /// use lspally::server::model::*;
    /// async fn example<S: ILanguageServer>(server: &S, context: Context, item: TypeHierarchyItem) -> ServerResult<Option<Vec<TypeHierarchyItem>>> {
    ///     let params = TypeHierarchySubtypesParams { item, work_done_progress_params: WorkDoneProgressParams { work_done_token: None }, partial_result_params: PartialResultParams::default() };
    ///     server.type_hierarchy_subtypes(context, params).await
    /// }
    /// ```
    fn type_hierarchy_subtypes(
        &self,
        context: Context,
        params: TypeHierarchySubtypesParams,
    ) -> impl Future<Output = ServerResult<Option<Vec<TypeHierarchyItem>>>> + Send;
    /// Observe a `$/cancelRequest` notification for an incoming request ID.
    /// The reader has already signalled the target request's cancellation token before queuing this
    /// hook. This hook may do application bookkeeping; its own context token is not the target token,
    /// and it may run after the request has finished.
    ///
    /// # Arguments
    /// - `self` `[in,out]`: implementation receiving this call; application state/cache changes are
    /// implementation-owned, not an output buffer.
    /// - `context` `[in]`: owned client handle, cancellation signal and negotiated coordinate unit for
    /// this call; see the [dispatch contract](ILanguageServer#dispatch-and-effects).
    /// - `params` `[in]`: numeric or string ID of the incoming request to cancel, not a work-done
    /// progress token.
    /// # Returns
    /// A sendable future yielding `Ok(())` when the notification is handled or no matching work
    /// remains; `Err(error)` on hook failure. No cancellation reply is sent.
    /// No direct output parameters are overwritten or appended; the returned value carries the result.
    /// # Errors
    /// The worker logs hook errors to stderr because this is a notification. Returning an error does
    /// not undo the cancellation already signalled by the reader.
    /// # Examples
    /// Requires an implementation and a context supplied by a live wrapper; this may perform
    /// application or client I/O.
    /// ```no_run
    /// use lspally::server::model::*;
    /// async fn example<S: ILanguageServer>(server: &S, context: Context) -> ServerResult<()> {
    ///     server.cancel(context, CancelParams { id: NumberOrString::Number(7) }).await
    /// }
    /// ```
    fn cancel(
        &self,
        context: Context,
        params: CancelParams,
    ) -> impl Future<Output = ServerResult<()>> + Send;
    /// Observe a `$/setTrace` notification selecting outgoing trace verbosity.
    /// The reader updates the session trace atomic before queuing the hook. The implementation may
    /// adjust its own tracing; [`ClientHandle::log_trace`] already filters output using the session
    /// setting.
    ///
    /// # Arguments
    /// - `self` `[in,out]`: implementation receiving this call; application state/cache changes are
    /// implementation-owned, not an output buffer.
    /// - `context` `[in]`: owned client handle, cancellation signal and negotiated coordinate unit for
    /// this call; see the [dispatch contract](ILanguageServer#dispatch-and-effects).
    /// - `params` `[in]`: off, messages-only or verbose trace setting.
    /// # Returns
    /// A sendable future yielding `Ok(())` after application tracing policy is handled; `Err(error)`
    /// on hook failure. No reply is sent.
    /// No direct output parameters are overwritten or appended; the returned value carries the result.
    /// # Errors
    /// The worker logs hook errors to stderr. Failure does not revert the reader's trace update.
    /// # Examples
    /// Requires an implementation and a context supplied by a live wrapper; this may perform
    /// application or client I/O.
    /// ```no_run
    /// use lspally::server::model::*;
    /// async fn example<S: ILanguageServer>(server: &S, context: Context) -> ServerResult<()> {
    ///     server.set_trace(context, SetTraceParams { value: TraceValue::Messages }).await
    /// }
    /// ```
    fn set_trace(
        &self,
        context: Context,
        params: SetTraceParams,
    ) -> impl Future<Output = ServerResult<()>> + Send;
    /// Start post-negotiation application activity after the client's `initialized` notification.
    /// The reader has already entered RUNNING, so this hook can use permitted client requests or
    /// dynamic registration. Negotiated position units are available in this context; there is no
    /// implicit registration by the wrapper.
    ///
    /// # Arguments
    /// - `self` `[in,out]`: implementation receiving this call; application state/cache changes are
    /// implementation-owned, not an output buffer.
    /// - `context` `[in]`: owned client handle, cancellation signal and negotiated coordinate unit for
    /// this call; see the [dispatch contract](ILanguageServer#dispatch-and-effects).
    /// - `params` `[in]`: the empty initialized payload; it carries no capabilities or new
    /// configuration.
    /// # Returns
    /// A sendable future yielding `Ok(())` after post-initialization work is accepted; `Err(error)` on
    /// hook failure. There is no notification reply.
    /// No direct output parameters are overwritten or appended; the returned value carries the result.
    /// # Errors
    /// The worker logs hook errors to stderr. Such an error does not reset the already-running
    /// lifecycle phase.
    /// # Examples
    /// Requires an implementation and a context supplied by a live wrapper; this may perform
    /// application or client I/O.
    /// ```no_run
    /// use lspally::server::model::*;
    /// async fn example<S: ILanguageServer>(server: &S, context: Context) -> ServerResult<()> {
    ///     server.initialized(context, InitializedParams {}).await
    /// }
    /// ```
    fn initialized(
        &self,
        context: Context,
        params: InitializedParams,
    ) -> impl Future<Output = ServerResult<()>> + Send;
    /// Perform final application cleanup after an explicit client `exit`.
    /// On the normal explicit-exit path, [`Wrapper::run`] closes the session and joins transport/worker
    /// tasks before invoking this hook. Task/I/O failures can return first; EOF/disconnection also skips
    /// this hook. Do not rely on client output here. The owner must also clean up when `run` returns.
    ///
    /// # Arguments
    /// - `self` `[in,out]`: implementation receiving this call; application state/cache changes are
    /// implementation-owned, not an output buffer.
    /// - `context` `[in]`: owned client handle, cancellation signal and negotiated coordinate unit for
    /// this call; see the [dispatch contract](ILanguageServer#dispatch-and-effects).
    /// - `params` `[in]`: unit payload; no exit options exist.
    /// # Returns
    /// A sendable future yielding `Ok(())` after final cleanup; `Err(error)` if it fails. No reply is
    /// sent; `Wrapper::run` maps this error to an I/O error.
    /// No direct output parameters are overwritten or appended; the returned value carries the result.
    /// # Errors
    /// Return final cleanup failures as `Err(error)`. Unlike ordinary notification errors, an error
    /// from the explicit-exit call made by `Wrapper::run` becomes its error return.
    /// # Algorithm
    /// Treat this as a cleanup hook, not a request to write an exit response. The wrapper records
    /// whether shutdown was requested, closes the session and joins its tasks first; it invokes this
    /// hook only for explicit exit, not disconnection.
    /// # Flow
    /// ```text
    /// [valid exit]             [EOF/disconnect]
    ///      |                          |
    ///  [record exit status]           |
    ///      |                          |
    ///      +------------+-------------+
    ///                   v
    ///           [close + join tasks]
    ///                   |
    ///            <explicit exit?>
    ///              /         \
    ///            yes         no
    ///             |           |
    ///         [exit hook]   [skip hook]
    ///             |           |
    ///             +-----+-----+
    ///                   v
    ///            [run result]
    /// ```
    /// # Examples
    /// Requires an implementation and a context supplied by a live wrapper; this may perform
    /// application or client I/O.
    /// ```no_run
    /// use lspally::server::model::*;
    /// async fn example<S: ILanguageServer>(server: &S, context: Context) -> ServerResult<()> {
    ///     server.exit(context, ()).await
    /// }
    /// ```
    fn exit(&self, context: Context, params: ()) -> impl Future<Output = ServerResult<()>> + Send;
    /// Accept changed client configuration settings for implementation-owned policy.
    /// The wrapper only converts arbitrary settings data and forwards it; it does not merge settings,
    /// reload files or rebuild indexes. The implementation chooses validation and application
    /// semantics.
    ///
    /// # Arguments
    /// - `self` `[in,out]`: implementation receiving this call; application state/cache changes are
    /// implementation-owned, not an output buffer.
    /// - `context` `[in]`: owned client handle, cancellation signal and negotiated coordinate unit for
    /// this call; see the [dispatch contract](ILanguageServer#dispatch-and-effects).
    /// - `params` `[in]`: application-defined settings tree, which may be null or another allowed
    /// arbitrary-data shape.
    /// # Returns
    /// A sendable future yielding `Ok(())` after processing or scheduling the update; `Err(error)` on
    /// rejection/failure. No reply is sent.
    /// No direct output parameters are overwritten or appended; the returned value carries the result.
    /// # Errors
    /// Implementation input/state/I/O errors, including `ServerError::unsupported()`, are logged to
    /// stderr by the worker; no notification response is sent. State already changed by the
    /// implementation is not rolled back.
    /// # Examples
    /// Requires an implementation and a context supplied by a live wrapper; this may perform
    /// application or client I/O.
    /// ```no_run
    /// use lspally::server::model::*;
    /// async fn example<S: ILanguageServer>(server: &S, context: Context) -> ServerResult<()> {
    ///     let params = DidChangeConfigurationParams { settings: Data::Object(Default::default()) };
    ///     server.did_change_configuration(context, params).await
    /// }
    /// ```
    fn did_change_configuration(
        &self,
        context: Context,
        params: DidChangeConfigurationParams,
    ) -> impl Future<Output = ServerResult<()>> + Send;
    /// Accept the client's initial text snapshot for an open document.
    /// The implementation owns open-text/version tracking and any analysis scheduling. The wrapper
    /// does not read the URI from disk or automatically install the snapshot.
    ///
    /// # Arguments
    /// - `self` `[in,out]`: implementation receiving this call; application state/cache changes are
    /// implementation-owned, not an output buffer.
    /// - `context` `[in]`: owned client handle, cancellation signal and negotiated coordinate unit for
    /// this call; see the [dispatch contract](ILanguageServer#dispatch-and-effects).
    /// - `params` `[in]`: URI, language ID, client version and full text; this text can differ from
    /// disk.
    /// # Returns
    /// A sendable future yielding `Ok(())` after accepting or scheduling the snapshot; `Err(error)` on
    /// failure. No reply is sent.
    /// No direct output parameters are overwritten or appended; the returned value carries the result.
    /// # Errors
    /// Implementation input/state/I/O errors, including `ServerError::unsupported()`, are logged to
    /// stderr by the worker; no notification response is sent. State already changed by the
    /// implementation is not rolled back.
    /// # Examples
    /// Requires an implementation and a context supplied by a live wrapper; this may perform
    /// application or client I/O.
    /// ```no_run
    /// use lspally::server::model::*;
    /// async fn example<S: ILanguageServer>(server: &S, context: Context) -> ServerResult<()> {
    ///     let params = DidOpenTextDocumentParams { text_document: TextDocumentItem { uri: Uri("file:///tmp/CMakeLists.txt".into()), language_id: "cmake".into(), version: 1, text: "project(example)\n".into() } };
    ///     server.did_open_text_document(context, params).await
    /// }
    /// ```
    fn did_open_text_document(
        &self,
        context: Context,
        params: DidOpenTextDocumentParams,
    ) -> impl Future<Output = ServerResult<()>> + Send;
    /// Accept an ordered batch of changes to a synchronized document.
    /// The implementation updates its own text/version state and may schedule analysis. The wrapper
    /// neither stores document text nor converts ranges to byte offsets.
    ///
    /// # Arguments
    /// - `self` `[in,out]`: implementation receiving this call; application state/cache changes are
    /// implementation-owned, not an output buffer.
    /// - `context` `[in]`: owned client handle, cancellation signal and negotiated coordinate unit for
    /// this call; see the [dispatch contract](ILanguageServer#dispatch-and-effects).
    /// - `params` `[in]`: document/version after the batch and ordered changes; absent range replaces
    /// the entire text, present range replaces a span.
    /// # Returns
    /// A sendable future yielding `Ok(())` after processing or scheduling the batch; `Err(error)` on
    /// invalid versions/ranges or implementation failure. No reply is sent.
    /// No direct output parameters are overwritten or appended; the returned value carries the result.
    /// # Errors
    /// Implementation input/state/I/O errors, including `ServerError::unsupported()`, are logged to
    /// stderr by the worker; no notification response is sent. State already changed by the
    /// implementation is not rolled back.
    /// # Algorithm
    /// Interpret each range using `context.encoding` against the text resulting from preceding changes
    /// in the batch, not one shared old snapshot. Track the supplied version as the resulting version.
    /// Validation, atomicity and recovery after a rejected change are implementation responsibilities.
    /// # Flow
    /// ```text
    /// [stored text]       [ordered change batch]
    ///       |                       |
    ///       +-----------+-----------+
    ///                   v
    ///            <next change?> <---------+
    ///              /       \              |
    ///            no        yes            |
    ///             |         |             |
    ///      [record version] <range?>      |
    ///                     /        \      |
    ///                  absent    present  |
    ///                     |         |     |
    ///                [replace] [patch span]
    ///                     |         |     |
    ///                     +---------+-----+
    /// ```
    /// # Examples
    /// Requires an implementation and a context supplied by a live wrapper; this may perform
    /// application or client I/O.
    /// ```no_run
    /// use lspally::server::model::*;
    /// async fn example<S: ILanguageServer>(server: &S, context: Context) -> ServerResult<()> {
    ///     let params = DidChangeTextDocumentParams { text_document: VersionedTextDocumentIdentifier { uri: Uri("file:///tmp/CMakeLists.txt".into()), version: 2 }, content_changes: vec![TextDocumentContentChangeEvent { range: None, range_length: None, text: "project(renamed)\n".into() }] };
    ///     server.did_change_text_document(context, params).await
    /// }
    /// ```
    fn did_change_text_document(
        &self,
        context: Context,
        params: DidChangeTextDocumentParams,
    ) -> impl Future<Output = ServerResult<()>> + Send;
    /// Observe that the client is about to save a synchronized document.
    /// This notification cannot return save edits. Use [`Self::will_save_wait_until`] when negotiated
    /// pre-save edits are needed; saving remains a client operation.
    ///
    /// # Arguments
    /// - `self` `[in,out]`: implementation receiving this call; application state/cache changes are
    /// implementation-owned, not an output buffer.
    /// - `context` `[in]`: owned client handle, cancellation signal and negotiated coordinate unit for
    /// this call; see the [dispatch contract](ILanguageServer#dispatch-and-effects).
    /// - `params` `[in]`: document identity and manual/after-delay/focus-out save reason.
    /// # Returns
    /// A sendable future yielding `Ok(())` after application pre-save bookkeeping; `Err(error)` on
    /// hook failure. No reply is sent.
    /// No direct output parameters are overwritten or appended; the returned value carries the result.
    /// # Errors
    /// Implementation input/state/I/O errors, including `ServerError::unsupported()`, are logged to
    /// stderr by the worker; no notification response is sent. State already changed by the
    /// implementation is not rolled back.
    /// # Examples
    /// Requires an implementation and a context supplied by a live wrapper; this may perform
    /// application or client I/O.
    /// ```no_run
    /// use lspally::server::model::*;
    /// async fn example<S: ILanguageServer>(server: &S, context: Context, document: TextDocumentIdentifier) -> ServerResult<()> {
    ///     let params = WillSaveTextDocumentParams { text_document: document, reason: TextDocumentSaveReason(1) };
    ///     server.will_save_text_document(context, params).await
    /// }
    /// ```
    fn will_save_text_document(
        &self,
        context: Context,
        params: WillSaveTextDocumentParams,
    ) -> impl Future<Output = ServerResult<()>> + Send;
    /// Release client-open document state after the client closes it.
    /// The implementation decides when to release overlays, revert to disk-backed state or clear
    /// diagnostics. The wrapper does not discard text or publish an empty report automatically.
    ///
    /// # Arguments
    /// - `self` `[in,out]`: implementation receiving this call; application state/cache changes are
    /// implementation-owned, not an output buffer.
    /// - `context` `[in]`: owned client handle, cancellation signal and negotiated coordinate unit for
    /// this call; see the [dispatch contract](ILanguageServer#dispatch-and-effects).
    /// - `params` `[in]`: identity of the document whose client-open lifetime ended.
    /// # Returns
    /// A sendable future yielding `Ok(())` after processing or scheduling closure; `Err(error)` on
    /// hook failure. No reply is sent.
    /// No direct output parameters are overwritten or appended; the returned value carries the result.
    /// # Errors
    /// Implementation input/state/I/O errors, including `ServerError::unsupported()`, are logged to
    /// stderr by the worker; no notification response is sent. State already changed by the
    /// implementation is not rolled back.
    /// # Examples
    /// Requires an implementation and a context supplied by a live wrapper; this may perform
    /// application or client I/O.
    /// ```no_run
    /// use lspally::server::model::*;
    /// async fn example<S: ILanguageServer>(server: &S, context: Context, document: TextDocumentIdentifier) -> ServerResult<()> {
    ///     server.did_close_text_document(context, DidCloseTextDocumentParams { text_document: document }).await
    /// }
    /// ```
    fn did_close_text_document(
        &self,
        context: Context,
        params: DidCloseTextDocumentParams,
    ) -> impl Future<Output = ServerResult<()>> + Send;
    /// Observe a completed client save and optionally its full saved text.
    /// The implementation may refresh analysis or schedule authorized save work; the wrapper does not
    /// write the file. Text inclusion follows negotiated save options.
    ///
    /// # Arguments
    /// - `self` `[in,out]`: implementation receiving this call; application state/cache changes are
    /// implementation-owned, not an output buffer.
    /// - `context` `[in]`: owned client handle, cancellation signal and negotiated coordinate unit for
    /// this call; see the [dispatch contract](ILanguageServer#dispatch-and-effects).
    /// - `params` `[in]`: saved document identity and optional full text, not an incremental patch.
    /// # Returns
    /// A sendable future yielding `Ok(())` after save-event processing or scheduling; `Err(error)` on
    /// hook failure. No reply is sent.
    /// No direct output parameters are overwritten or appended; the returned value carries the result.
    /// # Errors
    /// Implementation input/state/I/O errors, including `ServerError::unsupported()`, are logged to
    /// stderr by the worker; no notification response is sent. State already changed by the
    /// implementation is not rolled back.
    /// # Examples
    /// Requires an implementation and a context supplied by a live wrapper; this may perform
    /// application or client I/O.
    /// ```no_run
    /// use lspally::server::model::*;
    /// async fn example<S: ILanguageServer>(server: &S, context: Context, document: TextDocumentIdentifier) -> ServerResult<()> {
    ///     server.did_save_text_document(context, DidSaveTextDocumentParams { text_document: document, text: None }).await
    /// }
    /// ```
    fn did_save_text_document(
        &self,
        context: Context,
        params: DidSaveTextDocumentParams,
    ) -> impl Future<Output = ServerResult<()>> + Send;
    /// Accept an initial notebook snapshot and its synchronized cell-text documents.
    /// The implementation owns notebook/cell identity, metadata, version and text state. The wrapper
    /// only converts the payload; it does not open notebook files or run cells.
    ///
    /// # Arguments
    /// - `self` `[in,out]`: implementation receiving this call; application state/cache changes are
    /// implementation-owned, not an output buffer.
    /// - `context` `[in]`: owned client handle, cancellation signal and negotiated coordinate unit for
    /// this call; see the [dispatch contract](ILanguageServer#dispatch-and-effects).
    /// - `params` `[in]`: complete notebook structure plus full text snapshots for synchronized cells.
    /// # Returns
    /// A sendable future yielding `Ok(())` after accepting or scheduling notebook initialization;
    /// `Err(error)` on failure. No reply is sent.
    /// No direct output parameters are overwritten or appended; the returned value carries the result.
    /// # Errors
    /// Implementation input/state/I/O errors, including `ServerError::unsupported()`, are logged to
    /// stderr by the worker; no notification response is sent. State already changed by the
    /// implementation is not rolled back.
    /// # Examples
    /// Requires an implementation and a context supplied by a live wrapper; this may perform
    /// application or client I/O.
    /// ```no_run
    /// use lspally::server::model::*;
    /// async fn example<S: ILanguageServer>(server: &S, context: Context, notebook: NotebookDocument, cells: Vec<TextDocumentItem>) -> ServerResult<()> {
    ///     let params = DidOpenNotebookDocumentParams { notebook_document: notebook, cell_text_documents: cells };
    ///     server.did_open_notebook_document(context, params).await
    /// }
    /// ```
    fn did_open_notebook_document(
        &self,
        context: Context,
        params: DidOpenNotebookDocumentParams,
    ) -> impl Future<Output = ServerResult<()>> + Send;
    /// Accept notebook metadata, cell-structure, cell-data and cell-text changes.
    /// The implementation maintains the notebook and affected cell documents; the wrapper neither
    /// applies the changes nor executes notebook code.
    ///
    /// # Arguments
    /// - `self` `[in,out]`: implementation receiving this call; application state/cache changes are
    /// implementation-owned, not an output buffer.
    /// - `context` `[in]`: owned client handle, cancellation signal and negotiated coordinate unit for
    /// this call; see the [dispatch contract](ILanguageServer#dispatch-and-effects).
    /// - `params` `[in]`: notebook identity/resulting version and change event; structural changes can
    /// open/close cell text documents, and text changes carry their own document versions.
    /// # Returns
    /// A sendable future yielding `Ok(())` after processing or scheduling the notebook change;
    /// `Err(error)` on invalid/stale changes or implementation failure. No reply is sent.
    /// No direct output parameters are overwritten or appended; the returned value carries the result.
    /// # Errors
    /// Implementation input/state/I/O errors, including `ServerError::unsupported()`, are logged to
    /// stderr by the worker; no notification response is sent. State already changed by the
    /// implementation is not rolled back.
    /// # Examples
    /// Requires an implementation and a context supplied by a live wrapper; this may perform
    /// application or client I/O.
    /// ```no_run
    /// use lspally::server::model::*;
    /// async fn example<S: ILanguageServer>(server: &S, context: Context, notebook: VersionedNotebookDocumentIdentifier, change: NotebookDocumentChangeEvent) -> ServerResult<()> {
    ///     let params = DidChangeNotebookDocumentParams { notebook_document: notebook, change };
    ///     server.did_change_notebook_document(context, params).await
    /// }
    /// ```
    fn did_change_notebook_document(
        &self,
        context: Context,
        params: DidChangeNotebookDocumentParams,
    ) -> impl Future<Output = ServerResult<()>> + Send;
    /// Observe that the client saved a notebook.
    /// Only the notebook identity is supplied; retrieve maintained notebook/cell state as needed. The
    /// wrapper does not save cells or execute them.
    ///
    /// # Arguments
    /// - `self` `[in,out]`: implementation receiving this call; application state/cache changes are
    /// implementation-owned, not an output buffer.
    /// - `context` `[in]`: owned client handle, cancellation signal and negotiated coordinate unit for
    /// this call; see the [dispatch contract](ILanguageServer#dispatch-and-effects).
    /// - `params` `[in]`: saved notebook URI, without a new text snapshot or save reason.
    /// # Returns
    /// A sendable future yielding `Ok(())` after handling or scheduling the save event; `Err(error)`
    /// on hook failure. No reply is sent.
    /// No direct output parameters are overwritten or appended; the returned value carries the result.
    /// # Errors
    /// Implementation input/state/I/O errors, including `ServerError::unsupported()`, are logged to
    /// stderr by the worker; no notification response is sent. State already changed by the
    /// implementation is not rolled back.
    /// # Examples
    /// Requires an implementation and a context supplied by a live wrapper; this may perform
    /// application or client I/O.
    /// ```no_run
    /// use lspally::server::model::*;
    /// async fn example<S: ILanguageServer>(server: &S, context: Context, notebook: NotebookDocumentIdentifier) -> ServerResult<()> {
    ///     server.did_save_notebook_document(context, DidSaveNotebookDocumentParams { notebook_document: notebook }).await
    /// }
    /// ```
    fn did_save_notebook_document(
        &self,
        context: Context,
        params: DidSaveNotebookDocumentParams,
    ) -> impl Future<Output = ServerResult<()>> + Send;
    /// Release a notebook and the cell documents whose synchronized lifetimes ended.
    /// The implementation owns notebook/cell cleanup. The wrapper does not delete resources, close
    /// editor tabs or automatically clear cell diagnostics.
    ///
    /// # Arguments
    /// - `self` `[in,out]`: implementation receiving this call; application state/cache changes are
    /// implementation-owned, not an output buffer.
    /// - `context` `[in]`: owned client handle, cancellation signal and negotiated coordinate unit for
    /// this call; see the [dispatch contract](ILanguageServer#dispatch-and-effects).
    /// - `params` `[in]`: closed notebook identity and the list of cell-text document identities to
    /// close.
    /// # Returns
    /// A sendable future yielding `Ok(())` after processing or scheduling closure; `Err(error)` on
    /// hook failure. No reply is sent.
    /// No direct output parameters are overwritten or appended; the returned value carries the result.
    /// # Errors
    /// Implementation input/state/I/O errors, including `ServerError::unsupported()`, are logged to
    /// stderr by the worker; no notification response is sent. State already changed by the
    /// implementation is not rolled back.
    /// # Examples
    /// Requires an implementation and a context supplied by a live wrapper; this may perform
    /// application or client I/O.
    /// ```no_run
    /// use lspally::server::model::*;
    /// async fn example<S: ILanguageServer>(server: &S, context: Context, notebook: NotebookDocumentIdentifier, cells: Vec<TextDocumentIdentifier>) -> ServerResult<()> {
    ///     let params = DidCloseNotebookDocumentParams { notebook_document: notebook, cell_text_documents: cells };
    ///     server.did_close_notebook_document(context, params).await
    /// }
    /// ```
    fn did_close_notebook_document(
        &self,
        context: Context,
        params: DidCloseNotebookDocumentParams,
    ) -> impl Future<Output = ServerResult<()>> + Send;
    /// Accept client-reported create/change/delete events for watched resources.
    /// The implementation decides which disk state or caches to refresh. The wrapper does not install
    /// a watcher, read these resources or apply file operations.
    ///
    /// # Arguments
    /// - `self` `[in,out]`: implementation receiving this call; application state/cache changes are
    /// implementation-owned, not an output buffer.
    /// - `context` `[in]`: owned client handle, cancellation signal and negotiated coordinate unit for
    /// this call; see the [dispatch contract](ILanguageServer#dispatch-and-effects).
    /// - `params` `[in]`: ordered URI/change-kind records supplied by the client's watcher.
    /// # Returns
    /// A sendable future yielding `Ok(())` after processing or scheduling invalidation; `Err(error)`
    /// on hook failure. No reply is sent.
    /// No direct output parameters are overwritten or appended; the returned value carries the result.
    /// # Errors
    /// Implementation input/state/I/O errors, including `ServerError::unsupported()`, are logged to
    /// stderr by the worker; no notification response is sent. State already changed by the
    /// implementation is not rolled back.
    /// # Examples
    /// Requires an implementation and a context supplied by a live wrapper; this may perform
    /// application or client I/O.
    /// ```no_run
    /// use lspally::server::model::*;
    /// async fn example<S: ILanguageServer>(server: &S, context: Context) -> ServerResult<()> {
    ///     let params = DidChangeWatchedFilesParams { changes: vec![FileEvent { uri: Uri("file:///tmp/module.cmake".into()), typ: FileChangeType(2) }] };
    ///     server.did_change_watched_files(context, params).await
    /// }
    /// ```
    fn did_change_watched_files(
        &self,
        context: Context,
        params: DidChangeWatchedFilesParams,
    ) -> impl Future<Output = ServerResult<()>> + Send;
    /// Accept additions and removals to the client's workspace-folder set.
    /// The implementation owns root admission, unloading and index updates. The wrapper does not scan
    /// folders or change the server's configured roots automatically.
    ///
    /// # Arguments
    /// - `self` `[in,out]`: implementation receiving this call; application state/cache changes are
    /// implementation-owned, not an output buffer.
    /// - `context` `[in]`: owned client handle, cancellation signal and negotiated coordinate unit for
    /// this call; see the [dispatch contract](ILanguageServer#dispatch-and-effects).
    /// - `params` `[in]`: folder records added to and removed from the workspace, including URI and
    /// display name.
    /// # Returns
    /// A sendable future yielding `Ok(())` after processing or scheduling the folder update;
    /// `Err(error)` on rejection/failure. No reply is sent.
    /// No direct output parameters are overwritten or appended; the returned value carries the result.
    /// # Errors
    /// Implementation input/state/I/O errors, including `ServerError::unsupported()`, are logged to
    /// stderr by the worker; no notification response is sent. State already changed by the
    /// implementation is not rolled back.
    /// # Examples
    /// Requires an implementation and a context supplied by a live wrapper; this may perform
    /// application or client I/O.
    /// ```no_run
    /// use lspally::server::model::*;
    /// async fn example<S: ILanguageServer>(server: &S, context: Context) -> ServerResult<()> {
    ///     let params = DidChangeWorkspaceFoldersParams { event: WorkspaceFoldersChangeEvent { added: vec![WorkspaceFolder { uri: Uri("file:///tmp/example".into()), name: "example".into() }], removed: vec![] } };
    ///     server.did_change_workspace_folders(context, params).await
    /// }
    /// ```
    fn did_change_workspace_folders(
        &self,
        context: Context,
        params: DidChangeWorkspaceFoldersParams,
    ) -> impl Future<Output = ServerResult<()>> + Send;
    /// Handle a client request to cancel work identified by a progress token.
    /// Unlike [`Self::cancel`], the wrapper does not track a token-to-work mapping or cancel that work
    /// automatically. The implementation decides whether the work is still active and cancellable.
    ///
    /// # Arguments
    /// - `self` `[in,out]`: implementation receiving this call; application state/cache changes are
    /// implementation-owned, not an output buffer.
    /// - `context` `[in]`: owned client handle, cancellation signal and negotiated coordinate unit for
    /// this call; see the [dispatch contract](ILanguageServer#dispatch-and-effects).
    /// - `params` `[in]`: numeric or string work-done progress token, not an incoming request ID.
    /// # Returns
    /// A sendable future yielding `Ok(())` after handling the token, including a
    /// stale/already-finished token; `Err(error)` on hook failure. No reply is sent.
    /// No direct output parameters are overwritten or appended; the returned value carries the result.
    /// # Errors
    /// Implementation input/state/I/O errors, including `ServerError::unsupported()`, are logged to
    /// stderr by the worker; no notification response is sent. State already changed by the
    /// implementation is not rolled back.
    /// # Examples
    /// Requires an implementation and a context supplied by a live wrapper; this may perform
    /// application or client I/O.
    /// ```no_run
    /// use lspally::server::model::*;
    /// async fn example<S: ILanguageServer>(server: &S, context: Context) -> ServerResult<()> {
    ///     let params = WorkDoneProgressCancelParams { token: NumberOrString::String("indexing".into()) };
    ///     server.work_done_progress_cancel(context, params).await
    /// }
    /// ```
    fn work_done_progress_cancel(
        &self,
        context: Context,
        params: WorkDoneProgressCancelParams,
    ) -> impl Future<Output = ServerResult<()>> + Send;
    /// Update application state after the client has created resources.
    /// The operation has already occurred. The implementation may update indexes and identity
    /// mappings; this wrapper does not repeat the file operation.
    ///
    /// # Arguments
    /// - `self` `[in,out]`: implementation receiving this call; application state/cache changes are
    /// implementation-owned, not an output buffer.
    /// - `context` `[in]`: owned client handle, cancellation signal and negotiated coordinate unit for
    /// this call; see the [dispatch contract](ILanguageServer#dispatch-and-effects).
    /// - `params` `[in]`: created resource URIs; file-operation URI fields are strings.
    /// # Returns
    /// A sendable future yielding `Ok(())` after processing or scheduling the update; `Err(error)` on
    /// hook failure. No reply is sent.
    /// No direct output parameters are overwritten or appended; the returned value carries the result.
    /// # Errors
    /// Implementation input/state/I/O errors, including `ServerError::unsupported()`, are logged to
    /// stderr by the worker; no notification response is sent. State already changed by the
    /// implementation is not rolled back.
    /// # Examples
    /// Requires an implementation and a context supplied by a live wrapper; this may perform
    /// application or client I/O.
    /// ```no_run
    /// use lspally::server::model::*;
    /// async fn example<S: ILanguageServer>(server: &S, context: Context) -> ServerResult<()> {
    ///     let params = CreateFilesParams { files: vec![FileCreate { uri: "file:///tmp/new.cmake".into() }] };
    ///     server.did_create_files(context, params).await
    /// }
    /// ```
    fn did_create_files(
        &self,
        context: Context,
        params: CreateFilesParams,
    ) -> impl Future<Output = ServerResult<()>> + Send;
    /// Update application state after the client has renamed resources.
    /// The operation has already occurred. The implementation may update indexes and identity
    /// mappings; this wrapper does not repeat the file operation.
    ///
    /// # Arguments
    /// - `self` `[in,out]`: implementation receiving this call; application state/cache changes are
    /// implementation-owned, not an output buffer.
    /// - `context` `[in]`: owned client handle, cancellation signal and negotiated coordinate unit for
    /// this call; see the [dispatch contract](ILanguageServer#dispatch-and-effects).
    /// - `params` `[in]`: old/new URI pairs of completed renames; file-operation URI fields are
    /// strings.
    /// # Returns
    /// A sendable future yielding `Ok(())` after processing or scheduling the update; `Err(error)` on
    /// hook failure. No reply is sent.
    /// No direct output parameters are overwritten or appended; the returned value carries the result.
    /// # Errors
    /// Implementation input/state/I/O errors, including `ServerError::unsupported()`, are logged to
    /// stderr by the worker; no notification response is sent. State already changed by the
    /// implementation is not rolled back.
    /// # Examples
    /// Requires an implementation and a context supplied by a live wrapper; this may perform
    /// application or client I/O.
    /// ```no_run
    /// use lspally::server::model::*;
    /// async fn example<S: ILanguageServer>(server: &S, context: Context) -> ServerResult<()> {
    ///     let params = RenameFilesParams { files: vec![FileRename { old_uri: "file:///tmp/old.cmake".into(), new_uri: "file:///tmp/new.cmake".into() }] };
    ///     server.did_rename_files(context, params).await
    /// }
    /// ```
    fn did_rename_files(
        &self,
        context: Context,
        params: RenameFilesParams,
    ) -> impl Future<Output = ServerResult<()>> + Send;
    /// Update application state after the client has deleted resources.
    /// The operation has already occurred. The implementation may update indexes and identity
    /// mappings; this wrapper does not repeat the file operation.
    ///
    /// # Arguments
    /// - `self` `[in,out]`: implementation receiving this call; application state/cache changes are
    /// implementation-owned, not an output buffer.
    /// - `context` `[in]`: owned client handle, cancellation signal and negotiated coordinate unit for
    /// this call; see the [dispatch contract](ILanguageServer#dispatch-and-effects).
    /// - `params` `[in]`: deleted resource URIs; file-operation URI fields are strings.
    /// # Returns
    /// A sendable future yielding `Ok(())` after processing or scheduling the update; `Err(error)` on
    /// hook failure. No reply is sent.
    /// No direct output parameters are overwritten or appended; the returned value carries the result.
    /// # Errors
    /// Implementation input/state/I/O errors, including `ServerError::unsupported()`, are logged to
    /// stderr by the worker; no notification response is sent. State already changed by the
    /// implementation is not rolled back.
    /// # Examples
    /// Requires an implementation and a context supplied by a live wrapper; this may perform
    /// application or client I/O.
    /// ```no_run
    /// use lspally::server::model::*;
    /// async fn example<S: ILanguageServer>(server: &S, context: Context) -> ServerResult<()> {
    ///     let params = DeleteFilesParams { files: vec![FileDelete { uri: "file:///tmp/old.cmake".into() }] };
    ///     server.did_delete_files(context, params).await
    /// }
    /// ```
    fn did_delete_files(
        &self,
        context: Context,
        params: DeleteFilesParams,
    ) -> impl Future<Output = ServerResult<()>> + Send;

    /// Handle an unknown, non-reserved request using raw extension JSON.
    /// Known standard methods are routed to their typed hooks, even when parameters are invalid.
    /// Reserved `$/` and known server-to-client request names are rejected before this hook. Extension
    /// validation, authorization and side effects belong to the implementation.
    ///
    /// # Arguments
    /// - `self` `[in,out]`: implementation receiving this call; application state/cache changes are
    /// implementation-owned, not an output buffer.
    /// - `context` `[in]`: owned client handle, cancellation signal and negotiated coordinate unit for
    /// this call; see the [dispatch contract](ILanguageServer#dispatch-and-effects).
    /// - `params` `[in]`: extension method and optional object/array parameters; the wrapper retains
    /// the RPC ID and envelope.
    /// # Returns
    /// A sendable future yielding `Ok(value)` as the raw JSON response result, including
    /// `Value::Null`; `Err(error)` to return an RPC error with the original request ID.
    /// No direct output parameters are overwritten or appended; the returned value carries the result.
    /// # Errors
    /// Use `ServerError::unsupported()` for an unknown extension and `ServerError::invalid(...)` for
    /// invalid extension parameters. Application failures propagate as RPC errors; partial extension
    /// side effects are not rolled back.
    /// # Examples
    /// Requires an implementation and a context supplied by a live wrapper; this may perform
    /// application or client I/O.
    /// ```no_run
    /// use lspally::server::model::*;
    /// async fn example<S: ILanguageServer>(server: &S, context: Context) -> ServerResult<serde_json::Value> {
    ///     let params = Extension { method: "example/status".into(), params: Some(serde_json::json!({"verbose": false})) };
    ///     server.custom_request(context, params).await
    /// }
    /// ```
    fn custom_request(
        &self,
        context: Context,
        params: Extension,
    ) -> impl Future<Output = ServerResult<serde_json::Value>> + Send;
    /// Handle an unknown, non-reserved notification using raw extension JSON.
    /// Known standard notifications use typed hooks; unknown reserved `$/` or known server-to-client
    /// notification names are ignored before this hook. Extension-specific state changes and I/O
    /// belong to the implementation.
    ///
    /// # Arguments
    /// - `self` `[in,out]`: implementation receiving this call; application state/cache changes are
    /// implementation-owned, not an output buffer.
    /// - `context` `[in]`: owned client handle, cancellation signal and negotiated coordinate unit for
    /// this call; see the [dispatch contract](ILanguageServer#dispatch-and-effects).
    /// - `params` `[in]`: extension method and optional object/array parameters without an RPC ID.
    /// # Returns
    /// A sendable future yielding `Ok(())` after extension notification handling; `Err(error)` on
    /// rejection/failure. The worker logs errors to stderr and sends no reply.
    /// No direct output parameters are overwritten or appended; the returned value carries the result.
    /// # Errors
    /// Use `ServerError::unsupported()` for an unhandled extension or `ServerError::invalid(...)` for
    /// invalid input. Errors are logged rather than sent as responses, and prior side effects are not
    /// rolled back.
    /// # Examples
    /// Requires an implementation and a context supplied by a live wrapper; this may perform
    /// application or client I/O.
    /// ```no_run
    /// use lspally::server::model::*;
    /// async fn example<S: ILanguageServer>(server: &S, context: Context) -> ServerResult<()> {
    ///     let params = Extension { method: "example/invalidate".into(), params: Some(serde_json::json!({"uri": "file:///tmp/a"})) };
    ///     server.custom_notification(context, params).await
    /// }
    /// ```
    fn custom_notification(
        &self,
        context: Context,
        params: Extension,
    ) -> impl Future<Output = ServerResult<()>> + Send;
}
/// Bounds for one [`Wrapper`] connection; constructing limits performs no I/O.
/// [`Wrapper::with_limits`] accepts overrides. Runtime queue and pending-request
/// capacities are clamped to at least one; byte limits are used as supplied.
///
/// # Examples
/// ```
/// use lspally::server::model::Limits;
/// let limits = Limits { pending_requests: 32, ..Limits::default() };
/// assert_eq!(limits.pending_requests, 32);
/// ```
#[derive(Debug)]
pub struct Limits {
    /// Maximum queued business calls; overflow rejects requests or disconnects on notifications. Example: `limits.input_queue = 64`.
    pub input_queue: usize,
    /// Maximum queued writer packets; not a byte count. Example: `limits.output_queue = 64`.
    pub output_queue: usize,
    /// Maximum simultaneous outgoing requests awaiting peer responses. Example: `limits.pending_requests = 32`.
    pub pending_requests: usize,
    /// Maximum framing-header bytes accepted by the reader. Example: `limits.header_bytes = 8192`.
    pub header_bytes: usize,
    /// Maximum incoming JSON body bytes. Example: `limits.frame_bytes = 1024 * 1024`.
    pub frame_bytes: usize,
}
impl Default for Limits {
    /// Choose bounded queues and framing sizes; no I/O or shared state changes.
    ///
    /// # Arguments
    /// None.
    /// # Returns
    /// Limits with 128 input slots, 128 output slots, 128 pending requests,
    /// 8192 header bytes and 16 MiB body bytes.
    /// # Examples
    /// ```
    /// use lspally::server::model::Limits;
    /// assert_eq!(Limits::default().frame_bytes, 16 * 1024 * 1024);
    /// ```
    fn default() -> Self {
        Self {
            input_queue: 128,
            output_queue: 128,
            pending_requests: 128,
            header_bytes: 8192,
            frame_bytes: 16 * 1024 * 1024,
        }
    }
}
/// Framed LSP host for one [`ILanguageServer`] implementation.
/// Construction stores the server and limits without starting tasks or doing I/O.
/// [`Self::run`] consumes the wrapper and serializes business calls; its independent
/// reader can still receive a response while a handler awaits [`ClientHandle`].
///
/// # Flow
/// ```text
/// [peer input] ---> [reader] ---> [bounded business queue]
///                      |                   |
///                [responses]               v
///                      |          [one handler at a time]
///                      v                   |
///              [pending client calls] <----+
///                                          |
/// [peer output] <--- [writer] <--- [bounded output queue]
/// ```
/// # Examples
/// ```
/// use lspally::server::model::{ILanguageServer, Wrapper};
/// fn host<S: ILanguageServer>(server: S) -> Wrapper<S> { Wrapper::new(server) }
/// ```
pub struct Wrapper<S: ILanguageServer> {
    /// Shared handler implementation. Example within the wrapper: `self.server.clone()`.
    pub(super) server: Arc<S>,
    /// Per-session admission and framing bounds. Example: `self.limits.input_queue`.
    pub(super) limits: Limits,
}
impl<S: ILanguageServer> Wrapper<S> {
    /// Store a server with default limits; does not start transport tasks or I/O.
    ///
    /// # Arguments
    /// - `server` `[in]`: owned business implementation, moved into shared ownership.
    /// # Returns
    /// An unstarted wrapper using [`Limits::default`]; no output parameters.
    /// # Examples
    /// ```
    /// use lspally::server::model::{ILanguageServer, Wrapper};
    /// fn host<S: ILanguageServer>(server: S) -> Wrapper<S> { Wrapper::new(server) }
    /// ```
    pub fn new(server: S) -> Self {
        Self {
            server: Arc::new(server),
            limits: Limits::default(),
        }
    }
    /// Store a server and explicit limits without validating them or performing I/O.
    /// Runtime clamps queue and pending-request counts to at least one.
    ///
    /// # Arguments
    /// - `server` `[in]`: owned business implementation, moved into shared ownership.
    /// - `limits` `[in]`: per-session queue capacities and framing byte limits.
    /// # Returns
    /// An unstarted wrapper retaining the supplied limits; no output parameters.
    /// # Examples
    /// ```
    /// use lspally::server::model::{ILanguageServer, Limits, Wrapper};
    /// fn host<S: ILanguageServer>(server: S) -> Wrapper<S> {
    ///     Wrapper::with_limits(server, Limits { input_queue: 16, ..Limits::default() })
    /// }
    /// ```
    pub fn with_limits(server: S, limits: Limits) -> Self {
        Self {
            server: Arc::new(server),
            limits,
        }
    }
}
/// Cloneable access to the connected client, supplied through [`Context::client`].
/// Cloning shares session state and does no I/O. Outgoing methods use bounded queues
/// and enforce negotiated capabilities and lifecycle restrictions; see
/// [`Self::show_message_request`] for a request and [`Self::publish_diagnostics`]
/// for a replacement notification.
///
/// # Examples
/// ```
/// use lspally::server::model::{ClientHandle, Context};
/// // A running wrapper supplies a context to the business handler.
/// fn retain_client(context: &Context) -> ClientHandle { context.client.clone() }
/// ```
#[derive(Clone)]
pub struct ClientHandle {
    /// Shared connection state, not a new connection. Example in this crate: `client.session.clone()`.
    pub(super) session: Arc<Session>,
}
/// Why [`Wrapper::run`] stopped a connection; constructing a value has no side effects.
/// I/O and task failures use the run method's error return instead.
///
/// # Examples
/// ```
/// use lspally::server::model::ExitStatus;
/// let status = ExitStatus::Graceful;
/// assert_ne!(status, ExitStatus::Disconnected);
/// ```
#[derive(Debug, PartialEq, Eq)]
pub enum ExitStatus {
    /// Valid `exit` after a shutdown request was admitted.
    /// Example: `matches!(status, ExitStatus::Graceful)`.
    Graceful,
    /// Valid `exit` without a preceding admitted shutdown request.
    /// Example: `matches!(status, ExitStatus::Abnormal)`.
    Abnormal,
    /// EOF or a closed session without an explicit `exit`.
    /// Example: `matches!(status, ExitStatus::Disconnected)`.
    Disconnected,
}
/// One queued message and its write/flush acknowledgement; no I/O on construction.
/// The transport writer consumes packets from [`Session::writer`].
///
/// # Examples
/// ```text
/// let (done, received) = oneshot::channel();
/// let packet = WritePacket { message, done };
/// session.writer.send(packet).await?;
/// let flushed = received.await?;
/// ```
pub(super) struct WritePacket {
    /// Complete RPC envelope to serialize. Example: `matches!(packet.message, crate::rpc::Message::Response(_))`.
    pub message: crate::rpc::Message,
    /// Sender completed after flush, or with a textual write failure.
    /// Example: `let _ = packet.done.send(Ok(()));`.
    pub done: oneshot::Sender<Result<(), String>>,
}
/// Reader-admitted business call in arrival order; responses bypass this queue.
/// Construction has no I/O. The worker dispatches these calls to [`ILanguageServer`].
///
/// # Examples
/// ```text
/// let queued = Inbound::Request(request, Cancellation::default());
/// ```
pub(super) enum Inbound {
    /// Request with a registered cancellation signal.
    /// Example: `Inbound::Request(request, Cancellation::default())`.
    Request(
        /// RPC envelope including correlation ID. Example: `Inbound::Request(request, token)`.
        crate::rpc::Request,
        /// Signal shared with incoming-request bookkeeping. Example: `Inbound::Request(request, token.clone())`.
        Cancellation,
    ),
    /// Notification without a response channel. Example: `Inbound::Notification(notification)`.
    Notification(
        /// RPC notification envelope. Example: `Inbound::Notification(notification)`.
        crate::rpc::Notification,
    ),
}
/// Connection state shared by the reader, writer, worker and [`ClientHandle`] clones.
/// Construction does not start I/O. Atomics hold small lifecycle choices; maps and
/// sets have short bookkeeping locks, not locks spanning business-handler awaits.
/// The incoming map enables peer cancellation; the outgoing map correlates replies.
///
/// # Flow
/// ```text
/// [reader] ---- responses ----> [outgoing waiters] <---- [client calls]
///    |                                |                      |
/// requests/cancel                     |                   [writer]
///    v                                |                      |
/// [incoming tokens] <--- [worker]      +---- close() ----------+
///    |                                                       |
///    +---------------- [closed cancellation] ----------------+
/// ```
/// # Examples
/// ```text
/// let client = ClientHandle { session: session.clone() };
/// assert!(!client.session.closed.is_cancelled());
/// ```
pub(super) struct Session {
    /// Bounded queue to the sole transport writer. Example: `session.writer.capacity()`.
    pub writer: mpsc::Sender<WritePacket>,
    /// Lifecycle phase number from `wrapper::lifecycle` (NEW through SHUT_DOWN). Example: `session.phase.load(Ordering::Acquire)`.
    pub phase: AtomicU8,
    /// Shutdown admission flag; set before its queued handler runs. Example: `session.shutdown_requested.load(Ordering::Acquire)`.
    pub shutdown_requested: AtomicBool,
    /// Trace mode: 0 off, 1 messages, 2 verbose. Example: `session.trace.store(1, Ordering::Release)`.
    pub trace: AtomicU8,
    /// Negotiated coordinate unit: 0 UTF-8, 1 UTF-16, 2 UTF-32. Example: `session.encoding.load(Ordering::Acquire)`.
    pub encoding: AtomicU8,
    /// Next outgoing integer ID; never wraps. Example: `session.next_id.load(Ordering::Acquire)`.
    pub next_id: AtomicI32,
    /// Pending outgoing IDs and response senders; removed on response, abandonment or shutdown. Example: `session.outgoing.lock().unwrap().len()`.
    pub outgoing:
        std::sync::Mutex<HashMap<crate::rpc::RequestId, oneshot::Sender<crate::rpc::Response>>>,
    /// Active incoming IDs and shared cancellation signals. Example: `session.incoming.lock().await.len()`.
    pub incoming: Mutex<HashMap<crate::rpc::RequestId, Cancellation>>,
    /// Outgoing request methods accepted by the client capabilities. Example: `session.supported.lock().await.contains("workspace/applyEdit")`.
    pub supported: Mutex<HashSet<&'static str>>,
    /// Methods the client permits for dynamic registration. Example: `session.registerable.lock().await.contains("textDocument/completion")`.
    pub registerable: Mutex<HashSet<&'static str>>,
    /// Only progress token allowed before RUNNING, copied from initialize. Example: `session.initialization_progress.lock().await.is_some()`.
    pub initialization_progress: Mutex<Option<lsp_types::ProgressToken>>,
    /// Client position encodings plus mandatory UTF-16. Example: `session.offered_encodings.lock().await.contains(&"utf-16".into())`.
    pub offered_encodings: Mutex<Vec<String>>,
    /// Persistent session stop signal shared by all transport tasks. Example: `session.closed.is_cancelled()`.
    pub closed: Cancellation,
    /// Maximum pending outgoing calls after runtime clamping. Example: `session.pending_limit == 128`.
    pub pending_limit: usize,
}
