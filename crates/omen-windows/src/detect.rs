use omen_core::{DeviceInfo, OmenError};
use serde::Deserialize;
use wmi::WMIConnection;

#[derive(Deserialize)]
struct Cs {
    Manufacturer: String,
    Model: String,
    SystemSKUNumber: Option<String>,
}

#[derive(Deserialize)]
struct Cpu {
    Name: String,
}

#[derive(Deserialize)]
struct Gpu {
    Name: String,
}

pub fn parse_device_info(
    manufacturer: &str,
    model: &str,
    sku: &str,
    cpu: &str,
    gpu: Vec<String>,
) -> DeviceInfo {
    DeviceInfo {
        manufacturer: manufacturer.to_string(),
        model: model.to_string(),
        sku: sku.to_string(),
        cpu: cpu.to_string(),
        gpu,
    }
}

pub fn detect_model() -> Result<DeviceInfo, OmenError> {
    let con = WMIConnection::new().map_err(|e| OmenError::Unavailable(e.to_string()))?;
    let mut cs_list: Vec<Cs> = con
        .raw_query("SELECT Manufacturer, Model, SystemSKUNumber FROM Win32_ComputerSystem")
        .map_err(|e| OmenError::Unavailable(e.to_string()))?;
    let cs = cs_list
        .pop()
        .ok_or_else(|| OmenError::Unavailable("no computer system".into()))?;
    let cpus: Vec<Cpu> = con
        .raw_query("SELECT Name FROM Win32_Processor")
        .map_err(|e| OmenError::Unavailable(e.to_string()))?;
    let cpu = cpus.into_iter().next().map(|c| c.Name).unwrap_or_default();
    let gpus: Vec<Gpu> = con
        .raw_query("SELECT Name FROM Win32_VideoController")
        .map_err(|e| OmenError::Unavailable(e.to_string()))?;
    let gpu = gpus.into_iter().map(|g| g.Name).collect();
    Ok(parse_device_info(
        &cs.Manufacturer,
        &cs.Model,
        cs.SystemSKUNumber.as_deref().unwrap_or(""),
        &cpu,
        gpu,
    ))
}
