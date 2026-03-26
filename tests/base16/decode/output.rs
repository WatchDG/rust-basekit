use basekit::base16::{Base16DecodeConfig, DECODE_TABLE_BASE16_UPPERCASE, decode};

fn create_config() -> Base16DecodeConfig {
    Base16DecodeConfig::new(DECODE_TABLE_BASE16_UPPERCASE)
}

#[test]
fn test_decode_output_to_string_valid_utf8() {
    let config = create_config();
    let result = decode(&config, b"48656C6C6F");
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
fn test_decode_output_to_vec_then_string() {
    let config = create_config();
    let result = decode(&config, b"48656C6C6F");
    let output = result.unwrap();
    let bytes: Vec<u8> = output.into();
    let string = String::try_from(bytes).unwrap();
    assert_eq!(string, "Hello");
}
