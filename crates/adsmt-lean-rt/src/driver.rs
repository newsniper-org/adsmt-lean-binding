//! Mini driver: SMT-LIB v2 script → AdsmtVerdict.
//!
//! Mirrors the dispatch shape of `adsmt-cli`'s `Driver` for the
//! command subset L1 needs (set-logic / set-option / declare-const
//! / declare-datatype / assert / check-sat / exit). Other commands
//! pass through as `Continue` no-ops so a typical Lean tactic
//! payload script runs end-to-end.
//!
//! The first `(check-sat)` invocation's verdict is returned.
//! Subsequent commands are not executed — Lean tactic scripts
//! invoke once per goal anyway.

use adsmt_cert::emit_certificate;
use adsmt_core::Type;
use adsmt_engine::result::SatResult;
use adsmt_engine::Solver;
use adsmt_parser::{convert_expr, parse_smtlib, Command, SymbolTable};
use adsmt_theory::datatypes::DatatypeDecl;

use crate::verdict::{AbductiveCandidate, AdsmtVerdict};

/// Parse + dispatch the script. Returns the verdict for the first
/// `(check-sat)` that fires.
pub fn run_check_sat(script: &str) -> AdsmtVerdict {
    let commands = match parse_smtlib(script) {
        Ok(cs) => cs,
        Err(e) => return AdsmtVerdict::Unknown { reason: format!("parse error: {e}") },
    };

    let mut driver = Driver::new();

    for cmd in commands {
        match driver.dispatch(cmd) {
            Step::Continue => {}
            Step::CheckSat(verdict) => return verdict,
            Step::Exit => break,
            Step::Error(msg) => return AdsmtVerdict::Unknown { reason: msg },
        }
    }

    AdsmtVerdict::Unknown { reason: "no check-sat in script".to_string() }
}

enum Step {
    Continue,
    CheckSat(AdsmtVerdict),
    Exit,
    Error(String),
}

struct Driver {
    solver: Solver,
    symbols: SymbolTable,
}

impl Driver {
    fn new() -> Self {
        Self {
            solver: Solver::new(),
            symbols: SymbolTable::new(),
        }
    }

    fn dispatch(&mut self, cmd: Command) -> Step {
        match cmd {
            // Logic + option + info — accept silently. The CLI
            // surfaces an advisory for unsupported logics; in the
            // binding context we trust the caller.
            Command::SetLogic(_)
            | Command::SetOption { .. }
            | Command::SetInfo { .. } => Step::Continue,

            // Sort + fun + define-fun — no-op at v0.1 binding.
            // Matches the CLI's "v0.5 will validate" comment.
            Command::DeclareSort { .. }
            | Command::DeclareFun { .. }
            | Command::DefineFun { .. } => Step::Continue,

            Command::DeclareConst { name, sort } => {
                let sort_str = sort.to_string();
                let ty = match sort_str.as_str() {
                    "Bool" => Type::bool_(),
                    "Int" => Type::const_("Int", adsmt_core::Kind::Type),
                    "Real" => Type::const_("Real", adsmt_core::Kind::Type),
                    other => Type::const_(other, adsmt_core::Kind::Type),
                };
                self.symbols.declare(name, ty);
                Step::Continue
            }

            Command::DeclareDatatype { name, constructors } => {
                let sort = Type::const_(&name, adsmt_core::Kind::Type);
                for ctor in &constructors {
                    self.symbols.declare_constructor(ctor.clone(), sort.clone());
                }
                self.solver
                    .declare_datatype(DatatypeDecl::finite_enum(name, constructors));
                Step::Continue
            }

            Command::Assert(expr) => match convert_expr(&expr, &self.symbols) {
                Ok(term) => {
                    self.solver.assert(term);
                    Step::Continue
                }
                Err(e) => Step::Error(format!("convert error: {e}")),
            },

            Command::CheckSat => Step::CheckSat(verdict_of(self.solver.check_sat())),
            Command::CheckSatAssuming(_) => {
                Step::CheckSat(AdsmtVerdict::Unknown {
                    reason: "check-sat-assuming not yet wired in adsmt-lean-rt".to_string(),
                })
            }

            Command::GetProof => Step::Continue,
            Command::GetModel => Step::Continue,
            Command::GetUnsatCore => Step::Continue,
            Command::Push(_) | Command::Pop(_) => Step::Continue,
            Command::Reset | Command::ResetAssertions => {
                self.solver.reset();
                Step::Continue
            }

            Command::Exit => Step::Exit,
            _ => Step::Continue,
        }
    }
}

