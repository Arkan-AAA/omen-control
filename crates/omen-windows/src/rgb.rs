use omen_core::{OmenError, Rgb};
use std::process::Command;

const SDK_PATH: &str = "C:\\Program Files\\HP\\OmenInstallMonitor\\TyphonChasisLightingSDK.dll";

pub fn set_rgb(_zone: u8, color: Rgb) -> Result<(), OmenError> {
    if !std::path::Path::new(SDK_PATH).exists() {
        return Err(OmenError::Unsupported(
            "TyphonChasisLightingSDK.dll not found".into(),
        ));
    }
    let script = format!(
        "Add-Type -Path '{sdk}'; \
         $z = New-Object TyphonChasisLightingSDK.LedZoneColorInfo; \
         $z.Red = {r}; $z.Green = {g}; $z.Blue = {b}; \
         $zs = [TyphonChasisLightingSDK.LedZoneColorInfo[]]@($z); \
         $t = [TyphonChasisLightingSDK.TyphonChasisLightingHelper]::SetStaticColorMode(0, $zs, 255, 0, 0, 0); \
         $t.Wait(); Write-Output $t.Result",
        sdk = SDK_PATH,
        r = color.r,
        g = color.g,
        b = color.b
    );
    let out = Command::new("powershell")
        .args(["-NoProfile", "-Command", &script])
        .output()
        .map_err(OmenError::Io)?;
    let stdout = String::from_utf8_lossy(&out.stdout);
    if out.status.success() && stdout.trim().eq_ignore_ascii_case("true") {
        Ok(())
    } else {
        Err(OmenError::Other(format!(
            "RGB set failed: {}",
            stdout.trim()
        )))
    }
}
