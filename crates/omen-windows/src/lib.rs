use omen_core::{
    DeviceInfo, FanCurve, NetworkStats, OmenDevice, OmenError, PerformanceMode, Rgb, RgbZone,
    Vitals,
};

pub mod detect;
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
        PerformanceMode::Balanced
    }

    fn set_performance_mode(&self, _mode: PerformanceMode) -> Result<(), OmenError> {
        Err(OmenError::Unsupported(
            "performance mode switching requires helper (next milestone)".into(),
        ))
    }

    fn fan_curve(&self) -> Result<FanCurve, OmenError> {
        Ok(FanCurve::default())
    }

    fn set_fan_curve(&self, _curve: FanCurve) -> Result<(), OmenError> {
        Err(OmenError::Unsupported(
            "fan curve writes require helper (next milestone)".into(),
        ))
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
        Ok(NetworkStats::default())
    }
}
