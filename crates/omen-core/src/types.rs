use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DeviceInfo {
    pub manufacturer: String,
    pub model: String,
    pub sku: String,
    pub cpu: String,
    pub gpu: Vec<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct Vitals {
    pub cpu_temp_c: Option<f32>,
    pub gpu_temp_c: Option<f32>,
    pub fan_rpm: Option<u32>,
    pub cpu_usage: Option<f32>,
    pub gpu_usage: Option<f32>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum PerformanceMode {
    Balanced,
    Performance,
    Quiet,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct FanCurve {
    pub points: Vec<(u8, u8)>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub struct Rgb {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RgbZone {
    pub id: u8,
    pub name: String,
    pub color: Rgb,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct NetworkStats {
    pub rx_bytes: u64,
    pub tx_bytes: u64,
}
