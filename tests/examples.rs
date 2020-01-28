#[test]
fn all() {
    for file in glob::glob("resources/cti-stix2-json-schemas/examples/**/*.json").unwrap() {
        let file = file.unwrap();
        let s = std::fs::read_to_string(file).unwrap();
        let _: stix2::Bundle = serde_json::from_str(&s).unwrap();
    }
}

#[test]
fn std_fmt() {
    use std::str::FromStr;
    for file in glob::glob("resources/cti-stix2-json-schemas/examples/**/*.json").unwrap() {
        let file = file.unwrap();
        let s = std::fs::read_to_string(file).unwrap();
        let _ = stix2::Bundle::from_str(&s).unwrap();
    }
}
