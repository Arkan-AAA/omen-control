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

fn format_bytes(b: u64) -> String {
    if b >= 1_000_000_000 {
        format!("{:.2} GB", b as f64 / 1_000_000_000.0)
    } else if b >= 1_000_000 {
        format!("{:.2} MB", b as f64 / 1_000_000.0)
    } else if b >= 1_000 {
        format!("{:.2} KB", b as f64 / 1_000.0)
    } else {
        format!("{} B", b)
    }
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

    let weak_for_fan = ui.as_weak();
    let dev_for_fan = device.clone();
    ui.on_set_fan_speed(move |v| {
        let dev = dev_for_fan.clone();
        let weak = weak_for_fan.clone();
        std::thread::spawn(move || {
            let rpm = (v as u32).saturating_mul(50); // 0-100 -> 0-5000
            let msg = match dev.set_fan_speed(rpm) {
                Ok(_) => format!("Fan set to {} ({} rpm target)", v, rpm),
                Err(e) => format!("Fan failed: {}", e),
            };
            let w = weak.clone();
            slint::invoke_from_event_loop(move || {
                if let Some(ui) = w.upgrade() {
                    ui.set_fan_status(msg.into());
                }
            })
            .ok();
        });
    });
    let weak_for_light = ui.as_weak();
    let dev_for_light = device.clone();
    ui.on_set_lighting_color(move |hex| {
        let dev = dev_for_light.clone();
        let weak = weak_for_light.clone();
        std::thread::spawn(move || {
            let hex = hex.trim_start_matches('#');
            let (r, g, b) = if hex.len() == 6 {
                (
                    u8::from_str_radix(&hex[0..2], 16).unwrap_or(0),
                    u8::from_str_radix(&hex[2..4], 16).unwrap_or(0),
                    u8::from_str_radix(&hex[4..6], 16).unwrap_or(0),
                )
            } else {
                (0, 0, 0)
            };
            let msg = match dev.set_rgb(0, omen_core::Rgb { r, g, b }) {
                Ok(_) => format!("Color set to #{}", hex),
                Err(e) => format!("Lighting failed: {}", e),
            };
            let w = weak.clone();
            slint::invoke_from_event_loop(move || {
                if let Some(ui) = w.upgrade() {
                    ui.set_lighting_status(msg.into());
                }
            })
            .ok();
        });
    });
    let weak = ui.as_weak();
    let dev = device.clone();
    std::thread::spawn(move || {
        loop {
            let (cpu, gpu, fan, cpuu, gpuu, rx, tx) = match dev.vitals() {
                Ok(v) => {
                    let net = dev.network_stats().unwrap_or_default();
                    (
                        fmt_temp(v.cpu_temp_c),
                        fmt_temp(v.gpu_temp_c),
                        v.fan_rpm
                            .map(|x| x.to_string())
                            .unwrap_or_else(|| "n/a".into()),
                        fmt_pct(v.cpu_usage),
                        fmt_pct(v.gpu_usage),
                        format_bytes(net.rx_bytes),
                        format_bytes(net.tx_bytes),
                    )
                }
                Err(_) => (
                    "n/a".into(),
                    "n/a".into(),
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
                    ui.set_net_rx(rx.into());
                    ui.set_net_tx(tx.into());
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
