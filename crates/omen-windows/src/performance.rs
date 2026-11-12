use omen_core::{OmenError, PerformanceMode};
use std::process::Command;

const BALANCED: &str = "381b4222-f694-41f0-9685-ff5bb260df2e";
const HIGH_PERF: &str = "8c5e7fda-e8bf-4a96-9a85-a6e23a8c635c";
const POWER_SAVER: &str = "a1841308-3541-4fab-bc81-f71556f20b4a";

pub fn mode_to_guid(mode: PerformanceMode) -> &'static str {
    match mode {
        PerformanceMode::Balanced => BALANCED,
        PerformanceMode::Performance => HIGH_PERF,
        PerformanceMode::Quiet => POWER_SAVER,
    }
}

pub fn set_performance_mode(mode: PerformanceMode) -> Result<(), OmenError> {
    let guid = mode_to_guid(mode);
    let status = Command::new("powercfg")
        .args(["/setactive", guid])
        .status()
        .map_err(|e| OmenError::Io(e))?;
    if status.success() {
        Ok(())
    } else {
        Err(OmenError::Unsupported(format!(
            "powercfg /setactive {} failed (elevation required?)",
            guid
        )))
    }
}

pub fn current_mode() -> PerformanceMode {
    let out = Command::new("powercfg")
        .args(["/getactivescheme"])
        .output();
    if let Ok(o) = out {
        let s = String::from_utf8_lossy(&o.stdout);
        if s.contains(HIGH_PERF) {
            return PerformanceMode::Performance;
        }
        if s.contains(POWER_SAVER) {
            return PerformanceMode::Quiet;
        }
    }
    PerformanceMode::Balanced
}
