use omen_core::{
    DeviceInfo, FanCurve, NetworkStats, OmenDevice, OmenError, PerformanceMode, Rgb, RgbZone,
    Vitals,
};

pub mod detect;
pub mod fan;
pub mod performance;
pub mod vitals;

pub struct WindowsDevice {
    info: DeviceInfo,
}

impl WindowsDevice {
    pub fn detect() -> Result<Self, OmenError> {
        Ok(Self {
            info: detect::detect_model()?,
        })
    }

    pub fn read_vitals(&self) -> Vitals {
        vitals::read_vitals()
    }
}

impl OmenDevice for WindowsDevice {
    fn info(&self) -> DeviceInfo {
        self.info.clone()
    }

    fn vitals(&self) -> Result<Vitals, OmenError> {
        Ok(self.read_vitals())
    }

    fn performance_modes(&self) -> &'static [PerformanceMode] {
        &[
            PerformanceMode::Balanced,
            PerformanceMode::Performance,
            PerformanceMode::Quiet,
        ]
    }

    fn current_mode(&self) -> PerformanceMode {
        performance::current_mode()
    }

    fn set_performance_mode(&self, mode: PerformanceMode) -> Result<(), OmenError> {
        performance::set_performance_mode(mode)
    }

    fn fan_curve(&self) -> Result<FanCurve, OmenError> {
        Err(OmenError::Unsupported(
            "fan curve read requires hardware support (next milestone)".into(),
        ))
    }

    fn set_fan_curve(&self, _curve: FanCurve) -> Result<(), OmenError> {
        Err(OmenError::Unsupported(
            "fan curve writes require helper (next milestone)".into(),
        ))
    }

    fn set_fan_speed(&self, rpm: u32) -> Result<(), OmenError> {
        fan::set_fan_speed(rpm)
    }

    fn rgb_zones(&self) -> Vec<RgbZone> {
        vec![]
    }

    fn set_rgb(&self, _zone: u8, _color: Rgb) -> Result<(), OmenError> {
        Err(OmenError::Unsupported(
            "rgb writes require helper (next milestone)".into(),
        ))
    }

    fn network_stats(&self) -> Result<NetworkStats, OmenError> {
        use sysinfo::Networks;
        let networks = Networks::new_with_refreshed_list().unwrap_or_else(|_| Networks::new().unwrap());
        let mut rx = 0u64;
        let mut tx = 0u64;
        for (_name, net) in &networks {
            rx = rx.saturating_add(net.total_received());
            tx = tx.saturating_add(net.total_transmitted());
        }
        Ok(NetworkStats { rx_bytes: rx, tx_bytes: tx })
    }
}
