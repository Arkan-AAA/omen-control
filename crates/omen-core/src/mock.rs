use crate::device::OmenDevice;
use crate::error::OmenError;
use crate::types::*;
use std::sync::Mutex;

pub struct MockDevice {
    mode: Mutex<PerformanceMode>,
    fan: Mutex<FanCurve>,
}

impl Default for MockDevice {
    fn default() -> Self {
        Self {
            mode: Mutex::new(PerformanceMode::Balanced),
            fan: Mutex::new(FanCurve::default()),
        }
    }
}

impl OmenDevice for MockDevice {
    fn info(&self) -> DeviceInfo {
        DeviceInfo {
            manufacturer: "HP".into(),
            model: "Mock OMEN".into(),
            sku: "MOCK".into(),
            cpu: "Mock CPU".into(),
            gpu: vec!["Mock GPU".into()],
        }
    }

    fn vitals(&self) -> Result<Vitals, OmenError> {
        Ok(Vitals {
            cpu_temp_c: Some(42.0),
            gpu_temp_c: Some(50.0),
            fan_rpm: Some(1200),
            cpu_usage: Some(10.0),
            gpu_usage: Some(5.0),
        })
    }

    fn performance_modes(&self) -> &'static [PerformanceMode] {
        &[
            PerformanceMode::Balanced,
            PerformanceMode::Performance,
            PerformanceMode::Quiet,
        ]
    }

    fn current_mode(&self) -> PerformanceMode {
        *self.mode.lock().unwrap()
    }

    fn set_performance_mode(&self, mode: PerformanceMode) -> Result<(), OmenError> {
        *self.mode.lock().unwrap() = mode;
        Ok(())
    }

    fn fan_curve(&self) -> Result<FanCurve, OmenError> {
        Ok(self.fan.lock().unwrap().clone())
    }

    fn set_fan_curve(&self, curve: FanCurve) -> Result<(), OmenError> {
        *self.fan.lock().unwrap() = curve;
        Ok(())
    }

    fn rgb_zones(&self) -> Vec<RgbZone> {
        vec![]
    }

    fn set_rgb(&self, _zone: u8, _color: Rgb) -> Result<(), OmenError> {
        Ok(())
    }

    fn network_stats(&self) -> Result<NetworkStats, OmenError> {
        Ok(NetworkStats::default())
    }
}
