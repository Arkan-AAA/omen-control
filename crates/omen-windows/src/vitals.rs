use omen_core::{OmenError, Vitals};
use serde::Deserialize;
use tracing::warn;
use wmi::WMIConnection;

#[allow(non_snake_case)]
#[derive(Deserialize)]
struct Thermal {
    CurrentTemperature: Option<u32>,
}

#[allow(non_snake_case)]
#[derive(Deserialize)]
struct Proc {
    LoadPercentage: Option<u8>,
}

#[allow(non_snake_case)]
#[derive(Deserialize)]
struct Fan {
    DesiredSpeed: Option<u32>,
}

pub fn tenths_kelvin_to_celsius(tt: u32) -> Option<f32> {
    let c = (tt as f32 / 10.0) - 273.15;
    if c.is_finite() && (0.0..=125.0).contains(&c) {
        Some(c)
    } else {
        None
    }
}

#[allow(clippy::collapsible_if)]
pub fn read_vitals() -> Vitals {
    let mut v = Vitals::default();

    match WMIConnection::with_namespace_path("ROOT\\WMI") {
        Ok(con) => match con
            .raw_query::<Thermal>("SELECT CurrentTemperature FROM MSAcpi_ThermalZoneTemperature")
        {
            Ok(thermals) => {
                if let Some(t) = thermals.into_iter().next() {
                    if let Some(tt) = t.CurrentTemperature {
                        v.cpu_temp_c = tenths_kelvin_to_celsius(tt);
                    }
                }
            }
            Err(e) => warn!("thermal zone query failed: {e}"),
        },
        Err(e) => warn!("wmi ROOT\\WMI connection failed: {e}"),
    }

    match WMIConnection::new() {
        Ok(con) => {
            match con.raw_query::<Proc>("SELECT LoadPercentage FROM Win32_Processor") {
                Ok(procs) => {
                    if let Some(p) = procs.into_iter().next() {
                        v.cpu_usage = p.LoadPercentage.map(|x| x as f32);
                    }
                }
                Err(e) => warn!("processor query failed: {e}"),
            }
            match con.raw_query::<Fan>("SELECT DesiredSpeed FROM Win32_Fan") {
                Ok(fans) => {
                    if let Some(f) = fans.into_iter().next() {
                        v.fan_rpm = f.DesiredSpeed;
                    }
                }
                Err(e) => warn!("fan query failed: {e}"),
            }
        }
        Err(e) => warn!("wmi CIMV2 connection failed: {e}"),
    }

    match nvml_wrapper::Nvml::init() {
        Ok(nvml) => match nvml.device_by_index(0) {
            Ok(device) => {
                if let Ok(temp) =
                    device.temperature(nvml_wrapper::enum_wrappers::device::TemperatureSensor::Gpu)
                {
                    v.gpu_temp_c = Some(temp as f32);
                }
                if let Ok(util) = device.utilization_rates() {
                    v.gpu_usage = Some(util.gpu as f32);
                }
            }
            Err(e) => warn!("nvml device lookup failed: {e}"),
        },
        Err(e) => warn!("nvml init failed: {e}"),
    }

    v
}

#[allow(dead_code)]
fn _omen_error(_e: OmenError) {}
