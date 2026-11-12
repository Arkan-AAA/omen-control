# OMEN Control — Windows Milestone 0 (scaffold + read-only vitals) Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Create the Rust workspace, core domain types and device trait, a mock device, Windows device detection + read-only vitals (CPU/GPU temps, fan RPM where available), and a Slint UI skeleton that runs against the mock device, with CI.

**Architecture:** Cargo workspace with `omen-core` (types, trait, mock, config), `omen-windows` (WMI/NVML backend), `omen-ui` (Slint app). The UI depends only on the `OmenDevice` trait, so it runs against `MockDevice` without hardware. Real Windows backend is wired behind a cargo feature.

**Tech Stack:** Rust stable 1.99 (MSVC), Slint 1.18, sysinfo 0.40, nvml-wrapper 0.13, windows 0.62, wmi 0.18, serde + toml, tracing, thiserror, anyhow.

**Spec:** `docs/superpowers/specs/2026-11-12-omen-control-design.md` (and `.ru.md`).

## Global Constraints

- Toolchain: stable-x86_64-pc-windows-msvc; `rustfmt` + `clippy` clean.
- No `unsafe` outside `omen-windows` FFI boundaries; document each block.
- UI never blocks on hardware calls: device calls run on a worker thread, results sent via channels.
- `omen-core` must compile on Linux too (no Windows-only deps in core).
- All user-facing strings in English; code comments in English.
- Cargo features: `omen-ui` default = `mock`; `real` enables `omen-windows`.

## Review Focus

- Detection must not panic when WMI/NVML is unavailable; return `OmenError::Unavailable`.
- CPU temperature on Windows may be missing; UI must render "n/a", not crash.
- NVML init failure (no NVIDIA driver) must degrade gracefully since the target has Intel iGPU + RTX 5050.
- Slint build script must work in CI without a display.
- Mock device must be the default so `cargo test --workspace` passes on machines without hardware.

---

### Task 1: Scaffold the Cargo workspace

**Files:**
- Create: `Cargo.toml`
- Create: `rust-toolchain.toml`
- Create: `.gitignore`
- Create: `crates/omen-core/Cargo.toml`
- Create: `crates/omen-core/src/lib.rs`
- Create: `crates/omen-windows/Cargo.toml`
- Create: `crates/omen-windows/src/lib.rs`
- Create: `crates/omen-ui/Cargo.toml`
- Create: `crates/omen-ui/src/main.rs`
- Create: `crates/omen-ui/build.rs`

**Interfaces:**
- Produces: workspace with three crates; `omen-core` exposes `OmenDevice` trait (Task 2 fills details).

- [ ] **Step 1: Write the workspace manifest**

Create `Cargo.toml`:

```toml
[workspace]
resolver = "2"
members = ["crates/omen-core", "crates/omen-windows", "crates/omen-ui"]

[workspace.package]
version = "0.1.0"
edition = "2024"
license = "MIT OR Apache-2.0"
authors = ["OMEN Control contributors"]

[workspace.dependencies]
omen-core = { path = "crates/omen-core" }
omen-windows = { path = "crates/omen-windows" }
serde = { version = "1.0", features = ["derive"] }
toml = "1.2"
thiserror = "2.0"
anyhow = "1.0"
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["env-filter"] }
sysinfo = "0.40"
nvml-wrapper = "0.13"
windows = { version = "0.62", features = ["Win32_System_Wmi", "Win32_Foundation"] }
wmi = "0.18"
slint = "1.18"
slint-build = "1.18"
```

- [ ] **Step 2: Pin the toolchain**

Create `rust-toolchain.toml`:

```toml
[toolchain]
channel = "stable"
components = ["rustfmt", "clippy"]
targets = ["x86_64-pc-windows-msvc"]
```

- [ ] **Step 3: Create crate manifests (empty libs)**

`crates/omen-core/Cargo.toml`:
```toml
[package]
name = "omen-core"
version.workspace = true
edition.workspace = true
license.workspace = true

[dependencies]
serde.workspace = true
toml.workspace = true
thiserror.workspace = true
anyhow.workspace = true
tracing.workspace = true
```

