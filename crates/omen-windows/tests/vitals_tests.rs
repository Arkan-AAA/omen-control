use omen_windows::vitals::tenths_kelvin_to_celsius;

#[test]
fn converts_valid_temp() {
    // 3000 tenths K = 300.0 K = 26.85 C
    let c = tenths_kelvin_to_celsius(3000).unwrap();
    assert!((c - 26.85).abs() < 0.01);
}

#[test]
fn rejects_out_of_range() {
    assert!(tenths_kelvin_to_celsius(0).is_none()); // -273 C
    assert!(tenths_kelvin_to_celsius(100000).is_none()); // huge
}
