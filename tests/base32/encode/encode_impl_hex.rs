use basekit::base32::{ALPHABET_BASE32_HEX, Base32EncodeConfig, encode};

fn create_config() -> Base32EncodeConfig {
    Base32EncodeConfig::new(ALPHABET_BASE32_HEX, b'=')
}

#[test]
fn test_empty() {
    let config = create_config();
    let result = encode(&config, &[]);
    assert_eq!(Vec::<u8>::from(result), b"");
}

#[test]
fn test_base32hex_encodes_zeros_differently() {
    let config = create_config();
    let result = encode(&config, &[0]);
    assert_eq!(Vec::<u8>::from(result), b"00======");
}

#[test]
fn test_base32hex_encodes_five_zeros() {
    let config = create_config();
    let result = encode(&config, &[0, 0, 0, 0, 0]);
    assert_eq!(Vec::<u8>::from(result), b"00000000");
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

#[test]
fn test_round_trip_hello() {
    use basekit::base32::{Base32DecodeConfig, DECODE_TABLE_BASE32_HEX, decode};
    let encode_config = create_config();
    let decode_config = Base32DecodeConfig::new(DECODE_TABLE_BASE32_HEX, b'=');

    let original = b"Hello, World!";
    let encoded = encode(&encode_config, original);
    let decoded = decode(&decode_config, &Vec::<u8>::from(encoded)).unwrap();
    assert_eq!(decoded, original);
}

#[test]
fn test_round_trip_random() {
    use basekit::base32::{Base32DecodeConfig, DECODE_TABLE_BASE32_HEX, decode};
    let encode_config = create_config();
    let decode_config = Base32DecodeConfig::new(DECODE_TABLE_BASE32_HEX, b'=');

    let data: Vec<u8> = (0..1000).map(|i| (i * 17 % 256) as u8).collect();
    let encoded = encode(&encode_config, &data);
    let decoded = decode(&decode_config, &Vec::<u8>::from(encoded)).unwrap();
    assert_eq!(decoded, data);
}

#[test]
fn test_round_trip_foo_bar() {
    use basekit::base32::{Base32DecodeConfig, DECODE_TABLE_BASE32_HEX, decode};
    let encode_config = create_config();
    let decode_config = Base32DecodeConfig::new(DECODE_TABLE_BASE32_HEX, b'=');

    let original = b"foo bar";
    let encoded = encode(&encode_config, original);
    let decoded = decode(&decode_config, &Vec::<u8>::from(encoded)).unwrap();
    assert_eq!(decoded, original);
}

#[test]
fn test_round_trip_five_bytes() {
    use basekit::base32::{Base32DecodeConfig, DECODE_TABLE_BASE32_HEX, decode};
    let encode_config = create_config();
    let decode_config = Base32DecodeConfig::new(DECODE_TABLE_BASE32_HEX, b'=');

    let original = &[102u8, 111, 111, 98, 97];
    let encoded = encode(&encode_config, original);
    let decoded = decode(&decode_config, &Vec::<u8>::from(encoded)).unwrap();
    assert_eq!(decoded, original);
}

#[test]
fn test_round_trip_four_bytes() {
    use basekit::base32::{Base32DecodeConfig, DECODE_TABLE_BASE32_HEX, decode};
    let encode_config = create_config();
    let decode_config = Base32DecodeConfig::new(DECODE_TABLE_BASE32_HEX, b'=');

    let original = &[102u8, 111, 111, 98];
    let encoded = encode(&encode_config, original);
    let decoded = decode(&decode_config, &Vec::<u8>::from(encoded)).unwrap();
    assert_eq!(decoded, original);
}

#[test]
fn test_round_trip_three_bytes() {
    use basekit::base32::{Base32DecodeConfig, DECODE_TABLE_BASE32_HEX, decode};
    let encode_config = create_config();
    let decode_config = Base32DecodeConfig::new(DECODE_TABLE_BASE32_HEX, b'=');

    let original = &[102u8, 111, 111];
    let encoded = encode(&encode_config, original);
    let decoded = decode(&decode_config, &Vec::<u8>::from(encoded)).unwrap();
    assert_eq!(decoded, original);
}

#[test]
fn test_round_trip_two_bytes() {
    use basekit::base32::{Base32DecodeConfig, DECODE_TABLE_BASE32_HEX, decode};
    let encode_config = create_config();
    let decode_config = Base32DecodeConfig::new(DECODE_TABLE_BASE32_HEX, b'=');

    let original = &[102u8, 111];
    let encoded = encode(&encode_config, original);
    let decoded = decode(&decode_config, &Vec::<u8>::from(encoded)).unwrap();
    assert_eq!(decoded, original);
}

#[test]
fn test_round_trip_single_byte() {
    use basekit::base32::{Base32DecodeConfig, DECODE_TABLE_BASE32_HEX, decode};
    let encode_config = create_config();
    let decode_config = Base32DecodeConfig::new(DECODE_TABLE_BASE32_HEX, b'=');

    let original = &[102u8];
    let encoded = encode(&encode_config, original);
    let decoded = decode(&decode_config, &Vec::<u8>::from(encoded)).unwrap();
    assert_eq!(decoded, original);
}

#[test]
fn test_round_trip_all_zeros() {
    use basekit::base32::{Base32DecodeConfig, DECODE_TABLE_BASE32_HEX, decode};
    let encode_config = create_config();
    let decode_config = Base32DecodeConfig::new(DECODE_TABLE_BASE32_HEX, b'=');

    let original = &[0u8, 0, 0, 0, 0];
    let encoded = encode(&encode_config, original);
    let decoded = decode(&decode_config, &Vec::<u8>::from(encoded)).unwrap();
    assert_eq!(decoded, original);
}
