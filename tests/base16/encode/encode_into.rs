use basekit::base16::{ALPHABET_BASE16_UPPERCASE, Base16EncodeConfig, Base16Error, encode_into};

fn create_config() -> Base16EncodeConfig {
    Base16EncodeConfig::new(ALPHABET_BASE16_UPPERCASE)
}

#[test]
fn test_empty() {
    let config = create_config();
    let mut output = vec![0u8; 0];
    let result = encode_into(&config, &mut output, &[]);
    assert_eq!(result.unwrap(), 0);
    assert_eq!(output, b"");
}

#[test]
fn test_single_byte() {
    let config = create_config();
    let mut output = vec![0u8; 2];
    let result = encode_into(&config, &mut output, &[0xFF]);
    assert_eq!(result.unwrap(), 2);
    assert_eq!(output, b"FF");
}

#[test]
fn test_multiple_bytes() {
    let config = create_config();
    let mut output = vec![0u8; 8];
    let result = encode_into(&config, &mut output, &[0xDE, 0xAD, 0xBE, 0xEF]);
    assert_eq!(result.unwrap(), 8);
    assert_eq!(output, b"DEADBEEF");
}

#[test]
fn test_buffer_too_small() {
    let config = create_config();
    let mut output = vec![0u8; 1];
    let result = encode_into(&config, &mut output, &[0xFF]);
    assert!(matches!(
        result,
        Err(Base16Error::DestinationBufferTooSmall {
            needed: 2,
            provided: 1
        })
    ));
}

#[test]
fn test_buffer_exactly_right_size() {
    let config = create_config();
    let mut output = vec![0u8; 4];
    let result = encode_into(&config, &mut output, &[0x0A, 0xFF]);
    assert_eq!(result.unwrap(), 4);
    assert_eq!(output, b"0AFF");
}

#[test]
fn test_buffer_larger_than_needed() {
    let config = create_config();
    let mut output = vec![0u8; 10];
    let result = encode_into(&config, &mut output, &[0x0A, 0xFF]);
    assert_eq!(result.unwrap(), 4);
    assert_eq!(&output[..4], b"0AFF");
}

#[test]
fn test_roundtrip() {
    let config = create_config();
    let data = b"Hello, World!";
    let mut output = vec![0u8; data.len() * 2];
    let _ = encode_into(&config, &mut output, data);
    let encoded = String::from_utf8(output).unwrap();
    assert_eq!(encoded, "48656C6C6F2C20576F726C6421");
}
