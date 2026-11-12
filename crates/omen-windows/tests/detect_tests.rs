use omen_windows::detect::parse_device_info;

#[test]
fn parses_omen_model() {
    let info = parse_device_info(
        "HP",
        "HyperX OMEN 15",
        "D57E7UA#ABA",
        "Intel Core Ultra 7 356H",
        vec!["NVIDIA GeForce RTX 5050 Laptop GPU".into()],
    );
    assert_eq!(info.model, "HyperX OMEN 15");
    assert!(info.gpu.iter().any(|g| g.contains("RTX 5050")));
}
