# SigmaOS-process-monitor

Process monitor (states, CPU usage, lookup by name/state)

- **Category:** System Services
- **Engine:** process-monitor
- **Tier:** 2
- **Language:** Rust
- **Provides:** sigmaprocessmonitor

Part of the [SigmaOS](https://github.com/AaryanSinghChauhan09/SigmaOS)
multi-repository component ecosystem.

## Build & Test

```sh
cargo build
cargo test
```

## Contract

This component declares its interface in `COMPONENT.toml`:

- `provides` — capabilities other components may depend on
- `depends_on` — component names this component requires

## License

Apache-2.0 — see [LICENSE](LICENSE).
