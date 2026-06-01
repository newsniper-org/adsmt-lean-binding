-- adsmt-lean-binding — Lean 4 entry point.
--
-- Re-exports the verdict types + tactic surface for downstream
-- Lean projects to `import Adsmt` and get the full binding API.

import Adsmt.Verdict
import Adsmt.Solver
import Adsmt.Tactic
