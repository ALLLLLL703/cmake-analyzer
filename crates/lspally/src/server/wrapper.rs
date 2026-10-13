// AI-generated implementation.
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
