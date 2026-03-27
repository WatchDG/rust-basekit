use basekit::base64::{Base64DecodeConfig, Base64Error, DECODE_TABLE_BASE64, decode};

fn create_config() -> Base64DecodeConfig {
    Base64DecodeConfig::new(DECODE_TABLE_BASE64, Some(b'='))
}

#[test]
fn test_empty() {
    let config = create_config();
    let result = decode(&config, b"");
    assert_eq!(Vec::<u8>::from(result.unwrap()), b"");
}

#[test]
fn test_single_byte() {
    let config = create_config();
    let result = decode(&config, b"Zg==");
    assert_eq!(Vec::<u8>::from(result.unwrap()), &[102]);
}

#[test]
fn test_two_bytes() {
    let config = create_config();
    let result = decode(&config, b"Zm8=");
    assert_eq!(Vec::<u8>::from(result.unwrap()), &[102, 111]);
}

#[test]
fn test_three_bytes() {
    let config = create_config();
    let result = decode(&config, b"Zm9v");
    assert_eq!(Vec::<u8>::from(result.unwrap()), &[102, 111, 111]);
}

#[test]
fn test_four_bytes() {
    let config = create_config();
    let result = decode(&config, b"Zm9vYg==");
    assert_eq!(Vec::<u8>::from(result.unwrap()), &[102, 111, 111, 98]);
}

#[test]
fn test_five_bytes() {
    let config = create_config();
    let result = decode(&config, b"Zm9vYmE=");
    assert_eq!(Vec::<u8>::from(result.unwrap()), &[102, 111, 111, 98, 97]);
}

#[test]
fn test_six_bytes() {
    let config = create_config();
    let result = decode(&config, b"Zm9vYmFy");
    assert_eq!(
        Vec::<u8>::from(result.unwrap()),
        &[102, 111, 111, 98, 97, 114]
    );
}

#[test]
fn test_hello() {
    let config = create_config();
    let result = decode(&config, b"SGVsbG8=");
    assert_eq!(Vec::<u8>::from(result.unwrap()), b"Hello");
}

#[test]
fn test_all_zeros() {
    let config = create_config();
    let result = decode(&config, b"AAAA");
    assert_eq!(Vec::<u8>::from(result.unwrap()), &[0, 0, 0]);
}

#[test]
fn test_all_ones() {
    let config = create_config();
    let result = decode(&config, b"////");
    assert_eq!(Vec::<u8>::from(result.unwrap()), &[0xFF, 0xFF, 0xFF]);
}

#[test]
fn test_invalid_character() {
    let config = create_config();
    let result = decode(&config, b"Zm9v!");
    assert!(matches!(result, Err(Base64Error::InvalidCharacter(_, _))));
}

#[test]
fn test_invalid_character_high_byte() {
    let config = create_config();
    let result = decode(&config, b"Zm\xFFv");
    assert!(matches!(
        result,
        Err(Base64Error::InvalidCharacter(0xFF, _))
    ));
}

#[test]
fn test_invalid_padding_position() {
    let config = create_config();
    let result = decode(&config, b"=Zm9v");
    assert!(matches!(result, Err(Base64Error::InvalidPadding)));
}

#[test]
fn test_too_much_padding() {
    let config = create_config();
    let result = decode(&config, b"Zm9v===");
    assert!(matches!(result, Err(Base64Error::InvalidPadding)));
}

#[test]
fn test_invalid_character_at_different_positions() {
    let config = create_config();

    let result = decode(&config, b"!m9vYg==");
    assert!(matches!(result, Err(Base64Error::InvalidCharacter(_, 0))));

    let result = decode(&config, b"Z!9vYg==");
    assert!(matches!(result, Err(Base64Error::InvalidCharacter(_, 1))));

    let result = decode(&config, b"Zm!vYg==");
    assert!(matches!(result, Err(Base64Error::InvalidCharacter(_, 2))));
}
