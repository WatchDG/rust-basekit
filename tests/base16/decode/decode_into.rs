use basekit::base16::{
    Base16DecodeConfig, Base16Error, DECODE_TABLE_BASE16_UPPERCASE, decode_into,
};

fn create_config() -> Base16DecodeConfig {
    Base16DecodeConfig::new(DECODE_TABLE_BASE16_UPPERCASE)
}

#[test]
fn test_empty() {
    let config = create_config();
    let mut output = vec![0u8; 0];
    let result = decode_into(&config, &mut output, b"");
    assert_eq!(result.unwrap(), 0);
    assert_eq!(output, b"");
}

#[test]
fn test_single_byte() {
    let config = create_config();
    let mut output = vec![0u8; 1];
    let result = decode_into(&config, &mut output, b"FF");
    assert_eq!(result.unwrap(), 1);
    assert_eq!(output, &[0xFF]);
}

#[test]
fn test_multiple_bytes() {
    let config = create_config();
    let mut output = vec![0u8; 4];
    let result = decode_into(&config, &mut output, b"DEADBEEF");
    assert_eq!(result.unwrap(), 4);
    assert_eq!(output, &[0xDE, 0xAD, 0xBE, 0xEF]);
}

#[test]
fn test_buffer_too_small() {
    let config = create_config();
    let mut output = vec![0u8; 0];
    let result = decode_into(&config, &mut output, b"FF");
    assert!(matches!(
        result,
        Err(Base16Error::DestinationBufferTooSmall {
            needed: 1,
            provided: 0
        })
    ));
}

#[test]
fn test_buffer_exactly_right_size() {
    let config = create_config();
    let mut output = vec![0u8; 2];
    let result = decode_into(&config, &mut output, b"0AFF");
    assert_eq!(result.unwrap(), 2);
    assert_eq!(output, &[0x0A, 0xFF]);
}

#[test]
fn test_buffer_larger_than_needed() {
    let config = create_config();
    let mut output = vec![0u8; 10];
    let result = decode_into(&config, &mut output, b"0AFF");
    assert_eq!(result.unwrap(), 2);
    assert_eq!(&output[..2], &[0x0A, 0xFF]);
}

#[test]
fn test_odd_length_error() {
    let config = create_config();
    let mut output = vec![0u8; 10];
    let result = decode_into(&config, &mut output, b"FFF");
    assert!(matches!(result, Err(Base16Error::InvalidLength(3))));
}

#[test]
fn test_invalid_character_error() {
    let config = create_config();
    let mut output = vec![0u8; 10];
    let result = decode_into(&config, &mut output, b"FG");
    assert!(matches!(
        result,
        Err(Base16Error::InvalidCharacter(b'G', 1))
    ));
}

#[test]
fn test_roundtrip() {
    let config = create_config();
    let data = b"Hello, World!";
    let encoded = "48656C6C6F2C20576F726C6421";
    let mut output = vec![0u8; data.len()];
    let _ = decode_into(&config, &mut output, encoded.as_bytes()).unwrap();
    assert_eq!(&output[..], data);
}
