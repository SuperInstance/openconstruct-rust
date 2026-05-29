# Dependencies — openconstruct-rust

## Ecosystem Role

openconstruct-rust is the **high-performance Rust runtime** for OpenConstruct. It provides the native execution engine for constraint evaluation, scheduling, and real-time pipeline processing used by the core OpenConstruct framework.

---

## Upstream Dependencies

| Repository | Description |
|---|---|
| [openconstruct-abi](https://github.com/SuperInstance/openconstruct-abi) | ABI definitions ensuring Rust FFI compatibility |
| [openconstruct](https://github.com/SuperInstance/openconstruct) | Core OpenConstruct framework (design and specs) |

## Internal Module Dependencies

| Repository | Description |
|---|---|
| [plato-tick](https://github.com/SuperInstance/plato-tick) | Tick-level time-series primitives used in scheduling |
| [plato-adapters](https://github.com/SuperInstance/plato-adapters) | Adapter interfaces for subsystem integration |
| [plato-construct](https://github.com/SuperInstance/plato-construct) | Construct-level abstractions |

## Downstream Dependents

| Repository | Description |
|---|---|
| [openconstruct](https://github.com/SuperInstance/openconstruct) | Main framework consumes the Rust runtime |
| [cocapn-core](https://github.com/SuperInstance/cocapn-core) | Co-captain core links against Rust runtime |
| [cocapn-explain-rs](https://github.com/SuperInstance/cocapn-explain-rs) | Rust explainability module |
| [cocapn-health-rs](https://github.com/SuperInstance/cocapn-health-rs) | Rust health monitoring module |
| [caching-service-rs](https://github.com/SuperInstance/caching-service-rs) | Rust caching service |

## Documentation

- [OpenConstruct Docs](https://github.com/SuperInstance/openconstruct-docs)
- [SuperInstance Wiki](https://github.com/SuperInstance/superinstance-wiki)
