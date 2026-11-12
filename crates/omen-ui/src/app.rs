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

    let weak = ui.as_weak();
    let dev = device.clone();
    std::thread::spawn(move || {
        loop {
            let (cpu, gpu, fan) = match dev.vitals() {
                Ok(v) => (
                    fmt_temp(v.cpu_temp_c),
                    fmt_temp(v.gpu_temp_c),
                    v.fan_rpm
                        .map(|x| x.to_string())
                        .unwrap_or_else(|| "n/a".into()),
                ),
                Err(_) => ("n/a".into(), "n/a".into(), "n/a".into()),
            };

            let wt = weak.clone();
            let result = slint::invoke_from_event_loop(move || {
                if let Some(ui) = wt.upgrade() {
                    ui.set_cpu_temp(cpu.into());
                    ui.set_gpu_temp(gpu.into());
                    ui.set_fan_rpm(fan.into());
                }
            });
            if result.is_err() {
                break;
            }
            if weak.upgrade().is_none() {
                break;
            }
            std::thread::sleep(std::time::Duration::from_secs(2));
        }
    });

    ui.run()
}
