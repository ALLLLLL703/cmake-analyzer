// AI-generated implementation.
//! Transport, lifecycle and typed dispatch behind [`crate::server::Wrapper`].
//!
//! One reader accepts replies even while the single ordered business worker awaits
//! a client call. One writer serializes all output and acknowledges completed flushes.
//! Input and output queues are bounded; there is no parallel business-query execution.
//!
//! ```text
//! [input stream] --> [reader] ------ replies ------> [pending outgoing waits]
//!                       |
//!                [bounded input queue]
//!                       v
//!                [business worker] <-------------- [client calls]
//!                       |                               |
//!                       +------------+------------------+
//!                                    v
//!                         [bounded output queue]
//!                                    |
//!                                [writer] --> [output stream]
//! ```
mod client;
mod convert;
mod dispatch;
mod handlers;
mod lifecycle;
mod pending;
mod runtime;
mod transport;

// AI-generated external performance-test wiring; adapter visibility remains private.
#[cfg(test)]
#[path = "../../tests/performance/conversion.rs"]
mod conversion_performance;
#[cfg(test)]
#[path = "../../tests/unit/conversion.rs"]
mod conversion_tests;
