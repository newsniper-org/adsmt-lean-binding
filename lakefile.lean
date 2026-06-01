-- Lake workspace for adsmt-lean-binding.
--
-- Pulls in leo4's lake package (provides @[leo4_import] and the
-- canonical-ABI marshalling layer) and exposes the `Adsmt`
-- library containing the Lean-side mirror of the Rust verdict
-- types plus the smt_decide / smt_abduce tactic skeletons.

import Lake
open Lake DSL

package «adsmt-lean-binding» where
  -- nothing here yet; expand as the binding surface grows.

-- leo4 v1.0.0-rc.1 — the lake package side of the binding. The
-- Rust side pins the same tag in Cargo.toml's [workspace.dependencies].
require leo4 from git
  "https://github.com/newsniper-org/leo4.git" @ "v1.0.0-rc.1"

@[default_target]
lean_lib Adsmt where
  -- Lake auto-discovers files under lake/Adsmt/. Roots managed
  -- through Adsmt.lean at lake/.
  roots := #[`Adsmt]
