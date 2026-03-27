use basekit::base64::{Base64DecodeConfig, DECODE_TABLE_BASE64, decode};

fn create_config() -> Base64DecodeConfig {
    Base64DecodeConfig::new(DECODE_TABLE_BASE64, Some(b'='))
}

#[test]
fn test_decode_output_to_string_valid_utf8() {
    let config = create_config();
    let result = decode(&config, b"SGVsbG8=");
    let output = result.unwrap();
    let string = String::try_from(output).unwrap();
    assert_eq!(string, "Hello");
}

#[test]
fn test_decode_output_to_string_empty() {
    let config = create_config();
    let result = decode(&config, b"");
    let output = result.unwrap();
    let string = String::try_from(output).unwrap();
    assert_eq!(string, "");
}

#[test]
fn test_decode_output_to_string_longer_text() {
    let config = create_config();
    let result = decode(&config, b"SGVsbG8gV29ybGQh");
    let output = result.unwrap();
    let string = String::try_from(output).unwrap();
    assert_eq!(string, "Hello World!");
}

#[test]
fn test_decode_output_to_string_special_chars() {
    let config = create_config();
    let result = decode(&config, b"IyBuZXcgZmVhdHVyZSE=");
    let output = result.unwrap();
    let string = String::try_from(output).unwrap();
    assert_eq!(string, "# new feature!");
}

#[test]
fn test_decode_output_to_vec_then_string() {
    let config = create_config();
    let result = decode(&config, b"SGVsbG8=");
    let output = result.unwrap();
    let bytes: Vec<u8> = output.into();
    let string = String::try_from(bytes).unwrap();
    assert_eq!(string, "Hello");
}