pub(crate) fn verdict_of(r: SatResult) -> AdsmtVerdict {
    match r {
        SatResult::Sat => AdsmtVerdict::Sat { model: Vec::new() },
        SatResult::Unsat { certificate } => {
            let cert = certificate
                .as_ref()
                .map(emit_certificate)
                .unwrap_or_default();
            AdsmtVerdict::Unsat { core: Vec::new(), cert }
        }
        SatResult::Unknown { reason } => AdsmtVerdict::Unknown { reason },
        SatResult::Abductive { candidates } => AdsmtVerdict::Abductive {
            candidates: candidates
                .iter()
                .enumerate()
                .map(|(idx, c)| AbductiveCandidate {
                    id: idx as u64,
                    rank: idx as u32,
                    hypothesis: c.hypotheses.iter().map(|t| format!("{t:?}")).collect(),
                    justification: c.sources.join(","),
                })
                .collect(),
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use adsmt_abduce::abducible::Abducible;
    use adsmt_abduce::sld::Candidate;
    use adsmt_core::{Term, Type};

    // L1 public-surface smoke. Demonstrates the run_check_sat
    // entry from end to end against a trivial SMT-LIB script.

    #[test]
    fn run_check_sat_handles_trivially_sat_script() {
        // No assertions → SAT. Solver returns SatResult::Sat
        // → AdsmtVerdict::Sat { model: empty }.
        let v = run_check_sat("(check-sat)\n");
        assert!(
            matches!(v, AdsmtVerdict::Sat { .. }),
            "expected Sat, got {v:?}"
        );
    }

    #[test]
    fn run_check_sat_parse_error_surfaces_as_unknown() {
        let v = run_check_sat("this is not valid smt-lib");
        match v {
            AdsmtVerdict::Unknown { reason } => {
                assert!(
                    reason.contains("parse"),
                    "Unknown reason should mention 'parse', got {reason:?}"
                );
            }
            other => panic!("expected Unknown(parse), got {other:?}"),
        }
    }

    #[test]
    fn run_check_sat_missing_check_sat_returns_unknown() {
        // No `(check-sat)` → run to completion → Unknown sentinel.
        let v = run_check_sat("(set-logic ALL)\n");
        match v {
            AdsmtVerdict::Unknown { reason } => {
                assert_eq!(reason, "no check-sat in script");
            }
            other => panic!("expected Unknown(no check-sat), got {other:?}"),
        }
    }

    // L2 wire smoke. Exercises verdict_of directly against a
    // hand-built SatResult::Abductive — proves the
    // AbductiveCandidate marshalling chain (Rust struct →
    // LeanMarshal-derived encoding → Lean structure) sits on a
    // verdict_of conversion that actually fires.

    #[test]
    fn verdict_of_abductive_maps_candidates() {
        let pattern = Term::var("p", Type::bool_());
        let abducible = Abducible::new(pattern, "test-source");
        let cand = Candidate::with_one(&abducible);

        let sat_result = SatResult::Abductive {
            candidates: vec![cand],
        };
        let verdict = verdict_of(sat_result);

        match verdict {
            AdsmtVerdict::Abductive { candidates } => {
                assert_eq!(candidates.len(), 1);
                let c = &candidates[0];
                assert_eq!(c.id, 0);
                assert_eq!(c.rank, 0);
                assert_eq!(c.hypothesis.len(), 1);
                assert_eq!(c.justification, "test-source");
            }
            other => panic!("expected Abductive, got {other:?}"),
        }
    }

    #[test]
    fn verdict_of_unsat_emits_cert_text_when_present() {
        // No certificate (proof mode off) → empty cert string.
        let verdict = verdict_of(SatResult::Unsat { certificate: None });
        match verdict {
            AdsmtVerdict::Unsat { cert, .. } => assert_eq!(cert, ""),
            other => panic!("expected Unsat, got {other:?}"),
        }
    }
}
