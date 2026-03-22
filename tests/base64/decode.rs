use basekit::base64::{ALPHABET_BASE64, Base64Config, Base64Error, decode_v1};

fn create_config() -> Base64Config {
    Base64Config::new(ALPHABET_BASE64, b'=')
}

#[test]
fn test_empty() {
    let config = create_config();
    let result = decode_v1(&config, b"");
    assert_eq!(result.unwrap(), b"");
}

#[test]
fn test_single_byte() {
    let config = create_config();
    let result = decode_v1(&config, b"Zg==");
    assert_eq!(result.unwrap(), &[102]);
}

#[test]
fn test_two_bytes() {
    let config = create_config();
    let result = decode_v1(&config, b"Zm8=");
    assert_eq!(result.unwrap(), &[102, 111]);
}

#[test]
fn test_three_bytes() {
    let config = create_config();
    let result = decode_v1(&config, b"Zm9v");
    assert_eq!(result.unwrap(), &[102, 111, 111]);
}

#[test]
fn test_four_bytes() {
    let config = create_config();
    let result = decode_v1(&config, b"Zm9vYg==");
    assert_eq!(result.unwrap(), &[102, 111, 111, 98]);
}

#[test]
fn test_five_bytes() {
    let config = create_config();
    let result = decode_v1(&config, b"Zm9vYmE=");
    assert_eq!(result.unwrap(), &[102, 111, 111, 98, 97]);
}

#[test]
fn test_six_bytes() {
    let config = create_config();
    let result = decode_v1(&config, b"Zm9vYmFy");
    assert_eq!(result.unwrap(), &[102, 111, 111, 98, 97, 114]);
}

#[test]
fn test_hello() {
    let config = create_config();
    let result = decode_v1(&config, b"SGVsbG8=");
    assert_eq!(result.unwrap(), b"Hello");
}

#[test]
fn test_all_zeros() {
    let config = create_config();
    let result = decode_v1(&config, b"AAAA");
    assert_eq!(result.unwrap(), &[0, 0, 0]);
}

#[test]
fn test_all_ones() {
    let config = create_config();
    let result = decode_v1(&config, b"////");
    assert_eq!(result.unwrap(), &[0xFF, 0xFF, 0xFF]);
}

#[test]
fn test_roundtrip() {
    let config = create_config();
    let original = b"Hello, World!";
    let encoded = basekit::base64::encode_v1(&config, original);
    let decoded = decode_v1(&config, &encoded).unwrap();
    assert_eq!(decoded, original);
}

#[test]
fn test_roundtrip_large() {
    let config = create_config();
    let original = b"The quick brown fox jumps over the lazy dog";
    let encoded = basekit::base64::encode_v1(&config, original);
    let decoded = decode_v1(&config, &encoded).unwrap();
    assert_eq!(decoded, original);
}

#[test]
fn test_invalid_character() {
    let config = create_config();
    let result = decode_v1(&config, b"Zm9v!");
    assert!(matches!(result, Err(Base64Error::InvalidCharacter(_, _))));
}

#[test]
fn test_invalid_padding_position() {
    let config = create_config();
    let result = decode_v1(&config, b"=Zm9v");
    assert!(matches!(result, Err(Base64Error::InvalidPadding)));
}

#[test]
fn test_too_much_padding() {
    let config = create_config();
    let result = decode_v1(&config, b"Zm9v===");
    assert!(matches!(result, Err(Base64Error::InvalidPadding)));
}

#[test]
fn test_roundtrip_1kb() {
    let config = create_config();
    let original: Vec<u8> = (0..1024).map(|i| (i % 256) as u8).collect();
    let encoded = basekit::base64::encode_v1(&config, &original);
    let decoded = decode_v1(&config, &encoded).unwrap();
    assert_eq!(decoded, original);
}
