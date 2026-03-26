use basekit::base32::{Base32DecodeConfig, Base32Error, DECODE_TABLE_BASE32_HEX, decode};

fn create_config() -> Base32DecodeConfig {
    Base32DecodeConfig::new(DECODE_TABLE_BASE32_HEX, b'=')
}

#[test]
fn test_empty() {
    let config = create_config();
    let result = decode(&config, b"");
    assert_eq!(Vec::<u8>::from(result.unwrap()), b"");
}

#[test]
fn test_invalid_character() {
    let config = create_config();
    let result = decode(&config, b"0000!");
    assert!(matches!(result, Err(Base32Error::InvalidCharacter(_, _))));
}

#[test]
fn test_invalid_character_high_byte() {
    let config = create_config();
    let result = decode(&config, b"00\xFF00==");
    assert!(matches!(
        result,
        Err(Base32Error::InvalidCharacter(0xFF, _))
    ));
}

#[test]
fn test_invalid_padding_position() {
    let config = create_config();
    let result = decode(&config, b"=0000==");
    assert!(matches!(result, Err(Base32Error::InvalidPadding)));
}

#[test]
fn test_invalid_character_at_different_positions() {
    let config = create_config();

    let result = decode(&config, b"!00000");
    assert!(matches!(result, Err(Base32Error::InvalidCharacter(_, 0))));

    let result = decode(&config, b"0!0000");
    assert!(matches!(result, Err(Base32Error::InvalidCharacter(_, 1))));

    let result = decode(&config, b"00!000");
    assert!(matches!(result, Err(Base32Error::InvalidCharacter(_, 2))));
}

#[test]
fn test_round_trip_hello() {
    use basekit::base32::{ALPHABET_BASE32_HEX, Base32EncodeConfig, encode};
    let encode_config = Base32EncodeConfig::new(ALPHABET_BASE32_HEX, b'=');
    let decode_config = create_config();

    let original = b"Hello, World!";
    let encoded = encode(&encode_config, original);
    let decoded = Vec::<u8>::from(decode(&decode_config, &Vec::<u8>::from(encoded)).unwrap());
    assert_eq!(decoded, original);
}

#[test]
fn test_round_trip_random() {
    use basekit::base32::{ALPHABET_BASE32_HEX, Base32EncodeConfig, encode};
    let encode_config = Base32EncodeConfig::new(ALPHABET_BASE32_HEX, b'=');
    let decode_config = create_config();

    let data: Vec<u8> = (0..1000).map(|i| (i * 17 % 256) as u8).collect();
    let encoded = encode(&encode_config, &data);
    let decoded = Vec::<u8>::from(decode(&decode_config, &Vec::<u8>::from(encoded)).unwrap());
    assert_eq!(decoded, data);
}

#[test]
fn test_base32hex_encodes_zeros_differently() {
    use basekit::base32::{ALPHABET_BASE32_HEX, Base32EncodeConfig, encode};
    let encode_config = Base32EncodeConfig::new(ALPHABET_BASE32_HEX, b'=');

    let result = encode(&encode_config, &[0]);
    assert_eq!(Vec::<u8>::from(result), b"00======");
}

#[test]
fn test_base32hex_decodes_zeros_correctly() {
    let config = create_config();
    let result = decode(&config, b"00======").unwrap();
    assert_eq!(Vec::<u8>::from(result), &[0u8]);
}

#[test]
fn test_round_trip_foo_bar() {
    use basekit::base32::{ALPHABET_BASE32_HEX, Base32EncodeConfig, encode};
    let encode_config = Base32EncodeConfig::new(ALPHABET_BASE32_HEX, b'=');
    let decode_config = create_config();

    let original = b"foo bar";
    let encoded = encode(&encode_config, original);
    let decoded = Vec::<u8>::from(decode(&decode_config, &Vec::<u8>::from(encoded)).unwrap());
    assert_eq!(decoded, original);
}

#[test]
fn test_round_trip_five_bytes() {
    use basekit::base32::{ALPHABET_BASE32_HEX, Base32EncodeConfig, encode};
    let encode_config = Base32EncodeConfig::new(ALPHABET_BASE32_HEX, b'=');
    let decode_config = create_config();

    let original = &[102u8, 111, 111, 98, 97];
    let encoded = encode(&encode_config, original);
    let decoded = Vec::<u8>::from(decode(&decode_config, &Vec::<u8>::from(encoded)).unwrap());
    assert_eq!(decoded, original);
}

#[test]
fn test_round_trip_four_bytes() {
    use basekit::base32::{ALPHABET_BASE32_HEX, Base32EncodeConfig, encode};
    let encode_config = Base32EncodeConfig::new(ALPHABET_BASE32_HEX, b'=');
    let decode_config = create_config();

    let original = &[102u8, 111, 111, 98];
    let encoded = encode(&encode_config, original);
    let decoded = Vec::<u8>::from(decode(&decode_config, &Vec::<u8>::from(encoded)).unwrap());
    assert_eq!(decoded, original);
}

#[test]
fn test_round_trip_three_bytes() {
    use basekit::base32::{ALPHABET_BASE32_HEX, Base32EncodeConfig, encode};
    let encode_config = Base32EncodeConfig::new(ALPHABET_BASE32_HEX, b'=');
    let decode_config = create_config();

    let original = &[102u8, 111, 111];
    let encoded = encode(&encode_config, original);
    let decoded = Vec::<u8>::from(decode(&decode_config, &Vec::<u8>::from(encoded)).unwrap());
    assert_eq!(decoded, original);
}

#[test]
fn test_round_trip_two_bytes() {
    use basekit::base32::{ALPHABET_BASE32_HEX, Base32EncodeConfig, encode};
    let encode_config = Base32EncodeConfig::new(ALPHABET_BASE32_HEX, b'=');
    let decode_config = create_config();

    let original = &[102u8, 111];
    let encoded = encode(&encode_config, original);
    let decoded = Vec::<u8>::from(decode(&decode_config, &Vec::<u8>::from(encoded)).unwrap());
    assert_eq!(decoded, original);
}

#[test]
fn test_round_trip_single_byte() {
    use basekit::base32::{ALPHABET_BASE32_HEX, Base32EncodeConfig, encode};
    let encode_config = Base32EncodeConfig::new(ALPHABET_BASE32_HEX, b'=');
    let decode_config = create_config();

    let original = &[102u8];
    let encoded = encode(&encode_config, original);
    let decoded = Vec::<u8>::from(decode(&decode_config, &Vec::<u8>::from(encoded)).unwrap());
    assert_eq!(decoded, original);
}

#[test]
fn test_round_trip_all_zeros() {
    use basekit::base32::{ALPHABET_BASE32_HEX, Base32EncodeConfig, encode};
    let encode_config = Base32EncodeConfig::new(ALPHABET_BASE32_HEX, b'=');
    let decode_config = create_config();

    let original = &[0u8, 0, 0, 0, 0];
    let encoded = encode(&encode_config, original);
    let decoded = Vec::<u8>::from(decode(&decode_config, &Vec::<u8>::from(encoded)).unwrap());
    assert_eq!(decoded, original);
}
