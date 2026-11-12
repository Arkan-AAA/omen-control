mod app;

fn main() {
    tracing_subscriber::fmt::init();
    #[cfg(feature = "real")]
    let device: Box<dyn omen_core::OmenDevice> =
        Box::new(omen_windows::WindowsDevice::detect().expect("detect"));
    #[cfg(not(feature = "real"))]
    let device: Box<dyn omen_core::OmenDevice> = Box::new(omen_core::MockDevice::default());
    app::run(device).unwrap();
}
