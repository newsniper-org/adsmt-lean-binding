# `examples/runtime-smoke/` — leo4-oxilean-runner E2E

Fixture pair (cdylib + `Main.lean`) for the
`leo4-oxilean-runner::run_main_diagnostics` opt-in test
landed in leo4 RC.6 (F2, commit `f5cfedf`).

## What this verifies

Six wiring stages reach the runner's RC.6 F4 `USER_TYPES`
synthesis path with this binding's cdylib + Main.lean
pair:

| Stage | Diagnostic | Expected |
|---|---|---|
| 1. dlopen cdylib | (implicit; no early error) | `libadsmt_lean_rt.so` loads |
| 2. EXPORTS walk | `exports_seen` | 1 (single `#[leo4::export]` `run_check_sat`) |
| 3. Pair-register | `exports_registered` | == `exports_seen` |
| 4. Env bootstrap | `env_bootstrapped` | true |
| 5. USER_TYPES walk | `user_types_seen` | 2 (`AdsmtVerdict` + `AbductiveCandidate`) |
| 6. Parse + elab `Main.lean` | `decls_parsed` / `decls_elaborated` | both > 0 |

Confirmed RC.6 output (2026-06-02):

```
RunDiagnostics {
    exports_seen: 1,
    exports_registered: 1,
    decls_parsed: 1,
    decls_elaborated: 1,
    env_bootstrapped: true,
    resolver_ready: true,
    user_types_seen: 2,
}
```

## What this does NOT verify

The actual `oxilean_runtime::driver::run_main` driving of
the IO monad in `main` is **out of scope here**:

- OxiLean v0.1.3-leo4-ox7's bootstrap env (`leo4-oxilean-
  bootstrap::LEO4_PRIMITIVE_TYPES` + prelude) covers only
  pure type-theory — `Bool` / `Nat` / `String` / `List`
  / scalar carriers / arithmetic-typeclass-projection
  axioms. It does NOT include `IO`, `IO.println`, `pure`,
  or any monad-action surface needed to actually drive
  the runner's IO walker through a `run_check_sat` call.
- The full typed wrapper module that
  `leo4-oxilean-build --mode reverse` emits to
  `.leo4-emit/Adsmt/Rust.lean` uses
  `@[extern "<symbol>"]` + `deriving Leo4.LeanMarshal`
  syntax that the OxiLean PEG parser in the runner's
  parse step does not yet accept (string literal in
  attribute args; dotted name in `deriving`). The
  `leo4 run --impl rust-transpile` happy path loads it
  via Lake instead, then drives the user-binary call site
  separately.

So this fixture certifies: **the cdylib's leo4 ABI is
correctly structured for the runner's RC.5/6 typed
schema walk**, and **the leo4-oxilean-build wrapper emit
is symmetric with leo4-rust-emit's**. Driving an actual
typed Lean → Rust call through the OxiLean driver still
waits on upstream OxiLean stdlib expansion (or, in the
short term, the Lake-driven mslean4 reverse path).

## How to run

```sh
cargo build --release -p adsmt-lean-rt    # produce the cdylib

cd ~/leo4/sibling/leo4-oxilean-runner
LEO4_E2E_CDYLIB=/home/ybi/adsmt-lean-binding/target/release/libadsmt_lean_rt.so \
LEO4_E2E_MAIN_LEAN=/home/ybi/adsmt-lean-binding/examples/runtime-smoke/Main.lean \
  cargo test --release --test e2e_runner_smoke \
    run_main_diagnostics_walks_real_cdylib_and_main_lean \
    -- --ignored --nocapture
```

Expected: 1 passed, 0 failed, with the diagnostic struct
above printed.
