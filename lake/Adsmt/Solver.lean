-- Lean-side imports of the Rust functions exported via
-- `#[leo4::export]` in `crates/adsmt-lean-rt/src/lib.rs`.

import Adsmt.Verdict

namespace Adsmt

/-- Run `(check-sat)` on an SMT-LIB v2 script.
   Mirrors `adsmt_lean_rt::run_check_sat`.

   **v0.1 wire format**: returns a JSON-encoded verdict string.
   Typed `AdsmtVerdict` marshalling is L2 follow-up work — once
   leo4's `LeanMarshal` discipline accepts the inductive shape
   declared in `Verdict.lean`, this signature switches to
   `: IO AdsmtVerdict` directly. -/
@[leo4_import "adsmt_lean_rt::run_check_sat"]
opaque runCheckSat (script : String) : IO String

/-- Parse a v0.1 JSON wire-format verdict into the typed
   `AdsmtVerdict` family. Placeholder until L2 typed
   marshalling lands. -/
def parseVerdictJson (json : String) : AdsmtVerdict :=
  -- v0.1 stub — always reports the raw string as the
  -- Unknown reason. Real JSON dispatching follows.
  .unknown json

end Adsmt
