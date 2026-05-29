# OpenConstruct Rust SDK

Native Rust SDK for [OpenConstruct](https://github.com/SuperInstance) — the full-featured client for onboarding, module registry, policy engine, fleet discovery, and sense integration.

## Features

- **OpenConstructClient** — Full client with session management, phase tracking, config generation
- **Module Registry** — Load, filter, and select modules by name, domain, or tag
- **Policy Engine** — Evaluate actions against policy rules (allow/deny/ask)
- **Fleet Discovery** — Discover fleet nodes and find the best node for a given capability
- **Sense Integration** — Typed sense shadows and fusion via correlator
- **OnboardingConfig** — Final output with agent card, modules, and interface choice

## Usage

```rust
use openconstruct::{OpenConstructClient, InterfaceChoice};

// Build the client
let mut client = OpenConstructClient::builder()
    .agent_name("my-agent")
    .model("glm-5.1")
    .capabilities(["code_generation", "web_search"])
    .build();

// Start onboarding session
client.start()?;

// Select modules
client.select_modules(&["spectral-graph-core", "plato-room"])?;

// Choose interface
client.choose_interface(InterfaceChoice::Cli)?;

// Generate config
let config = client.generate_config()?;
println!("Agent card: {:?}", config.agent_card);

// Fleet discovery
let fleet = client.discover_fleet()?;
let best = fleet.best_node_for("inference")?;
println!("Best inference node: {} at {}", best.name, best.address);

// Policy check
let allowed = client.policy_check("vision.capture", "/dev/video0")?;
println!("Vision capture: {:?}", allowed);
```

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
openconstruct = "0.1"
```

## Architecture

| Module | Description |
|--------|-------------|
| `client` | Main `OpenConstructClient` with session lifecycle |
| `builder` | Builder pattern for client construction |
| `registry` | Module registry with filtering |
| `fleet` | Fleet discovery and node selection |
| `types` | Core types (sessions, modules, policies, sense) |

## License

MIT