`crates/omen-windows/Cargo.toml`:
```toml
[package]
name = "omen-windows"
version.workspace = true
edition.workspace = true
license.workspace = true

[dependencies]
omen-core.workspace = true
sysinfo.workspace = true
nvml-wrapper.workspace = true
windows.workspace = true
wmi.workspace = true
thiserror.workspace = true
tracing.workspace = true
```

`crates/omen-ui/Cargo.toml`:
```toml
[package]
name = "omen-ui"
version.workspace = true
edition.workspace = true
license.workspace = true

[dependencies]
omen-core.workspace = true
omen-windows = { workspace = true, optional = true }
slint.workspace = true
tracing.workspace = true
tracing-subscriber.workspace = true

[build-dependencies]
slint-build.workspace = true

[features]
default = ["mock"]
mock = []
real = ["dep:omen-windows"]
```

- [ ] **Step 4: Create empty lib/bin roots**

`crates/omen-core/src/lib.rs`: `pub mod placeholder {}` (temporary).
`crates/omen-windows/src/lib.rs`: `pub mod placeholder {}`.
`crates/omen-ui/src/main.rs`: `fn main() { println!("omen-ui"); }`.
`crates/omen-ui/build.rs`:
```rust
fn main() {
    slint_build::compile("ui/app-window.slint").unwrap();
}
```
Create empty `crates/omen-ui/ui/app-window.slint`.

- [ ] **Step 5: Add .gitignore**

Standard Rust gitignore.

- [ ] **Step 6: Verify scaffold builds**

Run: `cargo build --workspace`
Expected: builds with warnings only.

- [ ] **Step 7: Commit**

```bash
git add -A
git commit -m "chore: scaffold cargo workspace"
```

---

### Task 2: Core domain types, trait, mock device, config

**Files:**
- Create: `crates/omen-core/src/error.rs`
- Create: `crates/omen-core/src/types.rs`
- Create: `crates/omen-core/src/device.rs`
- Create: `crates/omen-core/src/config.rs`
- Create: `crates/omen-core/src/mock.rs`
- Modify: `crates/omen-core/src/lib.rs`
- Create: `crates/omen-core/tests/core_tests.rs`

**Interfaces:**
- Produces: `OmenError`, `DeviceInfo`, `Vitals`, `PerformanceMode`, `FanCurve`, `Rgb`, `RgbZone`, `NetworkStats`, `trait OmenDevice`, `MockDevice`, `Config`.

- [ ] **Step 1: Write failing tests**

`crates/omen-core/tests/core_tests.rs`:
```rust
use omen_core::{MockDevice, OmenDevice, PerformanceMode, Config};

#[test]
fn mock_reports_vitals() {
    let d = MockDevice::default();
    let v = d.vitals().unwrap();
    assert!(v.cpu_temp_c.is_some());
}

#[test]
fn mock_switches_performance_mode() {
    let d = MockDevice::default();
    d.set_performance_mode(PerformanceMode::Performance).unwrap();
    assert_eq!(d.current_mode(), PerformanceMode::Performance);
}

#[test]
fn config_roundtrips() {
    let c = Config::default();
    let s = toml::to_string(&c).unwrap();
    let back: Config = toml::from_str(&s).unwrap();
    assert_eq!(c, back);
}
```

- [ ] **Step 2: Run tests to verify they fail**

Run: `cargo test -p omen-core`
Expected: FAIL (modules missing).

- [ ] **Step 3: Implement error, types, trait, mock, config**

`error.rs`:
```rust
use thiserror::Error;

#[derive(Debug, Error)]
pub enum OmenError {
    #[error("device unavailable: {0}")]
    Unavailable(String),
    #[error("not supported: {0}")]
    Unsupported(String),
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
    #[error("other: {0}")]
    Other(String),
}
```

