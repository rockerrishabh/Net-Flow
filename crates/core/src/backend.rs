//! Network monitoring backend engine, interface enumeration, throughput computation,
//! latency probing, and diagnostics for Net-Flow.

pub mod adapter;
pub mod diagnostics;
pub mod latency;
pub mod phy;
pub mod rate;
pub mod session;
pub mod state;
pub mod types;

#[cfg(test)]
mod tests;

pub use adapter::*;
pub use diagnostics::*;
pub use latency::*;
pub use phy::*;
pub use rate::*;
pub use session::*;
pub use state::*;
pub use types::*;

/// Convert a null-terminated UTF-16 slice to Rust String.
pub(crate) fn wchar_to_string(slice: &[u16]) -> String {
    let len = slice.iter().position(|&c| c == 0).unwrap_or(slice.len());
    String::from_utf16_lossy(&slice[..len])
}
