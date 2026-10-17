// AI-generated library scaffold.
//! Reusable JSON-RPC envelopes and a bounded, stream-based Language Server Protocol runtime.
//!
//! [`rpc`] owns wire values and framing; [`server`] connects a business implementation
//! to asynchronous input/output. The library does not manage document text or execute CMake.
pub mod rpc;
pub mod server;
