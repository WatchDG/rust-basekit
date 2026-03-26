use basekit::base32::{ALPHABET_BASE32, Base32EncodeConfig, encode};

fn create_config() -> Base32EncodeConfig {
    Base32EncodeConfig::new(ALPHABET_BASE32, b'=')
}

#[test]
fn test_empty() {
    let config = create_config();
    let result = encode(&config, &[]);
    assert_eq!(Vec::<u8>::from(result), b"");
}

#[test]
fn test_single_byte() {
    let config = create_config();
    let result = encode(&config, &[102]);
    assert_eq!(Vec::<u8>::from(result), b"MY======");
}

#[test]
fn test_two_bytes() {
    let config = create_config();
    let result = encode(&config, &[102, 111]);
    assert_eq!(Vec::<u8>::from(result), b"MZXQ====");
}

#[test]
fn test_three_bytes() {
    let config = create_config();
    let result = encode(&config, &[102, 111, 111]);
    assert_eq!(Vec::<u8>::from(result), b"MZXW6===");
}

#[test]
fn test_four_bytes() {
    let config = create_config();
    let result = encode(&config, &[102, 111, 111, 98]);
    assert_eq!(Vec::<u8>::from(result), b"MZXW6YQ=");
}

#[test]
fn test_five_bytes() {
    let config = create_config();
    let result = encode(&config, &[102, 111, 111, 98, 97]);
    assert_eq!(Vec::<u8>::from(result), b"MZXW6YTB");
}

#[test]
fn test_foo_bar() {
    let config = create_config();
    let result = encode(&config, b"foo bar");
    assert_eq!(Vec::<u8>::from(result), b"MZXW6IDCMFZA====");
}

#[test]
fn test_all_zeros() {
    let config = create_config();
    let result = encode(&config, &[0, 0, 0, 0, 0]);
    assert_eq!(Vec::<u8>::from(result), b"AAAAAAAA");
}

#[test]
fn test_all_ones() {
    let config = create_config();
    let result = encode(&config, &[0xFF, 0xFF, 0xFF, 0xFF, 0xFF]);
    assert_eq!(Vec::<u8>::from(result), b"77777777");
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
fn test_empty_to_string() {
    let config = create_config();
    let result = encode(&config, &[]);
    assert_eq!(String::from(result), "");
}

#[test]
fn test_single_byte_to_string() {
    let config = create_config();
    let result = encode(&config, &[102]);
    assert_eq!(String::from(result), "MY======");
}

#[test]
fn test_hello_to_string() {
    let config = create_config();
    let result = encode(&config, b"Hello");
    assert_eq!(String::from(result), "JBSWY3DP");
}

#[test]
fn test_world_to_string() {
    let config = create_config();
    let result = encode(&config, b"World");
    assert_eq!(String::from(result), "K5XXE3DE");
}
