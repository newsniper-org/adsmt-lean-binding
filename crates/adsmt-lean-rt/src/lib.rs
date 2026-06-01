//! Rust-side leo4 binding for adsmt.
//!
//! Exposes adsmt's SMT engine to Lean 4 via leo4's canonical ABI
//! (Phase 9 reverse direction). The Lean side imports the symbols
//! exported here through `@[leo4_import]` declarations matching the
//! function signatures.
//!
//! Design: see `docs/thoughts/adsmt-leo4-integration.md` §4-A in the
//! adsmt main repo. This crate is the L1 implementation slot
//! (forward direction binding replacing `lu-smt` subprocess +
//! `adsmt-ffi` C ABI calls from Lean tactic code).
//!
//! Status (2026-06-01): v0.1 skeleton. The engine call site is
//! stubbed out; the binding surface + verdict marshalling are
//! the priority. Real engine wiring follows once the
//! `AdsmtVerdict` shape stabilises on the Lean side.

pub mod verdict;

pub use verdict::{AbductiveCandidate, AdsmtVerdict};

/// Run `(check-sat)` on an SMT-LIB v2 script.
///
/// Returns a typed [`AdsmtVerdict`] capturing one of the four
/// adsmt verdict shapes (sat / unsat / abductive / unknown).
///
/// Wire-equivalent to invoking the `lu-smt` CLI binary on the
/// same input, but without the subprocess fork — the engine
/// runs in-process via leo4's canonical ABI.
#[leo4::export]
pub fn run_check_sat(script: String) -> AdsmtVerdict {
    // v0.1 skeleton: stub verdict. Real engine wiring lands once
    // the adsmt testing channel's `adsmt-engine::Solver` exposes
    // the unified `check_sat(script_text)` entry point this
    // binding wants to call.
    let _ = script;
    AdsmtVerdict::Unknown {
        reason: "adsmt-lean-rt v0.1 skeleton — engine wiring pending".to_string(),
    }
}
