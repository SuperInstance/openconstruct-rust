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
