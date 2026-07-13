# OpenConstruct Rust SDK — The Full-Featured Native Client

Native Rust SDK for [OpenConstruct](https://github.com/SuperInstance/OpenConstruct). Onboarding, module registry, policy engine, fleet discovery, and sense integration — all in one crate.

## What This Gives You

Each feature is marked with its implementation status so you know what is
real today versus scaffolding for the ecosystem roadmap.

- **Builder pattern** `✅ implemented` — `OpenConstructClient::builder().agent_name("...").model("...").build()`
- **OnboardingConfig** `✅ implemented` — final output with agent card, modules, and interface choice
- **Policy engine** `✅ implemented` — evaluate actions against allow/deny/ask rules with first-match-wins ordering and `*`/prefix wildcard resources
- **Fleet discovery** `🟡 in-process` — `FleetManager` returns a built-in sample topology by default (`new()`) and snapshots the stored nodes via `discover()`. `best_node_for()` ranks online nodes by `load + latency_ms*0.01`. There is no network call yet; bring your own topology with `FleetManager::with_nodes()` / `add_node()`.
- **Sense integration** `🟡 partial` — `SenseManager::create_shadow()` stores typed shadows; `fuse()` merges them but is currently a **stub**: it requires ≥2 shadows and merges all of them regardless of the `correlation_id` argument, returning a hardcoded `confidence` of `0.92`. A real correlator that groups shadows by correlation id and computes confidence is tracked as future work.
- **Zero unsafe** `✅ verified` — no `unsafe` blocks anywhere in the crate.
- **FFI / C ABI** `❌ not provided by this crate` — see "How It Fits".

## Quick Start

This example is run as a doctest by `cargo test`, so it always matches the
real API.

```rust
use openconstruct::{InterfaceChoice, OpenConstructClient};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = OpenConstructClient::builder()
        .agent_name("my-agent")
        .model("glm-5.1")
        .capabilities(["code_generation", "web_search"])
        .build();

    client.start()?;
    client.select_modules(&["spectral-graph-core", "plato-room"])?;
    client.choose_interface(InterfaceChoice::Cli)?;

    let _config = client.generate_config()?;

    // Fleet discovery (in-process sample topology)
    let fleet = client.discover_fleet()?;
    let _best = fleet.best_node_for("inference")?;

    // Policy check
    let _allowed = client.policy_check("vision.capture", "/dev/video0")?;
    Ok(())
}
```

## Installation

```toml
[dependencies]
openconstruct = "0.1"
```

## Testing

```bash
cargo test          # unit + integration + README doctests
cargo clippy --all-targets -- -D warnings
cargo fmt --all -- --check
```

## How It Fits

- [OpenConstruct Documentation](https://github.com/SuperInstance/openconstruct-docs) — ecosystem-wide docs and guides

This crate is the pure-Rust client. It does **not** expose a C ABI itself: a
search of the source shows no `extern "C"`, `#[no_mangle]`, or `#[repr(C)]`
items. The linked [`openconstruct-abi`](https://github.com/SuperInstance/openconstruct-abi)
is a separate effort; if/when this crate grows an FFI surface, that status
will change here and in the [feature list](#what-this-gives-you) above. No
unverified claims are made about other-language bindings — those projects
speak for themselves.

## License

MIT
