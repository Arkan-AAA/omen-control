use omen_core::OmenDevice;
use slint::ComponentHandle;
use std::sync::Arc;

slint::include_modules!();

fn fmt_temp(v: Option<f32>) -> String {
    v.map(|x| format!("{:.1} C", x))
        .unwrap_or_else(|| "n/a".into())
}

pub fn run(device: Box<dyn OmenDevice>) -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;
    let device = Arc::new(device);

    if let Ok(v) = device.vitals() {
        ui.set_cpu_temp(fmt_temp(v.cpu_temp_c).into());
        ui.set_gpu_temp(fmt_temp(v.gpu_temp_c).into());
        ui.set_fan_rpm(
            v.fan_rpm
                .map(|x| x.to_string())
                .unwrap_or_else(|| "n/a".into())
                .into(),
        );
    }

    let weak = ui.as_weak();
    let dev = device.clone();
    std::thread::spawn(move || {
        loop {
            std::thread::sleep(std::time::Duration::from_secs(2));
            if let Ok(v) = dev.vitals() {
                let wt = weak.clone();
                let cpu = fmt_temp(v.cpu_temp_c);
                let gpu = fmt_temp(v.gpu_temp_c);
                let fan = v
                    .fan_rpm
                    .map(|x| x.to_string())
                    .unwrap_or_else(|| "n/a".into());
                slint::invoke_from_event_loop(move || {
                    if let Some(ui) = wt.upgrade() {
                        ui.set_cpu_temp(cpu.into());
                        ui.set_gpu_temp(gpu.into());
                        ui.set_fan_rpm(fan.into());
                    }
                })
                .ok();
            }
        }
    });

    ui.run()
}
