-- Lean-side imports of the Rust functions exported via
-- `#[leo4::export]` in `crates/adsmt-lean-rt/src/lib.rs`.

import Adsmt.Verdict

namespace Adsmt

/-- Run `(check-sat)` on an SMT-LIB v2 script.
   Mirrors `adsmt_lean_rt::run_check_sat`. -/
@[leo4_import "adsmt_lean_rt::run_check_sat"]
opaque runCheckSat (script : String) : IO AdsmtVerdict

end Adsmt
