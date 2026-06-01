-- Verdict mirror of `crates/adsmt-lean-rt/src/verdict.rs`.
--
-- The two definitions must round-trip byte-identically through
-- leo4's canonical ABI; if you change the shape here, change the
-- Rust side and rotate the schema hash.

namespace Adsmt

/-- A single abductive candidate. Mirror of the Rust
`AbductiveCandidate` struct. -/
structure AbductiveCandidate where
  /-- Stable id assigned by the engine for this candidate within
  the current `check-sat` call. -/
  id : UInt64
  /-- Rank position (0 = top candidate). User-cost ranking per
  the abductive engine's `rank` pass. -/
  rank : UInt32
  /-- Hypothesis atoms (SMT-LIB text per atom) that, if asserted
  in addition to the current context, would allow the engine
  to discharge the original goal. -/
  hypothesis : List String
  /-- Justification tag — one of `sld_chain` /
  `quantifier_exhausted` / `theory_gap` / etc. -/
  justification : String
  deriving Repr

/-- One of the four adsmt verdict variants. Mirror of the Rust
`AdsmtVerdict` enum. -/
inductive AdsmtVerdict where
  /-- SAT verdict with an optional model. -/
  | sat (model : List (String × String))
  /-- UNSAT verdict with an unsat core (assertion labels) and the
  certificate as S-expression text. -/
  | unsat (core : List String) (cert : String)
  /-- ABDUCTIVE verdict — ground reasoning could not discharge;
  one or more candidate hypothesis sets would. -/
  | abductive (candidates : List AbductiveCandidate)
  /-- UNKNOWN verdict with a human-readable reason. -/
  | unknown (reason : String)
  deriving Repr

end Adsmt
