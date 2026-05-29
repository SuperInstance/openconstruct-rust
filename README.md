# OpenConstruct Rust SDK — The Full-Featured Native Client

Native Rust SDK for [OpenConstruct](https://github.com/SuperInstance/OpenConstruct). Onboarding, module registry, policy engine, fleet discovery, and sense integration — all in one crate.

## What This Gives You

- **Builder pattern** — `OpenConstructClient::builder().agent_name("...").model("...").build()`
- **Policy engine** — evaluate actions against allow/deny/ask rules
- **Fleet discovery** — find the best node for a given capability
- **Sense integration** — typed sense shadows and fusion via correlator
- **OnboardingConfig** — final output with agent card, modules, and interface choice
- **Zero unsafe** — safe Rust throughout

## Quick Start

```rust
use openconstruct::{OpenConstructClient, InterfaceChoice};

let mut client = OpenConstructClient::builder()
    .agent_name("my-agent")
    .model("glm-5.1")
    .capabilities(["code_generation", "web_search"])
    .build();

client.start()?;
client.select_modules(&["spectral-graph-core", "plato-room"])?;
client.choose_interface(InterfaceChoice::Cli)?;

let config = client.generate_config()?;

// Fleet discovery
let fleet = client.discover_fleet()?;
let best = fleet.best_node_for("inference")?;

// Policy check
let allowed = client.policy_check("vision.capture", "/dev/video0")?;
```

## Installation

```toml
[dependencies]
openconstruct = "0.1"
```

## Testing

```bash
cargo test
```

## How It Fits
- [OpenConstruct Documentation](https://github.com/SuperInstance/openconstruct-docs) — ecosystem-wide docs and guides

This is the reference implementation. The [C ABI](https://github.com/SuperInstance/openconstruct-abi) is built from this crate, and all other bindings ([Python](https://github.com/SuperInstance/openconstruct-python), [Go](https://github.com/SuperInstance/openconstruct-go), [TypeScript](https://github.com/SuperInstance/openconstruct-ts), etc.) follow the same protocol.

## License

MIT
