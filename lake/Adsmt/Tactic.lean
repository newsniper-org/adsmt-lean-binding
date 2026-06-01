-- Tactic surface — smt_decide / smt_abduce skeletons.
--
-- v0.1 (2026-06-01): scaffolding only. The tactic bodies pipe
-- the goal through `runCheckSat` and dispatch on the verdict;
-- a future patch wires this into Lean's elaboration so it can
-- actually close goals (currently the function exists but
-- always returns Unknown — see `crates/adsmt-lean-rt/src/lib.rs`).

import Adsmt.Verdict
import Adsmt.Solver

namespace Adsmt

/-- Compose an SMT-LIB script from a (string-rendered) Lean goal.
   v0.1 stub — real elaboration→SMT-LIB lowering follows once
   the wire shape is fixed. -/
def goalToSmtScript (goal : String) : String :=
  s!"(set-logic ALL)\n(assert (not {goal}))\n(check-sat)\n"

/-- Run adsmt on the goal and report the verdict. v0.1 only
   echoes the verdict to the user; closing the goal lands in a
   follow-up commit. -/
def smtDecideStub (goal : String) : IO AdsmtVerdict := do
  let script := goalToSmtScript goal
  runCheckSat script

end Adsmt
