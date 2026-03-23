use basekit::base32::{Base32DecodeConfig, Base32Error, DECODE_TABLE_BASE32, decode};

fn create_config() -> Base32DecodeConfig {
    Base32DecodeConfig::new(DECODE_TABLE_BASE32, b'=')
}

#[test]
fn test_empty() {
    let config = create_config();
    let result = decode(&config, b"");
    assert_eq!(result.unwrap(), b"");
}

#[test]
fn test_single_byte() {
    let config = create_config();
    let result = decode(&config, b"MY======");
    assert_eq!(result.unwrap(), &[102u8]);
}

#[test]
fn test_two_bytes() {
    let config = create_config();
    let result = decode(&config, b"MZXQ====");
    assert_eq!(result.unwrap(), &[102, 111]);
}

#[test]
fn test_three_bytes() {
    let config = create_config();
    let result = decode(&config, b"MZXW6===");
    assert_eq!(result.unwrap(), &[102, 111, 111]);
}

#[test]
fn test_four_bytes() {
    let config = create_config();
    let result = decode(&config, b"MZXW6YQ=");
    assert_eq!(result.unwrap(), &[102, 111, 111, 98]);
}

#[test]
fn test_five_bytes() {
    let config = create_config();
    let result = decode(&config, b"MZXW6YTB");
    assert_eq!(result.unwrap(), &[102, 111, 111, 98, 97]);
}

#[test]
fn test_foo_bar() {
    let config = create_config();
    let result = decode(&config, b"MZXW6IDCMFZA====");
    assert_eq!(result.unwrap(), b"foo bar");
}

#[test]
fn test_all_zeros() {
    let config = create_config();
    let result = decode(&config, b"AAAAAAAA");
    assert_eq!(result.unwrap(), &[0, 0, 0, 0, 0]);
}

#[test]
fn test_all_ones() {
    let config = create_config();
    let result = decode(&config, b"77777777");
    assert_eq!(result.unwrap(), &[0xFF, 0xFF, 0xFF, 0xFF, 0xFF]);
}

#[test]
fn test_invalid_character() {
    let config = create_config();
    let result = decode(&config, b"MZXW!");
    assert!(matches!(result, Err(Base32Error::InvalidCharacter(_, _))));
}

#[test]
fn test_invalid_character_high_byte() {
    let config = create_config();
    let result = decode(&config, b"MZ\xFFW6===");
    assert!(matches!(
        result,
        Err(Base32Error::InvalidCharacter(0xFF, _))
    ));
}

#[test]
fn test_invalid_padding_position() {
    let config = create_config();
    let result = decode(&config, b"=MZXW6===");
    assert!(matches!(result, Err(Base32Error::InvalidPadding)));
}

#[test]
fn test_too_much_padding() {
    let config = create_config();
    let result = decode(&config, b"MZXW6YTB======");
    assert!(matches!(result, Err(Base32Error::InvalidPadding)));
}

#[test]
fn test_invalid_character_at_different_positions() {
    let config = create_config();

    let result = decode(&config, b"!ZXW6YTB");
    assert!(matches!(result, Err(Base32Error::InvalidCharacter(_, 0))));

    let result = decode(&config, b"M!XW6YTB");
    assert!(matches!(result, Err(Base32Error::InvalidCharacter(_, 1))));

    let result = decode(&config, b"MZ!W6YTB");
    assert!(matches!(result, Err(Base32Error::InvalidCharacter(_, 2))));
}

#[test]
fn test_round_trip_hello() {
    use basekit::base32::{ALPHABET_BASE32, Base32EncodeConfig, encode};
    let encode_config = Base32EncodeConfig::new(ALPHABET_BASE32, b'=');
    let decode_config = create_config();

    let original = b"Hello, World!";
    let encoded = encode(&encode_config, original);
    let decoded = decode(&decode_config, &encoded).unwrap();
    assert_eq!(decoded, original);
}

#[test]
fn test_round_trip_random() {
    use basekit::base32::{ALPHABET_BASE32, Base32EncodeConfig, encode};
    let encode_config = Base32EncodeConfig::new(ALPHABET_BASE32, b'=');
    let decode_config = create_config();

    let data: Vec<u8> = (0..1000).map(|i| (i * 17 % 256) as u8).collect();
    let encoded = encode(&encode_config, &data);
    let decoded = decode(&decode_config, &encoded).unwrap();
    assert_eq!(decoded, data);
}
