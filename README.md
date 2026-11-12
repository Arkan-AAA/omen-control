# OMEN Control

Open-source, native Rust replacement for HP OMEN Gaming Hub, focused on
hardware control for HP OMEN / HyperX laptops.

Status: Milestone 0 — Windows read-only vitals + UI skeleton.

## Features (M0)

- Device detection via SMBIOS/WMI
- CPU/GPU temperature, CPU/GPU usage, fan RPM (best effort)
- Slint-based native UI (mock device by default)
- Cross-platform-ready core (`omen-core`)

## Build (Windows)

Requires Rust (stable) and Visual Studio Build Tools (MSVC).

```sh
cargo build --workspace
cargo run -p omen-ui
```

Real hardware backend:

```sh
cargo run -p omen-ui --features real
```

## License

MIT OR Apache-2.0
