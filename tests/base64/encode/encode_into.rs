use basekit::base64::{ALPHABET_BASE64, Base64EncodeConfig, encode, encode_into};

fn create_config() -> Base64EncodeConfig {
    Base64EncodeConfig::new(ALPHABET_BASE64, b'=')
}

#[test]
fn test_empty() {
    let config = create_config();
    let mut dst = vec![0u8; 100];
    let len = encode_into(&config, &mut dst, &[]).unwrap();
    assert_eq!(len, 0);
    assert_eq!(&dst[..len], b"");
}

#[test]
fn test_single_byte() {
    let config = create_config();
    let mut dst = vec![0u8; 100];
    let len = encode_into(&config, &mut dst, &[102]).unwrap();
    assert_eq!(len, 4);
    assert_eq!(&dst[..len], b"Zg==");
}

#[test]
fn test_two_bytes() {
    let config = create_config();
    let mut dst = vec![0u8; 100];
    let len = encode_into(&config, &mut dst, &[102, 111]).unwrap();
    assert_eq!(len, 4);
    assert_eq!(&dst[..len], b"Zm8=");
}

#[test]
fn test_three_bytes() {
    let config = create_config();
    let mut dst = vec![0u8; 100];
    let len = encode_into(&config, &mut dst, &[102, 111, 111]).unwrap();
    assert_eq!(len, 4);
    assert_eq!(&dst[..len], b"Zm9v");
}

#[test]
fn test_four_bytes() {
    let config = create_config();
    let mut dst = vec![0u8; 100];
    let len = encode_into(&config, &mut dst, &[102, 111, 111, 98]).unwrap();
    assert_eq!(len, 8);
    assert_eq!(&dst[..len], b"Zm9vYg==");
}

#[test]
fn test_five_bytes() {
    let config = create_config();
    let mut dst = vec![0u8; 100];
    let len = encode_into(&config, &mut dst, &[102, 111, 111, 98, 97]).unwrap();
    assert_eq!(len, 8);
    assert_eq!(&dst[..len], b"Zm9vYmE=");
}

#[test]
fn test_six_bytes() {
    let config = create_config();
    let mut dst = vec![0u8; 100];
    let len = encode_into(&config, &mut dst, &[102, 111, 111, 98, 97, 114]).unwrap();
    assert_eq!(len, 8);
    assert_eq!(&dst[..len], b"Zm9vYmFy");
}

#[test]
fn test_hello() {
    let config = create_config();
    let mut dst = vec![0u8; 100];
    let len = encode_into(&config, &mut dst, b"Hello").unwrap();
    assert_eq!(len, 8);
    assert_eq!(&dst[..len], b"SGVsbG8=");
}

#[test]
fn test_all_zeros() {
    let config = create_config();
    let mut dst = vec![0u8; 100];
    let len = encode_into(&config, &mut dst, &[0, 0, 0]).unwrap();
    assert_eq!(len, 4);
    assert_eq!(&dst[..len], b"AAAA");
}

#[test]
fn test_all_ones() {
    let config = create_config();
    let mut dst = vec![0u8; 100];
    let len = encode_into(&config, &mut dst, &[0xFF, 0xFF, 0xFF]).unwrap();
    assert_eq!(len, 4);
    assert_eq!(&dst[..len], b"////");
}

#[test]
fn test_large_random() {
    let config = create_config();
    let data: Vec<u8> = (0..1024).map(|i| (i % 256) as u8).collect();
    let mut dst = vec![0u8; data.len() / 3 * 4 + 10];
    encode_into(&config, &mut dst, &data).unwrap();
}

#[test]
fn test_1mb_random() {
    let config = create_config();
    let data: Vec<u8> = (0..1024 * 1024).map(|i| (i % 256) as u8).collect();
    let mut dst = vec![0u8; data.len() / 3 * 4 + 10];
    encode_into(&config, &mut dst, &data).unwrap();
}

#[test]
fn test_exact_buffer_size() {
    let config = create_config();
    let data = b"Hello";
    let output_len = (data.len() / 3 + 1) * 4;
    let mut dst = vec![0u8; output_len];
    let len = encode_into(&config, &mut dst, data).unwrap();
    assert_eq!(len, output_len);
    assert_eq!(&dst[..len], b"SGVsbG8=");
}

#[test]
fn test_buffer_too_small_returns_error() {
    use basekit::base64::Base64Error;
    let config = create_config();
    let data = b"Hello";
    let mut dst = vec![0u8; 4];
    let result = encode_into(&config, &mut dst, data);
    assert!(result.is_err());
    assert!(matches!(
        result.unwrap_err(),
        Base64Error::DestinationBufferTooSmall { needed, provided }
        if needed == 8 && provided == 4
    ));
}

#[test]
fn test_consistency_with_encode() {
    let config = create_config();
    let data = b"Hello, World! The quick brown fox jumps over the lazy dog.";

    let result = encode(&config, data);

    let mut dst = vec![0u8; data.len() / 3 * 4 + 10];
    let len = encode_into(&config, &mut dst, data).unwrap();

    assert_eq!(len, result.len());
    assert_eq!(&dst[..len], &result[..]);
}