`types.rs`: define structs with serde derives:
- `DeviceInfo { manufacturer, model, sku, cpu, gpu: Vec<String> }`
- `Vitals { cpu_temp_c: Option<f32>, gpu_temp_c: Option<f32>, fan_rpm: Option<u32>, cpu_usage: Option<f32>, gpu_usage: Option<f32> }`
- `PerformanceMode { Balanced, Performance, Quiet }` (enum, serde, Copy, PartialEq, Eq, Debug)
- `FanCurve { points: Vec<(u8,u8)> }` (temp%, speed%)
- `Rgb { r: u8, g: u8, b: u8 }`
- `RgbZone { id: u8, name: String, color: Rgb }`
- `NetworkStats { rx_bytes: u64, tx_bytes: u64 }`

`device.rs`:
```rust
use crate::{types::*, error::OmenError};

pub trait OmenDevice: Send + Sync {
    fn info(&self) -> DeviceInfo;
    fn vitals(&self) -> Result<Vitals, OmenError>;
    fn performance_modes(&self) -> &'static [PerformanceMode];
    fn current_mode(&self) -> PerformanceMode;
    fn set_performance_mode(&self, mode: PerformanceMode) -> Result<(), OmenError>;
    fn fan_curve(&self) -> Result<FanCurve, OmenError>;
    fn set_fan_curve(&self, curve: FanCurve) -> Result<(), OmenError>;
    fn rgb_zones(&self) -> Vec<RgbZone>;
    fn set_rgb(&self, zone: u8, color: Rgb) -> Result<(), OmenError>;
    fn network_stats(&self) -> Result<NetworkStats, OmenError>;
}
```

`mock.rs`: `MockDevice` with `Mutex<PerformanceMode>` and `Mutex<FanCurve>`, implements trait returning canned vitals.

`config.rs`: `Config { performance: ..., theme: String }` with Default.

`lib.rs`: module declarations and re-exports.

- [ ] **Step 4: Run tests**

Run: `cargo test -p omen-core`
Expected: PASS.

- [ ] **Step 5: Commit**

```bash
git add crates/omen-core
git commit -m "feat(core): domain types, device trait, mock, config"
```

---

### Task 3: Windows device detection and read-only vitals

**Files:**
- Create: `crates/omen-windows/src/detect.rs`
- Create: `crates/omen-windows/src/vitals.rs`
- Create: `crates/omen-windows/src/lib.rs` (WindowsDevice)
- Create: `crates/omen-windows/tests/detect_tests.rs`

**Interfaces:**
- Consumes: `omen_core::{DeviceInfo, Vitals, OmenDevice, OmenError, PerformanceMode}`.
- Produces: `WindowsDevice::detect() -> Result<Self, OmenError>`, `detect_model() -> DeviceInfo` (testable pure-ish), `read_vitals(&self) -> Vitals`.

- [ ] **Step 1: Write failing detection test**

`crates/omen-windows/tests/detect_tests.rs`:
```rust
use omen_windows::detect::parse_device_info;

#[test]
fn parses_omen_model() {
    let info = parse_device_info("HP", "HyperX OMEN 15", "D57E7UA#ABA", "Intel Core Ultra 7 356H", vec!["NVIDIA GeForce RTX 5050 Laptop GPU".into()]);
    assert_eq!(info.model, "HyperX OMEN 15");
    assert!(info.gpu.iter().any(|g| g.contains("RTX 5050")));
}
```

- [ ] **Step 2: Run test to verify failure**

Run: `cargo test -p omen-windows`
Expected: FAIL.

- [ ] **Step 3: Implement detection + vitals**

`detect.rs`:
- `parse_device_info(manufacturer, model, sku, cpu, gpu) -> DeviceInfo` (pure, testable).
- `detect_model() -> Result<DeviceInfo, OmenError>` using WMI `Win32_ComputerSystem`, `Win32_Processor`, `Win32_VideoController`. Wrap WMI calls; on failure return `Unavailable`.

`vitals.rs`:
- `read_vitals() -> Vitals`: CPU temp via WMI `MSAcpi_ThermalZoneTemperature` (convert tenths of Kelvin to C), GPU via `nvml-wrapper` (init guarded; on failure None), fan RPM via WMI if available else None, usage via sysinfo.

