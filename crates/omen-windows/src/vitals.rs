use omen_core::Vitals;
use serde::Deserialize;
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

#[allow(clippy::collapsible_if)]
pub fn read_vitals() -> Vitals {
    let mut v = Vitals::default();

    if let Ok(con) = WMIConnection::with_namespace_path("ROOT\\WMI") {
        if let Ok(mut thermals) =
            con.raw_query::<Thermal>("SELECT CurrentTemperature FROM MSAcpi_ThermalZoneTemperature")
        {
            if let Some(t) = thermals.pop() {
                if let Some(tt) = t.CurrentTemperature {
                    v.cpu_temp_c = Some((tt as f32 / 10.0) - 273.15);
                }
            }
        }
    }

    if let Ok(con) = WMIConnection::new() {
        if let Ok(mut procs) = con.raw_query::<Proc>("SELECT LoadPercentage FROM Win32_Processor") {
            if let Some(p) = procs.pop() {
                v.cpu_usage = p.LoadPercentage.map(|x| x as f32);
            }
        }
    }

    if let Ok(nvml) = nvml_wrapper::Nvml::init() {
        if let Ok(device) = nvml.device_by_index(0) {
            if let Ok(temp) =
                device.temperature(nvml_wrapper::enum_wrappers::device::TemperatureSensor::Gpu)
            {
                v.gpu_temp_c = Some(temp as f32);
            }
            if let Ok(util) = device.utilization_rates() {
                v.gpu_usage = Some(util.gpu as f32);
            }
        }
    }

    v
}
