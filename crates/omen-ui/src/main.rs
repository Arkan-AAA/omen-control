mod app;

fn main() {
    tracing_subscriber::fmt::init();
    #[cfg(feature = "real")]
    let device: Box<dyn omen_core::OmenDevice> = match omen_windows::WindowsDevice::detect() {
        Ok(d) => Box::new(d),
        Err(e) => {
            tracing::error!("device detection failed: {}", e);
            Box::new(omen_core::MockDevice::default())
        }
    };
    #[cfg(not(feature = "real"))]
    let device: Box<dyn omen_core::OmenDevice> = Box::new(omen_core::MockDevice::default());
    app::run(device).unwrap();
}
