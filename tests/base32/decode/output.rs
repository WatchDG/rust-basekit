use basekit::base32::{Base32DecodeConfig, DECODE_TABLE_BASE32, decode};

fn create_config() -> Base32DecodeConfig {
    Base32DecodeConfig::new(DECODE_TABLE_BASE32, b'=')
}

#[test]
fn test_decode_output_to_string_valid_utf8() {
    let config = create_config();
    let result = decode(&config, b"MY======");
    let output = result.unwrap();
    let string = String::try_from(output).unwrap();
    assert_eq!(string, "f");
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
    let result = decode(&config, b"MZXW6YTB");
    let output = result.unwrap();
    let bytes: Vec<u8> = output.into();
    let string = String::try_from(bytes).unwrap();
    assert_eq!(string, "fooba");
}
