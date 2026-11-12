# Design: OMEN Control (working name) — Rust rewrite of OMEN Gaming Hub

Date: 2026-11-12
Status: Draft for approval

## Intent and why

Build an open-source, native Rust replacement for HP OMEN Gaming Hub focused on
**hardware control** for HP OMEN / HyperX laptops: performance profiles, RGB
lighting, fan control, system vitals, network booster. Target users are people
moving to Linux (especially Arch/CachyOS) who still want OMEN-style control,
plus Windows users who want a lighter, faster client. No HP account, cloud,
firmware updates, game library, or streaming features — those are out of scope
by decision (hardware parity only).

Inspiration: ArtCraft — pure Rust, native (no web views), open source, local-first.

## Goals

- Native Rust desktop app, cross-platform: **Windows first, then Linux (Arch/CachyOS)**.
- Hardware parity for: Performance Control, Lighting/RGB, Fan Control, System Vitals,
  Network Booster (best-effort), performance tuning settings.
- Styled, modern UI (Slint), gaming aesthetic, dark theme by default.
- Modular device abstraction so new OMEN/HyperX models can be added without
  rewriting the app.
- Open source on GitHub with reproducible builds and release artifacts.

## Non-goals

- HP account / cloud services / telemetry.
- Firmware / BIOS updates.
- Game library, launcher, streaming (Oasis).
- macOS support (not requested; architecture may allow later).
- Desktop OMEN machines in v1 (laptops first).

## Anti-goals (what would make this a failure even if it "works")

- Requiring the whole app to run as root/admin for normal use.
- Bricking or destabilizing hardware via unvalidated EC/ACPI writes.
- Hard-coding one model's protocol so adding a second model needs a rewrite.
- Shipping a web view / Electron-style wrapper (must stay pure native).

## Constraints

- No official HP drivers/APIs on Linux; must rely on community reverse
  engineering (omen-rgb, hp-omen-linux, nbfc, ryzenadj, lm-sensors, hidraw,
  sysfs, ACPI/EC).
- Windows side can use WMI (HpWmi), HID, NVML, but still needs reverse
  engineering for some calls.
- Privileged hardware access must be isolated (polkit helper on Linux,
  elevated helper service on Windows) so the UI runs unprivileged.
- Must not violate HP EULA in a way that blocks personal use / open sourcing
  of our own code; we ship only our own Rust code, never HP binaries.

## Approaches considered

1. **Full clone from scratch** (all protocols reversed, everything native).
   - Pros: maximum control, no external deps.
   - Cons: years of work, high risk of hardware damage, blocks shipping.
   - Verdict: rejected as primary; absorb incrementally.

2. **Linux-first wrapper over community tools + Rust GUI** (call omen-rgb,
   nbfc, ryzenadj as subprocesses).
   - Pros: fast MVP, leverages known-working protocols.
   - Cons: fragile UX, process overhead, poor error reporting.
   - Verdict: used as *tactic* for Linux hardware access later, behind a clean trait.

3. **Cross-platform core + native platform backends + Slint UI** (recommended).
   - Pros: testable core, shared UI, Windows and Linux both first-class,
     models added as data + small backend impls.
   - Cons: more upfront design, two backends to maintain.
   - Verdict: **chosen**.

## Chosen approach

Approach 3, with a Windows-first delivery: build and validate the Windows
backend first, keep the Linux backend behind the same trait for phase 3.

## Architecture

Cargo workspace:

```
omen-control/
  crates/
    omen-core/          # traits, types, errors, device model, config
    omen-platform/      # cfg-gated re-exports of platform backends
    omen-windows/       # Windows backend (WMI, HID, NVML)  [FIRST]
    omen-linux/         # Linux backend (hidraw, sysfs, polkit helper client)
    omen-ui/            # Slint app, screens, theming
    omen-helper/        # privileged helper binary (Windows service / Linux polkit)
  packaging/            # Windows installer, PKGBUILD, .deb/.rpm
  docs/
```

Key trait (in `omen-core`):

