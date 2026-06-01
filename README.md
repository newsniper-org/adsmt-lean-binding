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

This is a **separate repo** from adsmt main and leo4. It depends
on both via git-rev pins:

| Dependency | Pin | Why |
|---|---|---|
| leo4 | `tag = "v1.0.0-rc.1"` | Phase 10 cuttable state |
| adsmt-cert, adsmt-core, adsmt-engine, adsmt-parser | `branch = "testing"` | Consumer line until adsmt v1.0.0 cuts |

When adsmt main cuts v1.0.0 stable, the adsmt pins switch to
`tag = "v1.0.0"`. When leo4 cuts v1.0.0 stable, the leo4 pin
switches to `tag = "v1.0.0"`.

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

```bash
# Rust side (cdylib)
cargo build --release

# Lean side (after the Rust cdylib exists)
lake update
lake build
```

Lake-first / Cargo-second drive (per leo4 D8) is **not yet wired**
— for v0.1 the two halves build independently and the wrapper
emission is manual. A `build.rs` hook lands in a follow-up commit.

## License

Triple-licensed at the consumer's choice:

- BSD-2-Clause
- Apache-2.0
- LGPL-2.1-or-later

Matches the adsmt main project's triple (and the contribution side
of OxiZ-flowing code stays Apache-2-only at the upstream side, per
adsmt main's `oxiz_relationship.md`).
