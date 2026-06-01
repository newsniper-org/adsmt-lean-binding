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

pub mod driver;
pub mod verdict;

pub use verdict::{AbductiveCandidate, AdsmtVerdict};

/// Run an SMT-LIB v2 script and report the verdict for the first
/// `(check-sat)` invocation that fires (or `Unknown { reason:
/// "no check-sat in script" }` if the script ran to completion
/// without one).
///
/// Mirror of `lu-smt` CLI's dispatch loop, run in-process via
/// leo4's canonical ABI — no subprocess fork, no text-shaped
/// wire on the verdict return.
///
/// Parse errors and dispatch-time errors surface as
/// `AdsmtVerdict::Unknown { reason: "<error description>" }`.
/// Engine `unsat` certificates are rendered as S-expression text
/// in the `cert` field; consumers wanting the raw `Certificate`
/// AST should call the adsmt-cert library directly instead of
/// crossing the leo4 boundary.
#[leo4::export]
pub fn run_check_sat(script: String) -> AdsmtVerdict {
    driver::run_check_sat(&script)
}
