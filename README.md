# OMEN Control

Open-source, native Rust replacement for HP OMEN Gaming Hub, focused on
hardware control for HP OMEN / HyperX laptops.

Status: Milestone 0 — Windows read-only vitals + UI skeleton.

## Features

- Dashboard with live vitals (CPU/GPU temp, usage, fan RPM)
- Performance mode switching (Balanced/Performance/Quiet via power plans)
- Fan speed control (best-effort via Win32_Fan, may require admin)
- RGB lighting (best-effort via HP TyphonChasisLightingSDK)
- Network throughput monitoring
- System vitals overview
- Native Slint UI (dark theme)

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
