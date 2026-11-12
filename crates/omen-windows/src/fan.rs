use omen_core::OmenError;
use serde::Deserialize;
use wmi::WMIConnection;

#[allow(non_snake_case)]
#[derive(Deserialize)]
struct Win32Fan {
    __Path: String,
}

#[allow(non_snake_case)]
#[derive(Deserialize)]
struct SetSpeedOut {
    ReturnValue: u32,
}

#[allow(non_snake_case)]
#[derive(serde::Serialize)]
struct SetSpeedIn {
    DesiredSpeed: u64,
}

pub fn set_fan_speed(rpm: u32) -> Result<(), OmenError> {
    let con = WMIConnection::new().map_err(|e| OmenError::Unavailable(e.to_string()))?;
    let fans: Vec<Win32Fan> = con
        .raw_query("SELECT __Path FROM Win32_Fan")
        .map_err(|e| OmenError::Unavailable(e.to_string()))?;
    let fan = fans
        .into_iter()
        .next()
        .ok_or_else(|| OmenError::Unavailable("no Win32_Fan instance".into()))?;
    let out: SetSpeedOut = con
        .exec_instance_method::<Win32Fan, SetSpeedOut>(
            &fan.__Path,
            "SetSpeed",
            SetSpeedIn {
                DesiredSpeed: rpm as u64,
            },
        )
        .map_err(|e| OmenError::Other(e.to_string()))?;
    if out.ReturnValue == 0 {
        Ok(())
    } else {
        Err(OmenError::Other(format!(
            "SetSpeed returned {}",
            out.ReturnValue
        )))
    }
}
