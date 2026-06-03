# adsmt-lean-binding

**Status**: v0.1 skeleton (2026-06-01). L1 implementation slot per
adsmt's `docs/thoughts/adsmt-leo4-integration.md` §4-A.

leo4-based binding library that exposes the
[adsmt](https://github.com/newsniper-org/adsmt) SMT engine to Lean 4
via [leo4](https://github.com/newsniper-org/leo4)'s canonical ABI
(reverse direction, Phase 9).

## Why this exists

The adsmt Lean tactic harness historically called the `lu-smt` CLI
as a subprocess, or marshalled hypotheses through the `adsmt-ffi`
C ABI. Both work but carry friction (subprocess fork cost, manual
C-side type packing). leo4 v1.0.0-rc.1 provides a typed canonical
ABI that lets a Lean tactic invoke a Rust function as if it were
a native Lean call, with `@[leo4_import]` declarations doing the
marshalling.

This repo is the binding library that connects the two sides.

## Structure

```
adsmt-lean-binding/
├── Cargo.toml                    Rust workspace root
├── crates/
│   └── adsmt-lean-rt/            Rust-side binding crate (cdylib+rlib)
│       └── src/
│           ├── lib.rs            #[leo4::export] entry points
│           └── verdict.rs        Verdict type mirror
├── lakefile.lean                 Lake workspace root
├── lean-toolchain                Lean 4 toolchain pin (matches leo4)
├── lake/
│   └── Adsmt/
│       ├── Verdict.lean          @[leo4_import] verdict types
│       ├── Solver.lean           runCheckSat declaration
│       └── Tactic.lean           smt_decide / smt_abduce skeleton
└── LICENSE-*.txt                 Triple license (matches adsmt main)
```

## Version pins

This is a **separate repo** from adsmt main and leo4. The
published-form intent is:

| Dependency | Published pin | Why |
|---|---|---|
| leo4 | `tag = "v1.0.0-rc.5"` | Phase 10 cuttable state + RC.2-5 typed-enum fix chain (RC.5 closes the leo4-oxilean-build asymmetry — rust-transpile reverse path now emits typed wrappers) |
| adsmt-cert, adsmt-core, adsmt-engine, adsmt-parser | `branch = "testing"` | Consumer line until adsmt v1.0.0 cuts |

When adsmt main cuts v1.0.0 stable, the adsmt pins switch to
`tag = "v1.0.0"`. When leo4 cuts v1.0.0 stable, the leo4 pin
switches to `tag = "v1.0.0"`.

**v0.1 dev: local path deps.** Cargo.toml currently uses
`path = "../leo4/..."` and `path = "../AD1/..."` so the skeleton
builds without going through GitHub. The git-pinned alternatives
are in the same file as comment-only declarations — uncomment
those lines and remove the path entries when this repo goes
public or when the leo4 submodule chain is ready for direct git
consumption.

## Wire format

The L1 export `run_check_sat(script: String) -> AdsmtVerdict` is
**typed from day one** as of v0.1.1. leo4 v1.0.0-rc.4's patch
chain (RC.2 output-side fix + RC.3 forward-direction input
multi-candidate lookup + RC.4 reverse-direction input lift)
together let `#[leo4::export]` accept user-defined enum / struct
types in parameter and return positions without a String/JSON
wire workaround.

The leo4 schema-IDL handshake ensures byte-identical mangling
between `crates/adsmt-lean-rt/src/verdict.rs` and
`lake/Adsmt/Verdict.lean`; if the shape changes on one side, the
other must move in lockstep and the schema hash rotates.

(v0.1's initial String/JSON wire — landed in commit `1bc6733`
against RC.1 — was reverted in `8bd2821` once the RC.4 patch
chain closed the loop.)

## What L1 / L2 / L3 mean

Per adsmt's v1.0.0 scope expansion (memory
`v1_0_0_scope_expansion.md` in the adsmt main repo):

- **L1** — `lu-smt` subprocess replaced by typed leo4 binding (forward direction). **THIS REPO**.
- **L2** — typed `AbductiveCandidate` marshalling. THIS REPO (depends on L1 shape).
- **L3** — Lean → Rust callback path (OxiLean route). THIS REPO once oracle / cost-function use cases solidify.
- **L4** — Lean → Rust callback on mainline Lean 4. Waiting on leo4's `feat/mslean4-lecq-lecr-ipcs` branch.

The v0.1 skeleton in this commit covers the scaffolding for L1
(verdict types + the `run_check_sat` export stub). Real engine
wiring lands when the adsmt testing channel's `adsmt-engine::Solver`
exposes a unified `check_sat(script_text)` entry point this
binding can call.

## Build

Two paths verified under leo4 v1.0.0-rc.5:

### Path A — rust-transpile reverse (no Lake, no Lean toolchain) ✓

```bash
# 1. Build the cdylib.
cargo build --release -p adsmt-lean-rt

# 2. Emit the typed Lean wrapper from the cdylib's USER_TYPES slice.
~/leo4/sibling/leo4-oxilean-build/target/release/leo4-oxilean-build \
    --mode reverse \
    --cdylib target/release/libadsmt_lean_rt.so \
    --iface Adsmt \
    --out .leo4-emit/Adsmt/Rust.lean

# 3. (Optional) Drive the wrapper via leo4-oxilean evaluator.
#    See ~/leo4/examples/05-rust-export/README.md for the
#    cargo-run-with-LEO4_OXILEAN_* env-var pattern.
```

Step 2 emits the full typed surface — `AdsmtVerdict` /
`AbductiveCandidate` mirror inductives + the
`@[extern "leo4_rust__run_check_sat__str"] opaque run_check_sat`
declaration with the typed return.

### Path B — mslean4 reverse (Lake + leanc, hand-written goal-closing tactics)

```bash
# 1. Rust cdylib (same as Path A).
cargo build --release -p adsmt-lean-rt

# 2. Lake side (D8 pattern). Not yet wired — manual lake build
#    against `lakefile.lean`.
```

Lake-first / Cargo-second drive (per leo4 D8) is **not yet wired**
on Path B — for v0.2.x the two halves build independently. A
`build.rs` hook lands in a follow-up commit.

## License

Triple-licensed at the consumer's choice:

- BSD-2-Clause
- Apache-2.0
- LGPL-2.1-or-later

Matches the adsmt main project's triple (and the contribution side
of OxiZ-flowing code stays Apache-2-only at the upstream side, per
adsmt main's `oxiz_relationship.md`).
