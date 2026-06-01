-- Tactic surface — `smt_decide` and `smt_abduce`.
--
-- v0.2 (2026-06-01): shape only. The two tactic skeletons sit
-- alongside helper definitions documenting how the binding is
-- intended to be invoked. Full elaboration → SMT-LIB lowering
-- + verdict-driven goal-closing lands once the leo4 reverse-
-- direction wrapper emit pipeline is wired through Lake (D8
-- pattern), at which point this file gains real `@[tactic ...]`
-- attribute macros + a cert-replay path.
--
-- Until then, downstream Lean code uses `smtDecideStub` /
-- `smtAbduceStub` directly from `IO` and pattern-matches on the
-- returned `AdsmtVerdict`.

import Adsmt.Verdict
import Adsmt.Solver

namespace Adsmt

/-- Compose an SMT-LIB v2 script from a stringified Lean goal.
   v0.2 placeholder — assumes the input is already in SMT-LIB
   shape. Real Lean elaboration → SMT-LIB lowering is a separate
   step (handled by the Lake plugin once it's wired). -/
def goalToSmtScript (goal : String) : String :=
  s!"(set-logic ALL)\n(assert (not {goal}))\n(check-sat)\n"

/-- `smtDecideStub`: run adsmt against the SMT-LIB form of a
   Lean goal and report the verdict. Unsat means the negation
   was inconsistent — i.e. the original goal is provable; the
   caller wires this into actual goal-closing via the Lake-
   side elaboration pipeline.

   v0.2 returns the raw verdict; v0.3+ folds it into a real
   `@[tactic]` once leo4's lake wrapper emit lands. -/
def smtDecideStub (goal : String) : IO AdsmtVerdict := do
  let script := goalToSmtScript goal
  runCheckSat script

/-- `smtAbduceStub`: same call shape as `smtDecideStub`, but on
   an Abductive verdict the caller surfaces the candidate
   hypothesis atoms (rendered SMT-LIB text) — these become
   `sorry`-shaped placeholders in real Lean proof scripts.

   Returns a pair `(hypothesis_texts, verdict)`. The first
   component is empty unless the verdict is `Abductive`. -/
def smtAbduceStub (goal : String) : IO (List String × AdsmtVerdict) := do
  let verdict ← smtDecideStub goal
  match verdict with
  | .abductive cands =>
    let hyps := cands.bind (fun c => c.hypothesis)
    return (hyps, verdict)
  | _ => return ([], verdict)

/-- Convenience: human-readable description of a verdict. Used
   by `smtDecideStub` callers to render diagnostic messages
   while the real tactic surface is still under construction. -/
def describeVerdict : AdsmtVerdict → String
  | .sat _model        => "sat (a model exists — goal NOT provable)"
  | .unsat _ _         => "unsat (goal proved)"
  | .abductive cands   => s!"abductive ({cands.length} candidates) — try smt_abduce"
  | .unknown reason    => s!"unknown ({reason})"

end Adsmt
