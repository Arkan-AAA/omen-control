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
