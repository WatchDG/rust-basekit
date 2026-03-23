use basekit::base16::{ALPHABET_BASE16_UPPERCASE, Base16EncodeConfig, encode};

fn create_config() -> Base16EncodeConfig {
    Base16EncodeConfig::new(ALPHABET_BASE16_UPPERCASE)
}

#[test]
fn test_empty() {
    let config = create_config();
    let result = encode(&config, &[]);
    assert_eq!(result, b"");
}

#[test]
fn test_single_byte_zero() {
    let config = create_config();
    let result = encode(&config, &[0x00]);
    assert_eq!(result, b"00");
}

#[test]
fn test_single_byte_all_ones() {
    let config = create_config();
    let result = encode(&config, &[0xFF]);
    assert_eq!(result, b"FF");
}

#[test]
fn test_single_byte_value_10() {
    let config = create_config();
    let result = encode(&config, &[0x0A]);
    assert_eq!(result, b"0A");
}

#[test]
fn test_two_bytes() {
    let config = create_config();
    let result = encode(&config, &[0x0A, 0xFF]);
    assert_eq!(result, b"0AFF");
}

#[test]
fn test_three_bytes() {
    let config = create_config();
    let result = encode(&config, &[0x01, 0x02, 0x03]);
    assert_eq!(result, b"010203");
}

#[test]
fn test_four_bytes() {
    let config = create_config();
    let result = encode(&config, &[0xDE, 0xAD, 0xBE, 0xEF]);
    assert_eq!(result, b"DEADBEEF");
}

#[test]
fn test_hello() {
    let config = create_config();
    let result = encode(&config, b"Hello");
    assert_eq!(result, b"48656C6C6F");
}

#[test]
fn test_all_zeros() {
    let config = create_config();
    let result = encode(&config, &[0, 0, 0, 0]);
    assert_eq!(result, b"00000000");
}

#[test]
fn test_all_ones() {
    let config = create_config();
    let result = encode(&config, &[0xFF, 0xFF, 0xFF, 0xFF]);
    assert_eq!(result, b"FFFFFFFF");
}

#[test]
fn test_alternating() {
    let config = create_config();
    let result = encode(&config, &[0xAA, 0x55, 0xAA, 0x55]);
    assert_eq!(result, b"AA55AA55");
}

#[test]
fn test_large_random() {
    let config = create_config();
    let data: Vec<u8> = (0..1024).map(|i| (i % 256) as u8).collect();
    encode(&config, &data);
}

#[test]
fn test_1mb_random() {
    let config = create_config();
    let data: Vec<u8> = (0..1024 * 1024).map(|i| (i % 256) as u8).collect();
    encode(&config, &data);
}
