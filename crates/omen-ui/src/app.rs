use omen_core::{OmenDevice, PerformanceMode};
use slint::ComponentHandle;
use std::sync::Arc;

slint::include_modules!();

fn fmt_temp(v: Option<f32>) -> String {
    v.map(|x| format!("{:.1} C", x))
        .unwrap_or_else(|| "n/a".into())
}

fn fmt_pct(v: Option<f32>) -> String {
    v.map(|x| format!("{:.0}%", x))
        .unwrap_or_else(|| "n/a".into())
}

pub fn run(device: Box<dyn OmenDevice>) -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;
    let device = Arc::new(device);

    let weak_for_cb = ui.as_weak();
    let dev_for_cb = device.clone();
    ui.on_set_performance_mode(move |mode| {
        let dev = dev_for_cb.clone();
        let weak = weak_for_cb.clone();
        std::thread::spawn(move || {
            let pm = match mode.as_str() {
                "Performance" => PerformanceMode::Performance,
                "Quiet" => PerformanceMode::Quiet,
                _ => PerformanceMode::Balanced,
            };
            let msg = match dev.set_performance_mode(pm) {
                Ok(_) => format!("Mode set to {:?}", pm),
                Err(e) => format!("Failed: {}", e),
            };
            let w = weak.clone();
            slint::invoke_from_event_loop(move || {
                if let Some(ui) = w.upgrade() {
                    ui.set_perf_status(msg.into());
                }
            })
            .ok();
        });
    });

    let weak = ui.as_weak();
    let dev = device.clone();
    std::thread::spawn(move || {
        loop {
            let (cpu, gpu, fan, cpuu, gpuu) = match dev.vitals() {
                Ok(v) => (
                    fmt_temp(v.cpu_temp_c),
                    fmt_temp(v.gpu_temp_c),
                    v.fan_rpm
                        .map(|x| x.to_string())
                        .unwrap_or_else(|| "n/a".into()),
                    fmt_pct(v.cpu_usage),
                    fmt_pct(v.gpu_usage),
                ),
                Err(_) => (
                    "n/a".into(),
                    "n/a".into(),
                    "n/a".into(),
                    "n/a".into(),
                    "n/a".into(),
                ),
            };

            let wt = weak.clone();
            let result = slint::invoke_from_event_loop(move || {
                if let Some(ui) = wt.upgrade() {
                    ui.set_cpu_temp(cpu.into());
                    ui.set_gpu_temp(gpu.into());
                    ui.set_fan_rpm(fan.into());
                    ui.set_cpu_usage(cpuu.into());
                    ui.set_gpu_usage(gpuu.into());
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
