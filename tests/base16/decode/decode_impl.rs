use basekit::base16::{
    Base16DecodeConfig, Base16Error, DECODE_TABLE_BASE16_LOWERCASE, DECODE_TABLE_BASE16_UPPERCASE,
    decode,
};

fn create_config() -> Base16DecodeConfig {
    Base16DecodeConfig::new(DECODE_TABLE_BASE16_LOWERCASE)
}

fn create_config_uppercase() -> Base16DecodeConfig {
    Base16DecodeConfig::new(DECODE_TABLE_BASE16_UPPERCASE)
}

#[test]
fn test_empty() {
    let config = create_config();
    let result = decode(&config, b"");
    assert_eq!(Vec::<u8>::from(result.unwrap()), b"");
}

#[test]
fn test_single_byte_zero() {
    let config = create_config();
    let result = decode(&config, b"00");
    assert_eq!(Vec::<u8>::from(result.unwrap()), &[0x00]);
}

#[test]
fn test_single_byte_all_ones() {
    let config = create_config_uppercase();
    let result = decode(&config, b"FF");
    assert_eq!(Vec::<u8>::from(result.unwrap()), &[0xFF]);
}

#[test]
fn test_two_bytes() {
    let config = create_config_uppercase();
    let result = decode(&config, b"0AFF");
    assert_eq!(Vec::<u8>::from(result.unwrap()), &[0x0A, 0xFF]);
}

#[test]
fn test_three_bytes() {
    let config = create_config();
    let result = decode(&config, b"010203");
    assert_eq!(Vec::<u8>::from(result.unwrap()), &[0x01, 0x02, 0x03]);
}

#[test]
fn test_four_bytes() {
    let config = create_config_uppercase();
    let result = decode(&config, b"DEADBEEF");
    assert_eq!(Vec::<u8>::from(result.unwrap()), &[0xDE, 0xAD, 0xBE, 0xEF]);
}

#[test]
fn test_hello() {
    let config = create_config_uppercase();
    let result = decode(&config, b"48656C6C6F");
    assert_eq!(Vec::<u8>::from(result.unwrap()), b"Hello");
}

#[test]
fn test_all_zeros() {
    let config = create_config();
    let result = decode(&config, b"00000000");
    assert_eq!(Vec::<u8>::from(result.unwrap()), &[0, 0, 0, 0]);
}

#[test]
fn test_all_ones() {
    let config = create_config_uppercase();
    let result = decode(&config, b"FFFFFFFF");
    assert_eq!(Vec::<u8>::from(result.unwrap()), &[0xFF, 0xFF, 0xFF, 0xFF]);
}

#[test]
fn test_lowercase_decode_table() {
    let config = create_config();
    let result = decode(&config, b"deadbeef");
    assert_eq!(Vec::<u8>::from(result.unwrap()), &[0xDE, 0xAD, 0xBE, 0xEF]);
}

#[test]
fn test_lowercase_with_uppercase_table_fails() {
    let config = create_config_uppercase();
    let result = decode(&config, b"deadbeef");
    assert!(result.is_err());
}

#[test]
fn test_uppercase_with_lowercase_table_fails() {
    let config = create_config();
    let result = decode(&config, b"DEADBEEF");
    assert!(result.is_err());
}

#[test]
fn test_invalid_odd_length() {
    let config = create_config_uppercase();
    let result = decode(&config, b"F");
    assert!(matches!(result, Err(Base16Error::InvalidLength(1))));
}

#[test]
fn test_invalid_character() {
    let config = create_config_uppercase();
    let result = decode(&config, b"GG");
    assert!(matches!(
        result,
        Err(Base16Error::InvalidCharacter(b'G', 0))
    ));
}

#[test]
fn test_invalid_character_at_second_position() {
    let config = create_config_uppercase();
    let result = decode(&config, b"FG");
    assert!(matches!(
        result,
        Err(Base16Error::InvalidCharacter(b'G', 1))
    ));
}

#[test]
fn test_invalid_high_byte() {
    let config = create_config_uppercase();
    let result = decode(&config, &[b'F', 0xFF]);
    assert!(matches!(
        result,
        Err(Base16Error::InvalidCharacter(0xFF, 1))
    ));
}

#[test]
fn test_invalid_character_at_different_positions() {
    let config = create_config_uppercase();

    let result = decode(&config, b"!F");
    assert!(matches!(result, Err(Base16Error::InvalidCharacter(_, 0))));

    let result = decode(&config, b"F!");
    assert!(matches!(result, Err(Base16Error::InvalidCharacter(_, 1))));

    let result = decode(&config, b"FF!!");
    assert!(matches!(result, Err(Base16Error::InvalidCharacter(_, 2))));

    let result = decode(&config, b"!FFFFFFF");
    assert!(matches!(result, Err(Base16Error::InvalidCharacter(_, 0))));

    let result = decode(&config, b"FFFFFFF!");
    assert!(matches!(result, Err(Base16Error::InvalidCharacter(_, 7))));
}