```rust
pub trait OmenDevice: Send + Sync {
    fn info(&self) -> DeviceInfo;
    fn vitals(&self) -> Result<Vitals>;
    fn performance_modes(&self) -> &[PerformanceMode];
    fn set_performance_mode(&self, mode: PerformanceMode) -> Result<()>;
    fn fan_curve(&self) -> Result<FanCurve>;
    fn set_fan_curve(&self, curve: FanCurve) -> Result<()>;
    fn rgb_zones(&self) -> Vec<RgbZone>;
    fn set_rgb(&self, zone: RgbZoneId, color: Rgb) -> Result<()>;
    fn network_stats(&self) -> Result<NetworkStats>;
}
```

Backends implement this trait per model family. Model detection happens at
startup via SMBIOS (Windows: WMI `Win32_ComputerSystem`/`Win32_BaseBoard`,
Linux: `/sys/class/dmi/id/*`).

## UI (Slint)

Screens:
- Dashboard (vitals overview, current mode, quick toggles)
- Performance (mode selector, CPU/GPU power limits, fan curve editor)
- Lighting (per-zone RGB, effects, brightness)
- Fans (curve editor with graph, manual override)
- System Vitals (detailed sensors, history graph)
- Network (throughput, per-process, basic prioritization)
- Settings (autostart, theme, update check)

Theming: dark gaming theme, accent color configurable, compact layout for
laptop screens.

## Privilege model

- UI process runs as normal user.
- Privileged operations go through `omen-helper`, a small separate binary:
  - Windows (first): a Windows service running as SYSTEM with a named pipe; UI
    connects and sends typed requests.
  - Linux (later): polkit action `com.omencontrol.helper`, helper invoked via
    D-Bus or pkexec; validates and allowlists commands.
- Helper only exposes typed, validated operations (set mode, write fan curve,
  write RGB packet) — never arbitrary shell/root.

## Data & config

- User config: TOML in OS config dir (`~/.config/omen-control/` / `%APPDATA%`).
- Profiles: named performance/fan/RGB presets, exportable/importable.
- Logs: tracing to file + stderr, rotated.

## Platform specifics

Windows (first):
- WMI for system info and HP-specific classes, NVML for NVIDIA, HID for RGB.
- Packaging: MSI or portable zip via cargo-wix/nsis, GitHub Actions.

Linux (CachyOS/Arch, later):
- DMI detection, hwmon for temps/fans, hidraw for RGB, ryzenadj/ectool for
  power (via helper), tc/nftables for network prioritization (best-effort).
- Packaging: PKGBUILD (AUR), plus .deb/.rpm for Ubuntu/Fedora.
- Dependencies: lm-sensors, ryzenadj, (optional) nbfc.

## Testing strategy

- Unit tests in `omen-core` for types, config, parsing, fan-curve math.
- Mock device implementation for UI development without hardware.
- Hardware-in-the-loop: manual checklist per model family, documented in
  `docs/hardware-testing.md`.
- CI: cargo fmt, clippy, test on Windows (and Linux later); build release artifacts.

## Phased delivery (Windows first)

Phase 0 — Repo scaffold, CI, Slint hello-world, mock device (1-2 weeks)
Phase 1 — Windows vitals + performance mode switching (2-4 weeks)
Phase 2 — Windows fan control + RGB on user's own model (3-6 weeks)
Phase 3 — Linux (CachyOS/Arch) backend parity (3-6 weeks)
Phase 4 — Network booster, profiles, polish, packaging, docs (2-4 weeks)
Phase 5 — Additional models (community-driven)

## What is left to the builder (implementation details)

- Exact HID packet formats per model (must be reverse-engineered; start from
  user's own machine and community projects).
- Choice of fan-curve interpolation and safety clamps.
- Whether to use ryzenadj vs direct MSR writes (Linux phase).
- Network booster scope (monitoring only vs QoS) — recommend monitoring +
  simple per-process priority first.
- Exact Slint component library / icon set.

## Target hardware (v1)

- Primary (and first) target: **HyperX OMEN 15 (15-ga0099nr)**,
  SKU D57E7UA#ABA, Intel Core Ultra 7 356H, Intel Graphics + NVIDIA
  GeForce RTX 5050 Laptop GPU.
- Additional OMEN/HyperX models added incrementally in later phases.
- Hotkeys / background daemon included in v1.

## Open questions (remaining)

- Preferred accent color / visual style references (decidable later; theming isolated).

## Approval

Pending user approval of this design before planning/implementation.