`lib.rs`:
- `pub struct WindowsDevice { info: DeviceInfo }`
- impl `OmenDevice`: info, vitals (calls read_vitals), performance_modes returns `&[Balanced, Performance, Quiet]`, current_mode returns Balanced, set_performance_mode returns `Err(OmenError::Unsupported(...))` (helper deferred to next milestone), fan_curve returns default, rgb_zones empty, network_stats via sysinfo.

- [ ] **Step 4: Run tests**

Run: `cargo test -p omen-windows`
Expected: PASS (detection test; hardware tests ignored).

- [ ] **Step 5: Commit**

```bash
git add crates/omen-windows
git commit -m "feat(windows): device detection and read-only vitals"
```

---

### Task 4: Slint UI skeleton (mock by default)

**Files:**
- Create: `crates/omen-ui/ui/app-window.slint`
- Modify: `crates/omen-ui/build.rs`
- Modify: `crates/omen-ui/src/main.rs`
- Create: `crates/omen-ui/src/app.rs`

**Interfaces:**
- Consumes: `OmenDevice` trait, `Vitals`.
- Produces: runnable `omen-ui` binary showing dashboard vitals from `MockDevice` (or `WindowsDevice` under `real` feature).

- [ ] **Step 1: Write the Slint view**

`ui/app-window.slint`:
```slint
export component AppWindow inherits Window {
    in property <string> cpu-temp: "n/a";
    in property <string> gpu-temp: "n/a";
    in property <string> fan-rpm: "n/a";
    VerticalBox {
        Text { text: "OMEN Control"; font-size: 24px; }
        Text { text: "CPU temp: " + cpu-temp; }
        Text { text: "GPU temp: " + gpu-temp; }
        Text { text: "Fan: " + fan-rpm; }
    }
}
```

- [ ] **Step 2: Wire build + main**

`build.rs`:
```rust
fn main() {
    slint_build::compile("ui/app-window.slint").unwrap();
}
```

`src/app.rs`: function `run(device: Box<dyn OmenDevice>)` that creates `AppWindow`, spawns a thread polling `device.vitals()` every 2s updating properties via `slint::invoke_from_event_loop` or `Weak`.

`src/main.rs`:
```rust
fn main() {
    tracing_subscriber::fmt::init();
    #[cfg(feature = "real")]
    let device: Box<dyn omen_core::OmenDevice> = Box::new(omen_windows::WindowsDevice::detect().expect("detect"));
    #[cfg(not(feature = "real"))]
    let device: Box<dyn omen_core::OmenDevice> = Box::new(omen_core::MockDevice::default());
    omen_ui::app::run(device).unwrap();
}
```
(Adjust module path; `app` module in same crate.)

- [ ] **Step 3: Build UI (mock)**

Run: `cargo build -p omen-ui`
Expected: builds.

- [ ] **Step 4: Build UI (real feature, compile only)**

Run: `cargo build -p omen-ui --features real`
Expected: builds (may fail at runtime without hardware, compile ok).

- [ ] **Step 5: Commit**

```bash
git add crates/omen-ui
git commit -m "feat(ui): slint skeleton with mock/real device"
```

---

### Task 5: CI workflow

**Files:**
- Create: `.github/workflows/ci.yml`

- [ ] **Step 1: Add workflow**

```yaml
name: CI
on: [push, pull_request]
jobs:
  test:
    runs-on: windows-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
        with:
          components: rustfmt, clippy
      - run: cargo fmt --check
      - run: cargo clippy --workspace -- -D warnings
      - run: cargo test --workspace
      - run: cargo build -p omen-ui --features real
```

- [ ] **Step 2: Commit**

```bash
git add .github
git commit -m "ci: windows build/test workflow"
```

---

## Self-Review

- Spec coverage: hardware parity is partial (Milestone 0 = read-only vitals + UI). Performance modes, fan curves, RGB, network, hotkeys, helper are explicitly deferred to later milestones per this plan's scope; the design's phased delivery is preserved.
- Type consistency: `OmenDevice` signatures match Task 2; `WindowsDevice` implements them in Task 3.
- Review Focus items are addressed in tests/notes.
- Proportion: plan is shorter than the spec; bodies are signatures only.
