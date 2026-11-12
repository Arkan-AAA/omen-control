use omen_core::{Config, MockDevice, OmenDevice, PerformanceMode};

#[test]
fn mock_reports_vitals() {
    let d = MockDevice::default();
    let v = d.vitals().unwrap();
    assert!(v.cpu_temp_c.is_some());
}

#[test]
fn mock_switches_performance_mode() {
    let d = MockDevice::default();
    d.set_performance_mode(PerformanceMode::Performance)
        .unwrap();
    assert_eq!(d.current_mode(), PerformanceMode::Performance);
}

#[test]
fn config_roundtrips() {
    let c = Config::default();
    let s = toml::to_string(&c).unwrap();
    let back: Config = toml::from_str(&s).unwrap();
    assert_eq!(c, back);
}
