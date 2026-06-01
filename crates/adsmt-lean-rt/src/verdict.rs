//! Verdict types crossing the leo4 boundary.
//!
//! Each enum / struct here has a matching Lean counterpart in
//! `lake/Adsmt/Verdict.lean`. The leo4 schema-IDL handshake
//! ensures byte-identical mangling between the two sides; if the
//! shape changes here, the Lean side and any cached schema hash
//! must rotate together.

use leo4::LeanMarshal;

/// One of the four adsmt verdict variants. Mirror of the Lean
/// `inductive AdsmtVerdict` in `lake/Adsmt/Verdict.lean`.
#[derive(Clone, Debug, LeanMarshal)]
pub enum AdsmtVerdict {
    /// SAT verdict with an optional model (list of
    /// `(variable_name, value_smtlib_text)` pairs).
    Sat { model: Vec<(String, String)> },
    /// UNSAT verdict with an unsat core (assertion labels) and
    /// the certificate as S-expression text. Cert is empty
    /// when `--no-cert` was effectively in force.
    Unsat { core: Vec<String>, cert: String },
    /// ABDUCTIVE verdict — ground reasoning could not
    /// discharge; one or more candidate hypothesis sets would.
    Abductive { candidates: Vec<AbductiveCandidate> },
    /// UNKNOWN verdict with a human-readable reason.
    Unknown { reason: String },
}

/// A single abductive candidate. Mirror of the Lean
/// `structure AbductiveCandidate` in `lake/Adsmt/Verdict.lean`.
#[derive(Clone, Debug, LeanMarshal)]
pub struct AbductiveCandidate {
    /// Stable id assigned by the engine for this candidate
    /// within the current `check-sat` call.
    pub id: u64,
    /// Rank position (0 = top candidate). User-cost ranking
    /// per the abductive engine's `rank` pass.
    pub rank: u32,
    /// Hypothesis atoms (SMT-LIB text per atom) that, if
    /// asserted in addition to the current context, would
    /// allow the engine to discharge the original goal.
    pub hypothesis: Vec<String>,
    /// Justification tag — one of `sld_chain` /
    /// `quantifier_exhausted` / `theory_gap` / etc.
    pub justification: String,
}
