use basekit::base64::{ALPHABET_BASE64, Base64EncodeConfig, encode};

fn create_config() -> Base64EncodeConfig {
    Base64EncodeConfig::new(ALPHABET_BASE64, b'=')
}

#[test]
fn test_empty() {
    let config = create_config();
    let result = encode(&config, &[]);
    assert_eq!(result, b"");
}

#[test]
fn test_single_byte() {
    let config = create_config();
    let result = encode(&config, &[102]);
    assert_eq!(result, b"Zg==");
}

#[test]
fn test_two_bytes() {
    let config = create_config();
    let result = encode(&config, &[102, 111]);
    assert_eq!(result, b"Zm8=");
}

#[test]
fn test_three_bytes() {
    let config = create_config();
    let result = encode(&config, &[102, 111, 111]);
    assert_eq!(result, b"Zm9v");
}

#[test]
fn test_four_bytes() {
    let config = create_config();
    let result = encode(&config, &[102, 111, 111, 98]);
    assert_eq!(result, b"Zm9vYg==");
}

#[test]
fn test_five_bytes() {
    let config = create_config();
    let result = encode(&config, &[102, 111, 111, 98, 97]);
    assert_eq!(result, b"Zm9vYmE=");
}

#[test]
fn test_six_bytes() {
    let config = create_config();
    let result = encode(&config, &[102, 111, 111, 98, 97, 114]);
    assert_eq!(result, b"Zm9vYmFy");
}

#[test]
fn test_hello() {
    let config = create_config();
    let result = encode(&config, b"Hello");
    assert_eq!(result, b"SGVsbG8=");
}

#[test]
fn test_all_zeros() {
    let config = create_config();
    let result = encode(&config, &[0, 0, 0]);
    assert_eq!(result, b"AAAA");
}

#[test]
fn test_all_ones() {
    let config = create_config();
    let result = encode(&config, &[0xFF, 0xFF, 0xFF]);
    assert_eq!(result, b"////");
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
