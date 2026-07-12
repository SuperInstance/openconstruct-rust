# Production Hardening Pass — 2026-07-11

Branch: `production-round4-2026-07-11` (off `main`)

This file tracks the concrete production-readiness gaps found by reading the
actual source and tests, and the fixes applied. Each fix is verified
independently with `cargo build`, `cargo test`, `cargo clippy --all-targets --
-D warnings`, and `cargo fmt --check` before being pushed.

## Audit findings (pre-fix)

### Fake-green CI
- `.github/workflows/ci.yml` runs `cargo clippy -- -D warnings`, but the lib
  has **6 clippy errors** (`new_without_default` x4, `manual_strip`,
  `dead_code` on `FleetManager.nodes`). CI cannot have ever passed on the
  current HEAD.
- CI does **not** run `cargo fmt --check`; the tree is not `rustfmt`-clean.

### Real bugs / dead code
- `FleetManager.nodes` is constructed empty and never read; `discover()`
  returns fresh hardcoded nodes, so the field is dead and `FleetManager`'s
  state is meaningless.
- `OpenConstructError::AlreadyComplete` is returned by `start()` when a
  session already exists — the variant name/message ("onboarding already
  complete") describes a different state than the one being signalled.

### Stubs oversold in README
- README claims "The [C ABI] is built from this crate". There is **no**
  `extern "C"`, `#[no_mangle]`, or `#[repr(C)]` anywhere in `src/`. The
  crate is pure Rust with no FFI surface.
- README claims "fusion via correlator" for sense; `SenseManager::fuse()`
  ignores `correlation_id` matching, always merges every shadow, and
  hardcodes `confidence: 0.92`.

### Honest claims (verified true)
- "Zero unsafe" — confirmed: no `unsafe` blocks in `src/`.
- Builder pattern, policy engine, fleet discovery, OnboardingConfig — all
  genuinely implemented and tested.

## Plan

1. Scaffold commit (this file).
2. CI: add `cargo fmt --check`, make `clippy` use `--all-targets`, run on
   the hardening branch too.
3. Clippy: add missing `Default` impls; rewrite `matches_pattern` with
   `strip_suffix`; consume `FleetManager.nodes` so it is no longer dead.
4. Rename `AlreadyComplete` -> `SessionAlreadyStarted` with accurate
   message; keep a deprecated alias for semver-conscious downstream users.
5. Tests: cover policy default-Ask, `add_rule`, fleet offline-only-capability,
   sense `fuse()` None path, `FleetManager` round-trip.
6. README: replace the false C-ABI claim with an honest status marker;
   mark sense fusion as a stub.

## Status (post-fix)

All items above landed. Final verification (matches the CI workflow exactly):

```
cargo check --all-targets            # ok
cargo fmt --all -- --check           # clean
cargo clippy --all-targets -- -D warnings   # no warnings
cargo test                           # 13 unit + 29 integration + 1 doctest = 43 passing
```

Additional changes made during the pass:

- `OpenConstructClient::reset()` added so the renamed error's "call reset()"
  hint is backed by a real API; `reset()` returns the previous session id.
- Added `FleetManager::with_nodes()` / `add_node()` / `nodes()` so the
  previously-dead `nodes` field is a real, useful topology.
- Derived `PartialEq` on `ModuleDescriptor`, `PolicyRule`, `AgentCard`
  (non-breaking; enables value comparisons in user code and tests).
- Wired the README into the crate as `#![doc = include_str!("../README.md")]`
  and made the Quick Start a complete `fn main() -> Result<...>` example so
  it runs as a doctest. CI uses plain `cargo test` (not `--all-targets`) so
  the doctest actually executes — `--all-targets` skips doctests, which is a
  subtle way examples can rot silently.
- Softened `Cargo.toml` `description` ("full-featured" -> honest per-feature
  wording) and dropped the inaccurate `api-bindings` category (the crate has
  no FFI surface).
